use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("unimplemented")]
    Unimplemented,
}

pub fn run(input: &str) -> Result<String, AocError> {
    Err(AocError::Unimplemented)
}
