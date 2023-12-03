use crate::error::Error;
use std::cmp::max;

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
