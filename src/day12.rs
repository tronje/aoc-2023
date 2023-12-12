use anyhow::{anyhow, Context, Result};
use std::io::BufRead;

trait BitOps: Copy {
    fn bit(self, n: Self) -> bool;
    fn set_bit(&mut self, n: Self);
    fn clear_bit(&mut self, n: Self);
}

impl BitOps for u32 {
    fn bit(self, n: Self) -> bool {
        debug_assert!(n < Self::BITS);

        (self & (1 << n)) > 0
    }

    fn set_bit(&mut self, n: Self) {
        debug_assert!(n < Self::BITS);

        *self |= 1 << n
    }

    fn clear_bit(&mut self, n: Self) {
        debug_assert!(n < Self::BITS);

        *self &= !(1 << n)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Spring {
    Intact,
    Broken,
    Unknown,
}

#[derive(Debug)]
pub struct Record {
    arrangements: Vec<u32>,
    total: u32,
    lower: u32,
    upper: u32,
}

impl Record {
    fn matches(&self, n: u32) -> bool {
        if n | self.lower != n {
            return false;
        }

        if n & !self.upper > 0 {
            return false;
        }

        if n.count_ones() != self.total {
            return false;
        }

        let mut idx = 0;
        let mut last_bit_set = false;
        let mut group_len = 0;

        for bit in (0..u32::BITS).rev() {
            let bit_set = n.bit(bit);

            if !bit_set && last_bit_set {
                if idx >= self.arrangements.len() {
                    return false;
                }

                if self.arrangements[idx] != group_len {
                    return false;
                }

                idx += 1;
                group_len = 0;
            } else if bit_set {
                group_len += 1;
            }

            last_bit_set = bit_set;
        }

        if group_len > 0 {
            if idx >= self.arrangements.len() {
                return false;
            }

            return self.arrangements[idx] == group_len;
        }

        true
    }

    pub fn permutations(&self) -> u32 {
        (self.lower..=self.upper)
            .map(|n| self.matches(n))
            .filter(|b| *b)
            .count() as u32
    }

    pub fn parse<R>(reader: R) -> Result<Vec<Self>>
    where
        R: BufRead,
    {
        let mut records = Vec::new();
        let mut springs = Vec::new();

        for line in reader.lines() {
            let line = line?;

            let mut components = line.split(' ');

            for chr in components.next().context("invalid input")?.chars() {
                match chr {
                    '.' => springs.push(Spring::Intact),
                    '#' => springs.push(Spring::Broken),
                    '?' => springs.push(Spring::Unknown),
                    _ => return Err(anyhow!("invalid input")),
                }
            }

            let mut lower = 0;
            let mut upper = 0;

            for (bit, spring) in springs.iter().rev().enumerate() {
                match spring {
                    Spring::Broken => {
                        lower.set_bit(bit as u32);
                        upper.set_bit(bit as u32);
                    }

                    Spring::Unknown => {
                        upper.set_bit(bit as u32);
                    }

                    _ => {}
                }
            }

            let arrangements = components
                .next()
                .context("invalid input")?
                .split(',')
                .map(|s| s.parse::<u32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;

            let total = arrangements.iter().sum();

            records.push(Record {
                arrangements,
                total,
                lower,
                upper,
            });

            springs.clear();
        }

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    static EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn example_input() {
        let reader = BufReader::new(EXAMPLE.as_bytes());
        let records = Record::parse(reader).unwrap();

        for (idx, record) in records.iter().enumerate() {
            match idx {
                0 => assert_eq!(record.permutations(), 1),
                1 => assert_eq!(record.permutations(), 4),
                2 => assert_eq!(record.permutations(), 1),
                3 => assert_eq!(record.permutations(), 1),
                4 => assert_eq!(record.permutations(), 4),
                5 => assert_eq!(record.permutations(), 10),
                _ => unreachable!(),
            }
        }
    }
}
