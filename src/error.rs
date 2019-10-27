use std::error::Error as StdError;
use std::fmt;
use std::time::SystemTimeError;

#[derive(Debug)]
pub struct Error {
    err_msg: String,
}

impl Error {
    pub fn new(err_msg: String) -> Self {
        Self { err_msg }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err_msg)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.err_msg.as_str()
    }

    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}

impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Self::new(error)
    }
}
