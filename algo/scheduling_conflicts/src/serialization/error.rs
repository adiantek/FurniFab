use serde::{Deserialize, Serialize};
use std::char::ParseCharError;
use std::fmt::Display;
use std::io::Error as IOError;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;
use thiserror::Error;

/// Enum representing serializing and deserializing errors.
#[derive(Clone, Debug, Deserialize, Eq, Error, PartialEq, Serialize)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    #[error("deserialization any type is not supported")]
    AnyNotSupported,
    #[error("encountered an unexpected end of input")]
    EndOfInput,
    #[error("encountered an unexpected empty line")]
    EmptyLine,
    #[error("invalid hex length (must be multiple of 2)")]
    InvalidHexLength,
    #[error("expected a unit value '-'")]
    ExpectedUnit,
}

impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error::Message(error.to_string())
    }
}

impl From<ParseBoolError> for Error {
    fn from(error: ParseBoolError) -> Self {
        Error::Message(error.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::Message(error.to_string())
    }
}

impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Self {
        Error::Message(error.to_string())
    }
}

impl From<ParseCharError> for Error {
    fn from(error: ParseCharError) -> Self {
        Error::Message(error.to_string())
    }
}
