use std::num::ParseIntError;
use std::str::FromStr;

use microlp::{LinearExpr, OptimizationDirection, Problem};
use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    indicator_lights: u16,
    buttons: Vec<u16>,
    joltage: Vec<u16>,
}

impl FromStr for Machine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s.strip_prefix('[').ok_or(ParseError::SyntaxError)?;
        let (indicator_lights, rest) = rest.split_once("] ").ok_or(ParseError::SyntaxError)?;
        let indicator_lights = indicator_lights
            .bytes()
            .enumerate()
            .fold(0, |bits, (pos, ch)| bits | u16::from(ch & 1) << pos);
        let (buttons, rest) = rest.split_once(" {").ok_or(ParseError::SyntaxError)?;
        let buttons = buttons
            .split(' ')
            .map(|btn| {
                btn.strip_prefix('(')
                    .ok_or(ParseError::SyntaxError)?
                    .strip_suffix(')')
                    .ok_or(ParseError::SyntaxError)?
                    .split(',')
                    .try_fold(0_u16, |mask, light| Ok(mask | (1 << light.parse::<u8>()?)))
            })
            .collect::<Result<_, ParseError>>()?;
        let joltage = rest
            .strip_suffix('}')
            .ok_or(ParseError::SyntaxError)?
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            indicator_lights,
            buttons,
            joltage,
        })
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Vec<Machine>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day10, part1)]
fn part_1(machines: &[Machine]) -> u64 {
    let mut sum = 0;
    for machine in machines {
        let mut minimal = u32::MAX;
        for mask in 0..(1_u16 << machine.buttons.len()) {
            let num_active = mask.count_ones();
            let remaining_indicators = machine
                .buttons
                .iter()
                .enumerate()
                .filter(|(ix, _)| mask & (1 << ix) != 0)
                .fold(machine.indicator_lights, |m, (_, &b)| m ^ b);
            if remaining_indicators == 0 {
                minimal = minimal.min(num_active);
            }
        }
        assert!(minimal != u32::MAX, "No solution: {machine:?}");
        sum += u64::from(minimal);
    }
    sum
}

#[aoc(day10, part2)]
fn part_2(machines: &[Machine]) -> u64 {
    let mut sum = 0;
    for machine in machines {
        sum += minimum_presses(&machine.buttons, &machine.joltage);
    }
    sum
}

fn minimum_presses(buttons: &[u16], target: &[u16]) -> u64 {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let target_max = target.iter().copied().max().unwrap();
    let button_vars = buttons
        .iter()
        .map(|_| problem.add_integer_var(1.0, (0, i32::from(target_max))))
        .collect::<Vec<_>>();
    for (light_ix, &trg) in target.iter().enumerate() {
        let mut expr = LinearExpr::empty();
        for (btn_mask, &btn_var) in buttons.iter().zip(&button_vars) {
            if btn_mask & (1 << light_ix) != 0 {
                expr.add(btn_var, 1.0);
            }
        }
        problem.add_constraint(expr, microlp::ComparisonOp::Eq, f64::from(trg));
    }
    let solution = problem.solve().expect("Any solution");
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Value should be less than sum(target). Any problem with a solution > u64::MAX not present."
    )]
    #[expect(
        clippy::cast_sign_loss,
        reason = "All cofficients and variables are positive, so minimal solution should also be positive."
    )]
    {
        solution.objective().round() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\
    ";

    #[test]
    fn test_parse() {
        #![allow(clippy::unreadable_literal)]
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(
            result,
            [
                Machine {
                    indicator_lights: 0b0110,
                    buttons: vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011],
                    joltage: vec![3, 5, 4, 7],
                },
                Machine {
                    indicator_lights: 0b01000,
                    buttons: vec![0b11101, 0b01100, 0b10001, 0b00111, 0b11110],
                    joltage: vec![7, 5, 12, 7, 2],
                },
                Machine {
                    indicator_lights: 0b101110,
                    buttons: vec![0b011111, 0b011001, 0b110111, 0b000110],
                    joltage: vec![10, 11, 11, 5, 10, 5]
                }
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let machines = parse(EXAMPLE).unwrap();
        let result = part_1(&machines);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_2() {
        let machines = parse(EXAMPLE).unwrap();
        let result = part_2(&machines);
        assert_eq!(result, 33);
    }
}
