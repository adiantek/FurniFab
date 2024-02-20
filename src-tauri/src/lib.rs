pub mod conflicts;
pub mod flow;
pub(crate) mod python;

use pyo3::PyErr;
use serde::Serialize;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq, Serialize)]
pub enum Error {
    #[error("Error during serialization: {0}")]
    Serde(String),
    #[error("Error from python: {0}")]
    Python(String),
    #[error("Algorithm failed to produce a valid schedule. This is a bug.")]
    InvalidSchedule,
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Self {
        Error::Serde(error.to_string())
    }
}

impl From<PyErr> for Error {
    fn from(error: PyErr) -> Self {
        Error::Python(error.to_string())
    }
}
