use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("{0}")]
    AocHelper(#[from] aoc_helpers::Error),

    #[error("could not find map start")]
    StartNotFound,

    #[error("could not infer starting directions")]
    StartInferFailed,

    #[error("wall doesn't loop properly")]
    InvalidWall,
}
