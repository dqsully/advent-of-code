use std::collections::HashMap;

use num::Integer;

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

    let mut result: u64 = 1;

    for mut node in nodes.keys().cloned() {
        if node.ends_with('A') {
            let mut steps_taken = 0;

            for step in steps.iter().copied().cycle() {
                node = match step {
                    'L' => nodes[node].left,
                    'R' => nodes[node].right,
                    _ => panic!("unexpected step {}", step),
                };
                steps_taken += 1;

                if node.ends_with('Z') {
                    break;
                }
            }

            result = result.lcm(&steps_taken);
        }
    }

    Ok(result.to_string())
}

struct Node<'a> {
    left: &'a str,
    right: &'a str,
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
