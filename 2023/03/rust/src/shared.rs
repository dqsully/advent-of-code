use crate::error::Error;
use std::cmp::{max, min};

pub struct EngineSchematic<'a> {
    width: usize,
    height: usize,

    source: &'a [u8],
    numbers_map: Vec<Option<usize>>,
    numbers: Vec<u64>,
}

impl<'a> EngineSchematic<'a> {
    pub fn parse(source: &'a str) -> Result<EngineSchematic<'a>, Error> {
        let mut width = None;
        let mut height = 0;

        let mut numbers_map = Vec::new();
        let mut numbers = Vec::new();

        let mut num_buffer = String::new();

        for line in source.lines() {
            if let Some(width) = width {
                if line.as_bytes().len() != width {
                    return Err(Error::InconsistentSchematicWidth);
                }
            } else {
                width = Some(line.as_bytes().len());
            }

            height += 1;

            for byte in line.bytes() {
                match byte {
                    b'0'..=b'9' => {
                        num_buffer.push(byte as char);

                        numbers_map.push(Some(numbers.len()));
                    }
                    _ => {
                        if !num_buffer.is_empty() {
                            numbers.push(num_buffer.parse()?);
                            num_buffer.clear();
                        }

                        numbers_map.push(None);
                    }
                }
            }
        }

        Ok(EngineSchematic {
            width: width.unwrap_or(0),
            height,

            source: source.as_bytes(),
            numbers_map,
            numbers,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
        self.source
            .iter()
            .cloned()
            .filter(|&byte| byte != b'\n' && byte != b'\r')
            .scan((0, 0), |(x, y), byte| {
                let output = (*x, *y, byte);

                *x += 1;

                if *x >= self.width {
                    *x = 0;
                    *y += 1;
                }

                Some(output)
            })
    }

    pub fn number_id_at(&self, x: usize, y: usize) -> Option<usize> {
        self.numbers_map.get(x + y * self.width).cloned().flatten()
    }

    pub fn get_number(&self, id: usize) -> Option<u64> {
        self.numbers.get(id).cloned()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

// TODO: move neighbors_8 into a shared utility library
pub fn neighbors_8(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let x_range = max(1, x) - 1..=min(width - 2, x) + 1;
    let y_range = max(1, y) - 1..=min(height - 2, y) + 1;

    y_range
        .flat_map(move |y_o| x_range.clone().map(move |x_o| (x_o, y_o)))
        .filter(move |&(x_o, y_o)| x_o != x || y_o != y)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: EngineSchematic tests

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
    );
}
