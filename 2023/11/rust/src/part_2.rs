use aoc_helpers::{
    neighbors::Grid2D,
    text_map::TextMap,
};

use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    run_with_expansion_factor(input, 1000000)
}

fn run_with_expansion_factor(input: &str, expansion_factor: usize) -> Result<String, Error> {
    let collapsed_universe = TextMap::parse(input).unwrap();

    let mut empty_columns = vec![true; collapsed_universe.width()];
    let mut empty_rows = vec![true; collapsed_universe.height()];

    for (x, y, &c) in collapsed_universe.iter() {
        if c == b'#' {
            empty_columns[x] = false;
            empty_rows[y] = false;
        }
    }

    let empty_columns_accumulated = empty_columns
        .into_iter()
        .scan(0, |i, c| {
            if c {
                *i += 1;
            }

            Some(*i)
        })
        .collect::<Vec<_>>();
    let empty_rows_accumulated = empty_rows
        .into_iter()
        .scan(0, |i, c| {
            if c {
                *i += 1;
            }

            Some(*i)
        })
        .collect::<Vec<_>>();

    let mut galaxies = Vec::new();

    for (x, y, &c) in collapsed_universe.iter() {
        if c == b'#' {
            galaxies.push((
                x + empty_columns_accumulated[x] * (expansion_factor - 1),
                y + empty_rows_accumulated[y] * (expansion_factor - 1),
            ));
        }
    }

    let mut total_distances = 0;

    for (i, &(s_x, s_y)) in galaxies.iter().enumerate() {
        for &(e_x, e_y) in galaxies[i+1..].iter() {
            let dx = if s_x > e_x {
                s_x - e_x
            } else {
                e_x - s_x
            };
            let dy = if s_y > e_y {
                s_y - e_y
            } else {
                e_y - s_y
            };

            total_distances += dx + dy;
        }
    }

    Ok(total_distances.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

        let output = run_with_expansion_factor(input, 10);

        assert_eq!(output, Ok("1030".to_owned()));
    }

    #[test]
    fn example_2() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

        let output = run_with_expansion_factor(input, 100);

        assert_eq!(output, Ok("8410".to_owned()));
    }
}
