#[deriving(Eq, Clone)]
pub enum Endian {
    BigEndian, LittleEndian
}

#[deriving(Eq, Clone)]
pub enum AudioSubtype {
    PCMStream(PCMFormat), FLACStream
}

#[deriving(Eq, Clone)]
pub struct AudioFormat {
    sample_rate: uint, channels: uint
}

#[deriving(Eq, Clone)]
pub enum StreamType {
    AudioStream(AudioSubtype, AudioFormat), BinaryStream
}

#[deriving(Eq, Clone)]
pub enum SampleType {
    Float(uint), Signed(uint), Unsigned(uint), ALaw, MuLaw
}

#[deriving(Eq, Clone)]
pub struct PCMFormat {
    sample_type: SampleType,
    endian: Endian
}
