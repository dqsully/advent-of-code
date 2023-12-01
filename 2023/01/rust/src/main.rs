use std::{fs, error::Error};

mod part_1;
mod part_2;

fn main() {
    println!("2023/12/01 - Rust");

    let input = fs::read_to_string("../input.txt").unwrap();

    run_part(1, &input, part_1::run);
    run_part(2, &input, part_2::run);
}

fn run_part<'a, F, E>(num: i32, input: &'a str, func: F)
where
    F: FnOnce(&'a str) -> Result<String, E>,
    E: 'a + Error
{
    match func(input) {
        Ok(answer) => println!("Part {}: {}", num, answer),
        Err(error) => println!("Part {} error!: {}", num, error)
    }
}
