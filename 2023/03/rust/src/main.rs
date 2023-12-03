// #![deny(clippy::all, clippy::pedantic)]
use std::{error::Error, fs};

mod error;
mod part_1;
mod part_2;
mod shared;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    part_1::run(&input).print_answer(1);
    part_2::run(&input).print_answer(2);
}

trait AocAnswer {
    fn print_answer(&self, part_number: i32);
}

impl<E> AocAnswer for Result<String, E>
where
    E: Error,
{
    fn print_answer(&self, part_number: i32) {
        match self {
            Ok(answer) => println!("Part {part_number}: {answer}"),
            Err(error) => println!("Part {part_number} error!: {error}"),
        }
    }
}

impl AocAnswer for String {
    fn print_answer(&self, part_number: i32) {
        println!("Part {part_number}: {self}");
    }
}
