use crate::error::Error;
use crate::shared::EngineSchematic;
use aoc_helpers::neighbors::Grid2D;
use std::collections::HashSet;

pub fn run(input: &str) -> Result<String, Error> {
    let schematic = EngineSchematic::parse(input)?;

    let mut number_ids = HashSet::new();

    for (x, y, byte) in schematic.iter() {
        match byte {
            b'0'..=b'9' | b'.' => {} // Do nothing
            _ => {
                // Symbol
                for (x, y, _, _) in schematic.neighbors_8(x, y) {
                    if let Some(id) = schematic.number_id_at(x, y) {
                        number_ids.insert(id);
                    }
                }
            }
        }
    }

    let sum: u64 = number_ids
        .into_iter()
        .map(|id| schematic.get_number(id).unwrap())
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

        let output = run(input);

        assert_eq!(output, Ok("4361".to_owned()));
    }
}
