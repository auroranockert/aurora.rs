use result::Result;

pub trait Buffer {
    pub fn get_current_length(&self) -> uint;
    pub fn get_allocated_length(&self) -> uint;

    pub fn map(&mut self, &fn(&mut [u8]) -> Result<uint>) -> Result<uint>;
}
