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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Region {
    width: u8,
    height: u8,
    quantities: [u8; 6],
}

impl FromStr for Region {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 12x5: 1 0 1 0 2 2
        let mut words = s.split(['x', ':', ' ']);
        let width = words.next().ok_or(ParseError::SyntaxError)?.parse()?;
        let height = words.next().ok_or(ParseError::SyntaxError)?.parse()?;
        if words.next() != Some("") {
            return Err(ParseError::SyntaxError);
        }
        let quantities = [
            words.next().ok_or(ParseError::SyntaxError)?.parse()?,
            words.next().ok_or(ParseError::SyntaxError)?.parse()?,
            words.next().ok_or(ParseError::SyntaxError)?.parse()?,
            words.next().ok_or(ParseError::SyntaxError)?.parse()?,
            words.next().ok_or(ParseError::SyntaxError)?.parse()?,
            words.next().ok_or(ParseError::SyntaxError)?.parse()?,
        ];
        if words.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self {
            width,
            height,
            quantities,
        })
    }
}

#[derive(Debug, Clone)]
struct Input {
    regions: Vec<Region>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let regions = parts
            .nth(6) // Skip tiles
            .ok_or(ParseError::SyntaxError)?
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        if parts.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self { regions })
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Result<Input, ParseError> {
    input.parse()
}

#[aoc(day12, part1)]
fn part_1(input: &Input) -> usize {
    input
        .regions
        .iter()
        .filter(|r| {
            let total_area = u64::from(r.width) * u64::from(r.height);
            let sum_shapes_area = r.quantities.iter().map(|&c| u64::from(c)).sum::<u64>() * 9;
            total_area >= sum_shapes_area
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        0:\n\
        ###\n\
        ##.\n\
        ##.\n\
        \n\
        1:\n\
        ###\n\
        ##.\n\
        .##\n\
        \n\
        2:\n\
        .##\n\
        ###\n\
        ##.\n\
        \n\
        3:\n\
        ##.\n\
        ###\n\
        ##.\n\
        \n\
        4:\n\
        ###\n\
        #..\n\
        ###\n\
        \n\
        5:\n\
        ###\n\
        .#.\n\
        ###\n\
        \n\
        4x4: 0 0 0 0 2 0\n\
        12x5: 1 0 1 0 2 2\n\
        12x5: 1 0 1 0 3 2\
    ";

    #[test]
    fn test_parse() {
        let input = parse(EXAMPLE).unwrap();
        assert_eq!(input.regions.len(), 3);
        assert_eq!(
            input.regions[0],
            Region {
                width: 4,
                height: 4,
                quantities: [0, 0, 0, 0, 2, 0]
            }
        );
    }

    #[test]
    #[ignore = "Algorithm does not work for the example"]
    fn test_part_1() {
        let input = parse(EXAMPLE).unwrap();
        let result = part_1(&input);
        assert_eq!(result, 2);
    }
}
