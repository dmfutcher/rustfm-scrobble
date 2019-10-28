use std::error::Error as StdError;
use std::fmt;
use std::time::SystemTimeError;

/// Represents an Error that occurred while interacting with the Last.fm API
/// 
/// `ScrobblerError` contains an error message, which is set when an error occurs and exposed via Trait standard error
/// Trait implementations. 
/// 
/// Most error handling for clients can operate off the `Ok`/`Err` signaling from the `Result` types of API operations,
/// however this error type is exposed in case you want to implement more complex error handling.
#[derive(Debug)]
pub struct ScrobblerError {
    err_msg: String,
}

impl ScrobblerError {
    pub fn new(err_msg: String) -> Self {
        ScrobblerError { err_msg }
    }
}

impl fmt::Display for ScrobblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err_msg)
    }
}

impl StdError for ScrobblerError {
    fn description(&self) -> &str {
        self.err_msg.as_str()
    }

    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}

impl From<SystemTimeError> for ScrobblerError {
    fn from(error: SystemTimeError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<String> for ScrobblerError {
    fn from(error: String) -> Self {
        Self::new(error)
    }
}
