use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    // println!("=== Part 1 ===");
    //
    // fn part1<R: BufRead>(reader: R) -> Result<usize> {
    //     let mut matrix = vec![];
    //     reader.lines().map(|l| l.unwrap())
    //         .filter(|line| !line.is_empty())
    //         .for_each(|line| {
    //         let row: Vec<u8> = line.chars().map(|c| if c == '@' { 1 } else { 0 }).collect();
    //         matrix.push(row);
    //     });
    //     let col_count = matrix[0].len();
    //     let row_count = matrix.len();
    //     let mut count_around = vec![vec![0; col_count]; row_count];
    //
    //     for r in 0..row_count {
    //         for c in 0..col_count {
    //             // r, c + 1
    //             // r, c - 1
    //             count_around[r][c] = matrix[r][c];
    //             if c > 0 {
    //                 count_around[r][c] += matrix[r][c - 1];
    //             }
    //             if c + 1 < col_count {
    //                 count_around[r][c] += matrix[r][c + 1];
    //             }
    //
    //         }
    //     }
    //     println!("{:?}", count_around);
    //     let mut answer = 0;
    //     // around if small than 4
    //     for r in 0..row_count {
    //         for c in 0..col_count {
    //             let mut t = count_around[r][c] as i16;
    //             if r > 0 {
    //                 t += count_around[r - 1][c] as i16;
    //             }
    //             if r + 1 < row_count {
    //                 t += count_around[r + 1][c] as i16;
    //             }
    //             if t - 1 < 4 && matrix[r][c] == 1 {
    //                 answer += 1;
    //             }
    //         }
    //     }
    //
    //     Ok(answer)
    // }
    //
    // assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut matrix = vec![];
        reader
            .lines()
            .map(|l| l.unwrap())
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                let row: Vec<u8> = line.chars().map(|c| if c == '@' { 1 } else { 0 }).collect();
                matrix.push(row);
            });

        let mut answer = 0;
        loop {
            let t = solve(&mut matrix);
            if (t == 0) {
                break;
            }
            answer += t;
        }

        Ok(answer)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn solve(matrix: &mut Vec<Vec<u8>>) -> usize {
    let col_count = matrix[0].len();
    let row_count = matrix.len();
    let mut count_around = vec![vec![0; col_count]; row_count];

    for r in 0..row_count {
        for c in 0..col_count {
            // r, c + 1
            // r, c - 1
            count_around[r][c] = matrix[r][c];
            if c > 0 {
                count_around[r][c] += matrix[r][c - 1];
            }
            if c + 1 < col_count {
                count_around[r][c] += matrix[r][c + 1];
            }
        }
    }
    let mut answer = 0;
    // around if small than 4
    for r in 0..row_count {
        for c in 0..col_count {
            let mut t = count_around[r][c] as i16;
            if r > 0 {
                t += count_around[r - 1][c] as i16;
            }
            if r + 1 < row_count {
                t += count_around[r + 1][c] as i16;
            }
            if t - 1 < 4 && matrix[r][c] == 1 {
                matrix[r][c] = 0;
                answer += 1;
            }
        }
    }
    answer
}
