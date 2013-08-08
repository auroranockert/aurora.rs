use result::Result;

pub enum SeekFailure {
    UnknownError, OutOfRange, Overflow
}

pub trait Seek {
    pub fn seek_from_beginning(&mut self, position:u64) -> Result<SeekFailure>;
    pub fn seek_from_end(&mut self, position:u64) -> Result<SeekFailure>;

    pub fn seek(&mut self, position:i64) -> Result<SeekFailure>;
}

impl Seek for @Seek {
    pub fn seek_from_beginning(&mut self, position:u64) -> Result<SeekFailure> {
        return self.seek_from_beginning(position);
    }

    pub fn seek_from_end(&mut self, position:u64) -> Result<SeekFailure> {
        return self.seek_from_end(position);
    }

    pub fn seek(&mut self, position:i64) -> Result<SeekFailure> {
        return self.seek(position);
    }
}
