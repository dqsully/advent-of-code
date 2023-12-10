use aoc_helpers::{text_map::TextMapMut, neighbors::{Grid2D, Direction, Grid2DMut}, map::Map2D};

use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    let mut map = TextMapMut::parse(input)?;

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

    let start_c = match start_d {
        Direction::UpDown => b'|',
        Direction::LeftRight => b'-',
        Direction::UpRight => b'L',
        Direction::UpLeft => b'J',
        Direction::DownLeft => b'7',
        Direction::DownRight => b'F',
        _ => unreachable!(),
    };
    map.set(start_x, start_y, start_c);

    let mut wall_map = Map2D::new(map.width(), map.height(), false);
    wall_map.set(start_x, start_y, true);

    let mut next_direction = if start_d & Direction::Left != Direction::Nowhere {
        Direction::Left
    } else if start_d & Direction::Up != Direction::Nowhere {
        Direction::Up
    } else {
        Direction::Right
    };

    let mut x = start_x;
    let mut y = start_y;

    loop {
        let c: &u8;
        (x, y, c) = map.offset_direction(x, y, next_direction).unwrap();
        wall_map.set(x, y, true);

        if x == start_x && y == start_y {
            break;
        }

        let d = direction_for_byte(*c);

        next_direction = d ^ next_direction.reverse();
    }


    let mut inside_tiles = 0;
    let mut is_inside = Direction::Nowhere;
    let mut inside_map = Map2D::new(map.width(), map.height(), Direction::Nowhere);

    for (x, y, &is_wall) in wall_map.iter() {
        if is_wall {
            let byte = *map.get(x, y).unwrap();

            is_inside ^= direction_for_byte(byte) & Direction::UpDown;
        } else if is_inside == Direction::UpDown {
            inside_tiles += 1;
            inside_map.set(x, y, is_inside);
        }
    }

    Ok(inside_tiles.to_string())
}

fn direction_for_byte(byte: u8) -> Direction {
    match byte {
        b'|' => Direction::UpDown,
        b'-' => Direction::LeftRight,
        b'L' => Direction::UpRight,
        b'J' => Direction::UpLeft,
        b'7' => Direction::DownLeft,
        b'F' => Direction::DownRight,
        _ => Direction::Nowhere,
    }
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
