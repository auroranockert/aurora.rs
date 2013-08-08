use std::cast;

use io::read::{Read, ReadCore};

pub struct Bitstream {
    reader: @Read, byte:u32, offset:uint
}

impl Bitstream {
    pub fn new(reader:@Read) -> @mut Bitstream {
        return @mut Bitstream {
            reader: reader, byte: 0, offset: 8
        }
    }

    pub fn read(&mut self, bits:uint) -> u32 {
        if bits == 0 {
            return 0;
        }

        if bits > 32 {
            fail!("Too large read! (is 32-bits only!)")
        }

        let n_bits = bits + self.offset;

        let result = if n_bits <= 8 {
            (self.byte << (24 + self.offset)) >> (32 - bits)
        } else if n_bits <= 16 {
            let c0 = self.byte;

            self.byte = self.reader.read_u8_be() as u32;

            (((c0 << 8) | self.byte) << (16 + self.offset)) >> (32 - bits)
        } else if n_bits <= 24 {
            let c0 = self.byte;
            let c1 = self.reader.read_u8_be() as u32;

            self.byte = self.reader.read_u8_be() as u32;

            (((c0 << 16) | (c1 << 8) | self.byte) << (self.offset + 8)) >> (32 - bits)
        } else if n_bits <= 32 {
            let c0 = self.byte;
            let c1 = self.reader.read_u8_be() as u32;
            let c2 = self.reader.read_u8_be() as u32;

            self.byte = self.reader.read_u8_be() as u32;

            (((c0 << 24) | (c1 << 16) | (c2 << 8)| self.byte) << self.offset) >> (32 - bits)
        } else {
            fail!("Not supported yet!");
        };

        self.offset = if n_bits % 8 == 0 { 8 } else { n_bits % 8 };

        return result;
    }

    pub fn read_signed(&mut self, bits:uint) -> i32 {
        let value = unsafe { cast::transmute::<u32, i32>(self.read(bits)) };

        return (value << (32 - bits)) >> (32 - bits);
    }

    pub fn read_golomb_flac(&mut self, k:uint) -> i32 {
        let mut q = 0;

        while self.read(1) == 0 { q += 1; }

        let r = self.read(k);

        let value = (q * (1 << k) + r);

        return unsafe {
            cast::transmute::<u32, i32>(value >> 1) ^ -cast::transmute::<u32, i32>(value & 1)
        };
    }
}
