use std::hashmap;

use align;
use types;
use result::{Result, Ok, Error};

use buffers::buffer::Buffer;
use buffers::memory::MemoryBuffer;

use events::event;
use events::event::{Event, EventGenerator, EventQueue};

use parsers::wav;
use parsers::wav::WAVParser;

use samples::sample::{Sample, SampleQueue};

use sources::source;
use sources::source::{Source, SourceCharacteristics, PresentationDescriptor, State, Started, Paused, Stopped, StreamDescriptor};

use io::read::Read;
use io::seek::Seek;

struct WAVSource {
    presentation_descriptor: @mut PresentationDescriptor,
    event_queue: EventQueue,
    stream: Option<uint>, //WAVStreamSource>,
    parser: Option<@mut WAVParser>,
    shutdown: bool,
    state: State
}

impl WAVSource {
    pub fn new() -> (Result<uint>, Option<@mut WAVSource>) {
        return (Ok, Some(@mut WAVSource {
            presentation_descriptor: PresentationDescriptor::new(),
            event_queue: EventQueue::new(),
            stream: None,
            parser: None,
            shutdown: false,
            state: Stopped
        }));
    }

    fn check_shutdown(&self) -> Result<uint> {
        return if self.shutdown { Ok } else { Error(0) }; // TODO: Magic number if there ever was one
    }

    pub fn open(&mut self, reader:@Read, seeker:@Seek) -> Result<uint> {
        self.parser = match self.parser {
            None => match WAVParser::new(reader, seeker) {
                (Ok, Some(parser)) => Some(@mut parser),
                (err, _) => return err
            },
            Some(*) => return Error(0) // TODO: Magic number if there ever was one
        };

        match self.parser {
            Some(ref mut parser) => match parser.parse_wave_header() {
                Ok => (),
                err => return err
            },
            None => return Error(0)
        };

        return match self.validate_wave_format() {
            Ok => Ok,
            err => { self.shutdown(); err }
        };
    }

    pub fn create_stream(@mut self) -> (Result<uint>, Option<@mut WAVStreamSource>) {
        let result = WAVStreamSource::new(self);

        match result {
            (Ok, Some(stream)) => self.presentation_descriptor.add_stream(stream.descriptor),
            _ => ()
        }

        return result;
    }

    fn validate_wave_format(&self) -> Result<uint> { // TODO: Fix the limitations
        let parser = match self.parser {
            Some(parser) => parser,
            None => return Error(0)
        };

        let (format, ex) = match parser.format {
            wav::None => return Error(0),
            wav::Ex(format) => {
                match format.format_tag {
                    wav::WAVE_FORMAT_PCM | wav::WAVE_FORMAT_IEEE_FLOAT |
                    wav::WAVE_FORMAT_ALAW | wav::WAVE_FORMAT_MULAW => (format, None),
                    _ => return Error(2)
                }
            }
            wav::Extensible(format, ex) => (format, Some(ex))
        };

        match format.channels { // TODO: Support channel mappings
            1 | 2 => (),
            _ => return Error(3)
        }

        match ex {
            Some(_) => (),
            None => match format.bits_per_sample { // TODO: Should we _really_ fail here? WMP does…
                8 | 16 => (),
                32 | 64 => if format.format_tag != wav::WAVE_FORMAT_IEEE_FLOAT { // TODO: Probably wrong, but I have files with this format…
                    return Error(4)
                },
                _ => return Error(4)
            }
        }

        if format.block_align != (format.channels * (format.bits_per_sample / 8)) {
            return Error(5);
        }

        if format.average_bytes_per_second != (format.samples_per_second * (format.block_align as u32)) {
            return Error(6);
        }

        // TODO: Check overflow

        return Ok;
    }
}

impl EventGenerator for WAVSource {
    pub fn dequeue_event(&mut self) -> (Result<uint>, Option<Event>) {
        return match self.check_shutdown() {
            Ok => self.event_queue.dequeue_event(),
            err => (err, None)
        }
    }

    pub fn enqueue_event(&mut self, event:Event) -> Result<uint> {
        return match self.check_shutdown() {
            Ok => self.event_queue.enqueue_event(event),
            err => err
        }
    }
}

