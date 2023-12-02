use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("unimplemented")]
    Unimplemented,
}

pub fn run(input: &str) -> Result<String, AocError> {
    Err(AocError::Unimplemented)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let input = r"";

        let output = super::run(input);

        assert_eq!(output.unwrap(), "");
    }
}
