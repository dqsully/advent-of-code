use aoc_helpers::{
    map::Map2D,
    neighbors::{Direction, Grid2D, Grid2DMut},
};

use crate::{
    error::Error,
    shared::{direction_for_byte, find_start, infer_start_direction},
};

pub fn run(input: &str) -> Result<String, Error> {
    let mut map: Map2D<u8> = input.parse()?;

    let (start_x, start_y) = find_start(&map).ok_or(Error::StartNotFound)?;
    let (start_d, start_c) =
        infer_start_direction(&map, start_x, start_y).ok_or(Error::StartInferFailed)?;
    map.set(start_x, start_y, start_c);

    let wall_map = compute_wall(&map, start_x, start_y, start_d)?;

    Ok(count_inside(&wall_map).to_string())
}

fn compute_wall(
    map: &Map2D<u8>,
    start_x: usize,
    start_y: usize,
    start_d: Direction,
) -> Result<Map2D<Direction>, Error> {
    let mut wall_map = Map2D::new_parallel(&map, Direction::Nowhere);

    let mut next_direction = *start_d.cardinals().first().ok_or(Error::InvalidWall)?;
    let mut x = start_x;
    let mut y = start_y;

    loop {
        let c: &u8;
        (x, y, c) = map
            .offset_direction(x, y, next_direction)
            .ok_or(Error::InvalidWall)?;
        let d = direction_for_byte(*c).ok_or(Error::InvalidWall)?;

        wall_map.set(x, y, d);

        if x == start_x && y == start_y {
            break;
        }

        next_direction = d ^ next_direction.reverse();

        if next_direction.num_cardinals() != 1 {
            return Err(Error::InvalidWall);
        }
    }

    Ok(wall_map)
}

fn count_inside(wall_map: &Map2D<Direction>) -> usize {
    let mut inside_tiles = 0;
    let mut is_inside = Direction::Nowhere;
    let mut inside_map = Map2D::new_parallel(wall_map, Direction::Nowhere);

    for (x, y, &wall_d) in wall_map.iter() {
        if wall_d != Direction::Nowhere {
            is_inside ^= wall_d & Direction::UpDown;
        } else if is_inside == Direction::UpDown {
            inside_tiles += 1;
            inside_map.set(x, y, is_inside);
        }
    }

    inside_tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

        let output = run(input);

        assert_eq!(output, Ok("4".to_owned()));
    }

    #[test]
    fn example_2() {
        let input = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

        let output = run(input);

        assert_eq!(output, Ok("8".to_owned()));
    }

    #[test]
    fn example_3() {
        let input = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

        let output = run(input);

        assert_eq!(output, Ok("10".to_owned()));
    }
}
