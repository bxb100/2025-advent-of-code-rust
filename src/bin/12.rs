use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

type Point = (i32, i32);
type Shape = Vec<Point>;

fn normalize_shape(coords: HashSet<Point>) -> Shape {
    if coords.is_empty() {
        return Vec::new();
    }
    let min_x = coords.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = coords.iter().map(|(_, y)| *y).min().unwrap();

    let mut normalized: Shape = coords
        .into_iter()
        .map(|(x, y)| (x - min_x, y - min_y))
        .collect();
    normalized.sort();
    normalized
}

fn get_shape_dims(shape: &[Point]) -> (i32, i32) {
    if shape.is_empty() {
        return (0, 0);
    }
    let max_x = shape.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = shape.iter().map(|(_, y)| *y).max().unwrap();
    (max_x + 1, max_y + 1)
}

fn rotate90(shape: &Shape) -> HashSet<Point> {
    shape.iter().map(|(x, y)| (-y, *x)).collect()
}

fn flip_h(shape: &Shape) -> HashSet<Point> {
    shape.iter().map(|(x, y)| (-x, *y)).collect()
}

fn get_all_orientations(initial_shape: &HashSet<Point>) -> Vec<Shape> {
    let mut orientations = BTreeSet::new();

    let mut current_set = initial_shape.clone();

    for _ in 0..4 {
        let current_normalized = normalize_shape(current_set.clone());
        orientations.insert(current_normalized.clone());

        let flipped_set = flip_h(&current_normalized);
        let flipped_normalized = normalize_shape(flipped_set);
        orientations.insert(flipped_normalized);

        current_set = rotate90(&current_normalized);
    }

    orientations.into_iter().collect()
}

fn solve_region(
    width: usize,
    height: usize,
    shapes_orientations: &HashMap<usize, Vec<Shape>>,
    piece_counts: &[usize],
) -> bool {
    let mut pieces = Vec::new();
    for (id, &count) in piece_counts.iter().enumerate() {
        for _ in 0..count {
            pieces.push(id);
        }
    }

    if pieces.is_empty() {
        return true;
    }

    let total_cells: usize = pieces
        .iter()
        .filter_map(|&id| shapes_orientations.get(&id))
        .map(|shapes| shapes[0].len())
        .sum();

    if total_cells > width * height {
        return false;
    }

    pieces.sort_by_key(|&id| {
        let len = shapes_orientations
            .get(&id)
            .map(|s| s[0].len())
            .unwrap_or(0);
        Reverse(len)
    });

    let mut grid = vec![vec![false; width]; height];

    fn backtrack(
        piece_idx: usize,
        pieces: &[usize],
        grid: &mut [Vec<bool>],
        shapes_orientations: &HashMap<usize, Vec<Shape>>,
        width: usize,
        height: usize,
    ) -> bool {
        if piece_idx >= pieces.len() {
            return true;
        }

        let shape_id = pieces[piece_idx];
        let orientations = match shapes_orientations.get(&shape_id) {
            Some(o) => o,
            None => return false,
        };

        for shape in orientations {
            let (sw, sh) = get_shape_dims(shape);
            if sw as usize > width || sh as usize > height {
                continue;
            }

            let max_x = width - sw as usize;
            let max_y = height - sh as usize;

            for py in 0..=max_y {
                for px in 0..=max_x {
                    let mut fits = true;
                    for &(sx, sy) in shape {
                        if grid[py + sy as usize][px + sx as usize] {
                            fits = false;
                            break;
                        }
                    }

                    if fits {
                        for &(sx, sy) in shape {
                            grid[py + sy as usize][px + sx as usize] = true;
                        }

                        if backtrack(
                            piece_idx + 1,
                            pieces,
                            grid,
                            shapes_orientations,
                            width,
                            height,
                        ) {
                            return true;
                        }

                        for &(sx, sy) in shape {
                            grid[py + sy as usize][px + sx as usize] = false;
                        }
                    }
                }
            }
        }
        false
    }

    backtrack(0, &pieces, &mut grid, shapes_orientations, width, height)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        let mut i = 0;

        let mut shapes = HashMap::new();
        let mut regions = Vec::new();

        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() {
                i += 1;
                continue;
            }

            if line.contains(':') && !line.contains('x') {
                // Shape definition
                let id_str = line.trim_end_matches(':');
                let shape_id: usize = id_str.parse()?;
                i += 1;

                let mut coords = HashSet::new();
                let mut y = 0;
                while i < lines.len() {
                    let sl = &lines[i];
                    if sl.trim().is_empty() {
                        break;
                    }
                    // Check if next line is a header (digit:)
                    let t = sl.trim();
                    if let Some(c) = t.chars().next() {
                        if c.is_ascii_digit() && t.contains(':') {
                            break;
                        }
                    }

                    for (x, ch) in sl.chars().enumerate() {
                        if ch == '#' {
                            coords.insert((x as i32, y));
                        }
                    }
                    y += 1;
                    i += 1;
                }
                shapes.insert(shape_id, normalize_shape(coords));
            } else if line.contains(':') && line.contains('x') {
                // Region definition
                let parts: Vec<&str> = line.split(':').collect();
                let dims: Vec<&str> = parts[0].trim().split('x').collect();
                let w: usize = dims[0].parse()?;
                let h: usize = dims[1].parse()?;

                let counts: Vec<usize> = parts[1]
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                regions.push((w, h, counts));
                i += 1;
            } else {
                i += 1;
            }
        }

        // Precompute all orientations
        let mut shapes_orientations = HashMap::new();
        for (id, shape) in shapes {
            let set_shape: HashSet<Point> = shape.into_iter().collect();
            shapes_orientations.insert(id, get_all_orientations(&set_shape));
        }

        let mut count = 0;
        for (w, h, pieces) in regions {
            if solve_region(w, h, &shapes_orientations, &pieces) {
                count += 1;
            }
        }

        Ok(count)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
