use anyhow::{anyhow, Context, Result};
use std::io::BufRead;

#[derive(Debug)]
struct Range {
    dst: u64,
    src: u64,
    len: u64,
}

impl Range {
    fn contains(&self, src: u64) -> bool {
        src >= self.src && src < (self.src + self.len)
    }

    fn convert(&self, src: u64) -> u64 {
        debug_assert!(self.contains(src));

        let offset = src - self.src;
        self.dst + offset
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

    fn convert(&self, n: u64) -> u64 {
        for range in self.ranges.iter() {
            if range.contains(n) {
                return range.convert(n);
            }
        }

        n
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<ConversionMap>,
}

impl Almanac {
    fn convert_down(mut seed: u64, maps: &[ConversionMap]) -> u64 {
        for map in maps {
            seed = map.convert(seed);
        }

        seed
    }

    pub fn min_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| Self::convert_down(*seed, &self.maps))
            .min()
            .unwrap()
    }

    pub fn parse<R>(reader: R) -> Result<Self>
    where
        R: BufRead,
    {
        let mut seeds = Vec::new();
        let mut maps = Vec::new();

        let mut current_map = None;

        for line in reader.lines() {
            let line = line?;

            if line.is_empty() || line.chars().all(char::is_whitespace) {
                continue;
            }

            if line.starts_with("seeds:") {
                for seed in line.split(' ').skip(1).map(str::parse::<u64>) {
                    seeds.push(seed?);
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

        Ok(Self { seeds, maps })
    }
}
