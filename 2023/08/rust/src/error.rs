use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("invalid map format")]
    InvalidMapFormat,
}
