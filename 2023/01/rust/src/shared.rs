pub struct LineDigits(Option<(u32, u32)>);

impl LineDigits {
    pub fn new() -> LineDigits {
        LineDigits(None)
    }

    pub fn add_digit(&mut self, digit: u32) {
        match self.0 {
            None => self.0 = Some((digit, digit)),
            Some((_, ref mut last)) => *last = digit,
        }
    }

    pub fn get_number(&self) -> Option<u32> {
        self.0.map(|(first, last)| first * 10 + last)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_digit_returns_none() {
        let digits = LineDigits::new();

        assert_eq!(digits.get_number(), None);
    }

    #[test]
    fn single_added_digit_returns_repdigit() {
        let mut digits = LineDigits::new();

        digits.add_digit(1);

        assert_eq!(digits.get_number(), Some(11));
    }

    #[test]
    fn subsequent_added_digit_modifies_ones_place() {
        let mut digits = LineDigits::new();

        digits.add_digit(1);
        digits.add_digit(2);

        assert_eq!(digits.get_number(), Some(12));
    }
}
