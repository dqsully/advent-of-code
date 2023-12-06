use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    let (times, distances) = input.split_once('\n').unwrap();

    let time = times.split_once(':').unwrap().1.trim()
        .chars()
        .filter(|&c| c != ' ')
        .collect::<String>()
        .parse::<f64>().unwrap();
    let distance = distances.split_once(':').unwrap().1.trim()
        .chars()
        .filter(|&c| c != ' ')
        .collect::<String>()
        .parse::<f64>().unwrap();


    let o = (time*time/4.0 - distance).sqrt();

    let min = (-o + time/2.0).floor() as i32;
    let max = (o + time/2.0).ceil() as i32 - 1;

    Ok((max - min).to_string())
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
