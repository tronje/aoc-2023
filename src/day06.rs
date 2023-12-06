use anyhow::{Context, Result};
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn n_winning_holds(&self) -> u64 {
        let mut n = 0;

        for t in 0..self.time {
            let d = (self.time - t) * t;
            if d > self.distance {
                n += 1;
            }
        }

        n
    }

    pub fn solve(&self) -> u64 {
        let time = self.time as f64;
        let distance = self.distance as f64;

        let sqrt = (time.powi(2) - 4.0 * distance).sqrt();

        let lower = (-time - sqrt) / 2.0;
        let upper = (-time + sqrt) / 2.0;

        (upper.ceil() - lower.floor()) as u64 - 1
    }

    fn parse_numbers(s: &str) -> Vec<u64> {
        let mut n = 0;
        let mut factor = 1;
        let mut ns = Vec::new();

        for chr in s.chars().rev() {
            if let Some(digit) = chr.to_digit(10) {
                n += digit as u64 * factor;
                factor *= 10;
            } else if n > 0 {
                ns.push(n);
                n = 0;
                factor = 1;
            }
        }

        ns
    }

    fn parse_number(s: &str) -> u64 {
        let mut n = 0;
        let mut factor = 1;

        for chr in s.chars().rev() {
            if let Some(digit) = chr.to_digit(10) {
                n += digit as u64 * factor;
                factor *= 10;
            }
        }

        n
    }

    pub fn parse_p1<R>(reader: R) -> Result<Vec<Self>>
    where
        R: BufRead,
    {
        let mut lines = reader.lines();
        let times = Self::parse_numbers(&lines.next().context("invalid input")??);
        let distances = Self::parse_numbers(&lines.next().context("invalid input")??);

        let races = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Self { time, distance })
            .collect();

        Ok(races)
    }

    pub fn parse_p2<R>(reader: R) -> Result<Self>
    where
        R: BufRead,
    {
        let mut lines = reader.lines();
        let time = Self::parse_number(&lines.next().context("invalid input")??);
        let distance = Self::parse_number(&lines.next().context("invalid input")??);

        Ok(Self { time, distance })
    }
}

#[cfg(test)]
mod tests {
    use super::Race;
    use std::io::BufReader;

    const EXAMPLE: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn test_examples() {
        // part one
        let races = Race::parse_p1(BufReader::new(EXAMPLE.as_bytes())).unwrap();
        let naive_result = races.iter().map(Race::n_winning_holds).product::<u64>();
        let smart_result = races.iter().map(Race::solve).product();

        assert_eq!(naive_result, 288);
        assert_eq!(naive_result, smart_result);

        // part two
        let race = Race::parse_p2(BufReader::new(EXAMPLE.as_bytes())).unwrap();
        let naive_result = race.n_winning_holds();
        let smart_result = race.solve();

        assert_eq!(naive_result, 71503);
        assert_eq!(naive_result, smart_result);
    }
}
