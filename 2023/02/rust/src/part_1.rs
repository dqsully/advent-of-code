pub fn run(input: &str) -> String {
    let mut sum = 0;

    'line_loop: for (i, line) in input.lines().enumerate() {
        let game_input = line.split_once(':').unwrap().1.trim();

        for reveal in game_input.split(';') {
            for cube_set in reveal.split(',') {
                let (count, color) = cube_set.trim().split_once(' ').unwrap();

                let count = count.parse::<i32>().unwrap();

                match color {
                    "red" if count <= 12 => {},
                    "green" if count <= 13 => {},
                    "blue" if count <= 14 => {},
                    _ => {
                        continue 'line_loop;
                    }
                }
            }
        }

        sum += i + 1;
    }

    sum.to_string()
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

        assert_eq!(output, "8");
    }
}
