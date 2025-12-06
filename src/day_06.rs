use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Mul,
    Add,
}

impl Operator {
    const fn unit(self) -> u64 {
        match self {
            Self::Mul => 1,
            Self::Add => 0,
        }
    }
}

impl TryFrom<u8> for Operator {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'*' => Self::Mul,
            b'+' => Self::Add,
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

#[aoc(day6, part1)]
fn part_1(input: &str) -> u64 {
    let mut lines: Vec<_> = input.lines().collect();
    let ops = lines
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .map(|op| {
            let &[ch] = op.as_bytes() else {
                return Err(ParseError::SyntaxError);
            };
            Operator::try_from(ch)
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut results = ops.iter().map(|op| op.unit()).collect::<Vec<_>>();
    for line in lines {
        for ((res, op), num) in results
            .iter_mut()
            .zip(&ops)
            .zip(line.split_ascii_whitespace())
        {
            let num = num.parse::<u64>().unwrap();
            match op {
                Operator::Mul => *res *= num,
                Operator::Add => *res += num,
            }
        }
    }
    results.into_iter().sum()
}

#[aoc(day6, part2)]
fn part_2(input: &str) -> u64 {
    let mut lines = input.lines().collect::<Vec<_>>();
    let op_line = lines.pop().unwrap();
    let mut ops = op_line
        .match_indices(['*', '+'])
        .map(|(ix, s)| {
            let &[ch] = s.as_bytes() else {
                return Err(ParseError::SyntaxError);
            };
            let op: Operator = ch.try_into()?;
            Ok((op, (ix..(ix + 1))))
        })
        .collect::<Result<Vec<_>, ParseError>>()
        .unwrap();
    let mut end = op_line.len();
    for (_, rng) in ops.iter_mut().rev() {
        *rng = rng.start..end;
        end = rng.start;
    }
    let mut total_sum = 0;
    for &(op, ref rng) in &ops {
        let mut res = op.unit();
        for ix in rng.clone() {
            let num = lines
                .iter()
                .map(|l| l.as_bytes()[ix])
                .filter(|&ch| ch != b' ')
                .fold(0, |val, dig| val * 10 + u64::from(dig - b'0'));
            if num == 0 {
                continue;
            }
            match op {
                Operator::Mul => res *= num,
                Operator::Add => res += num,
            }
        }
        total_sum += res;
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_part_1() {
        let res = part_1(EXAMPLE1);
        assert_eq!(res, 4_277_556);
    }

    #[test]
    fn test_part_2() {
        let res = part_2(EXAMPLE1);
        assert_eq!(res, 3_263_827);
    }
}
