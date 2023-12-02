use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("no digits found in line {0:?}")]
    NoDigitsInLine(&'a str),
}
