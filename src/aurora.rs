pub mod align;
pub mod types;
pub mod fourcc;

#[macro_escape]
mod macros {
    macro_rules! fcc(($a:expr) => ({
        let b = bytes!($a); (b[0] as u32 << 24) | (b[1] as u32 << 16) | (b[2] as u32 << 8) | (b[3] as u32)
    }))
}

pub mod buffers {
    pub mod buffer;
    pub mod memory;
}

pub mod events {
    pub mod event;
}

pub mod parsers {
    pub mod riff;
    pub mod wav;
}

pub mod samples {
    pub mod sample;
}

pub mod sinks {
    pub mod sink;
    pub mod wav;
}

pub mod sources {
    pub mod source;
    pub mod wav;
}

pub mod streams {
    pub mod stream;
    pub mod wav;
}

pub mod attribute;
pub mod result;

mod main {
    use std::io;
    use std::path;
    use std::hashmap::HashMap;

    use result::Ok;
    use events::event;
    use events::event::EventGenerator;
    use sinks::sink::{Sink, StreamSink};
    use sinks::wav::WAVSink;
    use sources::wav::WAVSource;
    use streams::stream::Stream;

    #[main]
    fn main() {
        let reader = io::file_reader(&path::PosixPath("test.wav")).unwrap();
        let writer = io::file_writer(&path::PosixPath("output.wav"), &[io::Create, io::Truncate]).unwrap();

        let source = match WAVSource::new() {
            (Ok, Some(source)) => source,
            (err, _) => fail!(fmt!("Error: %?", err))
        };

        match source.open(reader) {
            Ok => (),
            err => fail!(fmt!("Error: %?", err))
        }

        let stream = match source.create_stream() {
            (Ok, Some(stream)) => stream,
            (err, _) => fail!(fmt!("Error: %?", err))
        };

        let sink = match WAVSink::new(writer) {
            (Ok, Some(sink)) => sink,
            (err, _) => fail!(fmt!("Error: %?", err))
        };

        let mut stream_sink = match sink.stream_sink_from_index(0) {
            (Ok, Some(stream_sink)) => stream_sink,
            (err, _) => fail!(fmt!("Error: %?", err))
        };

        stream_sink.set_stream_type(stream.descriptor.stream_type);

        loop {
            match stream.request_sample() {
                Ok => (),
                err => fail!(fmt!("Error! %?", err))
            }

            loop {
                match stream.dequeue_event() {
                    (Ok, None) => break,
                    (Ok, Some(event)) => {
                        match event.event_type {
                            event::EndOfStream => {
                                match sink.finalize() {
                                    Ok => return io::println("Reached end"),
                                    err => fail!(fmt!("Error finalizing sink: %?", err))
                                }
                            }
                            event::Sample(sample) => {
                                let event = event::Event::new(event::Sample(sample), Ok, HashMap::new());

                                stream_sink.enqueue_stream_sink_event(event);
                            }
                            event => fail!(fmt!("Unknown event! %?", event))
                        }
                    }
                    (err, _) => fail!(fmt!("Error! %?", err))
                }
            }
        }
    }
}
