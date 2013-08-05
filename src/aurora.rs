#[link(name = "aurora",vers = "0.1", uuid = "6a2ec5f9-3d3d-41a4-8109-4c66a9660323")];

#[license = "MIT"];
#[crate_type = "lib"];

#[author = "Jens Nockert"];
#[comment = "A media framework for Rust"];

#[macro_escape]
mod macros {
    macro_rules! fcc(($a:expr) => ({
        let b = bytes!($a); (b[0] as u32 << 24) | (b[1] as u32 << 16) | (b[2] as u32 << 8) | (b[3] as u32)
    }))
}

pub mod align;
pub mod attribute;
pub mod byteswap;
pub mod fourcc;
pub mod result;
pub mod types;

pub mod buffers {
    pub mod buffer;
    pub mod memory;
}

pub mod events {
    pub mod event;
}

pub mod io {
    pub mod file;
    pub mod read;
    pub mod seek;
    pub mod write;
}

pub mod parsers {
    pub mod riff;
    pub mod wav;
}

pub mod samples {
    pub mod sample;
}

pub mod sinks {
    pub mod sink;

    pub mod au;
    pub mod wav;
}

pub mod sources {
    pub mod source;
    pub mod wav;
}

pub mod streams {
    pub mod stream;
    pub mod wav;
}

pub mod transforms {
    pub mod transform;
    pub mod pcm;
}
