use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

/// Enum representing serializing and deserializing errors.
#[derive(Clone, Debug, Deserialize, Eq, Error, PartialEq, Serialize)]
pub enum Error {
    #[error("{0}")]
    Message(String),
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
