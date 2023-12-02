use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    Err(Error::Unimplemented)
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
