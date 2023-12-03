use std::cmp::{min, max};
use std::collections::HashSet;
use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    let mut num_map: Vec<Vec<Option<usize>>> = Vec::new();
    let mut nums: Vec<usize> = Vec::new();

    let mut num_txt = String::new();

    for (y, line) in input.lines().enumerate() {
        let mut num_row = Vec::new();

        for (x, ch) in line.chars().enumerate() {
            match ch {
                '0' ..= '9' => {
                    num_txt.push(ch);
                    num_row.push(Some(nums.len()));
                },
                _ => {
                    num_row.push(None);

                    if !num_txt.is_empty() {
                        nums.push(num_txt.parse().unwrap());
                        num_txt.clear();
                    }
                }
            }
        }

        num_map.push(num_row);
    }

    let mut part_ids = HashSet::new();
    let row_len = num_map[0].len();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '0' ..= '9' | '.' => {
                    // Do nothing
                }
                _ => {
                    // Symbol
                    for y_o in max(1, y) - 1 ..= min(num_map.len() - 2, y) + 1 {
                        for x_o in max(1, x) - 1 ..= min(row_len - 2, x) + 1 {
                            if let Some(id) = num_map[y_o][x_o] {
                                part_ids.insert(id);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut sum = 0;

    for &id in &part_ids {
        sum += nums[id];
    }

    return Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

        let output = run(input);

        assert_eq!(output, Ok("4361".to_owned()));
    }
}
