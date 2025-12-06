use std::ops::Index;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    const fn get_index(&self, (row, col): (usize, usize)) -> Option<usize> {
        if row < self.height || col < self.width {
            Some(row * self.width + col)
        } else {
            None
        }
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.get_index(index).expect("index out of range")]
    }
}

impl FromStr for Grid {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().len();
        let mut data = Vec::with_capacity(width * height);
        for line in s.lines() {
            data.extend_from_slice(line.as_bytes());
        }
        Ok(Self {
            data,
            width,
            height,
        })
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Grid {
    input.parse().unwrap()
}

#[aoc(day6, part1)]
fn part_1(grid: &Grid) -> u64 {
    let mut right = grid.width;
    let bottom = grid.height - 1;
    let mut total_sum = 0;
    for left in (0..grid.width).rev() {
        let mut res = match grid[(bottom, left)] {
            b'*' => 1,
            b'+' => 0,
            _ => continue,
        };
        for r in 0..bottom {
            let num = (left..right).fold(0, |val, c| {
                let ch = grid[(r, c)];
                if ch == b' ' {
                    val
                } else {
                    val * 10 + u64::from(ch - b'0')
                }
            });
            match grid[(bottom, left)] {
                b'*' => res *= num,
                b'+' => res += num,
                _ => unreachable!(),
            }
        }
        total_sum += res;
        right = left;
    }
    total_sum
}

#[aoc(day6, part2)]
fn part_2(grid: &Grid) -> u64 {
    let mut right = grid.width;
    let bottom = grid.height - 1;
    let mut total_sum = 0;
    for left in (0..grid.width).rev() {
        let mut res = match grid[(bottom, left)] {
            b'*' => 1,
            b'+' => 0,
            _ => continue,
        };
        for c in left..right {
            let num = (0..bottom).fold(0, |val, r| {
                let ch = grid[(r, c)];
                if ch == b' ' {
                    val
                } else {
                    val * 10 + u64::from(ch - b'0')
                }
            });
            if num == 0 {
                continue;
            }
            match grid[(bottom, left)] {
                b'*' => res *= num,
                b'+' => res += num,
                _ => unreachable!(),
            }
        }
        total_sum += res;
        right = left;
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_part_1() {
        let grid = parse(EXAMPLE1);
        let res = part_1(&grid);
        assert_eq!(res, 4_277_556);
    }

    #[test]
    fn test_part_2() {
        let grid = parse(EXAMPLE1);
        let res = part_2(&grid);
        assert_eq!(res, 3_263_827);
    }
}
