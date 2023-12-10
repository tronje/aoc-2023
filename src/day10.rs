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

    /// If you're walking in direction `self`, this gives the direction you'd go if you turned 90
    /// degrees to your left.
    fn left(self) -> Self {
        use Direction::*;

        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    /// If you're walking in direction `self`, this gives the direction you'd go if you turned 90
    /// degrees to your right.
    fn right(self) -> Self {
        use Direction::*;

        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pipe(Direction, Direction);

impl Pipe {
    /// See if `self` connects to `other`, if other is located in `direction` relative to `self`.
    fn connects_to(self, other: Self, direction: Direction) -> bool {
        if (self.0 != direction) && (self.1 != direction) {
            return false;
        }

        if (other.0 != direction.inverse()) && (other.1 != direction.inverse()) {
            return false;
        }

        true
    }

    fn is_corner(self) -> bool {
        self.0 != self.1.inverse()
    }

    fn is_right_turn(self, from: Direction) -> bool {
        debug_assert!(self.is_corner());

        if self.0 == from.inverse() {
            self.1 == from.right()
        } else if self.1 == from.inverse() {
            self.0 == from.right()
        } else {
            unreachable!()
        }
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

#[derive(Debug, Copy, Clone)]
enum BoundaryNode {
    Normal {
        left: Direction,
        right: Direction,
    },

    /// A corner can only be reached from a non-boundary tile by its two facing directions.
    ///
    /// Either both of these directions are on the outside of the boundary, or both are on the
    /// inside. So just save whether the reachable directions were left or right.
    Corner {
        left: bool,
    },
}

#[derive(Debug)]
struct LoopBoundary {
    width: usize,
    height: usize,
    nodes: Vec<Option<BoundaryNode>>,
    left_is_outside: bool,
}

impl LoopBoundary {
    fn construct(graph: &Graph) -> Self {
        let mut nodes = Vec::with_capacity(graph.width * graph.height);
        for _ in 0..(graph.width * graph.height) {
            nodes.push(None);
        }

        let loop_tiles = graph.loop_tiles();

        let start_pipe = Pipe(
            loop_tiles.iter().last().unwrap().2.inverse(),
            loop_tiles[0].2,
        );

        for (x, y, dir) in loop_tiles.iter() {
            let node = graph.get_unchecked(*x, *y);
            let pipe = match node {
                Node::Start => start_pipe,
                Node::Pipe(p) => p,
                Node::Ground => unreachable!(),
            };

            let idx = x + y * graph.width;

            if pipe.is_corner() {
                nodes[idx] = Some(BoundaryNode::Corner {
                    left: pipe.is_right_turn(*dir),
                });
            } else {
                let left = dir.left();
                let right = dir.right();
                nodes[idx] = Some(BoundaryNode::Normal { left, right });
            }
        }

        let mut this = Self {
            width: graph.width,
            height: graph.height,
            nodes,
            left_is_outside: false,
        };

        this.determine_outside();
        this
    }

    /// Returns true if walking from (x, y) toward `dir` hits the edge of the map without first
    /// hitting a boundary node.
    fn search_for_edge(&self, mut x: usize, mut y: usize, dir: Direction) -> bool {
        if (x > 0 && x < (self.width - 1)) && (y > 0 && y < (self.height - 1)) {
            match dir {
                Direction::North => y -= 1,
                Direction::East => x += 1,
                Direction::South => y += 1,
                Direction::West => x -= 1,
            };

            match self.get(x, y) {
                Some(_) => return false,
                None => return self.search_for_edge(x, y, dir),
            }
        }

        true
    }

    fn determine_outside(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let node = match self.get(x, y) {
                    Some(node) => node,
                    None => continue,
                };

                match node {
                    BoundaryNode::Normal { left, right } => {
                        if self.search_for_edge(x, y, *left) {
                            self.left_is_outside = true;
                            return;
                        } else if self.search_for_edge(x, y, *right) {
                            self.left_is_outside = false;
                            return;
                        }
                    }

                    // this means this won't work for boundaries comprised entirely out of
                    // corners... oh well
                    BoundaryNode::Corner { .. } => continue,
                }
            }
        }

        panic!("could not determine which side is outside")
    }

    fn get(&self, x: usize, y: usize) -> Option<&BoundaryNode> {
        let idx = x + y * self.width;
        self.nodes[idx].as_ref()
    }

    fn contains(&self, x: usize, mut y: usize) -> bool {
        if self.get(x, y).is_some() {
            // nodes making up the boundary itself do not count
            return false;
        }

        if x == 0 || x == (self.width - 1) {
            // at left or right edge, so can't be inside boundary
            return false;
        }

        if y == 0 || y == (self.height - 1) {
            // at top or bottom edge, so can't be inside boundary
            return false;
        }

        // Just go North from the given position.
        // Three things can happen now:
        //  1) we find a boundary, but its outside is facing us -> return false
        //  2) we find a boundary with its inside facing us -> return true
        //  3) we don't find a boundary, so we end up at the edge -> return false

        while y > 0 {
            y -= 1;

            if let Some(boundary_node) = self.get(x, y) {
                let outside = match boundary_node {
                    BoundaryNode::Normal { left, right } => {
                        if self.left_is_outside {
                            *left == Direction::South
                        } else {
                            *right == Direction::South
                        }
                    }

                    BoundaryNode::Corner { left } => {
                        if self.left_is_outside {
                            *left
                        } else {
                            !left
                        }
                    }
                };

                return !outside;
            }
        }

        false
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

    fn loop_tiles(&self) -> Vec<(usize, usize, Direction)> {
        let mut tiles = Vec::new();
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

                            tiles.push((x, y, dir));

                            current = neighbor;
                            last_dir = Some(dir);

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

        tiles
    }

    pub fn solve_p1(&self) -> usize {
        self.loop_tiles().len() / 2
    }

    pub fn solve_p2(&self) -> usize {
        let boundary = LoopBoundary::construct(self);
        let mut count = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                if boundary.contains(x, y) {
                    count += 1;
                }
            }
        }

        count
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
mod part1 {
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
    fn example() {
        let reader = BufReader::new(EXAMPLE_SIMPLE.as_bytes());
        let graph = Graph::parse(reader).unwrap();

        assert_eq!(graph.solve_p1(), 4);

        let reader = BufReader::new(EXAMPLE_COMPLEX.as_bytes());
        let graph = Graph::parse(reader).unwrap();

        assert_eq!(graph.solve_p1(), 8);
    }
}

#[cfg(test)]
mod part2 {
    use super::*;
    use std::io::BufReader;

    static EXAMPLE_A: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    static EXAMPLE_B: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    static EXAMPLE_C: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    static EXAMPLE_D: &str = ".S--7.
.|..|.
.L--J.
";

    fn mkgraph(s: &str) -> Graph {
        let reader = BufReader::new(s.as_bytes());
        Graph::parse(reader).unwrap()
    }

    #[test]
    fn example_a() {
        let g = mkgraph(EXAMPLE_A);
        assert_eq!(g.solve_p2(), 4);
    }

    #[test]
    fn example_b() {
        let g = mkgraph(EXAMPLE_B);
        assert_eq!(g.solve_p2(), 8);
    }

    #[test]
    fn example_c() {
        let g = mkgraph(EXAMPLE_C);
        assert_eq!(g.solve_p2(), 10);
    }

    #[test]
    fn example_d() {
        let g = mkgraph(EXAMPLE_D);
        assert_eq!(g.solve_p2(), 2);
    }
}
