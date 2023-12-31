use anyhow::{anyhow, Context, Result};
use std::io::BufRead;

#[derive(Debug)]
struct Range {
    dst: u64,
    src: u64,
    len: u64,
}

impl Range {
    fn contains_src(&self, src: u64) -> bool {
        src >= self.src && src < (self.src + self.len)
    }

    fn contains_dst(&self, dst: u64) -> bool {
        dst >= self.dst && dst < (self.dst + self.len)
    }

    fn convert_down(&self, src: u64) -> u64 {
        debug_assert!(self.contains_src(src));

        let offset = src - self.src;
        self.dst + offset
    }

    fn convert_up(&self, dst: u64) -> u64 {
        debug_assert!(self.contains_dst(dst));

        let offset = dst - self.dst;
        self.src + offset
    }
}

#[derive(Debug)]
struct ConversionMap {
    #[allow(unused)]
    from: String,
    #[allow(unused)]
    to: String,

    ranges: Vec<Range>,
}

impl ConversionMap {
    fn new<S1, S2>(from: S1, to: S2) -> Self
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self {
            from: from.as_ref().to_owned(),
            to: to.as_ref().to_owned(),
            ranges: Vec::new(),
        }
    }

    fn add_range(&mut self, dst: u64, src: u64, len: u64) {
        self.ranges.push(Range { dst, src, len });
    }

    fn convert_down(&self, n: u64) -> u64 {
        for range in self.ranges.iter() {
            if range.contains_src(n) {
                return range.convert_down(n);
            }
        }

        n
    }

    fn convert_up(&self, n: u64) -> u64 {
        for range in self.ranges.iter() {
            if range.contains_dst(n) {
                return range.convert_up(n);
            }
        }

        n
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct SeedRange {
    start: u64,
    len: u64,
}

impl SeedRange {
    fn new(start: u64, len: u64) -> Self {
        Self { start, len }
    }

    fn contains(&self, n: u64) -> bool {
        n >= self.start && n < (self.start + self.len)
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    seed_ranges: Vec<SeedRange>,
    maps: Vec<ConversionMap>,
}

impl Almanac {
    fn convert_down(mut seed: u64, maps: &[ConversionMap]) -> u64 {
        for map in maps {
            seed = map.convert_down(seed);
        }

        seed
    }

    fn convert_up(mut location: u64, maps: &[ConversionMap]) -> u64 {
        for map in maps.iter().rev() {
            location = map.convert_up(location);
        }

        location
    }

    /// Convert all seeds down to locations and return the minimum of those locations.
    pub fn part_one(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| Self::convert_down(*seed, &self.maps))
            .min()
            .unwrap()
    }

    /// Keep guessing locations, starting at 0, and convert backwards until a valid seed is found.
    ///
    /// The first location that maps to a valid seed must be the minimum.
    pub fn part_two(&mut self) -> u64 {
        for location in 0..u64::max_value() {
            let seed = Self::convert_up(location, &self.maps);

            if self.seed_ranges.iter().any(|sr| sr.contains(seed)) {
                return location;
            }
        }

        panic!()
    }

    pub fn parse<R>(reader: R) -> Result<Self>
    where
        R: BufRead,
    {
        let mut seeds = Vec::new();
        let mut seed_ranges = Vec::new();
        let mut maps = Vec::new();

        let mut current_map = None;

        for line in reader.lines() {
            let line = line?;

            if line.is_empty() || line.chars().all(char::is_whitespace) {
                continue;
            }

            if line.starts_with("seeds:") {
                // part one, single seeds
                for seed in line.split(' ').skip(1).map(str::parse::<u64>) {
                    seeds.push(seed?);
                }

                // part two, seed ranges
                let mut start = None;
                for n in line.split(' ').skip(1).map(str::parse::<u64>) {
                    match start.take() {
                        Some(start) => seed_ranges.push(SeedRange::new(start, n?)),
                        None => start = Some(n?),
                    }
                }

                continue;
            }

            if line.ends_with("map:") {
                let mut name = line.split(' ').next().context("invalid input")?.split('-');
                let from = name.next().context("invalid map name")?;
                let to = name.nth(1).context("invalid map name")?;

                let next_map = ConversionMap::new(from, to);

                if let Some(previous_map) = current_map.replace(next_map) {
                    maps.push(previous_map);
                }

                continue;
            }

            let mut values = line.split(' ');
            let dst = values.next().context("invalid input")?.parse::<u64>()?;
            let src = values.next().context("invalid input")?.parse::<u64>()?;
            let len = values.next().context("invalid input")?.parse::<u64>()?;

            match current_map.as_mut() {
                Some(map) => map.add_range(dst, src, len),
                None => return Err(anyhow!("invalid input")),
            };
        }

        match current_map.take() {
            Some(map) => maps.push(map),
            None => return Err(anyhow!("invalid input")),
        };

        Ok(Self {
            seeds,
            seed_ranges,
            maps,
        })
    }
}
