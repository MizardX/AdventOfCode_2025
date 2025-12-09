use std::collections::BTreeSet;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::shared::{Grid, Pos, UnionFind};

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
}

impl Point {
    #[allow(unused, reason = "tests")]
    const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    const fn area(self, other: Self) -> u64 {
        let dx = self.x.abs_diff(other.x) as u64 + 1;
        let dy = self.y.abs_diff(other.y) as u64 + 1;
        dx * dy
    }

    // fn left_turn(self, p2: Self, p3: Self) -> bool {
    //     let dx1 = p2.x.cast_signed() - self.x.cast_signed();
    //     let dy1 = p2.y.cast_signed() - self.y.cast_signed();
    //     let dx2 = p3.x.cast_signed() - p2.x.cast_signed();
    //     let dy2 = p3.y.cast_signed() - p2.y.cast_signed();
    //     dx1 * dy2 - dy1 * dx2 > 0
    // }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().ok_or(ParseError::SyntaxError)?.parse()?;
        let y = parts.next().ok_or(ParseError::SyntaxError)?.parse()?;
        if parts.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self { x, y })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self { x, y } = self;
        write!(f, "{x},{y}")
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Vec<Point>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day9, part1)]
fn part_1(points: &[Point]) -> u64 {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| points[..i].iter().map(move |p2| (p1, p2)))
        .map(|(&p1, &p2)| p1.area(p2))
        .max()
        .unwrap()
}

