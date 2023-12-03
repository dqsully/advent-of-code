use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("no game header found in line {0:?}")]
    NoGameHeaderInLine(&'a str),

    #[error("invalid game header {0:?}")]
    InvalidGameHeader(&'a str),

    #[error("invalid cube set {0:?}")]
    InvalidCubeSet(&'a str),

    #[error("unknown color `{0}`")]
    UnknownColor(&'a str),
}
