use result::{Result, Ok};
use attribute::Attributes;

use samples::sample;

pub enum EventType {
    FatalError,
    NonFatalError,

    RequestSample,
    Sample(sample::Sample),

    StreamStarted,

    NewStream,
    EndOfStream,

    Unknown
}

pub struct Event {
    event_type:EventType,
    result:Result<uint>,
    value:Attributes
}

impl Event {
    pub fn new(event_type:EventType, result:Result<uint>, value:Attributes) -> Event {
        return Event {
            event_type:event_type, result:result, value:value
        };
    }
}

pub trait EventGenerator {
    pub fn dequeue_event(&mut self) -> (Result<uint>, Option<Event>);
    pub fn enqueue_event(&mut self, event:Event) -> Result<uint>;
}

pub struct EventQueue {
    events:~[Event]
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue { events: ~[] }
    }
}

impl EventGenerator for EventQueue {
    pub fn dequeue_event(&mut self) -> (Result<uint>, Option<Event>) {
        return match self.events.shift_opt() {
            None => (Ok, None),
            event => (Ok, event)
        };
    }

    pub fn enqueue_event(&mut self, event:Event) -> Result<uint> {
        self.events.push(event);

        return Ok;
    }
}
