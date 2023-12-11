use anyhow::Result;
use aoc_2023::day11::Universe;
use std::fs::File;
use std::io::BufReader;

const PATH: &str = "inputs/day11/input";

fn part_one() -> Result<usize> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);
    let mut universe = Universe::parse(reader)?;
    universe.expand();

    let result = universe.solve_p1();
    Ok(result)
}

fn main() {
    let sum = part_one().unwrap();
    println!("Part one: {sum}");
}
