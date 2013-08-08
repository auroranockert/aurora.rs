use std::cast;
use std::uint;
use std::unstable::intrinsics;

use result::{Result, Ok, Error};
use byteswap::ByteSwap;

use fourcc;

pub enum ReadFailure {
    UnknownError, WouldBlock, EndOfStream(u64)
}

pub trait Read {
    fn skip_forward(&mut self, length:u64) -> Result<ReadFailure>;
    fn read(&mut self, bytes:&mut [u8], length:u64) -> Result<ReadFailure>;
}

pub trait ReadCore {
    pub fn read_u8_be(&mut self) -> u8; // TODO: Could be called read_u8, but that would collide with std::io::Reader…

    pub fn read_u16_be(&mut self) -> u16;
    pub fn read_u24_be(&mut self) -> u32;
    pub fn read_u32_be(&mut self) -> u32;
    pub fn read_u64_be(&mut self) -> u64;

    pub fn read_i8_be(&mut self) -> i8; // TODO: Could be called read_i8, but that would collide with std::io::Reader…

    pub fn read_i16_be(&mut self) -> i16;
    pub fn read_i32_be(&mut self) -> i32;
    pub fn read_i64_be(&mut self) -> i64;

    pub fn read_u16_le(&mut self) -> u16;
    pub fn read_u32_le(&mut self) -> u32;
    pub fn read_u64_le(&mut self) -> u64;

    pub fn read_i16_le(&mut self) -> i16;
    pub fn read_i32_le(&mut self) -> i32;
    pub fn read_i64_le(&mut self) -> i64;

    pub fn read_fourcc(&mut self) -> fourcc::FourCC;

    pub fn read_utf8_char(&mut self) -> char;
    
    pub fn read_partial(&mut self, buffer:&mut [u8], length:u64) -> u64;
}

impl<T:Read> ReadCore for T { // TODO: Don't throw away errors?
    pub fn read_u8_be(&mut self) -> u8 {
        let mut result = [0u8]; let n = 1;

        self.read(result.mut_slice(0, 1), n);

        return result[0].to_big_endian();
    }

    pub fn read_u16_be(&mut self) -> u16 {
        let mut result = [0u16]; let n = 2;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) }, n);

        return result[0].to_big_endian();
    }

    pub fn read_u24_be(&mut self) -> u32 {
        let mut result = [0u8, 0u8, 0u8]; let n = 3;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 3)) } , n);

        return (result[0] as u32 << 16) + (result[1] as u32 << 8) + (result[2] as u32);
    }

    pub fn read_u32_be(&mut self) -> u32 {
        let mut result = [0u32]; let n = 4;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) } , n);

        return result[0].to_big_endian();
    }

    pub fn read_u64_be(&mut self) -> u64 {
        let mut result = [0u64]; let n = 8;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) } , n);

        return result[0].to_big_endian();
    }

    pub fn read_i8_be(&mut self) -> i8 {
        let mut result = [0i8]; let n = 1;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) }, n);

        return result[0].to_big_endian();
    }

    pub fn read_i16_be(&mut self) -> i16 {
        let mut result = [0i16]; let n = 2;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) }, n);

        return result[0].to_big_endian();
    }

    pub fn read_i32_be(&mut self) -> i32 {
        let mut result = [0i32]; let n = 4;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) } , n);

        return result[0].to_big_endian();
    }

    pub fn read_i64_be(&mut self) -> i64 {
        let mut result = [0i64]; let n = 8;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) } , n);

        return result[0].to_big_endian();
    }

    pub fn read_u16_le(&mut self) -> u16 {
        let mut result = [0u16]; let n = 2;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) }, n);

        return result[0].to_little_endian();
    }

    pub fn read_u32_le(&mut self) -> u32 {
        let mut result = [0u32]; let n = 4;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) } , n);

        return result[0].to_little_endian();
    }

    pub fn read_u64_le(&mut self) -> u64 {
        let mut result = [0u64]; let n = 8;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) } , n);

        return result[0].to_little_endian();
    }

    pub fn read_i16_le(&mut self) -> i16 {
        let mut result = [0i16]; let n = 2;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) }, n);

        return result[0].to_little_endian();
    }

    pub fn read_i32_le(&mut self) -> i32 {
        let mut result = [0i32]; let n = 4;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) } , n);

        return result[0].to_little_endian();
    }

    pub fn read_i64_le(&mut self) -> i64 {
        let mut result = [0i64]; let n = 8;

        self.read(unsafe { cast::transmute(result.mut_slice(0, 1)) } , n);

        return result[0].to_little_endian();
    }

    pub fn read_fourcc(&mut self) -> fourcc::FourCC {
        return self.read_u32_be();
    }

    pub fn read_utf8_char(&mut self) -> char {
        let b0 = self.read_u8_be();
        let bytes = unsafe { intrinsics::ctlz8(cast::transmute(!b0)) };

        let mut result = match bytes {
            0 => ((b0 as u32 & 0x7F) <<  0),
            1 => fail!("UTF-8 with leading 10, this should be illegal!"),
            2 => ((b0 as u32 & 0x1F) <<  6),
            3 => ((b0 as u32 & 0x0F) << 12),
            4 => ((b0 as u32 & 0x07) << 18),
            5 => ((b0 as u32 & 0x03) << 24),
            6 => ((b0 as u32 & 0x01) << 30),
            _ => fail!("UTF-8 starts with a few too many 1s!"),
        };

        for uint::range(1, bytes as uint) |i| {
            result = result | ((self.read_u8_be() as u32) << ((bytes as uint - i - 1) * 6));
        }

        return result as char;
    }

    pub fn read_partial(&mut self, buffer:&mut [u8], length:u64) -> u64 {
        return match self.read(buffer, length) {
            Ok => length,
            Error(EndOfStream(n)) => n,
            Error(err) => fail!(fmt!("Partial read failed due to %?", err))
        };
    }
}

impl Read for @Read {
    pub fn skip_forward(&mut self, length:u64) -> Result<ReadFailure> {
        return self.skip_forward(length);
    }

    pub fn read(&mut self, bytes:&mut [u8], length:u64) -> Result<ReadFailure> {
        return self.read(bytes, length);
    }
}
