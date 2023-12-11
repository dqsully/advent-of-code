use aoc_helpers::{neighbors::Grid2D, text_map::TextMap};

use crate::error::Error;

pub fn run_with_expansion_factor(input: &str, expansion_factor: usize) -> Result<String, Error> {
    if expansion_factor == 0 {
        return Err(Error::NoExpansion);
    }

    let collapsed_universe = TextMap::parse(input)?;

    let mut empty_columns = vec![true; collapsed_universe.width()];
    let mut empty_rows = vec![true; collapsed_universe.height()];

    for (x, y, &c) in collapsed_universe.iter() {
        if c == b'#' {
            empty_columns[x] = false;
            empty_rows[y] = false;
        }
    }

    let column_expansion = accumulate_expansion(&empty_columns);
    let row_expansion = accumulate_expansion(&empty_rows);

    let mut galaxies = Vec::new();

    for (x, y, &c) in collapsed_universe.iter() {
        if c == b'#' {
            galaxies.push((
                x + column_expansion[x] * (expansion_factor - 1),
                y + row_expansion[y] * (expansion_factor - 1),
            ));
        }
    }

    let mut total_distances = 0;

    for (i, &(a_x, a_y)) in galaxies.iter().enumerate() {
        for &(b_x, b_y) in &galaxies[i + 1..] {
            total_distances += a_x.abs_diff(b_x) + a_y.abs_diff(b_y);
        }
    }

    Ok(total_distances.to_string())
}

fn accumulate_expansion(empty: &[bool]) -> Vec<usize> {
    empty
        .iter()
        .scan(0, |i, &c| {
            if c {
                *i += 1;
            }

            Some(*i)
        })
        .collect()
}
