use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unimplemented")]
    Unimplemented,
}
