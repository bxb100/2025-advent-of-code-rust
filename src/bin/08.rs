use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Mul;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = solve(reader, false)?;
        Ok(answer)
    }

    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        Ok(solve(reader, true)?)
    }

    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

impl Point {
    fn parse(s: String) -> Result<Point> {
        let coords: Vec<i64> = s
            .split(',')
            .map(|part| part.trim().parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        assert_eq!(
            coords.len(),
            3,
            "Expected 3 coordinates, got {}",
            coords.len()
        );

        Ok(Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }
}

fn solve<R: BufRead>(input: R, conj_all: bool) -> Result<usize> {
    let points: Vec<Point> = input
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(Point::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let n = points.len();
    let mut edges = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                u: i,
                v: j,
                dist_sq: points[i].dist_sq(&points[j]),
            });
        }
    }

    edges.sort_by_key(|e| e.dist_sq);

    let mut dsu = DSU::new(n);

    let iter: Box<dyn Iterator<Item = &Edge>> = if conj_all {
        Box::new(edges.iter())
    } else {
        // SAFETY: edges C(n, 2) > 10 in demo, > 1000 in real input
        let limit = if points.len() == 20 { 10 } else { 1000 };
        Box::new(edges.iter().take(limit))
    };

    let mut last = None;
    for edge in iter {
        if dsu.union(edge.u, edge.v) {
            last = Some(edge);
        }
    }

    // println!("{:?}", dsu);

    if conj_all {
        let last = last.unwrap();

        // println!("{:?}, {:?}", &points[last.u], &points[last.v]);

        return Ok(points[last.u].x.mul(points[last.v].x) as usize);
    }

    let mut circuit_sizes = Vec::new();
    for i in 0..n {
        if dsu.parent[i] == i {
            circuit_sizes.push(dsu.size[i]);
        }
    }

    let res = circuit_sizes
        .iter()
        // 降序排序
        .sorted_by_key(|&s| Reverse(s))
        .take(3)
        .product();

    Ok(res)
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn dist_sq(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

#[derive(Debug)]
struct Edge {
    u: usize,
    v: usize,
    dist_sq: i64,
}

// 并查集结构体
#[derive(Debug)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(), // 初始时每个点的父节点是自己
            size: vec![1; n],         // 初始时每个集合大小为 1
        }
    }

    // 查找根节点
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let i = self.find(i);
        let j = self.find(j);

        if i != j {
            if self.size[i] < self.size[j] {
                self.parent[i] = j;
                self.size[j] += self.size[i];
            } else {
                self.parent[j] = i;
                self.size[i] += self.size[j];
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        solve(BufReader::new(TEST.as_bytes()), true).unwrap();
    }
}