impl Source for WAVSource {
    pub fn presentation_descriptor(&self) -> (Result<uint>, Option<@mut PresentationDescriptor>) {
        return (Ok, Some(self.presentation_descriptor));
    }

    pub fn characteristics(&self) -> (Result<uint>, Option<SourceCharacteristics>) {
        return match self.check_shutdown() {
            Ok => (Ok, Some(source::SourceCharacteristics { pause: true, seek: true, live: false })),
            err => (err, None)
        };
    }

    pub fn start(&mut self) -> Result<uint> { /* Missing time/presentation information */
        match self.check_shutdown() {
            Ok => (),
            err => return err
        }
        
        // let (start_offset, restart) = (0, false);

        // match self.validate_presentation_descriptor(presentation) {
        //     Ok => (),
        //     err => return err
        // }

        // Sends the MENewStream or MEUpdatedStream event.
        // hr = QueueNewStreamEvent(pPresentationDescriptor);

        // Notify the stream of the new start time.
        // hr = m_pStream->SetPosition(llStartOffset);

        // Create the event.
        // hr = MFCreateMediaEvent(MESourceStarted, GUID_NULL, hr, &var, &pEvent);

        // Now  queue the event.
        // hr = m_pEventQueue->QueueEvent(pEvent);

        // Send the stream event.
        // hr = m_pStream->QueueEvent(MEStreamStarted, GUID_NULL, hr, &var);

        // Otherwise, deliver any queued samples.
        // hr = m_pStream->DeliverQueuedSamples();

        // If a failure occurred and we have not sent the
        // MESourceStarted/MESourceSeeked event yet, then it is
        // OK just to return an error code from Start().

        // If a failure occurred and we have already sent the
        // event (with a success code), then we need to raise an
        // MEError event.
        // hr = QueueEvent(MEError, GUID_NULL, hr, &var);

        return Ok;
    }

    pub fn pause(&mut self) -> Result<uint> {
        match self.check_shutdown() {
            Ok => (),
            err => return err
        }

        if self.state != Started {
            return Error(1); // TODO: Fix magic number
        }

        // match self.stream {
        //     Some(stream) => match stream.enqueue_event(…) {
        //         Ok => (),
        //         err => return err
        //     }, // TODO: Add actual event
        //     _ => ()
        // }

        // match self.enqueue_event(…) { // TODO: Add actual event
        //     Ok => (),
        //     err => return err
        // }

        self.state = Paused;

        return Ok;
    }

    pub fn stop(&mut self) -> Result<uint> {
        match self.check_shutdown() {
            Ok => (),
            err => return err
        }

        self.state = Stopped;
        
        // match stream.flush() {
        //     Ok => (),
        //     err => return err
        // }

        // match self.stream {
        //     Some(stream) => match stream.enqueue_event(…) {
        //         Ok => (),
        //         err => return err
        //     }, // TODO: Add actual event
        //     _ => ()
        // }

        // match self.enqueue_event(…) { // TODO: Add actual event
        //     Ok => (),
        //     err => return err
        // }

        return Ok;
    }

    pub fn shutdown(&mut self) -> Result<uint> {
        match self.check_shutdown() {
            Ok => (),
            err => return err
        }

        // match self.stream {
        //     Some(stream) => stream.shutdown(),
        //     None => ()
        // }

        // self.event_queue.shutdown();

        self.stream = None;
        self.parser = None;

        self.shutdown = true;

        return Ok;
    }
}

pub struct WAVStreamSource {
    shutdown:bool,
    current_position:u64,
    discontinuity: bool,
    end_of_stream:bool,

    descriptor: @mut StreamDescriptor,

    source:@mut WAVSource,
    event_queue:EventQueue,
    sample_queue:SampleQueue
}

impl WAVStreamSource {
    pub fn new(source:@mut WAVSource) -> (Result<uint>, Option<@mut WAVStreamSource>) {
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

        return (Ok, Some(@mut WAVStreamSource {
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

impl EventGenerator for WAVStreamSource {
    pub fn dequeue_event(&mut self) -> (Result<uint>, Option<Event>) {
        self.event_queue.dequeue_event()
    }

    pub fn enqueue_event(&mut self, event:Event) -> Result<uint> {
        self.event_queue.enqueue_event(event)
    }
    
}

impl source::StreamSource for WAVStreamSource {
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