use std::ops;

use types::StreamType;
use buffers::buffer::Buffer;

#[deriving(Clone)]
pub struct Sample {
    stream_type: StreamType,
    buffers: ~[@Buffer],
    end_of_stream: bool
}

impl Clone for @Buffer {
    pub fn clone(&self) -> @Buffer {
        return *self;
    }
}

impl Sample {
    pub fn new(stream_type:StreamType) -> Sample {
        return Sample { stream_type: stream_type, buffers: ~[], end_of_stream: false };
    }

    pub fn add_buffer(&mut self, buffer:@Buffer) {
        self.buffers.push(buffer);
    }

    pub fn length(&self) -> uint {
        return self.buffers.len();
    }

    pub fn remove_all_buffers(&mut self) {
        self.buffers.truncate(0);
    }
}

impl ops::Index<uint, @Buffer> for Sample {
    pub fn index(&self, i:&uint) -> @Buffer {
        return self.buffers[*i];
    }
}

#[deriving(Clone)]
pub struct SampleQueue {
    samples:~[Sample]
}

impl SampleQueue {
    pub fn new() -> SampleQueue {
        return SampleQueue { samples: ~[] };
    }

    pub fn dequeue_sample(&mut self) -> Option<Sample> {
        return self.samples.shift_opt();
    }

    pub fn enqueue_sample(&mut self, sample:Sample) {
        self.samples.push(sample);
    }
}
