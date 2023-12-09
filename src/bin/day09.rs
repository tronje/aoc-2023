use anyhow::Result;
use aoc_2023::day09::History;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PATH: &str = "inputs/day09/input";

fn part_one() -> Result<i32> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let result: i32 = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse::<History>())
        .map(Result::unwrap)
        .map(|h| h.next())
        .sum();

    Ok(result)
}

fn part_two() -> Result<i32> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let result: i32 = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse::<History>())
        .map(Result::unwrap)
        .map(|h| h.prev())
        .sum();

    Ok(result)
}

fn main() {
    let sum = part_one().unwrap();
    println!("Part one: {sum}");

    let sum = part_two().unwrap();
    println!("Part two: {sum}");
}
