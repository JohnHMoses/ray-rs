use std::error::Error;
use std::fmt;
use std::string::String;

#[derive(Debug)]
pub struct TokenizationError {
    message: String
}

impl TokenizationError {
    pub fn new<S: Into<String>>(message: S) -> TokenizationError {
        TokenizationError{message: message.into()}
    }
}

impl Error for TokenizationError {
    fn description(&self) -> &str {
        return self.message.as_ref();
    }
}

impl fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something went wrong")
    }
}