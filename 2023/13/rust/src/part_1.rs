use aoc_helpers::text_map::TextMap;

use crate::{error::Error, shared::{hash_columns, hash_rows}};

pub fn run(input: &str) -> Result<String, Error> {
    let boards = input.trim().split("\n\n").map(TextMap::parse);

    let mut sum = 0;

    for board in boards {
        let board = board?;

        let columns = hash_columns(&board);
        let rows = hash_rows(&board);

        if let Some(i) = find_reflection(&columns) {
            sum += i + 1;
        }
        if let Some(i) = find_reflection(&rows) {
            sum += (i + 1) * 100;
        }
    }

    Ok(sum.to_string())
}

fn find_reflection(input: &[u64]) -> Option<usize> {
    for (i, v) in input.windows(2).enumerate() {
        if v[0] == v[1] {
            // Possible reflection point
            let mut l = i;
            let mut r = i + 1;
            let mut is_reflection = true;

            while l > 0 && r < input.len() - 1 {
                l -= 1;
                r += 1;

                if input[l] != input[r] {
                    is_reflection = false;
                    break;
                }
            }

            if is_reflection {
                return Some(i);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

        let output = run(input);

        assert_eq!(output, Ok("405".to_owned()));
    }

    #[test]
    fn example_1() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        let output = run(input);

        assert_eq!(output, Ok("5".to_owned()));
    }

    #[test]
    fn example_2() {
        let input = r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

        let output = run(input);

        assert_eq!(output, Ok("400".to_owned()));
    }
}
