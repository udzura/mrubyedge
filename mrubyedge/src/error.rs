use std::error;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    General,
    InvalidOpCode,
    RuntimeError(String),
    TypeMismatch,
    NoMethodError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error nr {}", self)
    }
}

impl error::Error for Error {}

impl Error {
    pub fn message(&self) -> String{
        match self {
            Error::General => "General error".to_string(),
            Error::InvalidOpCode => "Invalid opcode".to_string(),
            Error::RuntimeError(msg) => msg.clone(),
            Error::TypeMismatch => "Type mismatch".to_string(),
            Error::NoMethodError(msg) => format!("Method not found: {}", msg),
        }
    }
}