use anyhow::Result;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::prelude::*;
use std::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FixedAddress {
    x: u32,
    y: u32,
}

impl FixedAddress {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address {
    x: Range<u32>,
    y: u32,
}

impl Address {
    fn new(x_start: u32, x_end: u32, y: u32) -> Self {
        Self {
            x: Range {
                start: x_start,
                end: x_end,
            },
            y,
        }
    }

    fn is_adjacent_to(&self, other: &FixedAddress) -> bool {
        if self.y.abs_diff(other.y) > 1 {
            return false;
        }

        if self.x.contains(&other.x) {
            return true;
        }

        if self.x.start > other.x && (self.x.start - other.x) <= 1 {
            return true;
        }

        if self.x.end <= other.x && (other.x - self.x.end) == 0 {
            return true;
        }

        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Number {
    pub num: u32,
    addr: Address,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    sym: char,
    addr: FixedAddress,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Gear {
    symbol: Symbol,
    pub ratio: u32,
}

#[derive(Debug)]
pub struct EngineSchematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl EngineSchematic {
    pub fn load<R>(reader: R) -> Result<Self>
    where
        R: BufRead,
    {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        let mut in_number = false;
        let mut num_x_start = 0;

        let mut y = 0;

        let mut buf = String::new();

        for line in reader.lines() {
            let line = line?;

            let mut x = 0;

            for c in line.chars() {
                if c.is_ascii_digit() {
                    buf.push(c);

                    if !in_number {
                        in_number = true;
                        num_x_start = x;
                    }

                    x += 1;
                    continue;
                }

                if in_number {
                    let num = buf.parse::<u32>()?;
                    let addr = Address::new(num_x_start, x, y);
                    numbers.push(Number { num, addr });

                    in_number = false;
                    buf.clear();
                }

                if c == '.' {
                    x += 1;
                    continue;
                }

                // found a symbol
                let addr = FixedAddress::new(x, y);
                symbols.push(Symbol { sym: c, addr });

                x += 1;
            }

            y += 1;
        }

        Ok(Self { numbers, symbols })
    }

    pub fn part_numbers(&self) -> HashSet<Number> {
        let mut part_nos = HashSet::new();

        for symbol in self.symbols.iter() {
            for number in self.numbers.iter() {
                if number.addr.is_adjacent_to(&symbol.addr) {
                    part_nos.insert(number.clone());
                }
            }
        }

        part_nos
    }

    pub fn gears(&self) -> Vec<Gear> {
        let mut gears = Vec::new();

        'outer: for symbol in self.symbols.iter() {
            if symbol.sym != '*' {
                continue;
            }

            let mut num_a = None;
            let mut num_b = None;

            for number in self.numbers.iter() {
                if number.addr.is_adjacent_to(&symbol.addr) {
                    if num_a.is_none() {
                        num_a.replace(number.num);
                    } else if num_b.is_none() {
                        num_b.replace(number.num);
                    } else {
                        // more than two adjacent numbers means it's not a valid gear
                        continue 'outer;
                    }
                }
            }

            if let (Some(a), Some(b)) = (num_a, num_b) {
                gears.push(Gear {
                    symbol: *symbol,
                    ratio: a * b,
                });
            }
        }

        gears
    }
}
