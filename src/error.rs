use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    NoPlayerFound
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoPlayerFound => f.write_str("Could not find player"),
        }
    }
}

impl std::error::Error for Error {}