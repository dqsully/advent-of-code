use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("inconsistent text map width")]
    InconsistentMapWidth,
}
