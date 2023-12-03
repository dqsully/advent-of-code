use crate::error::Error;
use crate::shared::LineDigits;

pub fn run(input: &str) -> Result<String, Error> {
    let mut sum = 0;

    for line in input.lines() {
        sum += number_for_line(line)?;
    }

    Ok(sum.to_string())
}

fn number_for_line(line: &str) -> Result<u32, Error> {
    let mut digits = LineDigits::new();

    for (line_idx, c) in line.char_indices() {
        if let Some(digit) = c.to_digit(10).or_else(|| digit_from_str(&line[line_idx..])) {
            digits.add_digit(digit);
        }
    }

    digits.get_number().ok_or(Error::NoDigitsInLine(line))
}

const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn digit_from_str(partial_line: &str) -> Option<u32> {
    for (num_idx, &num_text) in NUMBER_WORDS.iter().enumerate() {
        if partial_line.starts_with(num_text) {
            #[allow(clippy::cast_possible_truncation)]
            return Some(num_idx as u32 + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
";

        let output = run(input);

        assert_eq!(output.unwrap(), "281");
    }
}
