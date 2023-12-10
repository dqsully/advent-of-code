use std::{
    cmp::{max, min, Ordering},
    ops::Range,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Direction {
    Nowhere = 0,
    Up = 0x1,
    Left = 0x2,
    UpLeft = 0x3,
    Down = 0x4,
    UpDown = 0x5,
    DownLeft = 0x6,
    UpDownLeft = 0x7,
    Right = 0x8,
    UpRight = 0x9,
    LeftRight = 0xa,
    UpLeftRight = 0xb,
    DownRight = 0xc,
    UpDownRight = 0xd,
    DownLeftRight = 0xe,
    UpDownLeftRight = 0xf,
}

const CARDINALS: [Direction; 12] = [
    Direction::Up,    // 0
    Direction::Down,  // 1
    Direction::Left,  // 2
    Direction::Right, // 3
    Direction::Up,    // 4
    Direction::Down,  // 5
    Direction::Right, // 6
    Direction::Up,    // 7
    Direction::Left,  // 8
    Direction::Right, // 9
    Direction::Up,    // 10
    Direction::Right, // 11
];

const CARDINAL_OFFSETS: [Range<usize>; 16] = [
    0..0,   // Nowhere
    0..1,   // Up
    8..9,   // Left
    7..9,   // UpLeft
    1..2,   // Down
    0..2,   // UpDown
    1..3,   // DownLeft
    0..3,   // UpDownLeft
    11..12, // Right
    10..12, // UpRight
    8..10,  // LeftRight
    7..10,  // UpLeftRight
    5..7,   // DownRight
    4..7,   // UpDownRight
    1..4,   // DownLeftRight
    0..4,   // UpDownLeftRight
];

impl Direction {
    #[must_use]
    pub fn reverse(mut self) -> Direction {
        match self & Direction::LeftRight {
            Direction::Nowhere | Direction::LeftRight => {}
            _ => self ^= Direction::LeftRight,
        }

        match self & Direction::UpDown {
            Direction::Nowhere | Direction::UpDown => {}
            _ => self ^= Direction::UpDown,
        }

        self
    }

    #[must_use]
    pub fn num_cardinals(self) -> u32 {
        (self as u8).count_ones()
    }

    #[must_use]
    pub fn cardinals(self) -> &'static [Direction] {
        &CARDINALS[CARDINAL_OFFSETS[self as u8 as usize].clone()]
    }
}

impl std::ops::BitOr for Direction {
    type Output = Direction;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { *(&((self as u8) | (rhs as u8)) as *const u8).cast::<Direction>() }
    }
}

impl std::ops::BitOrAssign for Direction {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl std::ops::BitAnd for Direction {
    type Output = Direction;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { *(&((self as u8) & (rhs as u8)) as *const u8).cast::<Direction>() }
    }
}

impl std::ops::BitAndAssign for Direction {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl std::ops::BitXor for Direction {
    type Output = Direction;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { *(&((self as u8) ^ (rhs as u8)) as *const u8).cast::<Direction>() }
    }
}

impl std::ops::BitXorAssign for Direction {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn neighbors_8(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize, Direction)> {
    let x_range = (max(1, x) - 1)..min(width, x + 2);
    let y_range = (max(1, y) - 1)..min(height, y + 2);

    y_range
        .flat_map(move |y_o| x_range.clone().map(move |x_o| (x_o, y_o)))
        .filter(move |&(x_o, y_o)| x_o != x || y_o != y)
        .map(move |(x_o, y_o)| {
            let mut d = Direction::Nowhere;

            d |= match x_o.cmp(&x) {
                Ordering::Less => Direction::Left,
                Ordering::Equal => Direction::Nowhere,
                Ordering::Greater => Direction::Right,
            };

            d |= match y_o.cmp(&y) {
                Ordering::Less => Direction::Up,
                Ordering::Equal => Direction::Nowhere,
                Ordering::Greater => Direction::Down,
            };

            (x_o, y_o, d)
        })
}

#[allow(clippy::module_name_repetitions)]
pub fn neighbors_4(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize, Direction)> {
    neighbors_8(x, y, width, height).filter(move |&(x_o, y_o, _)| (x == x_o) ^ (y == y_o))
}

#[must_use]
pub fn offset_direction(
    mut x: usize,
    mut y: usize,
    width: usize,
    height: usize,
    mut d: Direction,
) -> Option<(usize, usize)> {
    if d & Direction::LeftRight == Direction::LeftRight {
        d ^= Direction::LeftRight;
    }
    if d & Direction::UpDown == Direction::UpDown {
        d ^= Direction::UpDown;
    }

    if d & Direction::Left == Direction::Left {
        if x > 0 {
            x -= 1;
        } else {
            return None;
        }
    } else if d & Direction::Right == Direction::Right {
        if x < width - 1 {
            x += 1;
        } else {
            return None;
        }
    }

    if d & Direction::Up == Direction::Up {
        if y > 0 {
            y -= 1;
        } else {
            return None;
        }
    } else if d & Direction::Down == Direction::Down {
        if y < height - 1 {
            y += 1;
        } else {
            return None;
        }
    }

    Some((x, y))
}

pub trait Grid2D {
    type Item;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn get(&self, x: usize, y: usize) -> Option<&Self::Item>;
    fn iter(&self) -> impl Iterator<Item = (usize, usize, &Self::Item)>;

