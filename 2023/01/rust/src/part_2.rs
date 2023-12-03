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
    pub use crate::error::Error;

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

        assert_eq!(output, Ok("281".to_owned()));
    }

    macro_rules! number_for_line_test {
        ($suite:ident, $($name:ident: $input:expr => $expected:expr,)*) => {
            mod $suite {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        let output = number_for_line($input);

                        assert_eq!(output, $expected);
                    }
                )*
            }
        }
    }

    number_for_line_test!(number_for_line_tests,
        example_line_1: "two1nine" => Ok(29),
        example_line_2: "eightwothree" => Ok(83),
        example_line_3: "abcone2threexyz" => Ok(13),
        example_line_4: "xtwone3four" => Ok(24),
        example_line_5: "4nineeightseven2" => Ok(42),
        example_line_6: "zoneight234" => Ok(14),
        example_line_7: "7pqrstsixteen" => Ok(76),

        overlap_oneight: "oneight" => Ok(18),
        overlap_twone: "twone" => Ok(21),
        overlap_threeight: "threeight" => Ok(38),
        overlap_fiveight: "fiveight" => Ok(58),
        overlap_sevenine: "sevenine" => Ok(79),
        overlap_eightwo: "eightwo" => Ok(82),
        overlap_eighthree: "eighthree" => Ok(83),
        overlap_nineight: "nineight" => Ok(98),
        overlap_twoneight: "twoneight" => Ok(28),
        overlap_eightwoneight: "eightwoneight" => Ok(88),

        no_numbers: "nonumbers" => Err(Error::NoDigitsInLine("nonumbers")),
    );

    macro_rules! digit_from_str_test {
        ($suite:ident, $($name:ident: $input:expr => $expected:expr,)*) => {
            mod $suite {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        let output = digit_from_str($input);

                        assert_eq!(output, $expected);
                    }
                )*
            }
        }
    }

    digit_from_str_test!(digit_from_str_tests,
        one: "one" => Some(1),
        two: "two" => Some(2),
        three: "three" => Some(3),
        four: "four" => Some(4),
        five: "five" => Some(5),
        six: "six" => Some(6),
        seven: "seven" => Some(7),
        eight: "eight" => Some(8),
        nine: "nine" => Some(9),

        must_be_at_start: "jibberishone" => None,
    );
}
