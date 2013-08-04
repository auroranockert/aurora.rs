extern mod aurora;

use std::hashmap::HashMap;

use aurora::types;
use aurora::result::Ok;

use aurora::events::event;
use aurora::events::event::EventGenerator;
use aurora::io::file::{File, READ_ONLY, WRITE_ONLY, CREATE_FILE, TRUNCATE_FILE};
use aurora::io::read::Read;
use aurora::io::seek::Seek;
use aurora::io::write::Write;
use aurora::sinks::sink::{Sink, StreamSink};
use aurora::sinks::wav::WAVSink;
use aurora::sources::wav::WAVSource;
use aurora::streams::stream::Stream;
use aurora::transforms::transform::Transform;
use aurora::transforms::pcm::PCMTransform;

fn main() {
    // let reader = io::file_reader(&path::PosixPath("media/wav/test-float.wav")).unwrap();
    // let writer = io::file_writer(&path::PosixPath("output.wav"), &[io::Create, io::Truncate]).unwrap();

    let input_file = @match File::open(~"media/wav/test-float.wav", READ_ONLY, 0x1B6) {
        Some(file) => file,
        None => fail!("Could not open input!")
    };

    let output_file = @match File::open(~"output.wav", WRITE_ONLY | CREATE_FILE | TRUNCATE_FILE, 0x1B6) {
        Some(file) => file,
        None => fail!("Could not open output!")
    };

    let source = match WAVSource::new() {
        (Ok, Some(source)) => source,
        (err, _) => fail!(fmt!("Error: %?", err))
    };

    match source.open(input_file as @Read, input_file as @Seek) {
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

    let sink = match WAVSink::new(output_file as @Write, output_file as @Seek) {
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
                                Ok => return,
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