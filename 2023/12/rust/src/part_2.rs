use std::collections::HashMap;

use crate::{error::Error, shared::SpringRow};

pub fn run(input: &str) -> Result<String, Error> {
    let mut total_possibilities = 0;

    let rows = input
        .lines()
        .map(|line| SpringRow::parse_line(line, 5))
        .collect::<Result<Vec<_>, Error>>()?;

    let mut memoized = HashMap::new();

    for row in &rows {
        total_possibilities += row.as_partial().compute_possibilities(&mut memoized);
    }

    Ok(total_possibilities.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

        let output = run(input);

        assert_eq!(output, Ok("525152".to_owned()));
    }

    macro_rules! line_test {
        ($suite:ident, $($name:ident: $input:expr => $expected:expr,)*) => {
            mod $suite {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        let row = SpringRow::parse_line($input, 5).unwrap();
                        let mut memo = HashMap::new();

                        assert_eq!(row.as_partial().compute_possibilities(&mut memo), $expected);
                    }
                )*
            }
        }
    }

    line_test!(line_tests,
        example_line_1: "???.### 1,1,3" => 1,
        example_line_2: ".??..??...?##. 1,1,3" => 16384,
        example_line_3: "?#?#?#?#?#?#?#? 1,3,1,6" => 1,
        example_line_4: "????.#...#... 4,1,1" => 16,
        example_line_5: "????.######..#####. 1,6,5" => 2500,
        example_line_6: "?###???????? 3,2,1" => 506_250,
    );
}
