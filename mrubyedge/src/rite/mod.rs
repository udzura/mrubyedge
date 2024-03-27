pub mod binfmt;
pub mod insn;
pub mod marker;
pub mod rite;

pub use rite::*;

use std::error;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    General,
    TooShort,
    InvalidFormat,
    InvalidOpCode,
    TypeMismatch,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error nr {}", self)
    }
}

impl error::Error for Error {}
