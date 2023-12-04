use std::cmp::{max, min};

pub fn neighbors_8(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let x_range = (max(1, x) - 1)..min(width, x + 2);
    let y_range = (max(1, y) - 1)..min(height, y + 2);

    y_range
        .flat_map(move |y_o| x_range.clone().map(move |x_o| (x_o, y_o)))
        .filter(move |&(x_o, y_o)| x_o != x || y_o != y)
}

pub trait Grid2D {
    type Item;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn get(&self, x: usize, y: usize) -> Option<&Self::Item>;
    fn iter(&self) -> impl Iterator<Item = (usize, usize, Self::Item)>;

    fn neighbors_8(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize, &Self::Item)> {
        neighbors_8(x, y, self.width(), self.height())
            .filter_map(|(x, y)| Some((x, y, self.get(x, y)?)))
    }
}

pub trait Grid2DMut: Grid2D {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item>;
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
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ],
        top_left: (0, 0, 3, 3) => vec![
            (1, 0),
            (0, 1),
            (1, 1),
        ],
        top: (1, 0, 3, 3) => vec![
            (0, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
        ],
        top_right: (2, 0, 3, 3) => vec![
            (1, 0),
            (1, 1),
            (2, 1),
        ],
        left: (0, 1, 3, 3) => vec![
            (0, 0),
            (1, 0),
            (1, 1),
            (0, 2),
            (1, 2),
        ],
        right: (2, 1, 3, 3) => vec![
            (1, 0),
            (2, 0),
            (1, 1),
            (1, 2),
            (2, 2),
        ],
        bottom_left: (0, 2, 3, 3) => vec![
            (0, 1),
            (1, 1),
            (1, 2),
        ],
        bottom: (1, 2, 3, 3) => vec![
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 2),
            (2, 2),
        ],
        bottom_right: (2, 2, 3, 3) => vec![
            (1, 1),
            (2, 1),
            (1, 2),
        ],

        one_by_one: (0, 0, 1, 1) => vec![],
        zero_by_zero: (0, 0, 0, 0) => vec![],
        one_by_three: (0, 1, 1, 3) => vec![
            (0, 0),
            (0, 2),
        ],
        three_by_one: (1, 0, 3, 1) => vec![
            (0, 0),
            (2, 0),
        ],
    );
}
