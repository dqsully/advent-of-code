use crate::{error::Error, shared::parse_camel_map};

pub fn run(input: &str) -> Result<String, Error> {
    let (steps, nodes) = parse_camel_map(input)?;

    let mut steps_taken = 0;
    let mut node = "AAA";

    for step in steps.iter().copied().cycle() {
        node = match step {
            'L' => nodes[node].left,
            'R' => nodes[node].right,
            _ => return Err(Error::InvalidMapFormat),
        };
        steps_taken += 1;

        if node == "ZZZ" {
            break;
        }
    }

    Ok(steps_taken.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

        let output = run(input);

        assert_eq!(output, Ok("2".to_owned()));
    }

    #[test]
    fn example_2() {
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

        let output = run(input);

        assert_eq!(output, Ok("6".to_owned()));
    }
}
