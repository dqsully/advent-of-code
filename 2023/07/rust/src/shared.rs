use std::{cmp::Ordering, collections::HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Card {
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
    CJ,
    CQ,
    CK,
    CA,
}

impl Card {
    pub fn parse(value: char, j_is_joker: bool) -> Option<Card> {
        match value {
            'J' => {
                if j_is_joker {
                    Some(Card::CJoker)
                } else {
                    Some(Card::CJ)
                }
            }
            '2' => Some(Card::C2),
            '3' => Some(Card::C3),
            '4' => Some(Card::C4),
            '5' => Some(Card::C5),
            '6' => Some(Card::C6),
            '7' => Some(Card::C7),
            '8' => Some(Card::C8),
            '9' => Some(Card::C9),
            'T' => Some(Card::CT),
            'Q' => Some(Card::CQ),
            'K' => Some(Card::CK),
            'A' => Some(Card::CA),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Hand<'a> {
    cards: &'a str,
    play: HandType,
    j_is_joker: bool,
}

impl<'a> Hand<'a> {
    pub fn parse(s: &str, j_is_joker: bool) -> Hand {
        let mut chars: HashMap<char, u32> = HashMap::new();

        for ch in s.chars() {
            chars.entry(ch).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut jokers = 0;

        let mut chars = chars
            .into_iter()
            .filter(|&ch| {
                if ch.0 == 'J' && j_is_joker {
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

        Hand {
            cards: s,
            play: hand_type,
            j_is_joker,
        }
    }
}

impl<'a> std::cmp::Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.play.cmp(&other.play).then_with(|| {
            for (s, o) in self.cards.chars().zip(other.cards.chars()) {
                let cmp = Card::parse(s, self.j_is_joker).cmp(&Card::parse(o, other.j_is_joker));

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
