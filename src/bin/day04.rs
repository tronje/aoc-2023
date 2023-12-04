use anyhow::Result;
use aoc_2023::day04::Card;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const PATH: &str = "inputs/day04/input";

fn part_one() -> Result<u32> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let mut cards = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let card = line.parse::<Card>()?;
        cards.push(card);
    }

    let result = cards.iter().map(Card::points).sum();
    Ok(result)
}

fn main() {
    let card_total = part_one().unwrap();
    println!("Part one: {card_total}");
}
