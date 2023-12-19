use std::error;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    General,
    TooShort,
    InvalidFormat,
    InvalidOpCode,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error nr {}", self)
    }
}

impl error::Error for Error {}
