use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("inconsistent schematic width")]
    InconsistentSchematicWidth,
    #[error("failed parsing number: {0}")]
    FailedNumberParsing(#[from] ParseIntError),
}
