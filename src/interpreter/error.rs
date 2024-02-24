use std::fmt::{Display, Formatter};

use super::return_value::Return;

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn throw<T>(message: impl ToString) -> Return<T> {
        Return::Error(Self::new(message))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERROR: {}", self.message)
    }
}
