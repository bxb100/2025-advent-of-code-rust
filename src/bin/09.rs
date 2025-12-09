use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = Grid::parse_reader(reader)?;
        let answer = solve(lines, false);
        Ok(answer)
    }

    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = Grid::parse_reader(reader)?;
        let answer = solve(lines, true);
        Ok(answer)
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn solve(red_grids: Vec<Grid>, need_validate: bool) -> usize {
    let n = red_grids.len();
    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = &red_grids[i];
            let p2 = &red_grids[j];

            let min_x = p1.0.min(p2.0);
            let max_x = p1.0.max(p2.0);
            let min_y = p1.1.min(p2.1);
            let max_y = p1.1.max(p2.1);

            let current_area = (max_x - min_x + 1) * (max_y - min_y + 1);
            if current_area <= max_area {
                continue;
            }
            if !need_validate || is_valid_rectangle(min_x, max_x, min_y, max_y, &red_grids) {
                max_area = current_area;
            }
        }
    }

    max_area as usize
}

/// 隐藏条件: 限制条件是一个正交多边形
///
/// define: https://en.wikipedia.org/wiki/Rectilinear_polygon
fn is_valid_rectangle(min_x: i64, max_x: i64, min_y: i64, max_y: i64, red_grids: &[Grid]) -> bool {
    let n = red_grids.len();

    // 检查是否有边穿过矩形内部
    for k in 0..n {
        let p_a = &red_grids[k];
        let p_b = &red_grids[(k + 1) % n]; // 下一个点，处理循环

        if p_a.0 == p_b.0 {
            let edge_x = p_a.0;
            let edge_y_min = p_a.1.min(p_b.1);
            let edge_y_max = p_a.1.max(p_b.1);

            if edge_x > min_x && edge_x < max_x {
                if max_y <= edge_y_min || min_y >= edge_y_max {
                    continue; // 完全在外面
                } else {
                    return false;
                }
            }
        } else {
            let edge_y = p_a.1;
            let edge_x_min = p_a.0.min(p_b.0);
            let edge_x_max = p_a.0.max(p_b.0);

            if edge_y > min_y && edge_y < max_y {
                if max_x <= edge_x_min || min_x >= edge_x_max {
                    continue;
                } else {
                    return false;
                }
            }
        }
    }

    let mid_x = (min_x as f64 + max_x as f64) / 2.0;
    let mid_y = (min_y as f64 + max_y as f64) / 2.0;

    let mut intersections = 0;
    // 判断这个矩形在多边形的内外
    for k in 0..n {
        let p_a = &red_grids[k];
        let p_b = &red_grids[(k + 1) % n];

        // 垂直向右发射
        if (p_a.1 as f64 > mid_y) != (p_b.1 as f64 > mid_y) {
            // 直线公式
            // $intersect_x = x_1 + (y - y_1) \times \frac{x_2 - x_1}{y_2 - y_1}$
            let intersect_x = (p_b.0 - p_a.0) as f64 * (mid_y - p_a.1 as f64)
                / (p_b.1 - p_a.1) as f64
                + p_a.0 as f64;

            if mid_x < intersect_x {
                intersections += 1;
            }
        }
    }

    // 偶数在外
    intersections % 2 == 1
}

#[derive(Debug)]
struct Grid(i64, i64);

impl Grid {
    fn parse_reader(reader: impl BufRead) -> Result<Vec<Grid>> {
        let mut grids = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() != 2 {
                return Err(anyhow!("Invalid grid line: {}", line));
            }
            let x: i64 = parts[0].parse()?;
            let y: i64 = parts[1].parse()?;
            grids.push(Grid(x, y));
        }

        Ok(grids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        solve(
            Grid::parse_reader(BufReader::new(TEST.as_bytes())).unwrap(),
            false,
        );
    }
}
