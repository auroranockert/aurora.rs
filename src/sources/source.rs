use std::ops;

use types;
use result::Result;

use events::event::EventGenerator;

#[deriving(Eq)]
pub enum State {
    Started,
    Paused,
    Stopped
}

pub struct SourceCharacteristics {
    live:bool, seek:bool, pause:bool
}

pub struct PresentationDescriptor {
    streams: ~[@mut StreamDescriptor]
}

impl PresentationDescriptor {
    pub fn new() -> @mut PresentationDescriptor {
        return @mut PresentationDescriptor {
            streams: ~[]
        };
    }

    pub fn add_stream(&mut self, stream:@mut StreamDescriptor) {
        self.streams.push(stream);
    }

    pub fn select_stream(&mut self, index:uint) {
        self[index].selected = true;
    }

    pub fn deselect_stream(&mut self, index:uint) {
        self[index].selected = false;
    }

    pub fn count(&mut self) -> uint {
        return self.streams.len();
    }

    // TODO: pub fn selected_streams -> ~[@mut StreamDescriptor]
}

impl ops::Index<uint, @mut StreamDescriptor> for PresentationDescriptor {
    pub fn index(&self, i:&uint) -> @mut StreamDescriptor {
        return self.streams[*i];
    }
}

pub struct StreamDescriptor {
    selected: bool,
    identifier: uint,
    stream_type: types::StreamType
}

impl StreamDescriptor {
    pub fn new(selected:bool, identifier:uint, stream_type:types::StreamType) -> @mut StreamDescriptor {
        return @mut StreamDescriptor {
            selected: selected,
            identifier: identifier,
            stream_type: stream_type
        }
    }
}

pub trait Source : EventGenerator {
    pub fn presentation_descriptor(&self) -> (Result<uint>, Option<@mut PresentationDescriptor>);

    pub fn characteristics(&self) -> (Result<uint>, Option<SourceCharacteristics>);

    pub fn start(&mut self) -> Result<uint>; /* Missing time/presentation-information */
    pub fn pause(&mut self) -> Result<uint>;
    pub fn stop(&mut self) -> Result<uint>;

    pub fn shutdown(&mut self) -> Result<uint>;
}

pub trait Stream : EventGenerator {
    pub fn descriptor(&mut self) -> (Result<uint>, Option<@mut StreamDescriptor>);
    
    pub fn request_sample(&mut self) -> Result<uint>;
}
