use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("{0}")]
    HelperError(#[from] aoc_helpers::Error),
    #[error("failed parsing number: {0}")]
    FailedNumberParsing(#[from] ParseIntError),
}
