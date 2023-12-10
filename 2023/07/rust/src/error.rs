use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("invalid hand")]
    InvalidHand,

    #[error("failed parsing number: {0}")]
    FailedNumberParsing(#[from] ParseIntError),
}
