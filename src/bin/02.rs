use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::{concatcp, str_get};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    // println!("=== Part 1 ===");
    //
    // fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
    //     let mut answer: Vec<usize> = Vec::new();
    //     let mut buf = vec![];
    //     loop {
    //         let num = reader.read_until(b',', &mut buf)?;
    //         if num == 0 {
    //             break;
    //         }
    //         let s = String::from_utf8_lossy(&buf);
    //         let (a, b) = s
    //             .trim_start_matches('\n')
    //             .trim_end_matches([',', '\n'])
    //             .split_once('-')
    //             .ok_or(anyhow!("Invalid input format"))?;
    //         let start: usize = dbg!(a).parse()?;
    //         let end: usize = dbg!(b).parse()?;
    //
    //         for num in start..=end {
    //             if num < 10 {
    //                 continue;
    //             }
    //             let digits = match num.checked_ilog10() {
    //                 Some(exp) => exp + 1,
    //                 None => continue,
    //             };
    //             if digits % 2 != 0 {
    //                 continue;
    //             }
    //
    //             let half = digits / 2;
    //             let divisor: usize = 10usize.pow(half);
    //             if (num / divisor) == (num % divisor) {
    //                 answer.push(num);
    //             }
    //         }
    //
    //         buf.clear();
    //     }
    //
    //     Ok(answer.iter().sum())
    // }
    //
    // assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut answer = vec![];
        let mut buf = vec![];
        loop {
            if reader.read_until(b',', &mut buf)? == 0 {
                break;
            }
            let s = String::from_utf8_lossy(&buf);
            let (a, b) = s
                .trim_start_matches('\n')
                .trim_end_matches([',', '\n'])
                .split_once('-')
                .ok_or(anyhow!("Invalid input format"))?;
            let start: usize = a.parse()?;
            let end: usize = b.parse()?;

            for i in start..=end {
                let s = i.to_string();
                let bytes = s.as_bytes();
                if check(bytes) {
                    answer.push(i);
                }
            }

            buf.clear();
        }
        println!("{:?}", answer);
        Ok(answer.iter().sum())
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    // 41662374059
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn check(bytes: &[u8]) -> bool {
    let len = bytes.len();
    for d in 1..=(len / 2) {
        // repeat pattern length d * t = len
        if !len.is_multiple_of(d) {
            continue;
        }
        let mut is_match = true;
        for index in (d..len) {
            // d = 3
            // 123 123 123
            // ^
            //     ^
            //         ^
            if bytes[index] != bytes[index - d] {
                is_match = false;
                break;
            }
        }
        if is_match {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::check;

    #[test]
    fn test_left_bit() {
        for step in 0..4 {
            let l: u64 = 2u64.pow(step + 1) - 1;
            let r: u64 = l << (step + 1);
            println!("step={} l={:b} right={:b}", step, l, r);
            if (88 & l) << (step + 1) == (88 & r) {
                println!("match");
            }
        }
    }

    #[test]
    fn test_bit_num() {
        let a = match 32u64.checked_ilog10() {
            Some(exp) => exp + 1,
            None => panic!("panic"),
        };
        println!("a={a}");
    }

    #[test]
    fn test_check() {
        assert!(check(b"1212"));
        assert!(check(b"111111111"));
    }
}
