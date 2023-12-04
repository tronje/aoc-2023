use anyhow::anyhow;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

impl Card {
    fn empty() -> Self {
        Self {
            id: 0,
            winning: HashSet::new(),
            have: HashSet::new(),
        }
    }

    pub fn points(&self) -> u32 {
        let mut points = 0;

        for have in self.have.iter() {
            if self.winning.contains(have) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        points
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = String::new();
        let mut card = Self::empty();

        let mut found_id = false;
        let mut found_pipe = false;

        for c in s.chars() {
            if c == '|' {
                found_pipe = true;
            } else if c.is_ascii_digit() {
                buf.push(c);
                continue;
            }

            if buf.is_empty() {
                continue;
            }

            let result = buf.parse::<u32>();
            buf.clear();

            let num = match result {
                Ok(num) => num,
                Err(_) => continue,
            };

            match (found_id, found_pipe) {
                (false, false) => {
                    card.id = num;
                    found_id = true;
                }
                (true, false) => {
                    card.winning.insert(num);
                }
                (true, true) => {
                    card.have.insert(num);
                }
                (false, true) => return Err(anyhow!("Invalid input!")),
            };
        }

        let num = buf.parse::<u32>()?;
        card.have.insert(num);

        Ok(card)
    }
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn provided_tests_part_one() {
        let tests = [
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8),
            ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2),
            ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
            ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0),
        ];

        for (idx, (test, solution)) in tests.iter().enumerate() {
            let card = test.parse::<Card>().unwrap();
            println!("{card:?}");
            assert_eq!(card.id, idx as u32 + 1);
            assert_eq!(card.points(), *solution);
        }
    }
}
