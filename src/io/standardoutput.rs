use std::libc;

use result::{Result, Ok, Error};

use io::write;

pub struct StandardOutput;

impl StandardOutput {
    pub fn new() -> StandardOutput {
        return StandardOutput;
    }
}

impl write::Write for StandardOutput {
    pub fn write(&mut self, bytes:&[u8]) -> Result<write::WriteFailure> {
        let n = unsafe {
            do bytes.as_imm_buf |buffer, length| {
                libc::write(1, buffer as *libc::c_void, length as libc::size_t)
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
