use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("no smallest number found")]
    NoSmallestFound,
}
