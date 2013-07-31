use result::{Result, Ok, Error};

use events::event::{Event, EventGenerator, EventQueue};
use parsers::wav;
use parsers::wav::WAVParser;
use sources::source;
use sources::source::{Source, SourceCharacteristics, PresentationDescriptor, State, Started, Paused, Stopped};
use streams::wav::WAVStream;

struct WAVSource {
    presentation_descriptor: @mut PresentationDescriptor,
    event_queue: EventQueue,
    stream: Option<uint>, //WAVStream>,
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

    pub fn open(&mut self, stream:@Reader) -> Result<uint> {
        self.parser = match self.parser {
            None => match WAVParser::new(stream) {
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

    pub fn create_stream(@mut self) -> (Result<uint>, Option<@mut WAVStream>) {
        let result = WAVStream::new(self);

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
