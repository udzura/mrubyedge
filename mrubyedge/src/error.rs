use std::error;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    General,
    InvalidOpCode,
    RuntimeError(String),
    TypeMismatch,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error nr {}", self)
    }
}

impl error::Error for Error {}
