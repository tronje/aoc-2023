use std::str::FromStr;

pub struct History {
    values: Vec<i32>,
}

impl History {
    fn derivative(&self) -> Self {
        let mut values = Vec::with_capacity(self.values.len());

        for i in 1..self.values.len() {
            let x = self.values[i] - self.values[i - 1];
            values.push(x);
        }

        Self { values }
    }

    pub fn next(&self) -> i32 {
        if self.values.iter().all(|v| *v == 0) {
            0
        } else {
            let x = self.values.iter().last().unwrap();
            x + self.derivative().next()
        }
    }

    pub fn prev(&self) -> i32 {
        if self.values.iter().all(|v| *v == 0) {
            0
        } else {
            let x = self.values[0];
            x - self.derivative().prev()
        }
    }
}

impl FromStr for History {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(' ')
            .map(str::parse::<i32>)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { values })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};

    static EXAMPLE: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45\n";

    #[test]
    fn example_input_part1() {
        let reader = BufReader::new(EXAMPLE.as_bytes());
        let result: i32 = reader
            .lines()
            .map(Result::unwrap)
            .map(|s| s.parse::<History>())
            .map(Result::unwrap)
            .map(|h| h.next())
            .sum();
        assert_eq!(result, 114);
    }

    #[test]
    fn example_input_part2() {
        let reader = BufReader::new(EXAMPLE.as_bytes());
        let result: i32 = reader
            .lines()
            .map(Result::unwrap)
            .map(|s| s.parse::<History>())
            .map(Result::unwrap)
            .map(|h| h.prev())
            .sum();
        assert_eq!(result, 2);
    }
}
