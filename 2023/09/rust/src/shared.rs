#[derive(Debug)]
pub struct OASISPredictor(Vec<i64>);

impl Iterator for OASISPredictor {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut counters = self.0.iter_mut().rev();
        let mut diff = *counters.next()?;

        for counter in counters {
            *counter += diff;
            diff = *counter;
        }

        Some(diff)
    }
}

impl From<&Vec<i64>> for OASISPredictor {
    fn from(numbers: &Vec<i64>) -> Self {
        let mut counters = Vec::new();

        for &number in numbers {
            let mut diff = number;

            for counter in &mut counters {
                diff -= *counter;
                *counter += diff;
            }

            if counters.is_empty() || diff != 0 {
                counters.push(diff);
            }
        }

        OASISPredictor(counters)
    }
}
