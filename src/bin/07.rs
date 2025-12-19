use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let data = reader
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<String>>();
        let answer = solve(data, false);
        Ok(answer)
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let data = reader
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<String>>();
        let answer = solve(data, true);
        Ok(answer)
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn solve(data: Vec<String>, is_part2: bool) -> usize {
    let mut calc: Vec<usize> = vec![];
    let mut ans = 0;
    // SAFETY: at least one line
    let first = &data[0];
    let width = first.len();

    for c in first.chars() {
        if c == 'S' {
            calc.push(1);
            continue;
        }
        calc.push(0);
    }
    for line in &data[1..] {
        for (i, c) in line.chars().enumerate() {
            if c == '^' {
                if i > 0 {
                    calc[i - 1] += calc[i];
                }
                if i + 1 < width {
                    calc[i + 1] += calc[i];
                }
                if calc[i] > 0 {
                    ans += 1;
                }
                calc[i] = 0;
            }
        }
    }

    if is_part2 {
        return calc.iter().sum();
    }

    ans
}

#[cfg(test)]
mod tests {}
