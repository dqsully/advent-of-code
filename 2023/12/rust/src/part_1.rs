use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    let mut total_possibilities = 0;

    for line in input.lines() {
        let line_possibilities = SpringRow::parse_line(line).compute_possibilities();

        total_possibilities += line_possibilities;
    }

    Ok(total_possibilities.to_string())
}

struct SpringRow<'a, T: AsRef<str>>(&'a str, T, bool);

impl<'a> SpringRow<'a, String> {
    fn parse_line(line: &str) -> SpringRow<String> {
        let (source, match_nums) = line.split_once(' ').unwrap();

        let mut match_str = ".".to_owned();

        for num_str in match_nums.split(',') {
            for _ in 0..num_str.parse().unwrap() {
                match_str.push('#');
            }

            match_str.push('.');
        }

        SpringRow(source, match_str, false)
    }
}

impl<'a, T: AsRef<str>> SpringRow<'a, T> {
    fn compute_possibilities(&self) -> u32 {
        let source = self.0;
        let match_str = self.1.as_ref();
        let last_was_hash = self.2;

        match match_str.as_bytes().first() {
            None => {

                if source.is_empty() {
                    1
                } else {
                    0
                }
            }
            Some(b'.') => {
                let max_offset = source.len() - match match_str.len() {
                    0 => unreachable!(),
                    1 => 0,
                    2 => panic!("len 2 match_str starts with '.'"),
                    3.. => match_str.len() - 2
                };

                let mut possibilities = 0;

                for offset in 0..=max_offset {

                    if !(offset == 0 && last_was_hash) || source.is_empty() {
                        possibilities += SpringRow(&source[offset..], &match_str[1..], false).compute_possibilities();
                    }

                    if let Some(b'#') = source.as_bytes().get(offset) {
                        break;
                    }
                }

                possibilities
            },
            Some(b'#') => {
                let source_byte = source.as_bytes().first().copied();

                match source_byte {
                    Some(b'#') | Some(b'?') => SpringRow(&source[1..], &match_str[1..], true).compute_possibilities(),
                    _ => 0
                }
            }
            Some(byte) => panic!("unexpected match_str byte {byte}"),
        }
    }
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

        assert_eq!(output, Ok("21".to_owned()));
    }

    macro_rules! line_test {
        ($suite:ident, $($name:ident: $input:expr => $expected:expr,)*) => {
            mod $suite {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        let row = SpringRow::parse_line($input);

                        assert_eq!(row.compute_possibilities(), $expected);
                    }
                )*
            }
        }
    }

    line_test!(line_tests,
        example_line_1: "???.### 1,1,3" => 1,
        example_line_2: ".??..??...?##. 1,1,3" => 4,
        example_line_3: "?#?#?#?#?#?#?#? 1,3,1,6" => 1,
        example_line_4: "????.#...#... 4,1,1" => 1,
        example_line_5: "????.######..#####. 1,6,5" => 4,
        example_line_6: "?###???????? 3,2,1" => 10,
    );
}
