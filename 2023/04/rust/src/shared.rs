use std::{collections::HashSet, num::ParseIntError};

use crate::error::Error;

pub struct Card {
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

impl Card {
    pub fn parse(line: &str) -> Result<Card, Error> {
        let card = line
            .split_once(':')
            .ok_or(Error::NoCardHeaderInLine(line))?
            .1
            .trim();

        let (winning_numbers, card_numbers) =
            card.split_once('|').ok_or(Error::NoCardSeparator(card))?;

        let winning_numbers = parse_numbers(winning_numbers).collect::<Result<HashSet<_>, _>>()?;
        let card_numbers = parse_numbers(card_numbers).collect::<Result<HashSet<_>, _>>()?;

        Ok(Card {
            winning_numbers,
            card_numbers,
        })
    }

    pub fn compute_wins(&self) -> usize {
        self.winning_numbers.intersection(&self.card_numbers).count()
    }
}

fn parse_numbers(input: &str) -> impl Iterator<Item = Result<u32, ParseIntError>> + '_ {
    input
        .trim()
        .split(' ')
        .filter(|txt| !txt.is_empty())
        .map(str::parse)
}
