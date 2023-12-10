use crate::{error::Error, shared::Hand};

pub fn run(input: &str) -> Result<String, Error> {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').ok_or(Error::InvalidHand)?;

            Ok((Hand::parse(hand, false), bid.parse::<u32>()?))
        })
        .collect::<Result<Vec<(Hand<'_>, u32)>, Error>>()?;

    hands.sort_unstable_by_key(|h| h.0.clone());

    let mut total_winnings = 0;

    for (i, hand) in hands.into_iter().enumerate() {
        total_winnings += (i + 1) * hand.1 as usize;
    }

    Ok(total_winnings.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

        let output = run(input);

        assert_eq!(output, Ok("6440".to_owned()));
    }
}
