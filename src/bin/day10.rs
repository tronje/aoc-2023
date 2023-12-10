use anyhow::Result;
use aoc_2023::day10::Graph;
use std::fs::File;
use std::io::BufReader;

const PATH: &str = "inputs/day10/input";

fn part_one() -> Result<usize> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);
    let graph = Graph::parse(reader)?;

    let result = graph.solve_p1();
    Ok(result)
}

fn part_two() -> Result<usize> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);
    let graph = Graph::parse(reader)?;

    let result = graph.solve_p2();
    Ok(result)
}

fn main() {
    let steps = part_one().unwrap();
    println!("Part one: {steps}");

    let contained = part_two().unwrap();
    println!("Part one: {contained}");
}
