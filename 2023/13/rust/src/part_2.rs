use std::hash::Hasher;

use aoc_helpers::{neighbors::Grid2D, text_map::TextMap};

use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    let boards = input
        .trim()
        .split("\n\n")
        .map(TextMap::parse);

    let mut sum = 0;

    for board in boards {
        let board = board?;

        let columns = (0..board.width())
            .map(|x| {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();

                for y in 0..board.height() {
                    hasher.write_u8(*board.get(x, y).unwrap());
                }

                hasher.finish()
            })
            .collect::<Vec<u64>>();
        let rows = (0..board.height())
            .map(|y| {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();

                for x in 0..board.width() {
                    hasher.write_u8(*board.get(x, y).unwrap());
                }

                hasher.finish()
            })
            .collect::<Vec<u64>>();

        let possible_vertical_reflections = find_possible_smudged_reflections(&columns);
        let possible_horizontal_reflections = find_possible_smudged_reflections(&rows);

        for &(i, l, r) in &possible_vertical_reflections {
            let mut errors = 0;

            for y in 0..board.height() {
                if board.get(l, y) != board.get(r, y) {
                    errors += 1;

                    if errors > 1 {
                        break;
                    }
                }
            }

            if errors == 1 {
                sum += i + 1;
                break;
            }
        }

        for &(i, t, b) in &possible_horizontal_reflections {
            let mut errors = 0;

            for x in 0..board.width() {
                if board.get(x, t) != board.get(x, b) {
                    errors += 1;

                    if errors > 1 {
                        break;
                    }
                }
            }

            if errors == 1 {
                sum += (i + 1) * 100;
                break;
            }
        }
    }

    Ok(sum.to_string())
}

fn find_possible_smudged_reflections(input: &[u64]) -> Vec<(usize, usize, usize)> {
    let mut possible_reflections = Vec::new();

    for (i, v) in input.windows(2).enumerate() {
        let mut l = i;
        let mut r = i + 1;
        let mut hash_error = None;

        if v[0] != v[1] {
            hash_error = Some((l, r));
        }

        while l > 0 && r < input.len() - 1 {
            l -= 1;
            r += 1;

            if input[l] != input[r] {
                if hash_error.is_some() {
                    hash_error = None;
                    break;
                }

                hash_error = Some((l, r));
            }
        }

        if let Some((l, r)) = hash_error {
            possible_reflections.push((i, l, r));
        }
    }

    possible_reflections
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

        assert_eq!(output, Ok("400".to_owned()));
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

        assert_eq!(output, Ok("300".to_owned()));
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

        assert_eq!(output, Ok("100".to_owned()));
    }
}
