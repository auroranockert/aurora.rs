use std::vec;

use result::Result;
use buffers::buffer::Buffer;

pub struct MemoryBuffer {
    data:~[u8]
}

impl MemoryBuffer {
    pub fn new(length:uint) -> @mut MemoryBuffer {
        @mut MemoryBuffer { data: vec::from_elem(length, 0u8) }
    }
}

impl Buffer for MemoryBuffer {
    pub fn get_current_length(&self) -> uint { self.data.len() }
    pub fn get_allocated_length(&self) -> uint { self.data.capacity() }

    pub fn map(&mut self, f:&fn(&mut [u8]) -> Result<uint>) -> Result<uint> { // TODO: Make this thread-safe
        f(self.data)
    }
}
