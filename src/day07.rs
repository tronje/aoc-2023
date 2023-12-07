use anyhow::{anyhow, Context};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Card(u32);

impl Card {
    fn as_int(self) -> u32 {
        self.0
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_int().cmp(&other.as_int())
    }
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self(14)),
            'K' => Ok(Self(13)),
            'Q' => Ok(Self(12)),
            'J' => Ok(Self(11)),
            'T' => Ok(Self(10)),
            _ => {
                if let Some(digit) = c.to_digit(10) {
                    if !(2..=9).contains(&digit) {
                        Err(anyhow!("not a valid card: {c}"))
                    } else {
                        Ok(Self(digit))
                    }
                } else {
                    Err(anyhow!("not a valid card: {c}"))
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Triple,
    FullHouse,
    Quartet,
    Quintet,
}

#[derive(Debug, Default)]
pub struct Hand {
    cards: [Card; 5],
    bet: u32,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut card_counts = [0; 13];

        for card in self.cards.iter() {
            card_counts[card.as_int() as usize - 2] += 1;
        }

        let max = *card_counts.iter().max().unwrap();

        if max == 5 {
            HandType::Quintet
        } else if max == 4 {
            HandType::Quartet
        } else if max == 3 {
            if card_counts.iter().any(|count| *count == 2) {
                HandType::FullHouse
            } else {
                HandType::Triple
            }
        } else if max == 2 {
            if card_counts.iter().filter(|&count| *count == 2).count() == 2 {
                HandType::TwoPair
            } else {
                HandType::Pair
            }
        } else {
            HandType::HighCard
        }
    }

    pub fn value(&self, rank: usize) -> u32 {
        self.bet * (rank as u32)
    }

    pub fn parse<R>(reader: R) -> anyhow::Result<Vec<Self>>
    where
        R: BufRead,
    {
        let mut hands = Vec::new();

        for line in reader.lines() {
            let line = line?;

            hands.push(line.parse()?);
        }

        Ok(hands)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.hand_type().cmp(&other.hand_type());

        match type_order {
            Ordering::Equal => {
                for (&card_a, card_b) in self.cards.iter().zip(other.cards) {
                    if card_a != card_b {
                        return card_a.cmp(&card_b);
                    }
                }

                Ordering::Equal
            }

            other => other,
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand = Self::default();

        let mut components = s.split(' ');

        for (i, c) in components
            .next()
            .context("invalid hand")?
            .chars()
            .enumerate()
        {
            hand.cards[i] = Card::try_from(c)?;
        }

        hand.bet = components.next().context("invalid hand")?.parse()?;

        Ok(hand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    static EXAMPLE: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn example_input() {
        let reader = BufReader::new(EXAMPLE.as_bytes());
        let mut hands = Hand::parse(reader).unwrap();
        hands.sort();

        let winnings: u32 = hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| hand.value(idx + 1))
            .sum();

        assert_eq!(winnings, 6440);
    }
}
