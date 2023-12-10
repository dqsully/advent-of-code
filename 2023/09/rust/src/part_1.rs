use std::num::ParseIntError;

use crate::{error::Error, shared::OASISPredictor};

pub fn run(input: &str) -> Result<String, Error> {
    let mut predictors = input
        .lines()
        .map(|line| {
            let numbers = line
                .split(' ')
                .map(str::parse)
                .collect::<Result<Vec<_>, ParseIntError>>()?;

            Ok(OASISPredictor::from(&numbers))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let sum_of_nexts = predictors
        .iter_mut()
        .map(|p| p.next().unwrap())
        .sum::<i64>();

    Ok(sum_of_nexts.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

        let output = run(input);

        assert_eq!(output, Ok("114".to_owned()));
    }
}
