use std::collections::HashMap;

use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    let mut total_possibilities = 0;

    let rows = input.lines().map(SpringRow::parse_line).collect::<Vec<_>>();

    let mut memoized = HashMap::new();

    for row in &rows {
        total_possibilities += row.as_partial().compute_possibilities(&mut memoized);
    }

    Ok(total_possibilities.to_string())
}

struct SpringRow{
    source: String,
    match_str: String,
}

impl SpringRow {
    fn parse_line(line: &str) -> SpringRow {
        let (source_folded, match_nums) = line.split_once(' ').unwrap();

        let mut source = String::new();
        let mut match_str = ".".to_owned();

        for _ in 0..5 {
            if !source.is_empty() {
                source.push('?');
            }

            source.push_str(source_folded);

            for num_str in match_nums.split(',') {
                for _ in 0..num_str.parse().unwrap() {
                    match_str.push('#');
                }

                match_str.push('.');
            }
        }

        SpringRow {source, match_str}
    }

    fn as_partial(&self) -> SpringPartial {
        SpringPartial {
            source: &self.source,
            match_str: &self.match_str,
            last_was_hash: false,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct SpringPartial<'a>{
    source: &'a str,
    match_str: &'a str,
    last_was_hash: bool,
}

impl<'a> SpringPartial<'a> {
    fn compute_possibilities(&self, memoized: &mut HashMap<SpringPartial<'a>, u64>) -> u64 {
        if let Some(&memoized_possibilities) = memoized.get(self) {
            return memoized_possibilities;
        }

        let out = match self.match_str.as_bytes().first() {
            None => {
                if self.source.is_empty() {
                    1
                } else {
                    0
                }
            }
            Some(b'.') => {
                let max_offset = self.source.len() - match self.match_str.len() {
                    0 => unreachable!(),
                    1 => 0,
                    2 => panic!("len 2 match_str starts with '.'"),
                    3.. => self.match_str.len() - 2
                };

                let mut possibilities = 0;

                for offset in 0..=max_offset {

                    if !(offset == 0 && self.last_was_hash) || self.source.is_empty() {
                        possibilities += SpringPartial{
                            source: &self.source[offset..],
                            match_str: &self.match_str[1..],
                            last_was_hash: false,
                        }.compute_possibilities(memoized);
                    }

                    if let Some(b'#') = self.source.as_bytes().get(offset) {
                        break;
                    }
                }

                possibilities
            },
            Some(b'#') => {
                let source_byte = self.source.as_bytes().first().copied();

                match source_byte {
                    Some(b'#') | Some(b'?') => SpringPartial{
                        source: &self.source[1..],
                        match_str: &self.match_str[1..],
                        last_was_hash: true,
                    }.compute_possibilities(memoized),
                    _ => 0
                }
            }
            Some(byte) => panic!("unexpected match_str byte {byte}"),
        };

        memoized.insert(self.clone(), out);

        out
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

        assert_eq!(output, Ok("525152".to_owned()));
    }

    macro_rules! line_test {
        ($suite:ident, $($name:ident: $input:expr => $expected:expr,)*) => {
            mod $suite {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        let row = SpringRow::parse_line($input);
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
        example_line_6: "?###???????? 3,2,1" => 506250,
    );
}
