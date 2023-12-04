use anyhow::Result;
use aoc_2023::day01::{calibration_value, Digits};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const PATH: &str = "inputs/day01/input";

fn part_one() -> Result<u32> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let mut total = 0;

    for line in reader.lines() {
        let line = line?;

        let digits = line.chars().filter_map(|chr| chr.to_digit(10));
        let value = calibration_value(digits);

        total += value;
    }

    Ok(total)
}

fn part_two() -> Result<u32> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let mut total = 0;

    for line in reader.lines() {
        let line = line?;

        let digits = Digits::new(&line);
        let value = calibration_value(digits);

        total += value
    }

    Ok(total)
}

fn main() {
    let total = part_one().unwrap();
    println!("Part one: {total}");

    let total = part_two().unwrap();
    println!("Part two: {total}");
}
