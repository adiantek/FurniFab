mod deserializer;
mod error;
mod serializer;

use error::Error;

type Result<T> = std::result::Result<T, Error>;
