//  copy from https://github.com/icub3d/advent-of-code/blob/main/aoc_2025/src/bin/day10.rs

use rayon::iter::{ParallelBridge, ParallelIterator};
use std::time::Instant;

const INPUT: &str = include_str!("../../input/10.txt");
const EPSILON: f64 = 1e-9;

fn parse(input: &str) -> impl Iterator<Item = Machine> + use<'_> {
    input.trim().lines().map(Machine::from)
}

#[derive(Debug)]
struct Machine {
    #[allow(dead_code)]
    lights: usize,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}
impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let lights = parts
            .next()
            .map(|l| {
                // We rev here to make calculating below easier.
                l.trim_matches(['[', ']'])
                    .chars()
                    .rev()
                    .fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
            })
            .unwrap();

        let mut parts: Vec<&str> = parts.collect();
        let joltages = parts
            .pop()
            .unwrap()
            .trim_matches(['{', '}'])
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        let mut buttons: Vec<Vec<usize>> = parts
            .iter()
            .map(|b| {
                b.trim_matches(['(', ')'])
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .collect();

        // Sorting seems to help here. Not sure why, was just trying stuff.
        // buttons.sort_by_key(|b| std::cmp::Reverse(b.len()));

        Self {
            lights,
            buttons,
            joltages,
        }
    }
}

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
    dependents: Vec<usize>,
    independents: Vec<usize>,
}

impl Matrix {
    // Make a matrix, do a Gaussian elimination and setup the fixed and free variables.
    fn from_machine(machine: &Machine) -> Self {
        let rows = machine.joltages.len();
        let cols = machine.buttons.len();
        let mut data = vec![vec![0.0; cols + 1]; rows];

        // Add all of our buttons.
        for (index, button) in machine.buttons.iter().enumerate() {
            for &r in button {
                // if r < rows {
                data[r][index] = 1.0;
                // }
            }
        }

        // Add our joltages to the last column
        for (r, &val) in machine.joltages.iter().enumerate() {
            data[r][cols] = val as f64;
        }

        let mut matrix = Self {
            data,
            rows,
            cols,
            dependents: Vec::new(),
            independents: Vec::new(),
        };

        println!("{matrix:?}");
        matrix.gaussian_elimination();
        println!("{matrix:?}");
        matrix
    }

    // https://en.wikipedia.org/wiki/Gaussian_elimination
    fn gaussian_elimination(&mut self) {
        let mut pivot = 0;

        let mut col = 0;
        while pivot < self.rows && col < self.cols {
            // Find the best pivot row for this column.
            let (best_row, best_value) = self
                .data
                .iter()
                .enumerate()
                .skip(pivot)
                .map(|(r, row)| (r, row[col].abs()))
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            // If the best value is zero, this is a free variable.
            if best_value < EPSILON {
                self.independents.push(col);
                col += 1;
                continue;
            }

            // Swap rows and mark this column as dependent.
            self.data.swap(pivot, best_row);
            self.dependents.push(col);

            // Normalize pivot row.
            let pivot_value = self.data[pivot][col];
            for val in &mut self.data[pivot][col..=self.cols] {
                *val /= pivot_value;
            }

            // Eliminate this column in all other rows.
            for r in 0..self.rows {
                if r != pivot {
                    let factor = self.data[r][col];
                    if factor.abs() > EPSILON {
                        let pivot_row = self.data[pivot][col..=self.cols].to_vec();
                        self.data[r][col..=self.cols]
                            .iter_mut()
                            .zip(&pivot_row)
                            .for_each(|(val, &pivot_val)| {
                                *val -= factor * pivot_val;
                            });
                    }
                }
            }

            pivot += 1;
            col += 1;
        }

        // Any remaining columns are free variables
        self.independents.extend(col..self.cols);
    }

