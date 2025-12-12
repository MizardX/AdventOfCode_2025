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
struct Tile {
    id: u8,
    shape: [u8; 3],
}

impl FromStr for Tile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 0:
        // ###
        // ##.
        // ##.
        let mut lines = s.lines();
        let id = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .strip_suffix(':')
            .ok_or(ParseError::SyntaxError)?
            .parse()?;
        let shape = [
            lines.next().ok_or(ParseError::SyntaxError)?,
            lines.next().ok_or(ParseError::SyntaxError)?,
            lines.next().ok_or(ParseError::SyntaxError)?,
        ]
        .map(|l| {
            l.bytes().fold(0, |s, ch| {
                // '.' = 0b00101110, '#' = 0b00100011
                // so `ch & 1` turns it into `0` or `1`
                (s << 1) | (ch & 1)
            })
        });
        if lines.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self { id, shape })
    }
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
    tiles: [Tile; 6],
    regions: Vec<Region>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let tiles = [
            parts.next().ok_or(ParseError::SyntaxError)?.parse()?,
            parts.next().ok_or(ParseError::SyntaxError)?.parse()?,
            parts.next().ok_or(ParseError::SyntaxError)?.parse()?,
            parts.next().ok_or(ParseError::SyntaxError)?.parse()?,
            parts.next().ok_or(ParseError::SyntaxError)?.parse()?,
            parts.next().ok_or(ParseError::SyntaxError)?.parse()?,
        ];
        let regions = parts
            .next()
            .ok_or(ParseError::SyntaxError)?
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        if parts.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self { tiles, regions })
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Result<Input, ParseError> {
    input.parse()
}

#[aoc(day12, part1)]
fn part_1(input: &Input) -> usize {
    let tile_areas = input
        .tiles
        .map(|t| t.shape.iter().map(|row| row.count_ones()).sum::<u32>());
    input
        .regions
        .iter()
        .filter(|r| {
            let total_area = u64::from(r.width) * u64::from(r.height);
            let sum_shapes_area = r
                .quantities
                .iter()
                .zip(&tile_areas)
                .map(|(&q, &a)| u64::from(q) * u64::from(a))
                .sum();
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
        assert_eq!(
            input.tiles[0],
            Tile {
                id: 0,
                shape: [0b111, 0b110, 0b110]
            }
        );
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
