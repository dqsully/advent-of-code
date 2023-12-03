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

    macro_rules! number_for_line_test_happy {
        ($suite:ident, $($name:ident: $input:expr => $expected:expr,)*) => {
            mod $suite {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        let output = number_for_line($input);

                        assert_eq!(output.unwrap(), $expected);
                    }
                )*
            }
        }
    }

    number_for_line_test_happy!(number_for_line_tests,
        example_line_1: "two1nine" => 29,
        example_line_2: "eightwothree" => 83,
        example_line_3: "abcone2threexyz" => 13,
        example_line_4: "xtwone3four" => 24,
        example_line_5: "4nineeightseven2" => 42,
        example_line_6: "zoneight234" => 14,
        example_line_7: "7pqrstsixteen" => 76,

        overlap_oneight: "oneight" => 18,
        overlap_twone: "twone" => 21,
        overlap_threeight: "threeight" => 38,
        overlap_fiveight: "fiveight" => 58,
        overlap_sevenine: "sevenine" => 79,
        overlap_eightwo: "eightwo" => 82,
        overlap_eighthree: "eighthree" => 83,
        overlap_nineight: "nineight" => 98,
        overlap_twoneight: "twoneight" => 28,
        overlap_eightwoneight: "eightwoneight" => 88,
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
