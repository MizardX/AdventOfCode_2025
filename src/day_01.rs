use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

#[aoc(day1, part1)]
fn part_1(input: &[u64]) -> u64 {
    input.iter().copied().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "1,2,3";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE1).unwrap();
        assert_eq!(result, [1, 2, 3]);
    }

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE1).unwrap();
        let result = part_1(&input);
        assert_eq!(result, 6);
    }
}
