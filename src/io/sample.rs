use std::vec;

use result::{Result, Ok, Error};
use samples::sample::{Sample, SampleQueue};

use io::read;

#[deriving(Clone)]
pub struct SampleStream {
    offset: u64, buffer: uint,
    current_sample: Option<Sample>,
    sample_queue: SampleQueue
}

impl SampleStream {
    pub fn new() -> SampleStream {
        return SampleStream {
            offset: 0,
            buffer: 0,
            current_sample: None,
            sample_queue: SampleQueue::new()
        };
    }

    pub fn enqueue_sample(&mut self, sample:Sample) {
        self.sample_queue.enqueue_sample(sample);
    }
}

impl read::Read for SampleStream {
    pub fn skip_forward(&mut self, _:u64) -> Result<read::ReadFailure> {
        fail!("Not implemented!")
    }

    pub fn read(&mut self, bytes:&mut [u8], length:u64) -> Result<read::ReadFailure> {
        let offset = self.offset; let mut first = true;
        let mut written = 0;

        self.offset += length;

        match self.current_sample {
            None => self.current_sample = self.sample_queue.dequeue_sample(),
            _ => ()
        }

        loop {
            match self.current_sample.clone() { // TODO: This is just silly…
                None => {
                    return Error(read::EndOfStream(offset - self.offset));
                }
                Some(sample) => {
                    if self.offset <= sample[self.buffer].get_current_length() as u64 {
                        unsafe {
                            sample[self.buffer].map(|b| {
                                let src_slice_start = if first { offset as uint } else { 0 };
                                let src_slice_length = (self.offset - offset).min(&(length - written)) as uint;

                                let dst_slice_start = written as uint;
                                let dst_slice_length = (length - written) as uint;

                                let src_slice = b.slice(src_slice_start, src_slice_start + src_slice_length);
                                let dst_slice = bytes.mut_slice(dst_slice_start, dst_slice_start + dst_slice_length);

                                vec::raw::copy_memory(dst_slice, src_slice, src_slice_length);

                                first = false; written += src_slice_length as u64; Ok
                            })
                        };

                        return Ok;
                    } else {
                        fail!("Not implemented!")
                    }
                }
            }
        }
    }
}
