use crate::{
    neighbors::{Grid2D, Grid2DMut},
    Error,
};

pub struct TextMap<'a> {
    map: Vec<&'a [u8]>,
    width: usize,
}

impl<'a> TextMap<'a> {
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

    fn iter(&self) -> impl Iterator<Item = (usize, usize, u8)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &byte)| (x, y, byte)))
    }
}

pub struct TextMapMut {
    map: Vec<Vec<u8>>,
    width: usize,
}

impl TextMapMut {
    pub fn parse(source: &str) -> Result<TextMapMut, Error> {
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

            map.push(line.to_owned());
        }

        Ok(TextMapMut {
            map,
            width: width.unwrap_or(0),
        })
    }
}

impl Grid2D for TextMapMut {
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

    fn iter(&self) -> impl Iterator<Item = (usize, usize, u8)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &byte)| (x, y, byte)))
    }
}

impl Grid2DMut for TextMapMut {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        self.map.get_mut(y).and_then(|row| row.get_mut(x))
    }
}
