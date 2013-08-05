use std::cast;
use std::unstable::intrinsics;

pub trait ByteSwap {
    pub fn to_big_endian(&self) -> Self;
    pub fn to_little_endian(&self) -> Self;
}

impl ByteSwap for u8 {
    pub fn to_big_endian(&self) -> u8 {
        return *self;
    }

    pub fn to_little_endian(&self) -> u8 {
        return *self;
    }
}

impl ByteSwap for u16 {
    pub fn to_big_endian(&self) -> u16 {
        return intrinsics::to_be16(*self as i16) as u16;
    }

    pub fn to_little_endian(&self) -> u16 {
        return intrinsics::to_le16(*self as i16) as u16;
    }
}

impl ByteSwap for u32 {
    pub fn to_big_endian(&self) -> u32 {
        return intrinsics::to_be32(*self as i32) as u32;
    }

    pub fn to_little_endian(&self) -> u32 {
        return intrinsics::to_le32(*self as i32) as u32;
    }
}


impl ByteSwap for u64 {
    pub fn to_big_endian(&self) -> u64 {
        return intrinsics::to_be64(*self as i64) as u64;
    }

    pub fn to_little_endian(&self) -> u64 {
        return intrinsics::to_le64(*self as i64) as u64;
    }
}

impl ByteSwap for i8 {
    pub fn to_big_endian(&self) -> i8 {
        return *self;
    }

    pub fn to_little_endian(&self) -> i8 {
        return *self;
    }
}

impl ByteSwap for i16 {
    pub fn to_big_endian(&self) -> i16 {
        return intrinsics::to_be16(*self);
    }

    pub fn to_little_endian(&self) -> i16 {
        return intrinsics::to_le16(*self);
    }
}

impl ByteSwap for i32 {
    pub fn to_big_endian(&self) -> i32 {
        return intrinsics::to_be32(*self);
    }

    pub fn to_little_endian(&self) -> i32 {
        return intrinsics::to_le32(*self);
    }
}

impl ByteSwap for i64 {
    pub fn to_big_endian(&self) -> i64 {
        return intrinsics::to_be64(*self);
    }

    pub fn to_little_endian(&self) -> i64 {
        return intrinsics::to_le64(*self);
    }
}

impl ByteSwap for f32 {
    pub fn to_big_endian(&self) -> f32 {
        return unsafe { cast::transmute(intrinsics::to_be32(cast::transmute(*self))) };
    }

    pub fn to_little_endian(&self) -> f32 {
        return unsafe { cast::transmute(intrinsics::to_le32(cast::transmute(*self))) };
    }
}

impl ByteSwap for f64 {
    pub fn to_big_endian(&self) -> f64 {
        return unsafe { cast::transmute(intrinsics::to_be64(cast::transmute(*self))) };
    }

    pub fn to_little_endian(&self) -> f64 {
        return unsafe { cast::transmute(intrinsics::to_le64(cast::transmute(*self))) };
    }
}

