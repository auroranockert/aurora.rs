use result::Result;

use types::StreamType;

use events::event::EventGenerator;

pub trait Stream : EventGenerator {
    pub fn descriptor(&mut self) -> (Result<uint>, Option<@mut StreamDescriptor>);
    
    pub fn request_sample(&mut self) -> Result<uint>;
}

pub struct StreamDescriptor {
    selected: bool,
    identifier: uint,
    stream_type: StreamType
}

impl StreamDescriptor {
    pub fn new(selected:bool, identifier:uint, stream_type:StreamType) -> @mut StreamDescriptor {
        return @mut StreamDescriptor {
            selected: selected,
            identifier: identifier,
            stream_type: stream_type
        }
    }
}
