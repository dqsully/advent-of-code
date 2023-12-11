use crate::{error::Error, shared::run_with_expansion_factor};

pub fn run(input: &str) -> Result<String, Error> {
    run_with_expansion_factor(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
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

        let output = run_with_expansion_factor(input, 10);

        assert_eq!(output, Ok("1030".to_owned()));
    }

    #[test]
    fn example_2() {
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

        let output = run_with_expansion_factor(input, 100);

        assert_eq!(output, Ok("8410".to_owned()));
    }
}
