use std::cmp::max;
use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

#[derive(Debug, Copy, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn is_within(&self, o: usize) -> bool {
        o >= self.start && o <= self.end
    }

    fn count(&self) -> usize {
        self.end - self.start + 1
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    // println!("=== Part 1 ===");
    //
    // fn part1<R: BufRead>(reader: R) -> Result<usize> {
    //     let mut lines = reader.lines();
    //     let mut ranges = vec![];
    //     let mut answer = 0;
    //     while let Some(x) = lines.next() {
    //         let x = x?;
    //         if x.trim().is_empty() {
    //             break;
    //         }
    //         let (a, b) = x
    //             .trim()
    //             .split_once('-')
    //             .ok_or(anyhow!("Invalid input format"))?;
    //         let start: usize = a.parse()?;
    //         let end: usize = b.parse()?;
    //         let range = Range { start, end };
    //         ranges.push(range);
    //     }
    //     for line in lines {
    //         let line = line?;
    //         if line.trim().is_empty() {
    //             continue;
    //         }
    //         let num: usize = line.trim().parse()?;
    //         if ranges.iter().any(|r| r.is_within(num)) {
    //             answer += 1;
    //         }
    //     }
    //
    //     Ok(answer)
    // }
    //
    // assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines();
        let mut ranges: Vec<Range> = vec![];
        while let Some(x) = lines.next() {
            let x = x?;
            if x.trim().is_empty() {
                break;
            }
            let (a, b) = x
                .trim()
                .split_once('-')
                .ok_or(anyhow!("Invalid input format"))?;
            let start: usize = a.parse()?;
            let end: usize = b.parse()?;
            let range = Range { start, end };

            ranges.push(range);
        }

        ranges.sort_by_key(|r| r.start);

        let mut merged: Vec<&mut Range> = Vec::new();

        let mut first = ranges.remove(0);

        merged.push(&mut first);

        for current in ranges.iter_mut() {
            let last = merged.last_mut().unwrap();

            if current.start <= last.end {
                last.end = max(last.end, current.end);
            } else {
                merged.push(current);
            }
        }

        // println!("{merged:?}");

        let ans = merged.iter().map(|r| r.count()).sum();

        Ok(ans)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
