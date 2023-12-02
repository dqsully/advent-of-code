use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError<'a> {
    #[error("no digits found in line {0:?}")]
    NoDigitsInLine(&'a str),
}

pub fn run(input: &str) -> Result<String, AocError> {
    let mut sum = 0;

    for line in input.lines() {
        sum += number_for_line(line)?;
    }

    Ok(sum.to_string())
}

fn number_for_line(line: &str) -> Result<u32, AocError> {
    let mut digits = LineDigits::new();

    for c in line.chars() {
        if let Some(digit) = c.to_digit(10) {
            digits.add_digit(digit);
        }
    }

    digits.get_number().ok_or(AocError::NoDigitsInLine(line))
}

struct LineDigits {
    first: Option<u32>,
    last: Option<u32>,
}

impl LineDigits {
    fn new() -> LineDigits {
        LineDigits {
            first: None,
            last: None,
        }
    }

    fn add_digit(&mut self, digit: u32) {
        self.first.get_or_insert(digit);
        self.last = Some(digit);
    }

    fn get_number(&self) -> Option<u32> {
        if let (Some(first), Some(last)) = (self.first, self.last) {
            Some(first * 10 + last)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let input = r"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
";

        let output = super::run(input);

        assert_eq!(output.unwrap(), "142");
    }
}
