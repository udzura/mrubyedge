extern crate plain;
use plain::Plain;

use super::Error;

#[repr(C)]
#[derive(Debug, Clone, Default)]
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
    pub fn from_bytes(buf: &[u8]) -> Result<Self, Error> {
        plain::from_bytes(buf).map_err(|_| Error::General).cloned()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SectionMiscHeader {
    pub ident: [u8; 4],
    pub size: [u8; 4],
}
unsafe impl Plain for SectionMiscHeader {}

impl SectionMiscHeader {
    pub fn from_bytes(buf: &[u8]) -> Result<Self, Error> {
        plain::from_bytes(buf).map_err(|_| Error::General).cloned()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SectionIrepHeader {
    pub ident: [u8; 4],
    pub size: [u8; 4],

    pub rite_version: [u8; 4],
}
unsafe impl Plain for SectionIrepHeader {}

impl SectionIrepHeader {
    pub fn from_bytes(buf: &[u8]) -> Result<Self, Error> {
        plain::from_bytes(buf).map_err(|_| Error::General).cloned()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IrepRecord {
    pub size: [u8; 4],
    pub nlocals: [u8; 2],
    pub nregs: [u8; 2],
    pub rlen: [u8; 2],
    pub clen: [u8; 2],
    pub ilen: [u8; 4],
}

unsafe impl Plain for IrepRecord {}

impl IrepRecord {
    pub fn from_bytes(buf: &[u8]) -> Result<Self, Error> {
        plain::from_bytes(buf).map_err(|_| Error::General).cloned()
    }
}
