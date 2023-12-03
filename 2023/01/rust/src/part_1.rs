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

    for c in line.chars() {
        if let Some(digit) = c.to_digit(10) {
            digits.add_digit(digit);
        }
    }

    digits.get_number().ok_or(Error::NoDigitsInLine(line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
";

        let output = run(input);

        assert_eq!(output, Ok("142".to_owned()));
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
        example_line_1: "1abc2" => Ok(12),
        example_line_2: "pqr3stu8vwx" => Ok(38),
        example_line_3: "a1b2c3d4e5f" => Ok(15),
        example_line_4: "treb7uchet" => Ok(77),

        no_numbers: "nonumbers" => Err(Error::NoDigitsInLine("nonumbers")),
    );
}
