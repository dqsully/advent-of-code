use std::collections::HashMap;

use crate::error::Error;

pub struct SpringRow {
    source: String,
    match_str: String,
}

impl SpringRow {
    pub fn parse_line(line: &str, folds: usize) -> Result<SpringRow, Error> {
        let (source_folded, match_nums) = line.split_once(' ').ok_or(Error::InvalidLine)?;

        let mut source = String::new();
        let mut match_str = ".".to_owned();

        for _ in 0..folds {
            if !source.is_empty() {
                source.push('?');
            }

            source.push_str(source_folded);

            for num_str in match_nums.split(',') {
                for _ in 0..num_str.parse()? {
                    match_str.push('#');
                }

                match_str.push('.');
            }
        }

        Ok(SpringRow { source, match_str })
    }

    pub fn as_partial(&self) -> SpringPartial {
        SpringPartial {
            source: &self.source,
            match_str: &self.match_str,
            last_was_hash: false,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SpringPartial<'a> {
    pub source: &'a str,
    pub match_str: &'a str,
    pub last_was_hash: bool,
}

impl<'a> SpringPartial<'a> {
    pub fn compute_possibilities(&self, memoized: &mut HashMap<SpringPartial<'a>, u64>) -> u64 {
        if let Some(&memoized_possibilities) = memoized.get(self) {
            return memoized_possibilities;
        }

        let out = match self.match_str.as_bytes().first() {
            None => {
                u64::from(self.source.is_empty())
            }
            Some(b'.') => {
                let max_offset = self.source.len()
                    - match self.match_str.len() {
                        0 => unreachable!(),
                        1 => 0,
                        2 => panic!("len 2 match_str starts with '.'"),
                        3.. => self.match_str.len() - 2,
                    };

                let mut possibilities = 0;

                for offset in 0..=max_offset {
                    if !(offset == 0 && self.last_was_hash) || self.source.is_empty() {
                        possibilities += SpringPartial {
                            source: &self.source[offset..],
                            match_str: &self.match_str[1..],
                            last_was_hash: false,
                        }
                        .compute_possibilities(memoized);
                    }

                    if let Some(b'#') = self.source.as_bytes().get(offset) {
                        break;
                    }
                }

                possibilities
            }
            Some(b'#') => {
                let source_byte = self.source.as_bytes().first().copied();

                match source_byte {
                    Some(b'#' | b'?') => SpringPartial {
                        source: &self.source[1..],
                        match_str: &self.match_str[1..],
                        last_was_hash: true,
                    }
                    .compute_possibilities(memoized),
                    _ => 0,
                }
            }
            Some(byte) => panic!("unexpected match_str byte {byte}"),
        };

        memoized.insert(self.clone(), out);

        out
    }
}
