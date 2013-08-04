use std::cast;

use result::Result;
use byteswap::ByteSwap;

use fourcc;

pub enum ReadFailure {
    UnknownError, WouldBlock, EndOfStream
}

pub trait Read {
    fn read(&mut self, bytes:&mut [u8], length:u64) -> Result<ReadFailure>;
}

pub trait ReadCore {
    pub fn read_u8_be(&mut self) -> u8; // TODO: Could be called read_u8, but that would collide with std::io::Reader…

    pub fn read_u16_be(&mut self) -> u16;
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
}

impl<T:Read> ReadCore for T {
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
}
