use crate::{error::Error, shared::valid_inputs_for_problem};

pub fn run(input: &str) -> Result<String, Error> {
    let (times, distances) = input.split_once('\n').unwrap();

    let time = parse_line(times)?;
    let distance = parse_line(distances)?;

    Ok(valid_inputs_for_problem(time, distance).to_string())
}

fn parse_line(line: &str) -> Result<f64, Error> {
    line.split_once(':')
        .ok_or(Error::MissingLineHeader)?
        .1
        .trim()
        .chars()
        .filter(|&c| c != ' ')
        .collect::<String>()
        .parse::<f64>()
        .map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"Time:      7  15   30
        Distance:  9  40  200
        ";

        let output = run(input);

        assert_eq!(output, Ok("288".to_owned()));
    }
}
