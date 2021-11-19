use std::fmt;

#[derive(Debug)]
pub enum Error {
    Fetch(String, reqwest::Error),
    BadStatus(String, reqwest::StatusCode),
    EmptyBody(String),
    ResponseBody(String, reqwest::Error),
    Deserialization(String, serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::error::Error::*;
        match self {
            Fetch(what, err) => write!(f, "Error while fetching {} from API: {}", what, err),
            BadStatus(what, status) => write!(f, "Got bad status code ({}): {}", what, status),
            EmptyBody(what) => write!(f, "Got response with empty body for {}", what),
            ResponseBody(what, err) => write!(f, "Failed to get response body ({}): {}", what, err),
            Deserialization(what, err) => {
                write!(f, "Error while deserializing ({}): {}", what, err)
            }
        }
    }
}

impl std::error::Error for Error {}
