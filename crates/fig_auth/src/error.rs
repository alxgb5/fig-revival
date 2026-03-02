//! Fig Auth Error - Local stub (AWS removed)

use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    Io(std::io::Error),
    Settings(fig_settings::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Generic(msg) => write!(f, "Auth error: {}", msg),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Settings(err) => write!(f, "Settings error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Generic(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Generic(s.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<fig_settings::Error> for Error {
    fn from(err: fig_settings::Error) -> Self {
        Error::Settings(err)
    }
}
