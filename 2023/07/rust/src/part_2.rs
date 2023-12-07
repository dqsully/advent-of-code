use std::{collections::HashMap, cmp::Ordering};

use crate::error::Error;

pub fn run(input: &str) -> Result<String, Error> {
    let mut hands: Vec<(Hand, u32)> = input.lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            (Hand::parse(hand), bid.parse().unwrap())
        })
        .collect();

    hands.sort_unstable_by_key(|h| h.0.clone());

    let mut total_winnings = 0;

    for (i, hand) in hands.into_iter().enumerate() {
        total_winnings += (i + 1) * hand.1 as usize;
    }

    Ok(total_winnings.to_string())
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    CJoker,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CQ,
    CK,
    CA,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'J' => Card::CJoker,
            '2' => Card::C2,
            '3' => Card::C3,
            '4' => Card::C4,
            '5' => Card::C5,
            '6' => Card::C6,
            '7' => Card::C7,
            '8' => Card::C8,
            '9' => Card::C9,
            'T' => Card::CT,
            'Q' => Card::CQ,
            'K' => Card::CK,
            'A' => Card::CA,
            _ => panic!("not a card: {value:?}")
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Hand<'a>(&'a str, HandType);

impl<'a> Hand<'a> {
    fn parse(s: &str) -> Hand {
        let mut chars: HashMap<char, u32> = HashMap::new();

        for ch in s.chars() {
            chars.entry(ch)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }

        let mut jokers = 0;

        let mut chars = chars.into_iter()
            .filter(|&ch| {
                if ch.0 == 'J' {
                    jokers = ch.1;
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<_>>();
        chars.sort_unstable_by_key(|ch| ch.1);
        chars.reverse();

        let hand_type = match chars.len() {
            0 | 1 => HandType::FiveOfAKind,
            2 => match chars[0].1 + jokers {
                4 => HandType::FourOfAKind,
                3 => HandType::FullHouse,
                _ => panic!("2 variants: {s:?}"),
            },
            3 => match chars[0].1 + jokers {
                3 => HandType::ThreeOfAKind,
                2 => HandType::TwoPair,
                _ => panic!("3 variants: {s:?}"),
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("more than 5 variants: {s:?}"),
        };

        Hand(s, hand_type)
    }
}

impl<'a> std::cmp::Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).then_with(|| {
            for (s, o) in self.0.chars().zip(other.0.chars()) {
                let cmp = Card::from(s).cmp(&Card::from(o));

                if cmp != Ordering::Equal {
                    return cmp;
                }
            }

            Ordering::Equal
        })
    }
}

impl<'a> std::cmp::PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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

        assert_eq!(output, Ok("5905".to_owned()));
    }
}
