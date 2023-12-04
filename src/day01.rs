use std::str::Chars;

/// Compute the calibration value from an iterator of numbers (digits).
pub fn calibration_value<I>(iter: I) -> u32
where
    I: Iterator<Item = u32>,
{
    let mut first = None;
    let mut last = None;

    for digit in iter {
        if first.is_none() {
            first.replace(digit);
        } else {
            last.replace(digit);
        }
    }
    let first = first.unwrap();

    let mut value = first * 10;

    match last {
        Some(last) => value += last,
        None => value += first,
    }

    value
}

/// An iterator over a string that yields its digits, spelled out or not.
///
/// E.g. takes a string like "two1nine" and yields `Some(2)`, `Some(1)`, `Some(9)`, `None`.
#[derive(Debug)]
pub struct Digits<'a> {
    chars: Chars<'a>,
    buf: String,
}

impl<'a> Digits<'a> {
    const SPELLED_DIGITS: [&'static str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    pub fn new(data: &'a str) -> Self {
        Self {
            chars: data.chars(),
            buf: String::new(),
        }
    }

    fn matches_any_prefix(s: &str) -> bool {
        for digit in Self::SPELLED_DIGITS {
            if digit.starts_with(s) {
                return true;
            }
        }

        false
    }

    fn str_to_digit(s: &str) -> Option<u32> {
        for (n, digit) in Self::SPELLED_DIGITS.iter().enumerate() {
            if &s == digit {
                return Some(n as u32);
            }
        }

        None
    }
}

impl<'a> Iterator for Digits<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = match self.chars.next() {
                Some(chr) => chr,
                None => return Self::str_to_digit(&self.buf),
            };

            if let Some(digit) = c.to_digit(10) {
                self.buf.clear();
                return Some(digit);
            }

            self.buf.push(c);

            if let Some(digit) = Self::str_to_digit(&self.buf) {
                self.buf.clear();
                self.buf.push(c);
                return Some(digit);
            }

            if !Self::matches_any_prefix(&self.buf) {
                self.buf.remove(0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_tests_part_one() {
        let tests = [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for (test, solution) in tests {
            let digits = test.chars().filter_map(|chr| chr.to_digit(10));
            let val = calibration_value(digits);
            assert_eq!(val, solution);
        }
    }

    #[test]
    fn provided_tests_part_two() {
        let tests = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];

        for (test, solution) in tests {
            let digits = Digits::new(test);
            let val = calibration_value(digits);
            assert_eq!(val, solution);
        }
    }

    #[test]
    fn finds_digits() {
        let test = "feafo3feoiamf9fkeaf4ekefa1";
        let mut digits = Digits::new(test);

        assert_eq!(digits.next(), Some(3));
        assert_eq!(digits.next(), Some(9));
        assert_eq!(digits.next(), Some(4));
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn finds_words() {
        let test = "oneifaofoajtwofeafnaifthreefnafnaffourofnoafiveewaffkalfdsix";
        let mut digits = Digits::new(test);

        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(3));
        assert_eq!(digits.next(), Some(4));
        assert_eq!(digits.next(), Some(5));
        assert_eq!(digits.next(), Some(6));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn finds_both() {
        let test = "onejfafa2foamfthreeoifaofmafourfoaefoiamf5xxao";
        let mut digits = Digits::new(test);

        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(3));
        assert_eq!(digits.next(), Some(4));
        assert_eq!(digits.next(), Some(5));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn finds_word_at_end() {
        let test = "feoiafoamftwo";
        let mut digits = Digits::new(test);

        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn finds_digit_at_end() {
        let test = "feoiafoamf2";
        let mut digits = Digits::new(test);

        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn finds_overlapping() {
        let test = "eightwone7eightwone";
        let mut digits = Digits::new(test);

        assert_eq!(digits.next(), Some(8));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), Some(7));
        assert_eq!(digits.next(), Some(8));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn test_zeight() {
        let test = "zeight";
        let mut digits = Digits::new(test);

        assert_eq!(digits.next(), Some(8));
        assert_eq!(digits.next(), None);
    }
}
