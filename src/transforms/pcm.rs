use std::cast;
use std::uint;

use types;

use result::{Ok, Error, Result};

use buffers::buffer::Buffer;
use buffers::memory::MemoryBuffer;
use events::event::Event;
use samples::sample::Sample;
use transforms::transform;
use transforms::transform::{Transform, TransformStream, Message};

struct PCMTransform {
    input_streams: @[@mut TransformStream],
    output_streams: @[@mut TransformStream],

    streams_added: [bool, ..2],

    sample: Option<Sample>
}

impl PCMTransform {
    pub fn new() -> (Result<uint>, Option<@mut PCMTransform>) {
        let result = @mut PCMTransform {
            input_streams: @[], output_streams: @[], streams_added: [false, false], sample: None
        };

        let t = result as @mut Transform;

        result.input_streams = @[@mut TransformStream {
            identifier: 0, direction: transform::InputStream, transform: t, stream_type: types::BinaryStream
        }];

        result.output_streams = @[@mut TransformStream {
            identifier: 1, direction: transform::OutputStream, transform: t, stream_type: types::BinaryStream
        }];

        return (Ok, Some(result));
    }
}

impl Transform for PCMTransform {
    pub fn input_streams(&mut self) -> @[@mut TransformStream] {
        return self.input_streams;
    }

    pub fn output_streams(&mut self) -> @[@mut TransformStream] {
        return self.output_streams
    }

    pub fn input_stream_limits(&mut self) -> (uint, uint) {
        return (1, 1);
    }

    pub fn output_stream_limits(&mut self) -> (uint, uint) {
        return (1, 1);
    }

    pub fn add_stream(&mut self, stream:&TransformStream) -> Result<uint> {
        match stream.identifier {
            0 => self.streams_added[0] = true,
            1 => self.streams_added[1] = true,
            _ => fail!("Tried to add a non-existant stream, this is a bug!")
        }

        return Ok;
    }

    pub fn process_message(&mut self, message:Message) -> Result<uint> {
        match message {
            transform::Flush => self.sample = None,
            transform::Drain => (), // No-op for this, since we just keep one sample around
            transform::StartOfStream(_) => (), // No-op, since we don't keep state
            transform::EndOfStream(_) => (), // No-op, since we don't keep state
        }
    
        return Ok;
    }

    pub fn process_event(&mut self, _:&TransformStream, _:Event) -> Result<uint> {
        fail!("Not implemented!");
    }

    pub fn process_input(&mut self, stream:&TransformStream, sample:Sample) -> Result<uint> {
        if stream.identifier != 0 {
            fail!("Called on a stream not from this transform!");
        }

        match self.sample {
            None => {
                self.sample = Some(sample); Ok
            }
            Some(_) => Error(10) // TODO: Not accepting samples at this time
        }
    }

