use std::uint;
use std::option;

use io::read::Read;
use io::seek::Seek;

use result::{Ok, Error, Result};
use parsers::riff::RIFFParser;

use io::read::ReadCore;

pub static WAVE_FORMAT_PCM:u16          = 0x0001;
pub static WAVE_FORMAT_IEEE_FLOAT:u16   = 0x0003;
pub static WAVE_FORMAT_ALAW:u16         = 0x0006;
pub static WAVE_FORMAT_MULAW:u16        = 0x0007;
pub static WAVE_FORMAT_EXTENSIBLE:u16   = 0xFFFE;

pub struct WaveFormat {
    format_tag:u16,
    channels:u16,
    samples_per_second:u32,
    average_bytes_per_second:u32,
    block_align:u16,
    bits_per_sample:u16,
    size:u16
}

pub struct WaveFormatExtensible {
    samples:u16,
    channel_mask:u32,
    sub_format:[u8, ..16] // TODO: Should be a GUID
}

pub enum Format {
    None,
    Ex(WaveFormat),
    Extensible(WaveFormat, WaveFormatExtensible)
}

pub struct WAVParser {
    riff:RIFFParser,

    format:Format,

    duration: u64
}

impl WAVParser {
    pub fn new(reader:@Read, seeker:@Seek) -> (Result<uint>, Option<WAVParser>) {
        let status = RIFFParser::new(reader, seeker, fcc!("RIFF"), 0);

        let parser = match status {
            (Ok, Some(riff)) => WAVParser {
                riff: riff,
                format: None,
                duration: 0
            },
            (err, _) => return (err, option::None)
        };

        if parser.riff.riff_type != fcc!("WAVE") {
            return (Error(1), option::None); // TODO: Magic number
        }

        return (Ok, Some(parser));
    }

    pub fn parse_wave_header(&mut self) -> Result<uint> {
        let mut result = Ok;

        while result == Ok {
            let fourcc = self.riff.current_chunk.fourcc;

            if fcc!("fmt ") == fourcc {
                result = self.read_format_block()
            } else if fcc!("data") == fourcc {
                break
            }

            if result == Ok {
                result = self.riff.move_to_next_chunk();
            }
        }

        // self.duration = ?

        return result;
    }

    fn read_format_block(&mut self) -> Result<uint> {
        // TODO: self.riff.current_chunk.fourcc() == fcc!("fmt ")
        match self.format {
            None => (),
            _ => fail!("Already parsed format block!")
        }

        let format_tag = self.riff.reader.read_u16_le();
        
        let min_format_size = match format_tag {
            WAVE_FORMAT_EXTENSIBLE => 40, _ => 16
        };

        // Some .wav files do not include the size field of the WAVEFormatEx
        // structure. For uncompressed PCM audio, field is always zero.
        let format_size = self.riff.current_chunk.size as u64;

        if format_size < min_format_size {
            return Error(1);
        }

        // We store a WAVEFORMATEX structure, so our format block must be at
        // least sizeof(WAVEFORMATEX) even if the format block in the file
        // is smaller. See note above about cbMinFormatSize.
        let read_size = (format_size > 17);

        let wave_format_ex = WaveFormat {
            format_tag: format_tag,
            channels: self.riff.reader.read_u16_le(),
            samples_per_second: self.riff.reader.read_u32_le(),
            average_bytes_per_second: self.riff.reader.read_u32_le(),
            block_align: self.riff.reader.read_u16_le(),
            bits_per_sample: self.riff.reader.read_u16_le(),
            size: if read_size { self.riff.reader.read_u16_le() } else { 0 }
        };

        self.format = match format_tag {
            WAVE_FORMAT_EXTENSIBLE => {
                let samples = self.riff.reader.read_u16_le();
                let channel_mask = self.riff.reader.read_u32_le();
                let mut sub_format = [0u8, ..16];

                for uint::range(0, 16) |i| { sub_format[i] = self.riff.reader.read_u8_be(); }

                let wave_format_extensible = WaveFormatExtensible {
                    samples: samples,
                    channel_mask: channel_mask,
                    sub_format: sub_format
                };

                Extensible(wave_format_ex, wave_format_extensible)
            }
            _ => {
                Ex(wave_format_ex)
            }
        };

        self.riff.bytes_remaining -= format_size as u64;

        return Ok
    }
}