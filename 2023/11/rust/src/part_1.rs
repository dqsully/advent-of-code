use crate::{error::Error, shared::run_with_expansion_factor};

pub fn run(input: &str) -> Result<String, Error> {
    run_with_expansion_factor(input, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

        let output = run(input);

        assert_eq!(output, Ok("374".to_owned()));
    }
}
