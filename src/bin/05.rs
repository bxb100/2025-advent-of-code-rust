use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

#[allow(dead_code)]
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
        let lines = reader.lines();
        let mut ranges: Vec<Range> = vec![];
        for x in lines {
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

        let mut max_i = 0;
        let mut ans = 0;
        for Range { start, end } in ranges {
            // the max_i means the previous end index add 1, so current end could be equaled
            if end >= max_i {
                println!("end: {end}, start: {}", max(start, max_i));
                ans += end - max(start, max_i) + 1;
                // +1 because the end has been added
                max_i = end + 1;
            }
        }

        Ok(ans)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
