use crate::error::Error;
use crate::shared::GameStats;

pub fn run(input: &str) -> Result<String, Error> {
    let mut sum = 0;

    for line in input.lines() {
        let stats = GameStats::parse_line(line)?;

        if stats.max_red <= 12 && stats.max_green <= 13 && stats.max_blue <= 14 {
            sum += stats.id;
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        let output = run(input);

        assert_eq!(output.unwrap(), "8");
    }
}
