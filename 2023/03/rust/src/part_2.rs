use crate::error::Error;
use crate::shared::EngineSchematic;
use aoc_helpers::neighbors::Grid2D;
use std::collections::HashSet;

pub fn run(input: &str) -> Result<String, Error> {
    let schematic = EngineSchematic::parse(input)?;

    let mut sum = 0;
    let mut gears = HashSet::with_capacity(8); // Reused to avoid excess heap allocations

    for (x, y, &byte) in schematic.iter() {
        if byte == b'*' {
            for (x, y, _, _) in schematic.neighbors_8(x, y) {
                if let Some(id) = schematic.number_id_at(x, y) {
                    gears.insert(id);
                }
            }

            if gears.len() == 2 {
                sum += gears
                    .iter()
                    .map(|&id| schematic.get_number(id).unwrap())
                    .product::<u64>();
            }

            gears.clear();
        }
    }

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

        assert_eq!(output, Ok("467835".to_owned()));
    }
}
