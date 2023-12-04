use anyhow::Result;
use aoc_2023::day02::{CubeSet, Game};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

const PATH: &str = "inputs/day02/input";

fn parse_games() -> Result<Vec<Game>> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let mut games = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let game = Game::from_str(&line)?;
        games.push(game);
    }

    Ok(games)
}

fn part_one() -> Result<u32> {
    let games = parse_games()?;
    let config = CubeSet::empty().with_red(12).with_green(13).with_blue(14);

    let result = games
        .iter()
        .filter(|game| game.is_valid(&config))
        .map(Game::id)
        .sum();

    Ok(result)
}

fn main() {
    let id_sum = part_one().unwrap();
    println!("{id_sum}");
}
