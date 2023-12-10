use aoc_helpers::neighbors::{Direction, Grid2D};

pub fn direction_for_byte(c: u8) -> Option<Direction> {
    match c {
        b'|' => Some(Direction::UpDown),
        b'-' => Some(Direction::LeftRight),
        b'L' => Some(Direction::UpRight),
        b'J' => Some(Direction::UpLeft),
        b'7' => Some(Direction::DownLeft),
        b'F' => Some(Direction::DownRight),
        b'.' => Some(Direction::Nowhere),
        _ => None,
    }
}

pub fn byte_for_direction(d: Direction) -> Option<u8> {
    match d {
        Direction::UpDown => Some(b'|'),
        Direction::LeftRight => Some(b'-'),
        Direction::UpRight => Some(b'L'),
        Direction::UpLeft => Some(b'J'),
        Direction::DownLeft => Some(b'7'),
        Direction::DownRight => Some(b'F'),
        Direction::Nowhere => Some(b'.'),
        _ => None,
    }
}

pub fn find_start<T: Grid2D<Item = u8>>(map: &T) -> Option<(usize, usize)> {
    map.iter()
        .find(|(_, _, &c)| c == b'S')
        .map(|(x, y, _)| (x, y))
}

pub fn infer_start_direction<T: Grid2D<Item = u8>>(
    map: &T,
    start_x: usize,
    start_y: usize,
) -> Option<(Direction, u8)> {
    let mut start_d = Direction::Nowhere;

    for (_, _, d, &c) in map.neighbors_4(start_x, start_y) {
        start_d |= d & direction_for_byte(c)?.reverse();
    }

    if start_d == Direction::Nowhere {
        None
    } else {
        Some((start_d, byte_for_direction(start_d)?))
    }
}
