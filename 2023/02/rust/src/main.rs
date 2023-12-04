#![deny(clippy::all, clippy::pedantic)]
use aoc_helpers::answer::AocAnswer;
use std::fs;

mod error;
mod part_1;
mod part_2;
mod shared;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    part_1::run(&input).print_aoc_answer(1);
    part_2::run(&input).print_aoc_answer(2);
}
