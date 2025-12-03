#[aoc(day3, part1)]
fn part_1(input: &str) -> u64 {
    let mut res = 0;
    for line in input.lines() {
        res += find_max_joltage(line.as_bytes(), 2);
    }
    res
}

#[aoc(day3, part2)]
fn part_2(input: &str) -> u64 {
    let mut res = 0;
    for line in input.lines() {
        res += find_max_joltage(line.as_bytes(), 12);
    }
    res
}

fn find_max_joltage(batteries: &[u8], count: usize) -> u64 {
    let mut value = 0;
    let mut start = 0;
    let len = batteries.len();
    for end in len - count + 1..=len {
        let mut max_digit = 0;
        let mut max_pos = 0;
        for (i, &ch) in batteries[start..end].iter().enumerate() {
            let dig = ch - b'0';
            if dig > max_digit {
                max_digit = dig;
                max_pos = start + i;
                if max_digit == 9 {
                    break;
                }
            }
        }
        value = 10 * value + u64::from(max_digit);
        start = max_pos + 1;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
        987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111\
    ";

    #[test]
    fn test_part_1() {
        let res = part_1(EXAMPLE1);
        assert_eq!(res, 357);
    }

    #[test]
    fn test_part_2() {
        let res = part_2(EXAMPLE1);
        assert_eq!(res, 3_121_910_778_619);
    }
}
