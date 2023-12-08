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
    pub fn solve(&self) -> u32 {
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

    #[test]
    fn example_input_part1() {
        let map = Map::parse(BufReader::new(EXAMPLE_A.as_bytes())).unwrap();
        assert_eq!(map.solve(), 2);

        let map = Map::parse(BufReader::new(EXAMPLE_B.as_bytes())).unwrap();
        assert_eq!(map.solve(), 6);
    }
}
