use std::num::ParseIntError;

use serde::Serialize;
use serde_json::Error as SerdeError;
use thiserror::Error;

pub mod bin_packing;
pub mod conflicts;
pub mod data;
pub mod flow;
pub mod python3api;
pub mod max_flow_min_cost;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    #[error("Error during serialization: {0}")]
    Serde(String),
    #[error("Error from python: {0}")]
    Python(String),
    #[error("Algorithm failed to produce a valid schedule. This is a bug.")]
    InvalidSchedule,
    #[error("Import / export error: {0}")]
    ImportExport(String),
    #[error("No path provided")]
    NoPath,
}

impl Serialize for Error {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Self {
        Error::Serde(error.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::Serde(error.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::ImportExport(error.to_string())
    }
}
