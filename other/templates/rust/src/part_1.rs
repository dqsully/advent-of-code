use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    Err(Error::Unimplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"";

        let output = run(input);

        assert_eq!(output, Ok("".to_owned()));
    }
}
