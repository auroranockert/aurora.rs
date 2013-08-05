use std::uint;

use result::{Ok, Error, Result};

use types;
use types::StreamType;

use events::event;
use events::event::{Event, EventQueue, EventGenerator};
use samples::sample::{Sample, SampleQueue};
use sinks::sink::{Sink, StreamSink, SinkCharacteristics};

use io::seek::Seek;
use io::write::{Write, WriteCore};

struct AuSink {
    stream: Option<@mut AuStreamSink>,
    shutdown: bool
}

struct AuStreamSink {
    sink: @mut AuSink,
    writer: @Write, seeker: Option<@Seek>,
    bytes_written: u64,

    stream_type: StreamType,

    event_queue: EventQueue,
    sample_queue: SampleQueue,

    shutdown: bool
}

impl AuSink {
    pub fn new(writer:@Write, seeker:Option<@Seek>) -> (Result<uint>, Option<@mut AuSink>) {
        let result = @mut AuSink {
            stream: None,
            shutdown: false
        };

        let status = AuStreamSink::new(result, writer, seeker);

        return match status {
            (Ok, Some(stream_sink)) => {
                result.stream = Some(stream_sink); (Ok, Some(result))
            }
            (err, _) => {
                (err, None)
            }
        };
    }

    fn check_shutdown(&self) -> Result<uint> {
        if self.shutdown { Error(0) } else { Ok } // TODO: Magic number if there ever was one
    }
}

impl Sink for AuSink {
    pub fn characteristics(&self) -> (Result<uint>, Option<SinkCharacteristics>) {
        return (Ok, Some(SinkCharacteristics {
            fixed_streams: true, rateless: true
        }));
    }

    pub fn stream_sink_from_index(&mut self, index:uint) -> (Result<uint>, Option<@mut StreamSink>) {
        if index == 0 {
            match self.stream {
                Some(stream) => return (Ok, Some(stream as @mut StreamSink)),
                None => fail!("Didn't have stream 0, should always be set on a AuSink, did you create it in a weird way?")
            }
        } else {
            return (Error(0), None);
        }
    }

    pub fn finalize(&mut self) -> Result<uint> {
        match self.check_shutdown() {
            Ok => match self.stream {
                Some(stream) => return stream.finalize(),
                None => fail!("Didn't have stream 0, should always be set on a AuSink, did you create it in a weird way?")
            },
            err => return err
        }
    }

    fn shutdown(&mut self) -> Result<uint> {
        self.shutdown = true;

        match self.check_shutdown() {
            Ok => match self.stream {
                Some(stream) => return stream.shutdown(),
                None => fail!("Didn't have stream 0, should always be set on a AuSink, did you create it in a weird way?")
            },
            err => return err
        }
    }
}

impl AuStreamSink {
    pub fn new(sink:@mut AuSink, writer:@Write, seeker:Option<@Seek>) -> (Result<uint>, Option<@mut AuStreamSink>) {
        return (Ok, Some(@mut AuStreamSink {
            sink: sink,
            writer: writer, seeker: seeker,
            bytes_written: 0,

            stream_type: types::BinaryStream,

            event_queue: EventQueue::new(),
            sample_queue: SampleQueue::new(),
            shutdown: false
        }));
    }

    fn shutdown(&mut self) -> Result<uint> {
        return match self.check_shutdown() {
            Ok => {
                self.shutdown = true; Ok
            },
            err => err
        };
    }

    fn check_shutdown(&self) -> Result<uint> {
        if self.shutdown { Error(0) } else { Ok } // TODO: Magic number if there ever was one
    }

    fn process_samples(&mut self) -> Result<uint> {
        loop {
            let result = match self.sample_queue.dequeue_sample() {
                Some(sample) => self.write_sample_to_stream(sample),
                None => return Ok
            };

            match result {
                Ok => (),
                err => return err
            }
        }
    }

    fn write_sample_to_stream(&mut self, sample:Sample) -> Result<uint> {
        for uint::range(0, sample.length()) |i| {
            let result = do sample[i].map() |buffer| {
                self.writer.write(buffer); self.bytes_written += (buffer.len() as u64); Ok
            };

            if result != Ok {
                return result;
            }
        }

        return Ok;
    }

    fn finalize(&mut self) -> Result<uint> {
        match self.check_shutdown() {
            Ok => (),
            err => return err
        }

        match self.process_samples() {
            Ok => (),
            err => return err
        }

        match self.seeker {
            Some(ref mut seeker) => {
                seeker.seek_from_beginning(8);
                self.writer.write_u32_be(self.bytes_written as u32); // TODO: Check for overflow
            }
            None => ()
        }

        return Ok;
    }
}

impl EventGenerator for AuStreamSink {
    pub fn dequeue_event(&mut self) -> (Result<uint>, Option<Event>) {
        return match self.check_shutdown() {
            Ok => self.event_queue.dequeue_event(),
            err => (err, None)
        }
    }

    pub fn enqueue_event(&mut self, event:Event) -> Result<uint> {
        match self.check_shutdown() {
            Ok => (),
            err => return err
        }

        return match event.event_type {
            event::Sample(sample) => {
                self.sample_queue.enqueue_sample(sample); Ok
            }
            _ => self.event_queue.enqueue_event(event)
        };
    }
}

impl StreamSink for AuStreamSink {
    pub fn sink(&self) -> @Sink {
        return self.sink as @Sink
    }

    pub fn set_stream_type(&mut self, stream_type:StreamType) -> Result<uint> {
        match self.stream_type {
            types::BinaryStream => (),
            _ => return Error(0) // TODO: Should not be set twice
        }

        match stream_type {
            types::AudioStream(types::PCMStream(pcm_format), audio_format) => {
                match pcm_format.endian {
                    types::BigEndian => (),
                    types::LittleEndian => fail!("Cannot put little endian stream into .au file")
                };

                let format = match pcm_format.sample_type {
                    types::MuLaw => 1,
                    types::Signed(8) => 2,
                    types::Signed(16) => 3,
                    types::Signed(24) => 4,
                    types::Signed(32) => 5,
                    types::Float(32) => 6,
                    types::Float(64) => 7,
                    types::ALaw => 27,
                    _ => fail!("Incompatible stream type for .au files")
                };

                self.writer.write_fourcc(fcc!(".snd"));
                self.writer.write_u32_be(24);
                self.writer.write_u32_be(0xFFFFFFFF);
                self.writer.write_u32_be(format);
                self.writer.write_u32_be(audio_format.sample_rate as u32);
                self.writer.write_u32_be(audio_format.channels as u32);
            },
            _ => return fail!("Cannot put non-PCM formats into .au file")
        }

        self.stream_type = stream_type;

        return Ok;
    }

    pub fn dequeue_stream_sink_event(&mut self) -> (Result<uint>, Option<Event>) {
        return self.dequeue_event();
    }

    pub fn enqueue_stream_sink_event(&mut self, event:Event) -> Result<uint> {
        return self.enqueue_event(event);
    }
}
