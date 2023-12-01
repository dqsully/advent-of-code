use std::fs;

const PART_2_NUMBERS: [(&str, i32); 19] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("0", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    println!("2023/12/01 - Rust");
    part_1();
    part_2();
}

fn part_1() {
    let input_txt = fs::read_to_string("../input.txt").unwrap();

    let mut sum = 0;

    for line in input_txt.lines() {
        let mut first_digit = None;
        let mut last_digit = None;

        for &b in line.as_bytes() {
            if b.is_ascii_digit() {
                let digit = (b - b'0') as i32;

                if first_digit.is_none() {
                    first_digit = Some(digit);
                }

                last_digit = Some(digit);
            }
        }

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            sum += first * 10 + last;
        } else {
            panic!("did not find any digits in line");
        }
    }

    println!("Part 1: {}", sum);
}

fn part_2() {
    let input_txt = fs::read_to_string("../input.txt").unwrap();

    let mut sum = 0;

    // TODO: less imperative

    for line in input_txt.lines() {
        let mut first_digit = None;
        let mut last_digit = None;

        let line = line.as_bytes();
        let mut i = 0;

        while i < line.len() {
            for (word, value) in PART_2_NUMBERS {
                let word = word.as_bytes();

                if i + word.len() > line.len() {
                    continue;
                }

                if &line[i .. (i + word.len())] == word {
                    if first_digit.is_none() {
                        first_digit = Some(value);
                    }

                    last_digit = Some(value);

                    break;
                }
            }

            i += 1;
        }

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            sum += first * 10 + last;
        } else {
            panic!("did not find any digits in line");
        }
    }

    println!("Part 2: {}", sum);
}
