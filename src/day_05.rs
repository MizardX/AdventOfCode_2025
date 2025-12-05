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

#[derive(Debug, Clone)]
struct Input {
    fresh_ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fresh_ranges = Vec::new();
        let mut ingredients = Vec::new();
        let mut lines = s.lines();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (start, end) = line.split_once('-').ok_or(ParseError::SyntaxError)?;
            fresh_ranges.push((start.parse()?, end.parse()?));
        }
        fresh_ranges.sort_unstable();
        for line in lines {
            ingredients.push(line.parse()?);
        }
        ingredients.sort_unstable();
        Ok(Self {
            fresh_ranges,
            ingredients,
        })
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Result<Input, ParseError> {
    input.parse()
}

#[aoc(day5, part1)]
fn part_1(input: &Input) -> usize {
    let mut ranges_it = input.fresh_ranges.iter();
    let mut exhausted = false;

    let Some(&(mut fresh_start, mut fresh_end)) = ranges_it.next() else {
        return 0;
    };
    let mut count = 0;
    for &ingredient in &input.ingredients {
        while !exhausted && ingredient > fresh_end {
            if let Some(&(start, end)) = ranges_it.next() {
                if start > fresh_end + 1 {
                    fresh_start = start;
                }
                fresh_end = end.max(fresh_end);
                if start > ingredient {
                    break;
                }
            } else {
                exhausted = true;
            }
        }
        if (fresh_start..=fresh_end).contains(&ingredient) {
            count += 1;
        }
    }
    count
}

#[aoc(day5, part2)]
fn part_2(input: &Input) -> u64 {
    if input.fresh_ranges.is_empty() {
        return 0;
    }
    let mut count = 0;
    let (mut fresh_start, mut fresh_end) = input.fresh_ranges[0];
    for &(start, end) in &input.fresh_ranges[1..] {
        if start > fresh_end + 1 {
            count += fresh_end - fresh_start + 1;
            fresh_start = start;
        }
        fresh_end = fresh_end.max(end);
    }
    count + fresh_end - fresh_start + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
        3-5\n\
        10-14\n\
        16-20\n\
        12-18\n\
        \n\
        1\n\
        5\n\
        8\n\
        11\n\
        17\n\
        32\
    ";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE1).unwrap();
        let result = part_1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE1).unwrap();
        let result = part_2(&input);
        assert_eq!(result, 14);
    }
}
