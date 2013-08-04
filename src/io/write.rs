use std::cast;

use result::Result;
use byteswap::ByteSwap;

use fourcc;

pub enum WriteFailure {
    UnknownError, WouldBlock, NoSpace
}

pub trait Write {
    pub fn write(&mut self, bytes:&[u8]) -> Result<WriteFailure>;
}

pub trait WriteCore {
    pub fn write_u8_be(&mut self, value:u8); // TODO: Could be called write_u8, but that would collide with std::io::Reader…

    pub fn write_u16_be(&mut self, value:u16);
    pub fn write_u32_be(&mut self, value:u32);
    pub fn write_u64_be(&mut self, value:u64);

    pub fn write_i8_be(&mut self, value:i8); // TODO: Could be called write_i8, but that would collide with std::io::Reader…

    pub fn write_i16_be(&mut self, value:i16);
    pub fn write_i32_be(&mut self, value:i32);
    pub fn write_i64_be(&mut self, value:i64);

    pub fn write_u16_le(&mut self, value:u16);
    pub fn write_u32_le(&mut self, value:u32);
    pub fn write_u64_le(&mut self, value:u64);

    pub fn write_i16_le(&mut self, value:i16);
    pub fn write_i32_le(&mut self, value:i32);
    pub fn write_i64_le(&mut self, value:i64);

    pub fn write_fourcc(&mut self, value:fourcc::FourCC);
}

impl<T:Write> WriteCore for T {
    pub fn write_u8_be(&mut self, value:u8) {
        let result = [value.to_big_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_u16_be(&mut self, value:u16) {
        let result = [value.to_big_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_u32_be(&mut self, value:u32) {
        let result = [value.to_big_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_u64_be(&mut self, value:u64) {
        let result = [value.to_big_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_i8_be(&mut self, value:i8) {
        let result = [value.to_big_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_i16_be(&mut self, value:i16) {
        let result = [value.to_big_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_i32_be(&mut self, value:i32) {
        let result = [value.to_big_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_i64_be(&mut self, value:i64) {
        let result = [value.to_big_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_u16_le(&mut self, value:u16) {
        let result = [value.to_little_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_u32_le(&mut self, value:u32) {
        let result = [value.to_little_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_u64_le(&mut self, value:u64) {
        let result = [value.to_little_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_i16_le(&mut self, value:i16) {
        let result = [value.to_little_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_i32_le(&mut self, value:i32) {
        let result = [value.to_little_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_i64_le(&mut self, value:i64) {
        let result = [value.to_little_endian()];

        self.write(unsafe { cast::transmute(result.slice(0, 1)) });
    }

    pub fn write_fourcc(&mut self, value:fourcc::FourCC) {
        self.write_u32_be(value as u32);
    }
}
