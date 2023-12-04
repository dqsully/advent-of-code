use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error<'a> {
    #[error("failed parsing number: {0}")]
    FailedNumberParsing(#[from] ParseIntError),

    #[error("no card header found in line {0:?}")]
    NoCardHeaderInLine(&'a str),

    #[error("no winning/drawn card separator in {0:?}")]
    NoCardSeparator(&'a str),
}
