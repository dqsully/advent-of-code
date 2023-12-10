use aoc_helpers::{text_map::TextMap, neighbors::{Grid2D, Direction}};

use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    let map = TextMap::parse(input)?;

    let (start_x, start_y, _) = map.iter()
        .find(|(_, _, c)| **c == b'S')
        .unwrap();

    let mut start_d = Direction::Nowhere;

    for (_, _, d, c) in map.neighbors_4(start_x, start_y) {
        start_d |= match d {
            Direction::Up if [b'7', b'|', b'F'].contains(c) => d,
            Direction::Right if [b'J', b'-', b'7'].contains(c) => d,
            Direction::Down if [b'J', b'|', b'L'].contains(c) => d,
            Direction::Left if [b'L', b'-', b'F'].contains(c) => d,
            _ => Direction::Nowhere
        };
    }

    let mut next_direction = if start_d & Direction::Left != Direction::Nowhere {
        Direction::Left
    } else if start_d & Direction::Up != Direction::Nowhere {
        Direction::Up
    } else {
        Direction::Right
    };

    let mut traveled = 0;
    let mut x = start_x;
    let mut y = start_y;

    loop {
        let c: &u8;
        (x, y, c) = map.offset_direction(x, y, next_direction).unwrap();
        traveled += 1;

        let d = match *c {
            b'|' => Direction::UpDown,
            b'-' => Direction::LeftRight,
            b'L' => Direction::UpRight,
            b'J' => Direction::UpLeft,
            b'7' => Direction::DownLeft,
            b'F' => Direction::DownRight,
            b'.' => panic!("pipe ran into ground"),
            b'S' => break,
            _ => panic!("unexpected char {:?}", *c as char),
        };

        next_direction = d ^ next_direction.reverse();
    }

    Ok((traveled / 2).to_string())
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
