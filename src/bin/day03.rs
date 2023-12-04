use anyhow::Result;
use aoc_2023::day03::EngineSchematic;
use std::fs::File;
use std::io::BufReader;

const PATH: &str = "inputs/day03/input";

fn part_one() -> Result<u32> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let schematic = EngineSchematic::load(reader)?;
    let part_nos = schematic.part_numbers();

    let result = part_nos.iter().map(|part_no| part_no.num).sum();
    Ok(result)
}

fn part_two() -> Result<u32> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let schematic = EngineSchematic::load(reader)?;
    let gears = schematic.gears();

    let result = gears.iter().map(|gear| gear.ratio).sum();
    Ok(result)
}

fn main() {
    let part_no_sum = part_one().unwrap();
    println!("Part one: {part_no_sum}");

    let gear_ratio_sum = part_two().unwrap();
    println!("Part two: {gear_ratio_sum}");
}
