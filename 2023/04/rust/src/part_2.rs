use crate::{error::Error, shared::Card};

pub fn run(input: &str) -> Result<String, Error> {
    let mut cards = 0;
    let mut tracker = CopiesTracker::new();

    for line in input.lines() {
        let card_wins = Card::parse(line)?.compute_wins();
        let copies = tracker.get_stored_copies() + 1;

        tracker.next_card();
        tracker.add_card(card_wins, copies);

        cards += copies;
    }

    Ok(cards.to_string())
}

#[derive(Debug)]
struct CopiesTracker {
    remaining: Vec<(usize, u32)>,
}

impl CopiesTracker {
    fn new() -> CopiesTracker {
        CopiesTracker {
            remaining: Vec::new(),
        }
    }

    fn get_stored_copies(&self) -> u32 {
        self.remaining.iter().map(|(_, copies)| copies).sum()
    }

    fn next_card(&mut self) {
        let mut i = 0;

        while i < self.remaining.len() {
            let n = &mut self.remaining[i].0;

            *n -= 1;

            if *n == 0 {
                self.remaining.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn add_card(&mut self, card_wins: usize, copies: u32) {
        if card_wins > 0 {
            self.remaining.push((card_wins, copies));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

        let output = run(input);

        assert_eq!(output, Ok("30".to_owned()));
    }
}
