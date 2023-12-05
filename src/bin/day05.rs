use anyhow::Result;
use aoc_2023::day05::Almanac;
use std::fs::File;
use std::io::BufReader;

const PATH: &str = "inputs/day05/input";

fn part_one() -> Result<u64> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let almanac = Almanac::parse(reader)?;
    Ok(almanac.part_one())
}

fn part_two() -> Result<u64> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let mut almanac = Almanac::parse(reader)?;
    Ok(almanac.part_two())
}

fn main() {
    let min_location = part_one().unwrap();
    println!("Part one: {min_location}");

    let min_location = part_two().unwrap();
    println!("Part two: {min_location}");
}
