use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum LLAError {
    InvalidAlgorithm(String),
}

impl Display for LLAError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            &LLAError::InvalidAlgorithm(ref msg) => write!(f, "{}", msg),
        }
    }
}


impl Error for LLAError {
    fn description(&self) -> &str {
        match self {
            &LLAError::InvalidAlgorithm(ref msg) => msg.as_str(),
        }
    }
}
