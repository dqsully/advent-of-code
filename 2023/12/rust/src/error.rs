use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("{0}")]
    AocHelper(#[from] aoc_helpers::Error),

    #[error("invalid line format")]
    InvalidLine,

    #[error("failed parsing number: {0}")]
    FailedNumberParsing(#[from] ParseIntError),
}
