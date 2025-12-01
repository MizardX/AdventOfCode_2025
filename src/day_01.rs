use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<i16>, ParseError> {
    input
        .lines()
        .map(|w| {
            Ok(match w.as_bytes()[0] {
                b'L' => -w[1..].parse::<i16>()?,
                b'R' => w[1..].parse::<i16>()?,
                _ => return Err(ParseError::SyntaxError),
            })
        })
        .collect()
}

fn unlock(input: &[i16]) -> (u64, u64) {
    let mut position = 50;
    let mut zeros = 0;
    let mut clicks = 0;
    for &delta in input {
        clicks += u64::from(delta.unsigned_abs()) / 100
            + u64::from(
                position == 0 || position + delta % 100 < 0 || position + delta % 100 > 100,
            );
        position = (position + delta).rem_euclid(100);
        zeros += u64::from(position == 0);
    }
    (zeros, clicks)
}

#[aoc(day1, part1)]
fn part_1(input: &[i16]) -> u64 {
    unlock(input).0
}

#[aoc(day1, part2)]
fn part_2(input: &[i16]) -> u64 {
    unlock(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE1).unwrap();
        assert_eq!(result, [-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]);
    }

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
        assert_eq!(result, 6);
    }
}
