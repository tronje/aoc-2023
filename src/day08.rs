use anyhow::anyhow;
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct NodeId(char, char, char);

impl NodeId {
    fn is_start(self) -> bool {
        self.2 == 'A'
    }

    fn is_end(self) -> bool {
        self.2 == 'Z'
    }
}

impl FromStr for NodeId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            Err(anyhow!("unexpected input!"))
        } else {
            let mut chars = s.chars();
            Ok(Self(
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            ))
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Node {
    left: NodeId,
    right: NodeId,
}

impl Node {
    fn next(self, instruction: Instruction) -> NodeId {
        match instruction {
            Instruction::Left => self.left,
            Instruction::Right => self.right,
        }
    }

    fn parse(s: &str) -> anyhow::Result<(NodeId, Self)> {
        // input should be of the form "ABC = (DEF, GHI)"

        let node_id = s[..3].parse()?;
        let left = s[7..10].parse()?;
        let right = s[12..15].parse()?;

        Ok((node_id, Node { left, right }))
    }
}

#[derive(Debug)]
pub struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<NodeId, Node>,
}

impl Map {
    pub fn solve_p1(&self) -> u32 {
        let mut current = NodeId::from_str("AAA").unwrap();
        let goal = NodeId::from_str("ZZZ").unwrap();

        let mut steps = 0;

        for instruction in self.instructions.iter().cycle() {
            current = self.nodes[&current].next(*instruction);
            steps += 1;

            if current == goal {
                return steps;
            }
        }

        unreachable!()
    }

    /// Calculate the offset of the first end-candidate node in a cycle.
    ///
    /// There must be a cycle in any path "through" the node graph; the problem defines an infinite
    /// instruction sequence, but the map is finite, so there must be a cycle at some point.
    ///
    /// This assumes that there is an end node (node ID ends with 'Z') somewhere on this cycle for
    /// the root node. Given the puzzle input, this assumption turned out to be true. If this
    /// assumption is false for some input, this function will loop endlessly.
    ///
    /// This also assumes that the entire path is part of the cycle.
    ///
    /// This could be made generic (removing the need for the assumptions to be true) by returning
    /// the offset along the path until the cycle starts, and the offsets from cycle-start for all
    /// end nodes. But that would make this whole program a good amount more complicated. The way
    /// this is is sufficient for solving the puzzle.
    fn candidate_offset(&self, root: NodeId) -> u32 {
        let mut current = root;
        let mut found = HashMap::new();

        let mut steps = 0;

        for instruction in self.instructions.iter().cycle() {
            current = self.nodes[&current].next(*instruction);
            steps += 1;

            if current.is_end() {
                if let Some(offset) = found.get(&current).copied() {
                    return offset;
                    // HACK: assume only one possible candidate
                    // found.retain(|_, node_offset| *node_offset >= offset);
                    // return found;
                } else {
                    found.insert(current, steps);
                }
            }
        }

        unreachable!()
    }

    pub fn solve_p2(&self) -> u64 {
        // For each start node, calculate the offset until an end-node is found.
        // Then, calculate the lowest common multiple of all these offsets. The assumption here is
        // that all paths are cycles; thus the solution is the LCM of these offsets.
        // This assumption was correct for my puzzle input, but is not correct for all conceivable
        // inputs. Since the instruction sequence is infinite, but the map is finite, all paths
        // through the resulting graph must end in a cycle, but it is possible that this cycle
        // contains more than one end-node, or that the end-node of the path occurs before the
        // cycle begins. Accounting for this would make the solution more complex, though, and I
        // can't be bothered right now.

        let mut lcm = 1;

        for offset in self
            .nodes
            .keys()
            .filter(|n| n.is_start())
            .map(|n| self.candidate_offset(*n))
            .map(|n| n as u64)
        {
            lcm = num::integer::lcm(offset, lcm);
        }

        lcm
    }

    pub fn parse<R>(reader: R) -> anyhow::Result<Self>
    where
        R: BufRead,
    {
        let mut instructions = Vec::new();
        let mut nodes = HashMap::new();

        for line in reader.lines() {
            let line = line?;

            if line.is_empty() {
                continue;
            }

            if instructions.is_empty() {
                for c in line.chars() {
                    match c {
                        'L' => instructions.push(Instruction::Left),
                        'R' => instructions.push(Instruction::Right),
                        _ => return Err(anyhow!("unexpected input")),
                    }
                }

                continue;
            }

            let (id, node) = Node::parse(&line)?;
            nodes.insert(id, node);
        }

        Ok(Self {
            instructions,
            nodes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    static EXAMPLE_A: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    static EXAMPLE_B: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    static EXAMPLE_P2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn example_input_part1() {
        let map = Map::parse(BufReader::new(EXAMPLE_A.as_bytes())).unwrap();
        assert_eq!(map.solve_p1(), 2);

        let map = Map::parse(BufReader::new(EXAMPLE_B.as_bytes())).unwrap();
        assert_eq!(map.solve_p1(), 6);
    }

    #[test]
    fn example_input_part2() {
        let map = Map::parse(BufReader::new(EXAMPLE_P2.as_bytes())).unwrap();
        assert_eq!(map.solve_p2(), 6);
    }
}
