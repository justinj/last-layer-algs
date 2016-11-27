use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum LLAError {
    InvalidAlgorithm(String),
    IOError(::std::io::Error),
}

impl Display for LLAError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            &LLAError::InvalidAlgorithm(ref msg) => write!(f, "Invalid algorithm: {}", msg),
            &LLAError::IOError(ref err) => write!(f, "{}", err),
        }
    }
}


impl Error for LLAError {
    fn description(&self) -> &str {
        match self {
            &LLAError::InvalidAlgorithm(ref msg) => msg.as_str(),
            &LLAError::IOError(ref err) => err.description(),
        }
    }
}

impl From<::std::io::Error> for LLAError {
    fn from(err: ::std::io::Error) -> LLAError {
        LLAError::IOError(err)
    }
}