    // Check if the given values for our independent variables are valid. If so, return the total button presses.
    fn valid(&self, values: &[usize]) -> Option<usize> {
        // We start with how many times we've pressed the free variables.
        let mut total = values.iter().sum::<usize>();

        // Calculate dependent variable values based on independent variables.
        for row in 0..self.dependents.len() {
            // Calculate this dependent by subtracting the sum of the free variable pushes from the solution.
            let val = self
                .independents
                .iter()
                .enumerate()
                .fold(self.data[row][self.cols], |acc, (i, &col)| {
                    acc - self.data[row][col] * (values[i] as f64)
                });

            // We need non-negative, whole numbers for a valid solution.
            if val < -EPSILON {
                return None;
            }
            let rounded = val.round();
            if (val - rounded).abs() > EPSILON {
                return None;
            }

            total += rounded as usize;
        }

        Some(total)
    }
}

fn dfs(matrix: &Matrix, idx: usize, values: &mut [usize], min: &mut usize, max: usize) {
    // When we've assigned all independent variables, check if it's a valid solution.
    if idx == matrix.independents.len() {
        if let Some(total) = matrix.valid(values) {
            if *min >= total {
                *min = total;
                // 此时可能存在的值解的集合是:
                let mut others: Vec<Vec<f64>> = vec![];
                for (i, v) in values.iter().enumerate() {
                    let mut other = vec![0f64; matrix.cols + 1];
                    other[matrix.independents[i]] = 1f64;
                    other[matrix.cols] = *v as f64;
                    others.push(other);
                }

                println!("{:?}{values:?}{others:?}", matrix.independents);
                let mut r = matrix.data.clone();
                r.extend(others);

                println!("{min} - {:?}", solve_linear_system(r));
            }
        }
        return;
    }

    // Try different values for the current independent variable.
    let total: usize = values[..idx].iter().sum();
    for val in 0..max {
        // Optimization: If we ever go above our min, we can't possibly do better.
        if total + val >= *min {
            break;
        }
        values[idx] = val;
        dfs(matrix, idx + 1, values, min, max);
    }
}

fn p2(input: &str) -> usize {
    parse(input)
        .par_bridge()
        .map(|machine| {
            let matrix = Matrix::from_machine(&machine);

            // Now we can DFS over a much smaller solution space.
            let max = *machine.joltages.iter().max().unwrap() + 1;
            let mut min = usize::MAX;
            let mut values = vec![0; matrix.independents.len()];

            dfs(&matrix, 0, &mut values, &mut min, max);

            min
        })
        .sum()
}

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
";

/// 求解 Ax = b，返回解向量。如果无解或无穷解（不满秩），返回 None。
fn solve_linear_system(vec: Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let n = vec.len();

    let mut a = vec![];
    let mut b = vec![];
    for row in vec {
        b.push(*row.last().unwrap());
        a.push(row[..row.len() - 1].to_vec());
    }

    // 容差，用于判断浮点数是否为0
    let epsilon = 1e-10;

    for i in 0..n {
        // 1. 列主元选择 (Partial Pivoting)
        // 寻找第 i 列中，从第 i 行到底部，绝对值最大的元素所在的行
        let mut pivot_row = i;
        for k in (i + 1)..n {
            if a[k][i].abs() > a[pivot_row][i].abs() {
                pivot_row = k;
            }
        }

        // 2. 奇异性检查 (Singularity Check)
        // 如果主元极其接近 0，说明矩阵不满秩
        if a[pivot_row][i].abs() < epsilon {
            return None; // 无法求解
        }

        // 3. 交换行 (Swap rows)
        // 交换系数矩阵 A 的行
        a.swap(i, pivot_row);
        // 交换常数向量 b 的行
        b.swap(i, pivot_row);

        // 4. 归一化 (Normalize pivot row)
        // 使 a[i][i] 变为 1
        let pivot = a[i][i];
        for j in i..n {
            a[i][j] /= pivot;
        }
        b[i] /= pivot;

        // 5. 消元 (Eliminate other rows)
        // 使第 i 列的其他行变为 0 (包括 i 行上面的行，这样最后得到的就是简化行阶梯型)
        for k in 0..n {
            if k != i {
                let factor = a[k][i];
                // 优化：列 j 从 i 开始即可，因为 i 之前的都已经为 0
                for j in i..n {
                    a[k][j] -= factor * a[i][j];
                }
                b[k] -= factor * b[i];
            }
        }
    }

    Some(b)
}

fn main() {
    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
    assert_eq!(solution, 17970);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        let result = p2(TEST);
        assert_eq!(result, 10);
    }
}
