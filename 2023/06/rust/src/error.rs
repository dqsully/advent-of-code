use std::num::ParseFloatError;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("missing line header")]
    MissingLineHeader,

    #[error("parse float error: {0}")]
    F64ParseError(#[from] ParseFloatError),
}
