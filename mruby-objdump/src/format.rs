extern crate simple_endian;

use std::mem;

use crate::error::Error;

use plain::Plain;
use simple_endian::u32be;

#[repr(C)]
#[derive(Debug)]
pub struct RiteBinaryHeader {
    pub ident: [u8; 4],
    pub major_version: [u8; 2],
    pub minor_version: [u8; 2],
    pub size: [u8; 4],
    pub compiler_name: [u8; 4],
    pub compiler_version: [u8; 4],
}
unsafe impl Plain for RiteBinaryHeader {}

impl RiteBinaryHeader {
    fn from_bytes(buf: &[u8]) -> Result<&Self, Error> {
        plain::from_bytes(buf).map_err(|_| Error::General)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SectionMiscHeader {
    pub ident: [u8; 4],
    pub size: [u8; 4],
}
unsafe impl Plain for SectionMiscHeader {}

#[repr(C)]
#[derive(Debug)]
pub struct SectionIrepHeader {
    pub ident: [u8; 4],
    pub size: [u8; 4],

    pub rite_version: [u8; 4],
}
unsafe impl Plain for SectionIrepHeader {}

pub fn load(src: &[u8]) -> Result<(), Error> {
    let mut size = src.len();
    let binheader_size = mem::size_of::<RiteBinaryHeader>();
    if size < binheader_size {
        return Err(Error::TooShort);
    }
    let bin_header = RiteBinaryHeader::from_bytes(&src[0..binheader_size])?;
    size -= binheader_size;

    dbg!(bin_header);
    let binsize_be = unsafe { mem::transmute::<[u8; 4], u32be>(bin_header.size) };
    let binsize: u32 = binsize_be.into();
    eprintln!("size");
    eprintln!("{}", binsize);

    Ok(())
}
