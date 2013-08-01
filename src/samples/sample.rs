use std::ops;

use types::StreamType;
use buffers::buffer::Buffer;

pub struct Sample {
    stream_type: StreamType,
    buffers:~[@Buffer]
}

impl Sample {
    pub fn new(stream_type:StreamType) -> Sample {
        return Sample { stream_type: stream_type, buffers: ~[] };
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
