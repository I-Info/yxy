//! Application Error
use Error::*;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Runtime(String),
    Auth(String),
    AuthExpired,
    Request(reqwest::Error),
    EmptyResp,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IO(e) => write!(f, "IO error: {}", e),
            Runtime(e) => write!(f, "Runtime error: {}", e),
            Auth(e) => write!(f, "Authorization error: {}", e),
            Request(e) => write!(f, "Request error: {}", e),
            EmptyResp => write!(f, "Get empty response"),
            AuthExpired => write!(f, "Authorization expired"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Request(e)
    }
}
