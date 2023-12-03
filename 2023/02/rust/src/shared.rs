use crate::error::Error;
use std::cmp::max;

#[derive(PartialEq, Eq, Debug)]
pub struct GameStats {
    pub id: i32,

    pub max_red: i32,
    pub max_green: i32,
    pub max_blue: i32,
}

impl GameStats {
    pub fn parse_line(line: &str) -> Result<GameStats, Error> {
        let (header, game) = line
            .split_once(':')
            .ok_or(Error::NoGameHeaderInLine(line))?;

        let game = game.trim();

        let game_id = header
            .split_once(' ')
            .ok_or(Error::InvalidGameHeader(header))?
            .1
            .parse::<i32>()
            .map_err(|_| Error::InvalidGameHeader(header))?;

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for cube_set in game.split([';', ',']) {
            let (count, color) = cube_set
                .trim()
                .split_once(' ')
                .ok_or(Error::InvalidCubeSet(cube_set))?;
            let count = count
                .parse::<i32>()
                .map_err(|_| Error::InvalidCubeSet(cube_set))?;

            match color {
                "red" => max_red = max(max_red, count),
                "green" => max_green = max(max_green, count),
                "blue" => max_blue = max(max_blue, count),
                _ => return Err(Error::UnknownColor(color)),
            }
        }

        Ok(GameStats {
            id: game_id,

            max_red,
            max_green,
            max_blue,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! line_test {
        ($suite:ident, $($name:ident: $input:expr => $expected:expr,)*) => {
            mod $suite {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        let stats = GameStats::parse_line($input);

                        assert_eq!(stats, $expected);
                    }
                )*
            }
        }
    }

    line_test!(line_tests,
        example_line_1: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" => Ok(
            GameStats {
                id: 1,
                max_red: 4,
                max_green: 2,
                max_blue: 6,
            }
        ),
        example_line_2: "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue" => Ok(
            GameStats {
                id: 2,
                max_red: 1,
                max_green: 3,
                max_blue: 4,
            }
        ),
        example_line_3: "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red" => Ok(
            GameStats {
                id: 3,
                max_red: 20,
                max_green: 13,
                max_blue: 6,
            }
        ),
        example_line_4: "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red" => Ok(
            GameStats {
                id: 4,
                max_red: 14,
                max_green: 3,
                max_blue: 15,
            }
        ),
        example_line_5: "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green" => Ok(
            GameStats {
                id: 5,
                max_red: 6,
                max_green: 3,
                max_blue: 2,
            }
        ),

        err_no_colon: "Game 1 4 red" => Err(Error::NoGameHeaderInLine("Game 1 4 red")),
        err_no_space_in_header: "Game1: 4 red" => Err(Error::InvalidGameHeader("Game1")),
        err_non_numeric_game_id: "Game one: 4 red" => Err(Error::InvalidGameHeader("Game one")),
        err_no_space_in_cube_set: "Game 1: 4red" => Err(Error::InvalidCubeSet("4red")),
        err_non_numeric_cube_set: "Game 1: four red" => Err(Error::InvalidCubeSet("four red")),
        err_unknown_color: "Game 1: 4 pink" => Err(Error::UnknownColor("pink")),
    );
}
