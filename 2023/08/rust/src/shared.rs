use std::collections::HashMap;

use crate::error::Error;

pub struct Node<'a> {
    pub left: &'a str,
    pub right: &'a str,
}

pub fn parse_camel_map(map: &str) -> Result<(Vec<char>, HashMap<&str, Node>), Error> {
    let (steps_text, nodes_text) = map.split_once("\n\n").ok_or(Error::InvalidMapFormat)?;

    let steps = steps_text.trim().chars().collect::<Vec<_>>();

    let mut nodes: HashMap<&str, Node> = HashMap::new();

    for line in nodes_text.lines() {
        let (key, values) = line.split_once(" = ").ok_or(Error::InvalidMapFormat)?;
        let (left, right) = values.trim_matches(&['(', ')']).split_once(", ").ok_or(Error::InvalidMapFormat)?;

        nodes.insert(key, Node {left, right});
    }

    Ok((steps, nodes))
}
