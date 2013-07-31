#[deriving(Eq)]
pub enum Endian {
    BigEndian, LittleEndian
}

#[deriving(Eq)]
pub enum AudioSubtype {
    PCMStream(PCMFormat)
}

#[deriving(Eq)]
pub struct AudioFormat {
    sample_rate: uint, channels: uint
}

#[deriving(Eq)]
pub enum StreamType {
    AudioStream(AudioSubtype, AudioFormat), BinaryStream
}

#[deriving(Eq)]
pub enum SampleType {
    Float(uint), Signed(uint), Unsigned(uint), ALaw, MuLaw
}

#[deriving(Eq)]
pub struct PCMFormat {
    sample_type: SampleType,
    endian: Endian
}
