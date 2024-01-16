use aoc_helpers::{neighbors::Grid2D, text_map::TextMap};

use crate::{
    error::Error,
    shared::{hash_columns, hash_rows},
};

pub fn run(input: &str) -> Result<String, Error> {
    let boards = input.trim().split("\n\n").map(TextMap::parse);

    let mut sum = 0;

    for board in boards {
        let board = board?;

        let columns = hash_columns(&board);
        let rows = hash_rows(&board);

        for (i, l, r) in find_possible_smudged_reflections(&columns) {
            if validate_smudged_column(&board, l, r) {
                sum += i + 1;
                break;
            }
        }

        for (i, t, b) in find_possible_smudged_reflections(&rows) {
            if validate_smudged_row(&board, t, b) {
                sum += (i + 1) * 100;
                break;
            }
        }
    }

    Ok(sum.to_string())
}

fn find_possible_smudged_reflections(
    input: &[u64],
) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    input.windows(2).enumerate().filter_map(|(i, v)| {
        let mut l = i;
        let mut r = i + 1;
        let mut smudged_line = None;

        if v[0] != v[1] {
            smudged_line = Some((i, l, r));
        }

        while l > 0 && r < input.len() - 1 {
            l -= 1;
            r += 1;

            if input[l] != input[r] {
                if smudged_line.is_none() {
                    smudged_line = Some((i, l, r));
                } else {
                    // There can only be one smudged line, meaning this isn't a
                    // proper smudged reflection
                    smudged_line = None;
                    break;
                }
            }
        }

        smudged_line
    })
}

fn validate_smudged_column(board: &TextMap, l: usize, r: usize) -> bool {
    let mut found_smudge = false;

    for y in 0..board.height() {
        if board.get(l, y) != board.get(r, y) {
            if found_smudge {
                return false;
            }

            found_smudge = true;
        }
    }

    found_smudge
}

fn validate_smudged_row(board: &TextMap, t: usize, b: usize) -> bool {
    let mut found_smudge = false;

    for x in 0..board.width() {
        if board.get(x, t) != board.get(x, b) {
            if found_smudge {
                return false;
            }

            found_smudge = true;
        }
    }

    found_smudge
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
