use result::Result;

use events::event::Event;
use samples::sample::Sample;

use types;

pub enum Message {
    Flush, Drain, StartOfStream(@mut TransformStream), EndOfStream(@mut TransformStream)
}

pub enum Direction {
    InputStream, OutputStream
}

pub struct TransformStream {
    identifier: uint, direction: Direction, transform: @mut Transform, stream_type: types::StreamType
}

impl TransformStream {
    pub fn add(&mut self) -> Result<uint> {
        let mut transform = self.transform;
        
        return transform.add_stream(self);
    }

    pub fn process_event(&mut self, event:Event) -> Result<uint> {
        let mut transform = self.transform;
        
        return transform.process_event(self, event);
    }

    pub fn process_input(&mut self, sample:Sample) -> Result<uint> {
        let mut transform = self.transform;

        return match self.direction {
            InputStream => transform.process_input(self, sample),
            OutputStream => fail!("Trying to process input of an output stream!")
        };
    }

    pub fn process_output(&mut self) -> (Result<uint>, Option<Sample>) {
        let mut transform = self.transform;

        return match self.direction {
            OutputStream => transform.process_output(self),
            InputStream => fail!("Trying to process output of an input stream!")
        };
    }
}

pub trait Transform {
    pub fn input_streams(&mut self) -> @[@mut TransformStream];
    pub fn output_streams(&mut self) -> @[@mut TransformStream];

    pub fn input_stream_limits(&mut self) -> (uint, uint);
    pub fn output_stream_limits(&mut self) -> (uint, uint);

    pub fn add_stream(&mut self, stream:&TransformStream) -> Result<uint>;

    pub fn process_message(&mut self, message:Message) -> Result<uint>;

    pub fn process_event(&mut self, stream:&TransformStream, event:Event) -> Result<uint>;
    pub fn process_input(&mut self, stream:&TransformStream, sample:Sample) -> Result<uint>;
    pub fn process_output(&mut self, stream:&TransformStream) -> (Result<uint>, Option<Sample>); /* TODO: Should be able to reuse samples */
}
