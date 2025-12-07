use thiserror::Error;

use crate::shared::{Grid, Pos};

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid tile")]
    InvalidTile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Splitter,
    Start,
}

impl TryFrom<u8> for Tile {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Self::Empty,
            b'^' => Self::Splitter,
            b'S' => Self::Start,
            _ => return Err(ParseError::InvalidTile),
        })
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Grid<Tile>, ParseError> {
    input.parse()
}

#[aoc(day7, part1)]
fn part_1(grid: &Grid<Tile>) -> u64 {
    simulate(grid).0
}

#[aoc(day7, part2)]
fn part_2(grid: &Grid<Tile>) -> u64 {
    simulate(grid).1
}

fn simulate(grid: &Grid<Tile>) -> (u64, u64) {
    let start = grid
        .all_positions()
        .take(grid.width()) // first row
        .find(|&pos| grid[pos] == Tile::Start)
        .expect("'S' in first row");

    let mut pending = vec![0; grid.width()];
    pending[start.col] = 1;

    let mut next = vec![0; grid.width()];

    let mut num_splits = 0;

    for row in (0..grid.width()).step_by(2) {
        for (col, &multitude) in pending.iter().enumerate() {
            if multitude == 0 {
                continue;
            }
            let pos = Pos::new(row, col);
            match grid[pos] {
                Tile::Empty | Tile::Start => {
                    next[pos.col] += multitude;
                }
                Tile::Splitter => {
                    num_splits += 1;
                    next[pos.col - 1] += multitude;
                    next[pos.col + 1] += multitude;
                }
            }
        }
        (pending, next) = (next, pending);
        next.fill(0);
    }
    let num_timelines = pending.into_iter().sum();
    (num_splits, num_timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
        .......S.......\n\
        ...............\n\
        .......^.......\n\
        ...............\n\
        ......^.^......\n\
        ...............\n\
        .....^.^.^.....\n\
        ...............\n\
        ....^.^...^....\n\
        ...............\n\
        ...^.^...^.^...\n\
        ...............\n\
        ..^...^.....^..\n\
        ...............\n\
        .^.^.^.^.^...^.\n\
        ...............\
    ";

    #[test]
    fn test_part_1() {
        let grid = parse(EXAMPLE1).unwrap();
        let result = part_1(&grid);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_2() {
        let grid = parse(EXAMPLE1).unwrap();
        let result = part_2(&grid);
        assert_eq!(result, 40);
    }
}