#[aoc(day9, part2)]
#[expect(clippy::too_many_lines)]
fn part_2(points: &[Point]) -> u64 {
    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();
    xs.insert(0);
    ys.insert(0);
    for pt in points {
        xs.insert(pt.x);
        xs.insert(pt.x + 1);
        ys.insert(pt.y);
        ys.insert(pt.y + 1);
    }
    let xs = xs.into_iter().collect::<Vec<_>>();
    let ys = ys.into_iter().collect::<Vec<_>>();
    let mut grid = Grid::new(vec![0_u64; xs.len() * ys.len()], xs.len(), ys.len());
    for (&p1, &p2) in points.iter().zip(points.iter().cycle().skip(1)) {
        if p1.x == p2.x {
            let xi = xs.partition_point(|&x| x < p1.x);
            let yi1 = ys.partition_point(|&y| y < p1.y);
            let yi2 = ys.partition_point(|&y| y < p2.y);
            for yi in yi1.min(yi2)..=yi1.max(yi2) {
                grid[Pos::new(yi, xi)] = 1;
            }
        } else if p1.y == p2.y {
            let yi = ys.partition_point(|&y| y < p1.y);
            let xi1 = xs.partition_point(|&x| x < p1.x);
            let xi2 = xs.partition_point(|&x| x < p2.x);
            for xi in xi1.min(xi2)..=xi1.max(xi2) {
                grid[Pos::new(yi, xi)] = 1;
            }
        }
    }
    let mut uf = UnionFind::new(xs.len() * ys.len());
    for yi in 0..ys.len() {
        for xi in 0..xs.len() {
            let ix = yi * xs.len() + xi;
            let pos = Pos::new(yi, xi);
            let val = grid[pos];
            if yi > 0 {
                let pos1 = Pos::new(yi - 1, xi);
                let ix1 = (yi - 1) * xs.len() + xi;
                if grid[pos1] == val {
                    uf.union(ix1, ix);
                }
            }
            if xi > 0 {
                let pos1 = Pos::new(yi, xi - 1);
                let ix1 = yi * xs.len() + xi - 1;
                if grid[pos1] == val {
                    uf.union(ix1, ix);
                }
            }
            if val == 0 && (xi == 0 || xi == xs.len() - 1 || yi == 0 || yi == ys.len() - 1) {
                uf.union(0, ix);
            }
        }
    }
    uf.union(0, xs.len() - 1);
    uf.union(0, (ys.len() - 1) * xs.len());
    uf.union(0, ys.len() * xs.len() - 1);
    let outside_root = uf.find(0);
    let edge_root = uf
        .roots()
        .map(|(r, _)| r)
        .find(|&r| grid[Pos::new(r / xs.len(), r % xs.len())] == 1)
        .unwrap();
    let inside_root = uf
        .roots()
        .map(|(r, _)| r)
        .find(|&r| r != outside_root && r != edge_root)
        .unwrap();
    for yi in 0..ys.len() {
        for xi in 0..xs.len() {
            let index = yi * xs.len() + xi;
            if uf.find(index) == inside_root {
                let pos = Pos::new(yi, xi);
                grid[pos] = 1;
            }
        }
    }
    for (yi, (&y1, &y2)) in ys
        .iter()
        .zip(ys[1..].iter().chain([ys.last().unwrap()]))
        .enumerate()
    {
        for (xi, (&x1, &x2)) in xs
            .iter()
            .zip(xs[1..].iter().chain([xs.last().unwrap()]))
            .enumerate()
        {
            let pos = Pos::new(yi, xi);
            let mut sum = i64::from(grid[pos] > 0) * i64::from(y2 - y1) * i64::from(x2 - x1);
            if xi > 0 {
                sum += i64::try_from(grid[Pos::new(yi, xi - 1)]).unwrap();
                if yi > 0 {
                    sum += i64::try_from(grid[Pos::new(yi - 1, xi)]).unwrap();
                    sum -= i64::try_from(grid[Pos::new(yi - 1, xi - 1)]).unwrap();
                }
            } else if yi > 0 {
                sum += i64::try_from(grid[Pos::new(yi - 1, xi)]).unwrap();
            }
            grid[pos] = sum.try_into().expect("positive");
        }
    }
    let mut max_area = 0;
    for (i, &p1) in points.iter().enumerate() {
        let xi1 = xs.partition_point(|&x| x < p1.x);
        let yi1 = ys.partition_point(|&y| y < p1.y);
        for &p2 in &points[i + 1..] {
            let xi2 = xs.partition_point(|&x| x < p2.x);
            let yi2 = ys.partition_point(|&y| y < p2.y);
            let (xi1, xi2) = (xi1.min(xi2), xi1.max(xi2));
            let (yi1, yi2) = (yi1.min(yi2), yi1.max(yi2));
            let expected_area = p1.area(p2);
            let mut grid_sum = grid[Pos::new(yi2, xi2)];
            if xi1 > 0 {
                if yi2 > 0 {
                    grid_sum += grid[Pos::new(yi1 - 1, xi1 - 1)];
                    grid_sum -= grid[Pos::new(yi1 - 1, xi2)];
                }
                grid_sum -= grid[Pos::new(yi2, xi1 - 1)];
            } else if yi1 > 0 {
                grid_sum -= grid[Pos::new(yi1 - 1, xi2)];
            }
            if grid_sum == expected_area {
                max_area = max_area.max(expected_area);
            }
        }
    }
    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
        7,1\n\
        11,1\n\
        11,7\n\
        9,7\n\
        9,5\n\
        2,5\n\
        2,3\n\
        7,3\
    ";

    const EXAMPLE2: &str = "\
        10,30\n\
        30,30\n\
        30,10\n\
        50,10\n\
        50,30\n\
        70,30\n\
        70,10\n\
        90,10\n\
        90,30\n\
        110,30\n\
        110,50\n\
        90,50\n\
        90,70\n\
        110,70\n\
        110,90\n\
        90,90\n\
        90,110\n\
        70,110\n\
        70,90\n\
        50,90\n\
        50,110\n\
        30,110\n\
        30,90\n\
        10,90\n\
        10,70\n\
        30,70\n\
        30,50\n\
        10,50\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE1).unwrap();
        assert_eq!(
            result,
            [
                Point::new(7, 1),
                Point::new(11, 1),
                Point::new(11, 7),
                Point::new(9, 7),
                Point::new(9, 5),
                Point::new(2, 5),
                Point::new(2, 3),
                Point::new(7, 3),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let points = parse(EXAMPLE1).unwrap();
        let result = part_1(&points);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part_2() {
        let points = parse(EXAMPLE1).unwrap();
        let result = part_2(&points);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_part_2_b() {
        let points = parse(EXAMPLE2).unwrap();
        let result = part_2(&points);
        assert_eq!(result, 61 * 61);
    }
}
