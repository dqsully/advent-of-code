use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("{0}")]
    AocHelper(#[from] aoc_helpers::Error),

    #[error("expansion factor of 0 not allowed (it would collapse the universe and make everything crash into each-other, destroying everything you ever knew and loved...)")]
    NoExpansion,
}
