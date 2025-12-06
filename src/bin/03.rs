use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Mul;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    // println!("=== Part 1 ===");
    //
    // fn part1<R: BufRead>(reader: R) -> Result<usize> {
    //     let mut answer = 0;
    //
    //     for line in reader.lines() {
    //         let line = line?;
    //         if line.trim().is_empty() {
    //             continue;
    //         }
    //         let jolts = find_maximum_jolts(line.trim().as_bytes(), 2);
    //         answer += jolts as usize;
    //     }
    //
    //     Ok(answer)
    // }
    //
    // assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let jolts = find_maximum_jolts(line.trim().as_bytes(), 12);
            answer += jolts as usize;
        }

        Ok(answer)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

// 818181911112111 -> 92
fn find_maximum_jolts(bytes: &[u8], cap: usize) -> usize {
    let mut stack: Vec<u8> = vec![];
    let len = bytes.len();
    for (index, b) in bytes.iter().enumerate() {
        while !stack.is_empty() && b > stack.last().unwrap() {
            if stack.len() + len - 1 - index < cap {
                break;
            }
            stack.pop();
        }
        stack.push(*b);
    }

    let mut ans = 0;
    for b in (1..=cap).rev() {
        ans += 10usize
            .pow(b as u32 - 1)
            .mul((stack.remove(0) - b'0') as usize);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_2_bit_maximum_jolts() {
        assert_eq!(98, find_maximum_jolts(b"987654321111111", 2));
        assert_eq!(89, find_maximum_jolts(b"811111111111119", 2));
        assert_eq!(78, find_maximum_jolts(b"234234234234278", 2));
        assert_eq!(92, find_maximum_jolts(b"818181911112111", 2));
    }

    #[test]
    fn test_find_12_bit_maximum_jolts() {
        assert_eq!(987654321111, find_maximum_jolts(b"987654321111111", 12));
        assert_eq!(811111111119, find_maximum_jolts(b"811111111111119", 12));
        assert_eq!(434234234278, find_maximum_jolts(b"234234234234278", 12));
        assert_eq!(888911112111, find_maximum_jolts(b"818181911112111", 12));
    }

    #[test]
    fn test_stack() {
        let mut stack: Vec<u32> = vec![];
        stack.push(1);
        stack.push(2);
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }
}