    pub fn process_output(&mut self, stream:&TransformStream) -> (Result<uint>, Option<Sample>) {
        if stream.identifier != 1 {
            fail!("Called on a stream not from this transform!");
        }

        let result = match self.sample {
            Some(ref mut sample) => {
                let mut result = Sample::new(self.output_streams[0].stream_type);

                let (input_format, input_pcm_format) = match self.input_streams[0].stream_type {
                    types::AudioStream(types::PCMStream(pcm_format), format) => (format, pcm_format),
                    _ => fail!("Did not set input format correctly, it is not a PCM stream?")
                };

                let (output_format, output_pcm_format) = match self.output_streams[0].stream_type {
                    types::AudioStream(types::PCMStream(pcm_format), format) => (format, pcm_format),
                    _ => fail!("Did not set output format correctly, it is not a PCM stream?")
                };

                if input_format != output_format {
                    fail!("Output format (channel / sample rate) is not the same as input format.");
                }

                if (input_pcm_format.endian != types::LittleEndian) || (output_pcm_format.endian != types::LittleEndian) {
                    fail!("Only supporting little endian for now (and big endian systems are currently fucked)");
                }

                let (input_sample_type, output_sample_type) = (input_pcm_format.sample_type, output_pcm_format.sample_type);

                if (input_sample_type == types::ALaw) || (output_sample_type == types::ALaw) {
                    fail!("A-Law samples are not currently supported");
                }

                if (input_sample_type == types::MuLaw) || (output_sample_type == types::MuLaw) {
                    fail!("Mu-Law samples are not currently supported");
                }

                let input_sample_size = match input_sample_type {
                    types::Float(bits) | types::Signed(bits) | types::Unsigned(bits) => (bits >> 3),
                    _ => fail!("Unsupported input sample type")
                };

                let output_sample_size = match output_sample_type {
                    types::Float(bits) | types::Signed(bits) | types::Unsigned(bits) => (bits >> 3),
                    _ => fail!("Unsupported output sample type")
                };

                /* TODO: This loop needs to be optimized!
                 *  - Any allocations need to be moved out of the hot path.
                 *  - All type checks need to be moved out of the hot path.
                 *  - Should be specialized per type combination, almost all topologies will need this.
                 */
                for uint::range(0, sample.length()) |i| {
                    let intermediate = MemoryBuffer::new((sample[i].get_current_length() / input_sample_size) * 8);
                    let result_buffer = MemoryBuffer::new((sample[i].get_current_length() / input_sample_size) * output_sample_size);

                    match intermediate.map(|inter| {
                        let inter_f64 = unsafe { cast::transmute::<&mut [u8], &mut [f64]>(inter) };

                        match sample[i].map(|src| {
                            match input_sample_type {
                                types::Signed(16) => from_s16(inter_f64, src),
                                types::Float(32) => from_f32(inter_f64, src),
                                types::Float(64) => from_f64(inter_f64, src),
                                _ => fail!("Currently invalid type, only floats allowed!")
                            }; Ok
                        }) {
                            Ok => (),
                            err => fail!(fmt!("Could not map! (%?)", err))
                        }

                        result_buffer.map(|dst| {
                            match output_sample_type {
                                types::Signed(16) => to_s16(dst, inter_f64),
                                types::Float(32) => to_f32(dst, inter_f64),
                                types::Float(64) => to_f64(dst, inter_f64),
                                _ => fail!("Currently only float output is allowed")
                            }; Ok
                        })
                        
                    }) {
                        Ok => (),
                        err => return (err, None)
                    }

                    result.add_buffer(result_buffer as @Buffer);
                }

                result
            }
            None => return (Error(11), None) // TODO: No samples available
        };

        self.sample = None;

        return (Ok, Some(result));
    }
}

fn from_s16(dst:&mut [f64], src:&[u8]) {
    use std::i16;

    let src = unsafe { cast::transmute::<&[u8], &[i16]>(src) };

    for uint::range(0, src.len()) |i| {
        dst[i] = -(src[i] as f64) / (i16::min_value as f64); // TODO: Use some tricks here…
    }
}

fn from_f32(dst:&mut [f64], src:&[u8]) {
    let src = unsafe { cast::transmute::<&[u8], &[f32]>(src) };

    for uint::range(0, src.len()) |i| {
        dst[i] = src[i] as f64;
    }
}

fn from_f64(dst:&mut [f64], src:&[u8]) {
    let src = unsafe { cast::transmute::<&[u8], &[f64]>(src) };

    for uint::range(0, src.len()) |i| {
        dst[i] = src[i] as f64;
    }
}

fn to_s16(dst:&mut [u8], src:&[f64]) {
    use std::i16;

    let dst = unsafe { cast::transmute::<&mut [u8], &mut [i16]>(dst) };

    for uint::range(0, src.len()) |i| {
        dst[i] = ((-src[i]) * (i16::min_value as f64)) as i16;
    }
}

fn to_f32(dst:&mut [u8], src:&[f64]) {
    let dst = unsafe { cast::transmute::<&mut [u8], &mut [f32]>(dst) };

    for uint::range(0, src.len()) |i| {
        dst[i] = src[i] as f32;
    }
}

fn to_f64(dst:&mut [u8], src:&[f64]) {
    let dst = unsafe { cast::transmute::<&mut [u8], &mut [f64]>(dst) };

    for uint::range(0, src.len()) |i| {
        dst[i] = src[i];
    }
}