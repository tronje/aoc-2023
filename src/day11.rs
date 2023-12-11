use anyhow::Result;
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn shortest_path(&self, other: &Self) -> usize {
        // Since we can't walk diagonally, the shortest path is just the sum of the distances in
        // each dimension.
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
pub struct Universe {
    galaxies: Vec<Galaxy>,
    width: usize,
    height: usize,
}

impl Universe {
    fn expand_row(&mut self, row: usize, n: usize) {
        self.galaxies
            .iter_mut()
            .filter(|g| g.y > row)
            .for_each(|g| g.y += n);
    }

    fn expand_column(&mut self, column: usize, n: usize) {
        self.galaxies
            .iter_mut()
            .filter(|g| g.x > column)
            .for_each(|g| g.x += n);
    }

    pub fn expand(&mut self, n: usize) {
        let mut empty_rows = Vec::new();
        let mut empty_columns = Vec::new();

        for row in 0..self.height {
            if !self.galaxies.iter().any(|g| g.y == row) {
                empty_rows.push(row);
            }
        }

        for column in 0..self.width {
            if !self.galaxies.iter().any(|g| g.x == column) {
                empty_columns.push(column);
            }
        }

        let mut count = 0;

        for row in empty_rows {
            self.expand_row(row + count, n);
            count += n;
        }

        count = 0;
        for column in empty_columns {
            self.expand_column(column + count, n);
            count += n;
        }
    }

    pub fn solve(&self) -> usize {
        let mut done_pairs = HashSet::new();
        let mut sum = 0;

        for a in self.galaxies.iter() {
            for b in self.galaxies.iter() {
                if a == b {
                    continue;
                }

                if done_pairs.contains(&(a, b)) || done_pairs.contains(&(b, a)) {
                    continue;
                }

                sum += a.shortest_path(b);
                done_pairs.insert((a, b));
            }
        }

        sum
    }

    pub fn parse<R>(reader: R) -> Result<Self>
    where
        R: BufRead,
    {
        let mut galaxies = Vec::new();
        let mut width = 0;

        let mut x = 0;
        let mut y = 0;

        for line in reader.lines() {
            let line = line?;

            if width == 0 {
                width = line.len();
            }

            for chr in line.chars() {
                if chr == '#' {
                    galaxies.push(Galaxy { x, y });
                }

                x += 1;
            }

            x = 0;
            y += 1;
        }

        Ok(Self {
            galaxies,
            width,
            height: y,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    static EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    static EXAMPLE_EXPANDED: &str = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
";

    fn mk_universe(s: &str) -> Universe {
        let reader = BufReader::new(s.as_bytes());
        Universe::parse(reader).unwrap()
    }

    #[test]
    fn test_expansion() {
        let mut universe = mk_universe(EXAMPLE);
        universe.expand(1);

        let expanded = mk_universe(EXAMPLE_EXPANDED);

        assert_eq!(universe.galaxies, expanded.galaxies);
    }

    #[test]
    fn example_input_p1() {
        let mut universe = mk_universe(EXAMPLE);
        universe.expand(1);

        assert_eq!(universe.solve(), 374);
    }

    #[test]
    fn example_input_p2() {
        let mut universe = mk_universe(EXAMPLE);
        universe.expand(9);
        assert_eq!(universe.solve(), 1030);

        let mut universe = mk_universe(EXAMPLE);
        universe.expand(99);
        assert_eq!(universe.solve(), 8410);
    }
}
