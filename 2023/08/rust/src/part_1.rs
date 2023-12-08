use std::collections::HashMap;

use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    let (steps_text, nodes_text) = input.split_once("\n\n").unwrap();

    let steps = steps_text.trim().chars().collect::<Vec<_>>();

    let mut nodes: HashMap<&str, Node> = HashMap::new();

    for line in nodes_text.lines() {
        let (key, values) = line.split_once(" = ").unwrap();
        let (left, right) = values.trim_matches(&['(', ')']).split_once(", ").unwrap();

        nodes.insert(key, Node {left, right});
    }

    let mut steps_taken = 0;
    let mut node = "AAA";

    for step in steps.iter().copied().cycle() {
        node = match step {
            'L' => nodes[node].left,
            'R' => nodes[node].right,
            _ => panic!("unexpected step {step:?}"),
        };
        steps_taken += 1;

        if node == "ZZZ" {
            break;
        }
    }

    Ok(steps_taken.to_string())
}

struct Node<'a> {
    left: &'a str,
    right: &'a str,
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
