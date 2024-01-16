use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use aoc_helpers::{text_map::TextMap, neighbors::Grid2D};

pub fn hash_column(board: &TextMap, x: usize) -> u64 {
    let mut hasher = DefaultHasher::new();

    for y in 0..board.height() {
        hasher.write_u8(*board.get(x, y).unwrap());
    }

    hasher.finish()
}

pub fn hash_row(board: &TextMap, y: usize) -> u64 {
    let mut hasher = DefaultHasher::new();

    for x in 0..board.width() {
        hasher.write_u8(*board.get(x, y).unwrap());
    }

    hasher.finish()
}

pub fn hash_columns(board: &TextMap) -> Vec<u64> {
    (0..board.width())
        .map(|x| hash_column(board, x))
        .collect()
}

pub fn hash_rows(board: &TextMap) -> Vec<u64> {
    (0..board.height())
        .map(|y| hash_row(board, y))
        .collect()
}
