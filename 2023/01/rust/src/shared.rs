pub struct LineDigits {
    first: Option<u32>,
    last: Option<u32>,
}

impl LineDigits {
    pub fn new() -> LineDigits {
        LineDigits {
            first: None,
            last: None,
        }
    }

    pub fn add_digit(&mut self, digit: u32) {
        self.first.get_or_insert(digit);
        self.last = Some(digit);
    }

    pub fn get_number(&self) -> Option<u32> {
        if let (Some(first), Some(last)) = (self.first, self.last) {
            Some(first * 10 + last)
        } else {
            None
        }
    }
}
