use aoc_helpers::{text_map::TextMap, neighbors::{Grid2D, Direction}};

use crate::{error::Error, shared::{infer_start_direction, direction_for_byte, find_start}};

pub fn run(input: &str) -> Result<String, Error> {
    // Parse the input as a 2d map of lines
    let map = TextMap::parse(input)?;

    // Find the 'S' starting point
    let (start_x, start_y) = find_start(&map).ok_or(Error::StartNotFound)?;

    // Infer what would be the start instead of 'S' based on its surroundings
    let (start_d, _) = infer_start_direction(&map, start_x, start_y).ok_or(Error::StartInferFailed)?;

    // Traverse the wall and count up its length
    Ok((compute_wall_length(&map, start_x, start_y, start_d)? / 2).to_string())
}

pub fn compute_wall_length(map: &TextMap, start_x: usize, start_y: usize, start_d: Direction) -> Result<usize, Error> {
    let mut traveled = 0;

    let mut next_direction = *start_d.cardinals().first().ok_or(Error::InvalidWall)?;
    let mut x = start_x;
    let mut y = start_y;

    loop {
        let c: &u8;
        (x, y, c) = map.offset_direction(x, y, next_direction).ok_or(Error::InvalidWall)?;

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
