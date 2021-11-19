use std::fmt;

#[derive(Debug)]
pub enum Error {
    Fetch(reqwest::Error),
    Deserialization(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::error::Error::*;
        match self {
            Fetch(err) => write!(f, "Error while fetching from API: {}", err),
            Deserialization(err) => write!(f, "Error while deserializing: {}", err),
        }
    }
}

impl std::error::Error for Error {}
