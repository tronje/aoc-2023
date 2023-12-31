use anyhow::Result;
use aoc_2023::day08::Map;
use std::fs::File;
use std::io::BufReader;

const PATH: &str = "inputs/day08/input";

fn part_one() -> Result<u32> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let map = Map::parse(reader)?;
    Ok(map.solve_p1())
}

fn part_two() -> Result<u64> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let map = Map::parse(reader)?;
    Ok(map.solve_p2())
}

fn main() {
    let steps = part_one().unwrap();
    println!("Part one: {steps}");

    let steps = part_two().unwrap();
    println!("Part two: {steps}");
}
