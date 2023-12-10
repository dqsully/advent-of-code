use crate::neighbors::{Grid2D, Grid2DMut};

pub struct Map2D<T> {
    map: Vec<T>,
    height: usize,
    width: usize,
}

impl<T> Map2D<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Map2D<T>
    where
        T: Clone
    {
        Map2D {
            map: vec![default_value; width * height],
            height,
            width,
        }
    }
}

impl<T> Grid2D for Map2D<T> {
    type Item = T;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.map.get(x + y * self.width)
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.map
            .iter()
            .enumerate()
            .map(|(i, v)| (i % self.width, i / self.width, v))
    }
}

impl<T> Grid2DMut for Map2D<T> {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.map.get_mut(x + y * self.width)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.map
            .iter_mut()
            .enumerate()
            .map(|(i, v)| (i % self.width, i / self.width, v))
    }
}
