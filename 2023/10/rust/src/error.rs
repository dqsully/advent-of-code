use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("{0}")]
    AocHelper(
        #[from]
        aoc_helpers::Error,
    ),

    #[error("unimplemented")]
    Unimplemented,
}
