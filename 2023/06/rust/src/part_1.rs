use crate::{error::Error, shared::valid_inputs_for_problem};

pub fn run(input: &str) -> Result<String, Error> {
    let (times, distances) = input.split_once('\n').unwrap();

    let times = parse_line(times)?;
    let distances = parse_line(distances)?;

    let mut options = 1;

    for (time, distance) in times.iter().copied().zip(distances.iter().copied()) {
        options *= valid_inputs_for_problem(time, distance);
    }

    Ok(options.to_string())
}

fn parse_line(line: &str) -> Result<Vec<f64>, Error> {
    line.split_once(':')
        .ok_or(Error::MissingLineHeader)?
        .1
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f64>().map_err(Error::from))
        .collect::<Result<Vec<f64>, Error>>()
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
