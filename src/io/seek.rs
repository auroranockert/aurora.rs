use result::Result;

pub enum SeekFailure {
    UnknownError, OutOfRange, Overflow
}

pub trait Seek {
    pub fn seek_from_beginning(&mut self, position:u64) -> Result<SeekFailure>;
    pub fn seek_from_end(&mut self, position:u64) -> Result<SeekFailure>;

    pub fn seek(&mut self, position:i64) -> Result<SeekFailure>;
}
