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

pub mod transforms {
    pub mod transform;
    pub mod pcm;
}

pub mod attribute;
pub mod result;

mod main {
    use std::io;
    use std::path;
    use std::hashmap::HashMap;

    use types;
    use result::Ok;

    use events::event;
    use events::event::EventGenerator;
    use sinks::sink::{Sink, StreamSink};
    use sinks::wav::WAVSink;
    use sources::wav::WAVSource;
    use streams::stream::Stream;
    use transforms::transform::Transform;
    use transforms::pcm::PCMTransform;

    #[main]
    fn main() {
        let reader = io::file_reader(&path::PosixPath("test-float.wav")).unwrap();
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

        let transform = match PCMTransform::new() {
            (Ok, Some(transform)) => transform,
            (err, _) => fail!(fmt!("Error: %?", err))
        };

        let transform_input = transform.input_streams()[0];
        let transform_output = transform.output_streams()[0];

        let audio_format = match stream.descriptor.stream_type {
            types::AudioStream(types::PCMStream(*), format) => format,
            _ => fail!("Not a PCM stream!")
        };

        let pcm_format = types::PCMFormat {
            sample_type: types::Signed(16), endian: types::LittleEndian
        };

        let output_type = types::AudioStream(types::PCMStream(pcm_format), audio_format);

        transform_input.stream_type = stream.descriptor.stream_type;
        transform_output.stream_type = output_type;

        transform_input.add();
        transform_output.add();

        let sink = match WAVSink::new(writer) {
            (Ok, Some(sink)) => sink,
            (err, _) => fail!(fmt!("Error: %?", err))
        };

        let mut stream_sink = match sink.stream_sink_from_index(0) {
            (Ok, Some(stream_sink)) => stream_sink,
            (err, _) => fail!(fmt!("Error: %?", err))
        };

        stream_sink.set_stream_type(transform_output.stream_type);

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
                                match transform_input.process_input(sample) {
                                    Ok => (),
                                    err => fail!(fmt!("Failed to process input! %?", err))
                                }

                                let sample = match transform_output.process_output() {
                                    (Ok, Some(sample)) => sample,
                                    (err, _) => fail!(fmt!("Failed to process output! %?", err))
                                };

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
