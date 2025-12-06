use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
        Ok(solve_part1(&lines))
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;

        Ok(solve_part2(&s))
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn solve_part1(lines: &[String]) -> usize {
    let mut vec = vec![];
    for line in &lines[..lines.len() - 1] {
        let v: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        vec.push(v);
    }
    let mut ans = 0;
    let symbols = lines.last().unwrap();
    for (index, symbol) in symbols.split_whitespace().enumerate() {
        match symbol {
            "+" => {
                ans += vec.iter().map(|r| r[index]).sum::<u64>() as usize;
            }
            "*" => {
                ans += vec.iter().map(|r| r[index]).product::<u64>() as usize;
            }
            _ => panic!("Unknown symbol"),
        }
    }
    ans
}

fn solve_part2(data: &str) -> usize {
    let mut column_data = vec![];
    let mut ans = 0;
    for line in data.lines() {
        for (i, c) in line.chars().enumerate() {
            if column_data.len() == i {
                // init
                column_data.push(0);
            }
            if let Some(d) = c.to_digit(10) {
                // sum d{i} * 10^(len-1-i)
                column_data[i] *= 10;
                column_data[i] += d as usize;
            } else if c == '+' {
                ans += column_data[i..]
                    .iter()
                    .take_while(|&&x| x != 0)
                    .sum::<usize>();
            } else if c == '*' {
                ans += column_data[i..]
                    .iter()
                    .take_while(|&&x| x != 0)
                    .product::<usize>();
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_whitespace() {
        let input = "*   +   *   +  ";
        let v: Vec<_> = input.split_whitespace().collect();
        assert_eq!(vec!["*", "+", "*", "+"], v);
    }

    #[test]
    fn test_part1() {
        let lines = TEST.lines().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(4277556, solve_part1(&lines));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3263827, solve_part2(TEST));
    }
}
