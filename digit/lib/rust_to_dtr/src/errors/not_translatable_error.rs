use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum NotTranslatableError {
    Custom(String),
}

// Implementing `std::fmt::Display` for user-friendly error messages
impl fmt::Display for NotTranslatableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotTranslatableError::Custom(ref message) => {
                write!(f, "NotTranslatableError: {}", message)
            }
        }
    }
}

// Implementing `std::error::Error` for interoperability with other error handling code
impl Error for NotTranslatableError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
