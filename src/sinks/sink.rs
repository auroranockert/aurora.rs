use result::Result;
use types::StreamType;

use events::event::{Event, EventGenerator};

pub struct SinkCharacteristics {
    fixed_streams: bool, rateless: bool
}

pub trait Sink {
    pub fn characteristics(&self) -> (Result<uint>, Option<SinkCharacteristics>);

    pub fn stream_sink_from_index(&mut self, index:uint) -> (Result<uint>, Option<@mut StreamSink>);

    pub fn finalize(&mut self) -> Result<uint>;

    pub fn shutdown(&mut self) -> Result<uint>;
}

pub trait StreamSink : EventGenerator {
    pub fn sink(&self) -> @Sink;

    pub fn set_stream_type(&mut self, stream_type:StreamType) -> Result<uint>;

    pub fn dequeue_stream_sink_event(&mut self) -> (Result<uint>, Option<Event>); // TODO: Just workarounds
    pub fn enqueue_stream_sink_event(&mut self, event:Event) -> Result<uint>; // TODO: Just workarounds
    
}
