#[allow(clippy::module_name_repetitions)]
pub trait AocAnswer {
    fn print_aoc_answer(&self, part_number: i32);
}

impl<E> AocAnswer for Result<String, E>
where
    E: std::error::Error,
{
    fn print_aoc_answer(&self, part_number: i32) {
        match self {
            Ok(answer) => println!("Part {part_number}: {answer}"),
            Err(error) => println!("Part {part_number} error!: {error}"),
        }
    }
}

impl AocAnswer for String {
    fn print_aoc_answer(&self, part_number: i32) {
        println!("Part {part_number}: {self}");
    }
}
