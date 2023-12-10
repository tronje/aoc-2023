use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    fn inverse(self) -> Self {
        use Direction::*;

        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pipe(Direction, Direction);

impl Pipe {
    /// See of `self` connects to `other`, if other is located in `direction` relative to `self`.
    fn connects_to(self, other: Self, direction: Direction) -> bool {
        if (self.0 != direction) && (self.1 != direction) {
            return false;
        }

        if (other.0 != direction.inverse()) && (other.1 != direction.inverse()) {
            return false;
        }

        true
    }
}

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            '|' => Ok(Self(Direction::North, Direction::South)),
            '-' => Ok(Self(Direction::East, Direction::West)),
            'L' => Ok(Self(Direction::North, Direction::East)),
            'J' => Ok(Self(Direction::North, Direction::West)),
            'F' => Ok(Self(Direction::South, Direction::East)),
            '7' => Ok(Self(Direction::South, Direction::West)),
            _ => Err(anyhow!("not a pipe: {c}")),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Node {
    Start,
    Ground,
    Pipe(Pipe),
}

impl Node {
    fn connects_to(self, other: Self, direction: Direction) -> bool {
        match (self, other) {
            (Self::Start, Self::Pipe(p)) => {
                (p.0 == direction.inverse()) || (p.1 == direction.inverse())
            }
            (Self::Pipe(p), Self::Start) => (p.0 == direction) || (p.1 == direction),
            (Self::Pipe(a), Self::Pipe(b)) => a.connects_to(b, direction),
            _ => false,
        }
    }
}

impl TryFrom<char> for Node {
    type Error = anyhow::Error;

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Ground),
            other => Ok(Self::Pipe(Pipe::try_from(other)?)),
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    width: usize,
    height: usize,
    nodes: Vec<Node>,
}

impl Graph {
    fn get(&self, x: usize, y: usize) -> Option<Node> {
        let idx = x + y * self.width;
        self.nodes.get(idx).copied()
    }

    fn get_unchecked(&self, x: usize, y: usize) -> Node {
        self.get(x, y).unwrap()
    }

    fn get_neighbor(
        &self,
        x: usize,
        y: usize,
        direction: Direction,
    ) -> Option<(usize, usize, Node)> {
        let (x, y) = match direction {
            Direction::North if y > 0 => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West if x > 0 => (x - 1, y),

            _ => return None,
        };

        self.get(x, y).map(|n| (x, y, n))
    }

    fn find_start(&self) -> (usize, usize) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get_unchecked(x, y) == Node::Start {
                    return (x, y);
                }
            }
        }

        panic!("no start node");
    }

    pub fn solve_p1(&self) -> usize {
        let mut steps = 0;
        let (mut x, mut y) = self.find_start();

        let mut current = self.get_unchecked(x, y);
        let mut last_dir = None;

        'outer: loop {
            'inner: for dir in Direction::ALL {
                if Some(dir.inverse()) == last_dir {
                    continue 'inner;
                }

                match self.get_neighbor(x, y, dir) {
                    Some((new_x, new_y, neighbor)) => {
                        if current.connects_to(neighbor, dir) {
                            x = new_x;
                            y = new_y;

                            current = neighbor;
                            last_dir = Some(dir);

                            steps += 1;

                            if current == Node::Start {
                                break 'outer;
                            } else {
                                continue 'outer;
                            }
                        }
                    }

                    None => continue 'inner,
                }
            }

            panic!("no neighbor")
        }

        steps / 2
    }

    pub fn parse<R>(reader: R) -> Result<Self>
    where
        R: BufRead,
    {
        let mut graph = Graph {
            width: 0,
            height: 0,
            nodes: Vec::new(),
        };

        for line in reader.lines() {
            let line = line?;

            if graph.width == 0 {
                graph.width = line.len();
            }

            for c in line.chars() {
                let node = Node::try_from(c)?;
                graph.nodes.push(node);
            }

            graph.height += 1;
        }

        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    static EXAMPLE_SIMPLE: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    static EXAMPLE_COMPLEX: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[test]
    fn get_node() {
        use Direction::*;

        let reader = BufReader::new(EXAMPLE_SIMPLE.as_bytes());
        let graph = Graph::parse(reader).unwrap();

        assert_eq!(graph.get(0, 0), Some(Node::Pipe(Pipe(East, West))));
        assert_eq!(graph.get(1, 1), Some(Node::Start));
        assert_eq!(graph.get(2, 2), Some(Node::Pipe(Pipe(South, West))));
        assert_eq!(graph.get(3, 3), Some(Node::Pipe(Pipe(North, West))));
        assert_eq!(graph.get(4, 4), Some(Node::Pipe(Pipe(South, East))));
    }

    #[test]
    fn example_input_p1() {
        let reader = BufReader::new(EXAMPLE_SIMPLE.as_bytes());
        let graph = Graph::parse(reader).unwrap();

        assert_eq!(graph.solve_p1(), 4);

        let reader = BufReader::new(EXAMPLE_COMPLEX.as_bytes());
        let graph = Graph::parse(reader).unwrap();

        assert_eq!(graph.solve_p1(), 8);
    }
}
