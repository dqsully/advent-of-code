use aoc_helpers::neighbors::Grid2D;
use aoc_helpers::text_map::TextMap;

use crate::error::Error;
pub struct EngineSchematic<'a> {
    text_map: TextMap<'a>,
    numbers_map: Vec<Option<usize>>,
    numbers: Vec<u64>,
}

impl<'a> EngineSchematic<'a> {
    pub fn parse(source: &'a str) -> Result<EngineSchematic<'a>, Error> {
        let text_map = TextMap::parse(source)?;

        let mut numbers_map = Vec::new();
        let mut numbers = Vec::new();

        let mut num_buffer = String::new();

        for (_, _, &byte) in text_map.iter() {
            if let b'0'..=b'9' = byte {
                num_buffer.push(byte as char);

                numbers_map.push(Some(numbers.len()));
            } else {
                if !num_buffer.is_empty() {
                    numbers.push(num_buffer.parse()?);
                    num_buffer.clear();
                }

                numbers_map.push(None);
            }
        }

        Ok(EngineSchematic {
            text_map,
            numbers_map,
            numbers,
        })
    }

    pub fn number_id_at(&self, x: usize, y: usize) -> Option<usize> {
        self.numbers_map
            .get(x + y * self.text_map.width())
            .copied()
            .flatten()
    }

    pub fn get_number(&self, id: usize) -> Option<u64> {
        self.numbers.get(id).copied()
    }
}

impl<'a> std::ops::Deref for EngineSchematic<'a> {
    type Target = TextMap<'a>;

    fn deref(&self) -> &Self::Target {
        &self.text_map
    }
}

// TODO: EngineSchematic tests
