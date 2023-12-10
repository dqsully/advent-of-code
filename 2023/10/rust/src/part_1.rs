use aoc_helpers::{
    neighbors::{Direction, Grid2D},
    text_map::TextMap,
};

use crate::{
    error::Error,
    shared::{direction_for_byte, find_start, infer_start_direction},
};

pub fn run(input: &str) -> Result<String, Error> {
    let map = TextMap::parse(input)?;

    let (start_x, start_y) = find_start(&map).ok_or(Error::StartNotFound)?;
    let (start_d, _) =
        infer_start_direction(&map, start_x, start_y).ok_or(Error::StartInferFailed)?;

    Ok((compute_wall_length(&map, start_x, start_y, start_d)? / 2).to_string())
}

pub fn compute_wall_length(
    map: &TextMap,
    start_x: usize,
    start_y: usize,
    start_d: Direction,
) -> Result<usize, Error> {
    let mut traveled = 0;

    let mut next_direction = *start_d.cardinals().first().ok_or(Error::InvalidWall)?;
    let mut x = start_x;
    let mut y = start_y;

    loop {
        let c: &u8;
        (x, y, c) = map
            .offset_direction(x, y, next_direction)
            .ok_or(Error::InvalidWall)?;

        traveled += 1;

        if x == start_x && y == start_y {
            break;
        }

        let d = direction_for_byte(*c).ok_or(Error::InvalidWall)?;

        next_direction = d ^ next_direction.reverse();

        if next_direction.num_cardinals() != 1 {
            return Err(Error::InvalidWall);
        }
    }

    Ok(traveled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

        let output = run(input);

        assert_eq!(output, Ok("8".to_owned()));
    }
}
