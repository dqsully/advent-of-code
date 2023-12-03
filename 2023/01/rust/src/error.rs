use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error<'a> {
    #[error("no digits found in line {0:?}")]
    NoDigitsInLine(&'a str),
}
