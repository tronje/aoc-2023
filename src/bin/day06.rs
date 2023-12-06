use anyhow::Result;
use aoc_2023::day06::Race;
use std::fs::File;
use std::io::BufReader;

const PATH: &str = "inputs/day06/input";

fn part_one() -> Result<u64> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let races = Race::parse_p1(reader)?;
    let result = races.iter().map(Race::n_winning_holds).product();
    Ok(result)
}

fn part_two() -> Result<u64> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let race = Race::parse_p2(reader)?;
    Ok(race.solve())
}

fn main() {
    let result = part_one().unwrap();
    println!("Part one: {result}");

    let result = part_two().unwrap();
    println!("Part two: {result}");
}
