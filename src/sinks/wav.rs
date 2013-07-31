use std::io;
use std::uint;

use result::{Ok, Error, Result};

use types;
use types::StreamType;

use events::event;
use events::event::{Event, EventQueue, EventGenerator};
use parsers::wav;
use samples::sample::{Sample, SampleQueue};
use sinks::sink::{Sink, StreamSink, SinkCharacteristics};

struct WAVSink {
    stream: Option<@mut WAVStreamSink>,
    shutdown: bool
}

struct WAVStreamSink {
    sink: @mut WAVSink,
    stream: @Writer,
    bytes_written: u64,

    stream_type: StreamType,

    event_queue: EventQueue,
    sample_queue: SampleQueue,

    shutdown: bool
}

impl WAVSink {
    pub fn new(stream:@Writer) -> (Result<uint>, Option<@mut WAVSink>) {
        let result = @mut WAVSink {
            stream: None,
            shutdown: false
        };

        let status = WAVStreamSink::new(result, stream);

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

impl Sink for WAVSink {
    pub fn characteristics(&self) -> (Result<uint>, Option<SinkCharacteristics>) {
        return (Ok, Some(SinkCharacteristics {
            fixed_streams: true, rateless: true
        }));
    }

    pub fn stream_sink_from_index(&mut self, index:uint) -> (Result<uint>, Option<@mut StreamSink>) {
        if index == 0 {
            match self.stream {
                Some(stream) => return (Ok, Some(stream as @mut StreamSink)),
                None => fail!("Didn't have stream 0, should always be set on a WAVSink, did you create it in a weird way?")
            }
        } else {
            return (Error(0), None);
        }
    }

    pub fn finalize(&mut self) -> Result<uint> {
        match self.check_shutdown() {
            Ok => match self.stream {
                Some(stream) => return stream.finalize(),
                None => fail!("Didn't have stream 0, should always be set on a WAVSink, did you create it in a weird way?")
            },
            err => return err
        }
    }

    fn shutdown(&mut self) -> Result<uint> {
        self.shutdown = true;

        match self.check_shutdown() {
            Ok => match self.stream {
                Some(stream) => return stream.shutdown(),
                None => fail!("Didn't have stream 0, should always be set on a WAVSink, did you create it in a weird way?")
            },
            err => return err
        }
    }
}

impl WAVStreamSink {
    pub fn new(sink:@mut WAVSink, stream:@Writer) -> (Result<uint>, Option<@mut WAVStreamSink>) {
        return (Ok, Some(@mut WAVStreamSink {
            sink: sink,
            stream: stream,
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
        for uint::range(0, sample.buffer_count()) |i| {
            let result = do sample.get_buffer(i).map() |buffer| {
                self.stream.write(buffer); self.bytes_written += (buffer.len() as u64); Ok
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

        let file_size = self.bytes_written + 36;

        return match self.stream_type {
            types::AudioStream(types::PCMStream(pcm_format), format) => {
                if pcm_format.endian == types::BigEndian {
                    return Error(0); // TODO: Magic number
                }

                if format.channels > 2 {
                    return Error(0); // TODO: Support Multichannel
                }

                self.stream.seek(0, io::SeekSet);

                self.stream.write_be_u32(fcc!("RIFF"));
                self.stream.write_le_u32(file_size as u32); // TODO: Check for overflow
                self.stream.write_be_u32(fcc!("WAVE"));

                self.stream.write_be_u32(fcc!("fmt "));
                self.stream.write_le_u32(18);

                let (tag, bits, bytes) = match pcm_format.sample_type {
                    types::Unsigned(bits) | types::Signed(bits) => {
                        (wav::WAVE_FORMAT_PCM, bits, (bits + 7) >> 3)
                    }
                    types::Float(bits) => {
                        (wav::WAVE_FORMAT_IEEE_FLOAT, bits, (bits + 7) >> 3)
                    }
                    types::ALaw => {
                        (wav::WAVE_FORMAT_ALAW, 8, 1)
                    }
                    types::MuLaw => {
                        (wav::WAVE_FORMAT_MULAW, 8, 1)
                    }
                };

                if (8 * bytes != bits) || (bits > 16) {
                    if tag != wav::WAVE_FORMAT_IEEE_FLOAT {
                        return Error(0); // TODO: Support WAVE_FORMAT_EXTENSIBLE
                    }
                }

                self.stream.write_le_u16(tag);
                self.stream.write_le_u16(format.channels as u16);
                self.stream.write_le_u32(format.sample_rate as u32);
                self.stream.write_le_u32((bytes * format.channels * format.sample_rate) as u32);
                self.stream.write_le_u16((bytes * format.channels) as u16);
                self.stream.write_le_u16(bits as u16);
                self.stream.write_le_u16(0);

                self.stream.write_be_u32(fcc!("data"));
                self.stream.write_le_u32(self.bytes_written as u32); // TODO: Check for overflow

                self.stream.flush();

                Ok
            },
            _ => Error(0) // TODO: Magic number…
        }
    }
}

impl EventGenerator for WAVStreamSink {
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

impl StreamSink for WAVStreamSink {
    pub fn sink(&self) -> @Sink {
        return self.sink as @Sink
    }

    pub fn set_stream_type(&mut self, stream_type:StreamType) -> Result<uint> {
        match self.stream_type {
            types::BinaryStream => (),
            _ => return Error(0) // TODO: Should not be set twice
        }

        match stream_type {
            types::AudioStream(types::PCMStream(_), _) => {
                self.stream.seek(18, io::SeekSet);
            },
            _ => return Error(0) // TODO: Support non-PCM formats
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