use std::collections::HashMap;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NodeId {
    Svr,
    You,
    Dac,
    Fft,
    Out,
    Other(usize),
}

impl NodeId {
    const fn index(self) -> usize {
        match self {
            Self::Svr => 0,
            Self::You => 1,
            Self::Dac => 2,
            Self::Fft => 3,
            Self::Out => 4,
            Self::Other(ix) => ix,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    id: NodeId,
    neighbors: Vec<NodeId>,
}

impl Node {
    const fn new(id: NodeId) -> Self {
        Self {
            id,
            neighbors: Vec::new(),
        }
    }

    fn neighbors(&self) -> impl Iterator<Item = NodeId> {
        self.neighbors.iter().copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Graph {
    names: Vec<String>,
    nodes: Vec<Node>,
}

impl Graph {
    const fn len(&self) -> usize {
        self.nodes.len()
    }

    fn node(&self, id: NodeId) -> &Node {
        &self.nodes[id.index()]
    }
}

impl FromStr for Graph {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lookup = HashMap::new();
        let mut names = Vec::new();
        let mut nodes = Vec::new();
        for (name, id) in [
            ("svr", NodeId::Svr),
            ("you", NodeId::You),
            ("dac", NodeId::Dac),
            ("fft", NodeId::Fft),
            ("out", NodeId::Out),
        ] {
            names.push(name.to_string());
            lookup.insert(name, id);
            nodes.push(Node::new(id));
        }
        for line in s.lines() {
            let (source, dests) = line.split_once(": ").ok_or(ParseError::SyntaxError)?;
            let next_id = lookup.len();
            let source = *lookup.entry(source).or_insert_with(|| {
                let id = NodeId::Other(next_id);
                nodes.push(Node::new(id));
                names.push(source.to_string());
                id
            });
            for dest in dests.split(' ') {
                let next_id = lookup.len();
                let dest = *lookup.entry(dest).or_insert_with(|| {
                    let id = NodeId::Other(next_id);
                    nodes.push(Node::new(id));
                    names.push(dest.to_string());
                    id
                });
                nodes[source.index()].neighbors.push(dest);
            }
        }
        Ok(Self { names, nodes })
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Result<Graph, ParseError> {
    input.parse()
}

#[aoc(day11, part1)]
fn part_1(graph: &Graph) -> u64 {
    fn dfs(graph: &Graph, visited: &mut [bool], id: NodeId) -> u64 {
        if id == NodeId::Out {
            return 1;
        }
        let mut count = 0;
        for next in graph.node(id).neighbors() {
            if visited[next.index()] {
                continue;
            }
            visited[next.index()] = true;
            count += dfs(graph, visited, next);
            visited[next.index()] = false;
        }
        count
    }
    dfs(graph, &mut vec![false; graph.len()], NodeId::You)
}

#[aoc(day11, part2)]
fn part_2(graph: &Graph) -> u64 {
    // TODO:
    // Try topological sort
    // Count with dynamic programming
    // DONE:
    // Possible exploit: treat high degree nodes as gates
    let mut in_count = vec![0; graph.len()];
    for node in &graph.nodes {
        for &next in &node.neighbors {
            in_count[next.index()] += 1;
        }
    }
    let mut targets = Vec::new();
    for node in &graph.nodes {
        if matches!(
            node.id,
            NodeId::Svr | NodeId::Fft | NodeId::Dac | NodeId::Out
        ) || node.neighbors.len() > 5
            || in_count[node.id.index()] > 5
        {
            targets.push(node.id);
        }
    }
    let mut paths = HashMap::<NodeId, HashMap<NodeId, u64>>::new();
    let mut pending = Vec::new();
    for &trg in &targets {
        pending.push((trg, trg));
    }
    while let Some((origin, cur)) = pending.pop() {
        if cur != origin && targets.contains(&cur) {
            *paths.entry(origin).or_default().entry(cur).or_default() += 1;
            continue;
        }
        for next in graph.node(cur).neighbors() {
            pending.push((origin, next));
        }
    }
    dfs(&paths, false, false, NodeId::Svr)
}

fn dfs(paths: &HashMap<NodeId, HashMap<NodeId, u64>>, fft: bool, dac: bool, id: NodeId) -> u64 {
    if id == NodeId::Out {
        return u64::from(fft && dac);
    }
    let mut total = 0;
    for (&next, &count) in &paths[&id] {
        total += count
            * dfs(
                paths,
                fft || id == NodeId::Fft,
                dac || id == NodeId::Dac,
                next,
            );
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
        aaa: you hhh\n\
        you: bbb ccc\n\
        bbb: ddd eee\n\
        ccc: ddd eee fff\n\
        ddd: ggg\n\
        eee: out\n\
        fff: out\n\
        ggg: out\n\
        hhh: ccc fff iii\n\
        iii: out\
    ";

    #[test]
    fn test_part_1() {
        let graph = parse(EXAMPLE1).unwrap();
        let result = part_1(&graph);
        assert_eq!(result, 5);
    }

    const EXAMPLE2: &str = "\
        svr: aaa bbb\n\
        aaa: fft\n\
        fft: ccc\n\
        bbb: tty\n\
        tty: ccc\n\
        ccc: ddd eee\n\
        ddd: hub\n\
        hub: fff\n\
        eee: dac\n\
        dac: fff\n\
        fff: ggg hhh\n\
        ggg: out\n\
        hhh: out\
    ";

    #[test]
    fn test_part_2() {
        let graph = parse(EXAMPLE2).unwrap();
        let result = part_2(&graph);
        assert_eq!(result, 2);
    }
}
