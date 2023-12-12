use anyhow::Result;
use aoc_2023::day12::Record;
use std::fs::File;
use std::io::BufReader;

const PATH: &str = "inputs/day12/input";

fn part_one() -> Result<u32> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);
    let records = Record::parse(reader)?;

    let result = records.iter().map(Record::permutations).sum();
    Ok(result)
}

fn main() {
    let sum = part_one().unwrap();
    println!("Part one: {sum}");
}
