use anyhow::Result;
use aoc_2023::day04::Card;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const PATH: &str = "inputs/day04/input";

fn parse_cards() -> Result<Vec<Card>> {
    let f = File::open(PATH)?;
    let reader = BufReader::new(f);

    let mut cards = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let card = line.parse::<Card>()?;
        cards.push(card);
    }

    Ok(cards)
}

fn part_one() -> Result<u32> {
    let cards = parse_cards()?;
    let result = cards.iter().map(Card::points).sum();
    Ok(result)
}

fn part_two() -> Result<u32> {
    let cards = parse_cards()?;

    // how many times we have each card
    let mut counts = cards.iter().map(|_| 1).collect::<Vec<u32>>();

    for card in cards.iter() {
        let matches = card.matches();

        if matches > 0 {
            for i in (card.id() + 1)..=(card.id() + matches) {
                // Add each card we win as many times as we have the winning card.
                // Say we are looking at card 5 and it wins us card 6. But, due to prior cards, we
                // already have 3 of card 5. So we must add 3 of card 6 here.
                // This works out because no card can ever win a card below it, so we can't forget
                // to count any.
                counts[i as usize - 1] += counts[card.id() as usize - 1];
            }
        }
    }

    let result = counts.iter().sum();
    Ok(result)
}

fn main() {
    let card_total = part_one().unwrap();
    println!("Part one: {card_total}");

    let n_cards = part_two().unwrap();
    println!("Part two: {n_cards}");
}
