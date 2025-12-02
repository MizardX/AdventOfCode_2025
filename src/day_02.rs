use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<(u64, u64)>, ParseError> {
    input
        .split(',')
        .map(|r| {
            let (start, end) = r.split_once('-').ok_or(ParseError::SyntaxError)?;
            Ok((start.parse()?, end.parse()?))
        })
        .collect()
}

#[aoc(day2, part1)]
fn part_1(ranges: &[(u64, u64)]) -> u64 {
    let mut sum = 0;
    for &(start, end) in ranges {
        sum += (start..=end)
            .filter(|x| match x {
                10..=99 => x % 11 == 0,
                1_000..=9_999 => x % 101 == 0,
                100_000..=999_999 => x % 1_001 == 0,
                10_000_000..=99_999_999 => x % 10_001 == 0,
                1_000_000_000..=9_999_999_999 => x % 100_001 == 0,
                _ => false,
            })
            .sum::<u64>();
    }
    sum
}

#[aoc(day2, part2)]
fn part_2(ranges: &[(u64, u64)]) -> u64 {
    let mut sum = 0;
    for &(start, end) in ranges {
        sum += (start..=end)
            .filter(|x| match x {
                10..=99 => x % 11 == 0,
                100..=999 => x % 111 == 0,
                1_000..=9_999 => x % 101 == 0 || x % 1_111 == 0,
                10_000..=99_999 => x % 11_111 == 0,
                100_000..=999_999 => x % 1_001 == 0 || x % 10_101 == 0 || x % 111_111 == 0,
                1_000_000..=9_999_999 => x % 1_111_111 == 0,
                10_000_000..=99_999_999 => {
                    x % 10_001 == 0 || x % 1_010_101 == 0 || x % 11_111_111 == 0
                }
                100_000_000..=999_999_999 => x % 1_001_001 == 0 || x % 111_111_111 == 0,
                1_000_000_000..=9_999_999_999 => {
                    x % 100_001 == 0 || x % 101_010_101 == 0 || x % 1_111_111_111 == 0
                }
                _ => false,
            })
            .sum::<u64>();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE1).unwrap();
        assert_eq!(
            result,
            [
                (11, 22),
                (95, 115),
                (998, 1_012),
                (1_188_511_880, 1_188_511_890),
                (222_220, 222_224),
                (1_698_522, 1_698_528),
                (446_443, 446_449),
                (38_593_856, 38_593_862),
                (565_653, 565_659),
                (824_824_821, 824_824_827),
                (2_121_212_118, 2_121_212_124)
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let ranges = parse(EXAMPLE1).unwrap();
        let result = part_1(&ranges);
        assert_eq!(result, 1_227_775_554);
    }

    #[test]
    fn test_part_2() {
        let ranges = parse(EXAMPLE1).unwrap();
        let result = part_2(&ranges);
        assert_eq!(result, 4_174_379_265);
    }
}
