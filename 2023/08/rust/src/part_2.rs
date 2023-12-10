use num::Integer;

use crate::{error::Error, shared::parse_camel_map};

pub fn run(input: &str) -> Result<String, Error> {
    let (steps, nodes) = parse_camel_map(input)?;

    let mut result: u64 = 1;

    for mut node in nodes.keys().copied().filter(|n| n.ends_with('A')) {
        let mut steps_taken = 0;

        for step in steps.iter().copied().cycle() {
            node = match step {
                'L' => nodes[node].left,
                'R' => nodes[node].right,
                _ => return Err(Error::InvalidMapFormat),
            };
            steps_taken += 1;

            if node.ends_with('Z') {
                break;
            }
        }

        result = result.lcm(&steps_taken);
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

        let output = run(input);

        assert_eq!(output, Ok("6".to_owned()));
    }
}
