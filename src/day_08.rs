use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    #[allow(unused, reason = "tests")]
    const fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    const fn dist_sq(self, other: Self) -> u64 {
        let dx = self.x.abs_diff(other.x) as u64;
        let dy = self.y.abs_diff(other.y) as u64;
        let dz = self.z.abs_diff(other.z) as u64;
        dx * dx + dy * dy + dz * dz
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().ok_or(ParseError::SyntaxError)?.parse()?;
        let y = parts.next().ok_or(ParseError::SyntaxError)?.parse()?;
        let z = parts.next().ok_or(ParseError::SyntaxError)?.parse()?;
        if parts.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self { x, y, z })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self { x, y, z } = self;
        write!(f, "{x},{y},{z}")
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Vec<Point>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day8, part1)]
fn part_1(points: &[Point]) -> u64 {
    groups_after_connecting(points, 1000)
}

#[aoc(day8, part2)]
fn part_2(points: &[Point]) -> u64 {
    last_connection(points)
}

fn groups_after_connecting(points: &[Point], connections: usize) -> u64 {
    let mut pairs = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points[..i].iter().enumerate() {
            let dist_sq = p1.dist_sq(*p2);
            pairs.push((dist_sq, j, i));
        }
    }
    let (small, _, _) = pairs.select_nth_unstable(connections);
    let mut uf = UnionFind::new(points.len());
    for &(_, i, j) in small.iter() {
        uf.union(i, j);
    }
    let mut sizes = uf.roots().map(|(_, s)| s).collect::<Vec<_>>();
    sizes.sort_unstable();
    let mut prod = 1;
    for &size in sizes.iter().rev().take(3) {
        prod *= u64::try_from(size).unwrap();
    }
    prod
}

fn last_connection(points: &[Point]) -> u64 {
    let mut pairs = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points[..i].iter().enumerate() {
            let dist_sq = p1.dist_sq(*p2);
            pairs.push((Reverse(dist_sq), j, i));
        }
    }
    let mut heap = BinaryHeap::<_>::from(pairs);
    let mut uf = UnionFind::new(points.len());
    let mut last_union = None;
    while uf.num_roots() > 1
        && let Some((_, i, j)) = heap.pop()
    {
        if uf.union(i, j) {
            last_union = Some((i, j));
        }
    }
    let (i, j) = last_union.expect("At least one union");
    u64::from(points[i].x) * u64::from(points[j].x)
}

#[derive(Debug, Clone, Copy)]
struct UFNode {
    parent: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct UnionFind {
    nodes: Vec<UFNode>,
    num_roots: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let nodes = (0..size).map(|parent| UFNode { parent, size: 1 }).collect();
        Self {
            nodes,
            num_roots: size,
        }
    }

    fn find(&mut self, mut index: usize) -> usize {
        let mut parent = self.nodes[index].parent;
        while index != parent {
            let grand_parent = self.nodes[parent].parent;
            self.nodes[index].parent = grand_parent;
            index = grand_parent;
            parent = self.nodes[index].parent;
        }
        index
    }

    fn union(&mut self, mut index1: usize, mut index2: usize) -> bool {
        index1 = self.find(index1);
        index2 = self.find(index2);
        if index1 == index2 {
            return false;
        }
        if self.nodes[index1].size < self.nodes[index2].size {
            (index1, index2) = (index2, index1);
        }
        self.nodes[index2].parent = index1;
        self.nodes[index1].size += self.nodes[index2].size;
        self.num_roots -= 1;
        true
    }

    fn roots(&self) -> impl Iterator<Item = (usize, usize)> {
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(ix, n)| (n.parent == ix).then_some((ix, n.size)))
    }

    const fn num_roots(&self) -> usize {
        self.num_roots
    }
}

impl Display for UnionFind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sizes = self.roots().map(|(_, s)| s).collect::<Vec<_>>();
        sizes.sort_unstable();
        let mut fmt_list = f.debug_list();
        let mut last = 0;
        let mut count = 0;
        for &size in sizes.iter().rev() {
            if size == last {
                count += 1;
            } else {
                if count > 0 {
                    fmt_list.entry(&(last, count));
                }
                last = size;
                count = 1;
            }
        }
        if count > 0 {
            fmt_list.entry(&(last, count));
        }
        fmt_list.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXMAPLE: &str = "\
        162,817,812\n\
        57,618,57\n\
        906,360,560\n\
        592,479,940\n\
        352,342,300\n\
        466,668,158\n\
        542,29,236\n\
        431,825,988\n\
        739,650,466\n\
        52,470,668\n\
        216,146,977\n\
        819,987,18\n\
        117,168,530\n\
        805,96,715\n\
        346,949,466\n\
        970,615,88\n\
        941,993,340\n\
        862,61,35\n\
        984,92,344\n\
        425,690,689\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXMAPLE).unwrap();
        assert_eq!(
            result,
            [
                Point::new(162, 817, 812),
                Point::new(57, 618, 57),
                Point::new(906, 360, 560),
                Point::new(592, 479, 940),
                Point::new(352, 342, 300),
                Point::new(466, 668, 158),
                Point::new(542, 29, 236),
                Point::new(431, 825, 988),
                Point::new(739, 650, 466),
                Point::new(52, 470, 668),
                Point::new(216, 146, 977),
                Point::new(819, 987, 18),
                Point::new(117, 168, 530),
                Point::new(805, 96, 715),
                Point::new(346, 949, 466),
                Point::new(970, 615, 88),
                Point::new(941, 993, 340),
                Point::new(862, 61, 35),
                Point::new(984, 92, 344),
                Point::new(425, 690, 689),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let points = parse(EXMAPLE).unwrap();
        let result = groups_after_connecting(&points, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_2() {
        let points = parse(EXMAPLE).unwrap();
        let result = last_connection(&points);
        assert_eq!(result, 25272);
    }
}
