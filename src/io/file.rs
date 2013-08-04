use std::libc;

use result::{Result, Ok, Error};

use io::read;
use io::seek;
use io::write;

pub struct File {
    fd: libc::c_int
}

static READ_ONLY:uint       = 0x0000; // TODO: This probably only works on OS X / Linux…
static WRITE_ONLY:uint      = 0x0001;
static READ_WRITE:uint      = 0x0002;

static CREATE_FILE:uint     = 0x0200;
static TRUNCATE_FILE:uint   = 0x0400;


impl File {
    pub fn open(path:~str, flags:uint, mode:uint) -> Option<File> {
        let fd = unsafe {
            path.as_c_str(|buf| {
                libc::open(buf as *libc::c_char, flags as libc::c_int, mode as libc::c_int)
            })
        };

        return if fd < 0 {
            None
        } else {
            Some(File { fd: fd })
        }
    }
}

impl Drop for File {
    fn drop(&self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}

impl read::Read for File {
    pub fn read(&mut self, bytes:&mut [u8], length:u64) -> Result<read::ReadFailure> {
        let n = unsafe {
            do bytes.as_mut_buf |buffer, length| {
                libc::read(self.fd, buffer as *mut libc::c_void, length as libc::size_t)
            }
        };

        return if n < 0 {
            Error(read::UnknownError)
        } else if n < (length as libc::off_t) {
            Error(read::EndOfStream)
        } else if n > (length as libc::off_t) {
            fail!("Read more than expected, this is probably a serious error in aurora/libc…")
        } else {
            Ok
        }
    }
}

impl seek::Seek for File {
    pub fn seek_from_beginning(&mut self, position:u64) -> Result<seek::SeekFailure> {
        return match unsafe { libc::lseek(self.fd, position as libc::off_t, 0) } {
            -1 => Error(seek::UnknownError),
            _ => Ok
        };
    }

    pub fn seek_from_end(&mut self, position:u64) -> Result<seek::SeekFailure> {
        return match unsafe { libc::lseek(self.fd, position as libc::off_t, 2) } {
            -1 => Error(seek::UnknownError),
            _ => Ok
        };
    }

    pub fn seek(&mut self, position:i64) -> Result<seek::SeekFailure> {
        return match unsafe { libc::lseek(self.fd, position as libc::off_t, 1) } {
            -1 => Error(seek::UnknownError),
            _ => Ok
        };
    }
}

impl write::Write for File {
    pub fn write(&mut self, bytes:&[u8]) -> Result<write::WriteFailure> {
        let n = unsafe {
            do bytes.as_imm_buf |buffer, length| {
                libc::write(self.fd, buffer as *libc::c_void, length as libc::size_t)
            }
        };

        return if n < 0 {
            Error(write::UnknownError)
        } else if n != (bytes.len() as libc::off_t) {
            Error(write::UnknownError)
        } else {
            Ok
        }
    }
}

impl read::Read for @read::Read {
    pub fn read(&mut self, bytes:&mut [u8], length:u64) -> Result<read::ReadFailure> {
        return self.read(bytes, length);
    }
}

impl seek::Seek for @seek::Seek {
    pub fn seek_from_beginning(&mut self, position:u64) -> Result<seek::SeekFailure> {
        return self.seek_from_beginning(position);
    }

    pub fn seek_from_end(&mut self, position:u64) -> Result<seek::SeekFailure> {
        return self.seek_from_end(position);
    }

    pub fn seek(&mut self, position:i64) -> Result<seek::SeekFailure> {
        return self.seek(position);
    }
}

impl write::Write for @write::Write {
    pub fn write(&mut self, bytes:&[u8]) -> Result<write::WriteFailure> {
        return self.write(bytes);
    }
}
