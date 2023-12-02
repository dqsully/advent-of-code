use thiserror::Error;
use std::cmp;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("unimplemented")]
    Unimplemented,
}

pub fn run(input: &str) -> Result<String, AocError> {
    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        let game_input = line.split_once(':').unwrap().1.trim();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for reveal in game_input.split(';') {
            for cube_set in reveal.split(',') {
                let (count, color) = cube_set.trim().split_once(' ').unwrap();

                let count = count.parse::<u64>().unwrap();

                match color {
                    "red" => min_red = cmp::max(min_red, count),
                    "green" => min_green = cmp::max(min_green, count),
                    "blue" => min_blue = cmp::max(min_blue, count),
                    _ => {
                        panic!("unknown color")
                    }
                }
            }
        }

        sum += min_red * min_green * min_blue;
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        let output = super::run(input);

        assert_eq!(output.unwrap(), "2286");
    }
}
