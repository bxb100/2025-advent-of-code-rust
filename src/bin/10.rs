use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::cell::{Cell, RefCell};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use z3::ast::{Ast, Bool, Int};
use z3::DeclKind::OR;
use z3::SortKind::Array;
use z3::{Config, Optimize, SatResult};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>();
        let mut ans = 0;

        for line in lines {
            let l = parse_line(&line);
            ans += solve_min_xor_elements(l.1, l.0)?.len();
        }

        Ok(ans)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.trim().is_empty())
            .join("\n");
        let ans = solve(&lines);
        Ok(ans as usize)
    }

    assert_eq!(33, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    // [.##.]
    let target: u64 = line
        .find('[')
        .and_then(|start| line.find(']').map(|end| &line[start + 1..end]))
        .map(|bits| {
            bits.chars()
                .enumerate()
                .map(|(i, c)| if c == '#' { 2u64.pow(i as u32) } else { 0 })
                .sum()
        })
        .unwrap();

    // (3) (1,3) (2) (2,3) (0,2) (0,1)
    let nums: Vec<u64> = line
        .split(' ')
        .filter_map(|part| {
            if part.starts_with('(') && part.ends_with(')') {
                let nums_str = &part[1..part.len() - 1];
                let num: u64 = nums_str
                    .split(',')
                    .filter_map(|s| s.parse::<u64>().ok())
                    .fold(0, |acc, n| acc | 2u64.pow(n as u32));
                Some(num)
            } else {
                None
            }
        })
        .collect();

    (target, nums)
}

thread_local! {
    static GET_ALL: Cell<bool> =  const { Cell::new(false) };
}

fn solve_min_xor_elements(nums: Vec<u64>, target: u64) -> Result<Vec<u64>> {
    // 队列存储: (当前的异或值, 用了哪些原始数字)
    let mut queue: VecDeque<(u64, Vec<u64>)> = VecDeque::new();

    // 记录访问过的异或值，避免重复计算死循环
    // Key: 异或值, Value: 达到这个值所需的最少数量
    let mut visited: HashMap<u64, usize> = HashMap::new();

    // 初始化：放入单个数字
    for &num in &nums {
        if num == target {
            return Ok(vec![num]);
        }
        if let Entry::Vacant(e) = visited.entry(num) {
            e.insert(1);
            queue.push_back((num, vec![num]));
        }
    }

    // BFS 开始
    while let Some((curr_val, mut path)) = queue.pop_front() {
        // 尝试异或数组中的每一个原始数字
        for &num in &nums {
            let next_val = curr_val ^ num;

            // 如果我们找到了目标
            if next_val == target {
                path.push(num);
                return Ok(path);
            }

            // 只有当我们发现了新的异或值，或者找到了更短路径到达该值时才入队
            // (注：BFS本身保证了先到的一定是最短或等长，所以只需通过 visited 判重)
            if let Entry::Vacant(e) = visited.entry(next_val) {
                e.insert(path.len() + 1);
                let mut next_path = path.clone();
                next_path.push(num);
                queue.push_back((next_val, next_path));
            }
        }
    }

    Err(anyhow!("无法组合出目标值"))
}

fn solve(input_data: &str) -> i64 {
    let mut total_presses = 0;

    // Regex to capture groups inside () and the target group inside {}
    let button_re = Regex::new(r"\(([\d,]+)\)").unwrap();
    let target_re = Regex::new(r"\{([\d,]+)\}").unwrap();

    let print_all_solutions: bool = GET_ALL.get();

    for line in input_data.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // --- 1. Parsing ---
        // Extract targets
        let target_caps = target_re.captures(line).expect("Failed to parse targets");
        let targets: Vec<i64> = target_caps[1]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let num_counters = targets.len();

        // Extract buttons
        // Each button is a Vec<usize> of the counters it affects
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        for cap in button_re.captures_iter(line) {
            let indices: Vec<usize> = cap[1].split(',').map(|s| s.parse().unwrap()).collect();
            buttons.push(indices);
        }

        // --- 2. Z3 Modeling ---
        let opt = Optimize::new();

        // Create a variable x_i for each button (number of presses)
        let mut x_vars = Vec::new();
        for i in 0..buttons.len() {
            let var_name = format!("x_{}", i);
            let x = Int::new_const(var_name);

            // Constraint: Presses must be non-negative (x >= 0)
            opt.assert(&x.ge(Int::from_i64(0)));
            x_vars.push(x);
        }

        // Constraint: For each counter j, sum of button effects == target[j]
        for j in 0..num_counters {
            let mut sum_expr = Int::from_i64(0);

            for (i, btn_indices) in buttons.iter().enumerate() {
                // If button i affects counter j
                if btn_indices.contains(&j) {
                    sum_expr += &x_vars[i];
                }
            }

            // The sum of effects must exactly equal the target joltage
            opt.assert(&sum_expr.eq(Int::from_i64(targets[j])));
        }

        // Objective: Minimize sum(x)
        let total_for_machine: Int = x_vars.iter().fold(Int::from_i64(0), |acc, x| acc + x);
        opt.minimize(&total_for_machine);

        let mut have_solution = false;

        if print_all_solutions {
            println!("All solutions for line: {}", line);
        }
        while opt.check(&[]) == SatResult::Sat {
            let model = opt.get_model().unwrap();
            println!(
                "{}",
                x_vars
                    .iter()
                    .map(|x| format!("{x:?}=>{}", model.eval(x, true).unwrap().as_i64().unwrap()))
                    .join(",")
            );

            if print_all_solutions {
                // block this solution
                let mut blocking_clause = Vec::new();
                for x in &x_vars {
                    let val = model.eval(x, true).unwrap().as_i64().unwrap();
                    blocking_clause.push(x.eq(Int::from_i64(val)).not());
                }
                opt.assert(&Bool::or(&blocking_clause));
                continue;
            }

            // Sum up the values from the model
            let machine_min: i64 = x_vars
                .iter()
                .map(|x| model.eval(x, true).unwrap().as_i64().unwrap())
                .sum();

            total_presses += machine_min;
            have_solution = true;
            break;
        }

        if print_all_solutions {
            continue;
        }

        if !have_solution {
            panic!("No solution found for line: {}", line);
        }
    }

    total_presses
}

#[cfg(test)]
mod tests {
    use crate::{solve, solve_min_xor_elements, GET_ALL, TEST};

    #[test]
    fn test_part2_all_solutions() {
        GET_ALL.set(true);
        solve(TEST);
    }

    #[test]
    fn test_solve_space() {
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        let a = solve_min_xor_elements(vec![8, 10, 4, 12, 5, 3], 6).unwrap();
        println!("{a:?}");
        // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        let a = solve_min_xor_elements(vec![29, 12, 17, 7, 30], 8).unwrap();
        println!("{a:?}");
        // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        let a = solve_min_xor_elements(vec![31, 25, 55, 5], 46).unwrap();
        println!("{a:?}");
    }
}
