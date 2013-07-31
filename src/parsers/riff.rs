use std::io;
use std::sys;

use fourcc::FourCC;
use result::{Ok, Error, Result};

pub struct RIFFChunk {
    fourcc: FourCC,
    size:u32
}

pub struct RIFFList {
    fourcc:FourCC,
    size:u32,
    list_type:FourCC
}

impl RIFFChunk {
    fn is_list(&self) -> bool {
        self.fourcc == fcc!("LIST") // TODO: Add magic number
    }
}

impl RIFFList {
    fn is_list(&self) -> bool {
        self.fourcc == fcc!("LIST") // TODO: Add magic number
    }
}

pub struct RIFFParser {
    stream:@Reader,
    riff_id:FourCC,
    riff_type:FourCC,

    container_size:u64,
    container_offset:u64,

    current_chunk:RIFFChunk,
    current_chunk_offset:u64,

    bytes_remaining:u64
}

impl RIFFParser {
    pub fn new(stream:@Reader, id:FourCC, container_offset:u64) -> (Result<uint>, Option<RIFFParser>) {
        let chunk = RIFFChunk { fourcc:0, size:0 };
        let mut parser = RIFFParser {
            stream:stream, riff_id:id, riff_type:0,
            container_offset:container_offset, container_size:0,
            current_chunk:chunk, current_chunk_offset:0,
            bytes_remaining:0
        };

        let result = parser.read_riff_header();

        return match result {
            Ok => (Ok, Some(parser)),
            err => (err, None)
        };
    }

    fn chunk_actual_size(&self) -> u64 {
        return (sys::size_of::<RIFFChunk>() as u64) + (self.current_chunk.size as u64)
    }

    fn read_riff_header(&mut self) -> Result<uint> {
        if self.container_offset % 2 != 0 { // RIFF chunks are 2-byte aligned
            return Error(0); // TODO: Magic number
        }

        if self.container_offset < 0 { // Container offset should be positive
            return Error(0); // TODO: Magic number
        }

        // TODO: Should probably check for size overflow here

        self.stream.seek(self.container_offset as int, io::SeekSet);

        let header = RIFFList { // TODO: Should change to a non-blocking stream implementation? WTF happens on failure?
            fourcc:self.stream.read_be_u32(), // TODO: Read FourCC?
            size:self.stream.read_le_u32(),
            list_type:self.stream.read_be_u32() // TODO: Read FourCC?
        };

        if header.fourcc != self.riff_id {
            return Error(0); // TODO: Magic number
        }

        self.riff_type = header.list_type;
        self.container_size = (header.size as u64) + (sys::size_of::<RIFFChunk>() as u64);
        self.current_chunk_offset = self.container_offset + (sys::size_of::<RIFFList>() as u64);

        return self.read_chunk_header();
    }

    fn read_chunk_header(&mut self) -> Result<uint> {
        // TODO: Should probably check for size overflow here

        self.current_chunk = RIFFChunk { // TODO: Should change to a non-blocking stream implementation? WTF happens on failure?
            fourcc:self.stream.read_be_u32(), // TODO: Read FourCC?
            size:self.stream.read_le_u32()
        };

        self.bytes_remaining = self.current_chunk.size as u64;

        return Ok;
    }

    pub fn move_to_next_chunk(&mut self) -> Result<uint> {
        // TODO: Check that current_chunk_offset > container_offset
        // TODO: Check that current_chunk_offset >= 0
        // TODO: Check that container_offset >= 0

        self.current_chunk_offset += self.chunk_actual_size();

        // Are we at the end of the RIFF?
        if (self.current_chunk_offset - self.container_offset) >= self.container_size {
            return Error(1);
        }

        // TODO: Check for overflow?

        self.stream.seek(self.current_chunk_offset as int, io::SeekSet);

        match self.read_chunk_header() {
            Ok => (),
            err => return err
        }

        let max_chunk_size = self.container_size - (self.current_chunk_offset - self.container_offset);
        
        if max_chunk_size < self.chunk_actual_size() {
            return Error(0);
        }

        self.bytes_remaining = (self.current_chunk.size as u64);

        return Ok;
    }

    fn move_to_chunk_offset(&mut self, offset:u64) -> Result<uint> {
        if offset > (self.current_chunk.size as u64){
            return Error(0)
        }

        self.stream.seek((self.current_chunk_offset + offset + (sys::size_of::<RIFFChunk>() as u64)) as int, io::SeekSet);
        self.bytes_remaining = (self.current_chunk.size as u64) - offset;

        return Ok;
    }

    fn move_to_start_of_chunk(&mut self) -> Result<uint> {
        return self.move_to_chunk_offset(0);
    }

    pub fn read_data_from_chunk(&mut self, length:u64, data:&mut [u8]) -> (Result<uint>, u64) {
        if length > self.bytes_remaining {
            return (Error(0), 0);
        }

        let read = self.stream.read(data, length as uint) as u64; // TODO: Fix streams to use u64…

        self.bytes_remaining -= read;

        return (Ok, read);
    }
}