    fn neighbors_8(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize, Direction, &Self::Item)> {
        neighbors_8(x, y, self.width(), self.height())
            .filter_map(|(x, y, d)| Some((x, y, d, self.get(x, y)?)))
    }
    fn neighbors_4(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize, Direction, &Self::Item)> {
        neighbors_4(x, y, self.width(), self.height())
            .filter_map(|(x, y, d)| Some((x, y, d, self.get(x, y)?)))
    }

    fn offset_direction(
        &self,
        x: usize,
        y: usize,
        d: Direction,
    ) -> Option<(usize, usize, &Self::Item)> {
        offset_direction(x, y, self.width(), self.height(), d)
            .and_then(|(x, y)| Some((x, y, self.get(x, y)?)))
    }
}

impl<T> Grid2D for &T
where
    T: Grid2D,
{
    type Item = T::Item;

    fn width(&self) -> usize {
        (**self).width()
    }

    fn height(&self) -> usize {
        (**self).height()
    }

    fn get(&self, x: usize, y: usize) -> Option<&Self::Item> {
        (**self).get(x, y)
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize, &Self::Item)> {
        (**self).iter()
    }
}

impl<T> Grid2D for &mut T
where
    T: Grid2D,
{
    type Item = T::Item;

    fn width(&self) -> usize {
        (**self).width()
    }

    fn height(&self) -> usize {
        (**self).height()
    }

    fn get(&self, x: usize, y: usize) -> Option<&Self::Item> {
        (**self).get(x, y)
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize, &Self::Item)> {
        (**self).iter()
    }
}

pub trait Grid2DMut: Grid2D {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item>;
    fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut Self::Item)>;

    fn set(&mut self, x: usize, y: usize, item: Self::Item) -> Option<Self::Item> {
        let space = self.get_mut(x, y)?;

        Some(std::mem::replace(space, item))
    }
}

impl<T> Grid2DMut for &mut T
where
    T: Grid2DMut,
{
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item> {
        (**self).get_mut(x, y)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut Self::Item)> {
        (**self).iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! neighbors_8_test {
        ($suite:ident, $($name:ident: $input:expr => $expected:expr,)*) => {
            mod $suite {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        let mut outputs = Vec::new();
                        let input = $input;

                        for output in neighbors_8(input.0, input.1, input.2, input.3) {
                            outputs.push(output);
                        }

                        assert_eq!(outputs, $expected);
                    }
                )*
            }
        }
    }

    neighbors_8_test!(neighbors_8_tests,
        all_8: (1, 1, 3, 3) => vec![
            (0, 0, Direction::UpLeft),
            (1, 0, Direction::Up),
            (2, 0, Direction::UpRight),
            (0, 1, Direction::Left),
            (2, 1, Direction::Right),
            (0, 2, Direction::DownLeft),
            (1, 2, Direction::Down),
            (2, 2, Direction::DownRight),
        ],
        top_left: (0, 0, 3, 3) => vec![
            (1, 0, Direction::Right),
            (0, 1, Direction::Down),
            (1, 1, Direction::DownRight),
        ],
        top: (1, 0, 3, 3) => vec![
            (0, 0, Direction::Left),
            (2, 0, Direction::Right),
            (0, 1, Direction::DownLeft),
            (1, 1, Direction::Down),
            (2, 1, Direction::DownRight),
        ],
        top_right: (2, 0, 3, 3) => vec![
            (1, 0, Direction::Left),
            (1, 1, Direction::DownLeft),
            (2, 1, Direction::Down),
        ],
        left: (0, 1, 3, 3) => vec![
            (0, 0, Direction::Up),
            (1, 0, Direction::UpRight),
            (1, 1, Direction::Right),
            (0, 2, Direction::Down),
            (1, 2, Direction::DownRight),
        ],
        right: (2, 1, 3, 3) => vec![
            (1, 0, Direction::UpLeft),
            (2, 0, Direction::Up),
            (1, 1, Direction::Left),
            (1, 2, Direction::DownLeft),
            (2, 2, Direction::Down),
        ],
        bottom_left: (0, 2, 3, 3) => vec![
            (0, 1, Direction::Up),
            (1, 1, Direction::UpRight),
            (1, 2, Direction::Right),
        ],
        bottom: (1, 2, 3, 3) => vec![
            (0, 1, Direction::UpLeft),
            (1, 1, Direction::Up),
            (2, 1, Direction::UpRight),
            (0, 2, Direction::Left),
            (2, 2, Direction::Right),
        ],
        bottom_right: (2, 2, 3, 3) => vec![
            (1, 1, Direction::UpLeft),
            (2, 1, Direction::Up),
            (1, 2, Direction::Left),
        ],

        one_by_one: (0, 0, 1, 1) => vec![],
        zero_by_zero: (0, 0, 0, 0) => vec![],
        one_by_three: (0, 1, 1, 3) => vec![
            (0, 0, Direction::Up),
            (0, 2, Direction::Down),
        ],
        three_by_one: (1, 0, 3, 1) => vec![
            (0, 0, Direction::Left),
            (2, 0, Direction::Right),
        ],
    );
}
