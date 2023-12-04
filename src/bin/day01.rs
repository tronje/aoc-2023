use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const PATH: &str = "inputs/day01/input";

pub fn part_one() -> Result<u32> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let mut total = 0;

    for line in reader.lines() {
        let line = line?;

        let mut first = None;
        let mut last = None;

        for chr in line.chars() {
            if let Some(digit) = chr.to_digit(10) {
                if first.is_none() {
                    first.replace(digit);
                } else {
                    last.replace(digit);
                }
            }
        }

        let first = first.unwrap();

        let mut value = first * 10;

        match last {
            Some(last) => value += last,
            None => value += first,
        }

        total += value;
    }

    Ok(total)
}

fn main() {
    let total = part_one().unwrap();
    println!("{total}");
}
