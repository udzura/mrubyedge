pub mod eval;
pub mod klass;
pub mod rite;
pub mod value;
pub mod vm;

pub mod mrb_helper;

use std::error;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    General,
    InvalidOpCode,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error nr {}", self)
    }
}

impl error::Error for Error {}
