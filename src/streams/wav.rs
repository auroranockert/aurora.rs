use std::hashmap;

use align;
use types;
use result::{Result, Ok, Error};

use buffers::buffer::Buffer;
use buffers::memory::MemoryBuffer;
use events::event;
use events::event::{Event, EventQueue, EventGenerator};
use parsers::wav;
use samples::sample::{Sample, SampleQueue};
use sources::source;
use sources::wav::WAVSource;
use streams::stream::{Stream, StreamDescriptor};

pub struct WAVStream {
    shutdown:bool,
    current_position:u64,
    discontinuity: bool,
    end_of_stream:bool,

    descriptor: @mut StreamDescriptor,

    source:@mut WAVSource,
    event_queue:EventQueue,
    sample_queue:SampleQueue
}

impl WAVStream {
    pub fn new(source:@mut WAVSource) -> (Result<uint>, Option<@mut WAVStream>) {
        let parser = match source.parser {
            Some(parser) => parser,
            None => return (Error(0), None)
        };

        let (format, ex) = match parser.format {
            wav::Ex(format) => (format, None),
            wav::Extensible(format, ex) => (format, Some(ex)),
            _ => return (Error(0), None)
        };

        let audio_format = types::AudioFormat {
            sample_rate: format.samples_per_second as uint, channels: format.channels as uint
        };

        let sample_type = match ex {
            Some(ex) => {
                let sf = ex.sub_format;

                match (sf[1] as u16 << 8) | (sf[0] as u16) { // TODO: Maybe not throw away the rest of the GUID?
                    wav::WAVE_FORMAT_PCM => types::Signed(format.bits_per_sample as uint),
                    wav::WAVE_FORMAT_IEEE_FLOAT => types::Float(format.bits_per_sample as uint),
                    wav::WAVE_FORMAT_ALAW => {
                        if format.bits_per_sample != 8 {
                            return (Error(1), None)
                        }

                        types::ALaw
                    }
                    wav::WAVE_FORMAT_MULAW => {
                        if format.bits_per_sample != 8 {
                            return (Error(1), None)
                        }

                        types::MuLaw
                    }
                    _ => return (Error(5), None)
                }
            }
            None => {
                match format.format_tag {
                    wav::WAVE_FORMAT_PCM => types::Signed(format.bits_per_sample as uint),
                    wav::WAVE_FORMAT_IEEE_FLOAT => types::Float(format.bits_per_sample as uint), // TODO: Is this valid? Are other types valid here?
                    wav::WAVE_FORMAT_EXTENSIBLE | _ => return (Error(2), None)
                }
            }
        };

        let pcm_format = types::PCMFormat {
            sample_type: sample_type, endian: types::LittleEndian
        };

        let sd = StreamDescriptor::new(true, 0, types::AudioStream(types::PCMStream(pcm_format), audio_format));

        return (Ok, Some(@mut WAVStream {
            shutdown: false,
            current_position: 0,
            discontinuity: false,
            end_of_stream: false,

            descriptor: sd,

            source: source,
            event_queue: EventQueue::new(),
            sample_queue: SampleQueue::new()
        }));
    }

    fn check_shutdown(&self) -> Result<uint> {
        if self.shutdown { Error(0) } else { Ok } // TODO: Magic number if there ever was one
    }

    fn check_end_of_stream(&mut self) -> Result<uint> {
        let parser = match self.source.parser {
            Some(parser) => parser,
            None => return Error(0)
        };

        let format = match parser.format {
            wav::Ex(format) | wav::Extensible(format, _) => format,
            _ => return Error(0)
        };

        if parser.riff.bytes_remaining < (format.block_align as u64) {
            // The remaining data is smaller than the audio block size. (In theory there shouldn't be
            // partial bits of data at the end, so we should reach an even zero bytes, but the file
            // might not be authored correctly.)
            self.end_of_stream = true;

            return self.enqueue_event(Event::new(event::EndOfStream, Ok, hashmap::HashMap::new()));
        }

        return Ok;
    }

    fn create_audio_sample(&mut self) -> (Result<uint>, Option<Sample>) {
        let format = match self.source.parser {
            Some(parser) => match parser.format {
                wav::Ex(format) | wav::Extensible(format, _) => format,
                _ => return (Error(0), None)
            },
            None => return (Error(0), None)
        };

        let buffer_size = align::block_align(format.average_bytes_per_second as u64, format.block_align as u64);
        let buffer_size = buffer_size.min(&match self.source.parser {
            Some(parser) => parser.riff.bytes_remaining,
            None => return (Error(0), None)
        });

        let buffer = MemoryBuffer::new(buffer_size as uint); // TODO: Check for overflow

        let result = do buffer.map() |data| { match self.source.parser {
            Some(parser) => match parser.riff.read_data_from_chunk(buffer_size, data) { (result, _) => result }, // TODO: Maybe not ignore length..?
            None => Error(0)
        }};

        match result {
            Ok => (),
            err => return (err, None)
        }

        let mut sample = Sample::new(self.descriptor.stream_type);

        sample.add_buffer(buffer as @Buffer);
        // sample.set_time(self.current_position);
        // let duration = ???
        // sample.set_duration(duration); self.current_position += duration;
        // sample.attributes.set("discontinuity", self.discontinuity);

        return (Ok, Some(sample));
    }

    fn deliver_sample(&mut self, sample:Sample) -> Result<uint> {
        match self.enqueue_event(Event::new(event::Sample(sample), Ok, hashmap::HashMap::new())) {
            Ok => (),
            err => return err
        }

        return self.check_end_of_stream();
    }
}

impl EventGenerator for WAVStream {
    pub fn dequeue_event(&mut self) -> (Result<uint>, Option<Event>) {
        self.event_queue.dequeue_event()
    }

    pub fn enqueue_event(&mut self, event:Event) -> Result<uint> {
        self.event_queue.enqueue_event(event)
    }
    
}

impl Stream for WAVStream {
    pub fn descriptor(&mut self) -> (Result<uint>, Option<@mut StreamDescriptor>) {
        return match self.check_shutdown() {
            Ok => (Ok, Some(self.descriptor)),
            err => (err, None)
        };
    }

    pub fn request_sample(&mut self) -> Result<uint> {
        match self.check_shutdown() {
            Ok => (),
            err => return err
        }

        if self.end_of_stream {
            return Error(1); // TODO: Magic number if there ever was one
        }

        let state = self.source.state;

        // if state == source::Stopped
        //     return Error(2); // TODO: Magic number if there ever was one
        // }

        let sample = match self.create_audio_sample() {
            (Ok, Some(sample)) => sample,
            (err, _) => return err
        };

        return if state == source::Paused {
            self.sample_queue.enqueue_sample(sample); Ok
        } else {
            self.deliver_sample(sample)
        };
    }
}