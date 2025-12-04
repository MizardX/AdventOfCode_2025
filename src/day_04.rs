use std::collections::{HashSet, VecDeque};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        (pos.row.saturating_sub(1)..self.height.min(pos.row + 2))
            .flat_map(move |r1| {
                (pos.col.saturating_sub(1)..self.width.min(pos.col + 2))
                    .map(move |c1| Pos::new(r1, c1))
            })
            .filter(move |&neighbor| neighbor != pos)
    }

    const fn convert_index(&self, pos: Pos) -> Option<usize> {
        if pos.row < self.height && pos.col < self.width {
            Some(pos.row * self.width + pos.col)
        } else {
            None
        }
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Pos) -> &Self::Output {
        let index = self.convert_index(pos).expect("Index out of range");
        &self.data[index]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        let index = self.convert_index(pos).expect("Index out of range");
        &mut self.data[index]
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid tile")]
    InvalidTile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Roll,
}

impl TryFrom<u8> for Tile {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Self::Empty,
            b'@' => Self::Roll,
            _ => return Err(ParseError::InvalidTile),
        })
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<u8>,
{
    type Err = T::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let height = lines.clone().count();
        let width = lines.clone().next().unwrap_or_default().len();
        let mut data = Vec::with_capacity(width * height);
        for line in lines {
            for ch in line.bytes() {
                data.push(ch.try_into()?);
            }
        }
        Ok(Self {
            data,
            width,
            height,
        })
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Result<Grid<Tile>, ParseError> {
    input.parse()
}

#[aoc(day4, part1)]
fn part_1(grid: &Grid<Tile>) -> usize {
    let mut count = 0;
    for r in 0..grid.height {
        for c in 0..grid.width {
            let pos = Pos::new(r, c);
            if grid[pos] != Tile::Roll {
                continue;
            }
            let neighbors = grid
                .neighbors(pos)
                .filter(|&n| grid[n] == Tile::Roll)
                .count();
            count += usize::from(neighbors < 4);
        }
    }
    count
}

#[aoc(day4, part2)]
fn part_2(grid: &Grid<Tile>) -> usize {
    let mut removed = HashSet::<Pos>::new();
    let mut pending = VecDeque::<Pos>::new();
    for r in 0..grid.height {
        for c in 0..grid.width {
            let pos = Pos::new(r, c);
            if grid[pos] != Tile::Roll {
                continue;
            }
            let neighbors = grid
                .neighbors(pos)
                .filter(|&n| grid[n] == Tile::Roll)
                .count();
            if neighbors < 4 && removed.insert(pos) {
                pending.push_back(pos);
            }
        }
    }
    while let Some(pos) = pending.pop_front() {
        for next in grid.neighbors(pos) {
            if grid[next] != Tile::Roll {
                continue;
            }
            let neighbors = grid
                .neighbors(next)
                .filter(|&n| grid[n] == Tile::Roll && !removed.contains(&n))
                .count();
            if neighbors < 4 && removed.insert(next) {
                pending.push_back(next);
            }
        }
    }
    removed.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMLE1: &str = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.\
    ";

    #[test]
    fn test_part_1() {
        let grid = parse(EXAMLE1).unwrap();
        let res = part_1(&grid);
        assert_eq!(res, 13);
    }

    #[test]
    fn test_part_2() {
        let grid = parse(EXAMLE1).unwrap();
        let res = part_2(&grid);
        assert_eq!(res, 43);
    }
}
