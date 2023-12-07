use anyhow::{anyhow, Context};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct Card<const JOKERS: bool>(u32);

impl<const JOKERS: bool> Card<JOKERS> {
    fn as_idx(self) -> usize {
        (self.0 - 2) as usize
    }

    fn as_int(self) -> u32 {
        if JOKERS {
            match self.0 {
                11 => 1,
                n => n,
            }
        } else {
            self.0
        }
    }
}

impl<const JOKERS: bool> PartialOrd for Card<JOKERS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const JOKERS: bool> Ord for Card<JOKERS> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_int().cmp(&other.as_int())
    }
}

impl<const JOKERS: bool> TryFrom<char> for Card<JOKERS> {
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

#[derive(Debug, Default)]
pub struct Hand<const JOKERS: bool> {
    cards: [Card<JOKERS>; 5],
    bet: u32,
}

impl<const JOKERS: bool> Hand<JOKERS> {
    /// Hand type is nicely expressed by a tuple of the two most significant numbers of the card
    /// count.
    fn hand_type(&self) -> (u32, u32) {
        let mut card_counts = [0; 13];

        for card in self.cards.iter() {
            card_counts[card.as_idx()] += 1;
        }

        let jokers = if JOKERS {
            std::mem::replace(&mut card_counts[Card::<JOKERS>(11).as_idx()], 0)
        } else {
            0
        };

        let mut most = 0;
        let mut second_most = 0;

        for &count in card_counts.iter() {
            if count > most {
                second_most = most;
                most = count;
            } else if count > second_most {
                second_most = count;
            }
        }

        let rem = (most + jokers) % 5;
        ((most + jokers).min(5), second_most + rem)
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

        hands.sort();
        Ok(hands)
    }
}

impl<const JOKERS: bool> PartialEq for Hand<JOKERS> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl<const JOKERS: bool> Eq for Hand<JOKERS> {}

impl<const JOKERS: bool> PartialOrd for Hand<JOKERS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const JOKERS: bool> Ord for Hand<JOKERS> {
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

impl<const JOKERS: bool> FromStr for Hand<JOKERS> {
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
    fn example_input_part1() {
        let reader = BufReader::new(EXAMPLE.as_bytes());
        let hands = Hand::<false>::parse(reader).unwrap();

        let winnings: u32 = hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| hand.value(idx + 1))
            .sum();

        assert_eq!(winnings, 6440);
    }

    #[test]
    fn example_input_part2() {
        let reader = BufReader::new(EXAMPLE.as_bytes());
        let hands = Hand::<true>::parse(reader).unwrap();

        let winnings: u32 = hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| hand.value(idx + 1))
            .sum();

        assert_eq!(winnings, 5905);
    }
}
