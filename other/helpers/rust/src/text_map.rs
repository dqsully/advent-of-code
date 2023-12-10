use crate::{neighbors::Grid2D, Error};

pub struct TextMap<'a> {
    map: Vec<&'a [u8]>,
    width: usize,
}

impl<'a> TextMap<'a> {
    /// # Errors
    /// * `Error::InconsistentMapWidth` - map has inconsistent line length (map width)
    pub fn parse(source: &str) -> Result<TextMap, Error> {
        let mut map = Vec::new();
        let mut width = None;

        for line in source.lines() {
            let line = line.as_bytes();

            if let Some(width) = width {
                if line.len() != width {
                    return Err(Error::InconsistentMapWidth);
                }
            } else {
                width = Some(line.len());
            }

            map.push(line);
        }

        Ok(TextMap {
            map,
            width: width.unwrap_or(0),
        })
    }
}

impl<'a> Grid2D for TextMap<'a> {
    type Item = u8;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn get(&self, x: usize, y: usize) -> Option<&u8> {
        self.map.get(y).and_then(|row| row.get(x))
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize, &u8)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, byte)| (x, y, byte)))
    }
}
