use std::fmt::{Display, Formatter};

pub enum Return<T> {
    Explicit(T),
    Implicit(T),
    Error(T),
}

impl<T: Display> Display for Return<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Return::Explicit(value) | Return::Implicit(value) | Return::Error(value) => {
                value.fmt(f)
            }
        }
    }
}

#[macro_export]
macro_rules! return_value {
    ($result:expr) => {
        match $result {
            Return::Implicit(value) => value,
            result => return result,
        }
    };
}
