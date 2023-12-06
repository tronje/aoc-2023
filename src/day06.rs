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

    pub fn parse<R>(reader: R) -> Result<Vec<Self>>
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
}
