use anyhow::Result;
use aoc_2023::day07::Hand;
use std::fs::File;
use std::io::BufReader;

const PATH: &str = "inputs/day07/input";

fn part_one() -> Result<u32> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let mut hands = Hand::parse(reader)?;
    hands.sort();

    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.value(idx + 1))
        .sum();

    Ok(winnings)
}

fn main() {
    let winnings = part_one().unwrap();
    println!("Part one: {winnings}");
}
