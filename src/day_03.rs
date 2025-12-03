#[aoc(day3, part1)]
fn part_1(input: &str) -> u64 {
    let mut res = 0;
    for line in input.lines() {
        let mut max_before = 0;
        let mut max_val = 0;
        for ch in line.bytes() {
            max_val = max_val.max(max_before * 10 + (ch - b'0'));
            max_before = max_before.max(ch - b'0');
        }
        res += u64::from(max_val);
    }
    res
}

#[aoc(day3, part2)]
fn part_2(input: &str) -> u64 {
    let mut res = 0;
    let mut digits = Vec::new();
    for line in input.lines() {
        digits.clear();
        for ch in line.bytes() {
            let dig = ch - b'0';
            digits.push(dig);
            if digits.len() > 12 {
                let mut found_lt = false;
                for (i, (d1, d2)) in digits.iter().zip(digits.iter().skip(1)).enumerate() {
                    if d1 < d2 {
                        found_lt = true;
                        digits.remove(i);
                        break;
                    }
                }
                if !found_lt {
                    digits.pop();
                }
            }
        }
        res += digits.iter().copied().fold(0, |s, d| s * 10 + u64::from(d));
    }
    res
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
