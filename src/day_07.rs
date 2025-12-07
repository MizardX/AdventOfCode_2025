use std::collections::{HashMap, VecDeque};

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

    let mut pending = VecDeque::new();
    pending.push_back(start);

    let mut timelines = HashMap::new();
    timelines.insert(start, 1);

    let mut num_timelines = 0;
    let mut num_splits = 0;

    while let Some(pos) = pending.pop_front() {
        let multitude = *timelines.get(&pos).unwrap();
        match grid[pos] {
            Tile::Empty | Tile::Start => {
                if pos.row + 2 < grid.height() {
                    let below = Pos::new(pos.row + 2, pos.col);
                    *timelines.entry(below).or_insert_with(|| {
                        pending.push_back(below);
                        0
                    }) += multitude;
                } else {
                    num_timelines += multitude;
                }
            }
            Tile::Splitter => {
                num_splits += 1;
                // Add the row below, as to not make the frontline jagged
                if pos.row + 2 < grid.height() {
                    if pos.col > 0 {
                        let left = Pos::new(pos.row + 2, pos.col - 1);
                        *timelines.entry(left).or_insert_with(|| {
                            pending.push_back(left);
                            0
                        }) += multitude;
                    }
                    if pos.col + 1 < grid.width() {
                        let right = Pos::new(pos.row + 2, pos.col + 1);
                        *timelines.entry(right).or_insert_with(|| {
                            pending.push_back(right);
                            0
                        }) += multitude;
                    }
                } else {
                    num_timelines += 2 * multitude;
                }
            }
        }
    }
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
