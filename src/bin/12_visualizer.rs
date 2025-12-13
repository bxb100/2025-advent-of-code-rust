use anyhow::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

type Point = (i32, i32);
type Shape = Vec<Point>;

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

12x5: 1 0 1 0 3 2
";

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
    shape.iter().map(|(x, y)| (-*y, *x)).collect()
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

// Colors for different shapes (ANSI escape codes)
const COLORS: &[&str] = &[
    "\x1b[31m", // Red
    "\x1b[32m", // Green
    "\x1b[33m", // Yellow
    "\x1b[34m", // Blue
    "\x1b[35m", // Magenta
    "\x1b[36m", // Cyan
    "\x1b[91m", // Bright Red
    "\x1b[92m", // Bright Green
    "\x1b[93m", // Bright Yellow
    "\x1b[94m", // Bright Blue
];

const RESET: &str = "\x1b[0m";

fn render_shape_to_lines(shape: &Shape, id: usize) -> Vec<String> {
    let (w, h) = get_shape_dims(shape);
    let color = COLORS[id % COLORS.len()];

    // Easier way: build strings row by row
    let mut lines = Vec::new();
    for y in 0..h {
        let mut line = String::new();
        for x in 0..w {
            let is_set = shape.contains(&(x, y));
            if is_set {
                line.push_str(&format!("{}██{}", color, RESET));
            } else {
                line.push_str("  ");
            }
        }
        lines.push(line);
    }
    lines
}

fn generate_legend(
    shapes_orientations: &HashMap<usize, Vec<Shape>>,
    piece_counts: &[usize],
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("Required Shapes:".to_string());
    lines.push("".to_string());

    let mut ids: Vec<usize> = Vec::new();
    for (id, &count) in piece_counts.iter().enumerate() {
        if count > 0 {
            ids.push(id);
        }
    }
    // Sort logic not strictly needed if we iterate 0..len, but explicit sort feels safer if iteration order changes
    ids.sort();

    // Configuration for tiling
    const COLUMNS: usize = 3; // Shapes per row
    const COL_WIDTH_CHARS: usize = 20; // Fixed width for each column block (enough for "Shape X:" + shape width)

    // Calculate how many terminal rows we need actually depend on the shapes.
    // We will build `COLUMNS` buffers and then merge them line by line.

    let mut id_chunks: Vec<Vec<usize>> = Vec::new();
    let mut chunk = Vec::new();
    for id in ids {
        chunk.push(id);
        if chunk.len() == COLUMNS {
            id_chunks.push(chunk);
            chunk = Vec::new();
        }
    }
    if !chunk.is_empty() {
        id_chunks.push(chunk);
    }

    for row_ids in id_chunks {
        // Render each shape in this row to a block of lines
        let mut row_blocks: Vec<Vec<String>> = Vec::new();
        let mut max_block_height = 0;

        for id in row_ids {
            let mut block = Vec::new();
            if let Some(shapes) = shapes_orientations.get(&id) {
                if let Some(first_shape) = shapes.first() {
                    let count = piece_counts[id];
                    block.push(format!("Shape {} (x{}):", id, count));
                    let shape_lines = render_shape_to_lines(first_shape, id);
                    block.extend(shape_lines);
                }
            }
            if block.len() > max_block_height {
                max_block_height = block.len();
            }
            row_blocks.push(block);
        }

        // Merge blocks into full lines
        for h in 0..max_block_height {
            let mut full_line = String::new();
            for block in &row_blocks {
                let default_line = String::new();
                let line_content = block.get(h).unwrap_or(&default_line);

                // We need to pad this content to COL_WIDTH_CHARS.
                // But content has ANSI codes!
                // calculate visible length
                let visible_len = if line_content.contains('\x1b') {
                    // Primitive strip for calc: remove 5 chars per block (approx) or use logic
                    // Correct way:
                    let mut len = 0;
                    let mut in_esc = false;
                    for c in line_content.chars() {
                        if c == '\x1b' {
                            in_esc = true;
                        }
                        if !in_esc {
                            len += 1;
                        }
                        if in_esc && c == 'm' {
                            in_esc = false;
                        }
                    }
                    len
                } else {
                    line_content.len()
                };

                full_line.push_str(line_content);
                if COL_WIDTH_CHARS > visible_len {
                    full_line.push_str(&" ".repeat(COL_WIDTH_CHARS - visible_len));
                }
            }
            lines.push(full_line);
        }
        lines.push("".to_string()); // Spacer between shape rows
    }

    lines
}

fn draw_grid(grid: &[Vec<Option<usize>>], legend_lines: &[String], steps: usize) {
    // Clear screen and move cursor to top-left
    print!("\x1b[2J\x1b[1;1H");

    let mut output_lines = Vec::new();

    output_lines.push(format!(
        "Solving Region ({}x{})... Steps: {}",
        grid[0].len(),
        grid.len(),
        steps
    ));
    output_lines.push(format!("+{}", "-".repeat(grid[0].len() * 2)));

    for row in grid {
        let mut line = String::from("|");
        for cell in row {
            match cell {
                Some(id) => {
                    let color = COLORS[id % COLORS.len()];
                    line.push_str(&format!("{}██{}", color, RESET));
                }
                None => line.push_str("  "),
            }
        }
        line.push('|');
        output_lines.push(line);
    }
    output_lines.push(format!("+{}", "-".repeat(grid[0].len() * 2)));

    // Combine grid lines and legend lines side-by-side
    let max_lines = std::cmp::max(output_lines.len(), legend_lines.len());

    // Width padding for grid part
    // The visual width of the grid rows is (grid_width * 2) + 2 (borders)
    let grid_visual_width = grid[0].len() * 2 + 2;

    // We want a safe padding that covers the title too if it's long
    // But title has no ANSI, so .len() works.
    // We'll calculate max len of non-ansi lines to be safe
    let mut max_visual_len = grid_visual_width;
    for line in &output_lines {
        // If line contains ESC, assume it's a grid row with known width
        if !line.contains('\x1b') {
            max_visual_len = std::cmp::max(max_visual_len, line.len());
        }
    }

    let pad_width = max_visual_len + 5;

    for i in 0..max_lines {
        let grid_part = if i < output_lines.len() {
            &output_lines[i]
        } else {
            ""
        };

        let visible_len = if grid_part.contains('\x1b') {
            grid_visual_width
        } else {
            grid_part.len()
        };

        let padding = if pad_width > visible_len {
            pad_width - visible_len
        } else {
            1
        };

        print!("{}", grid_part);
        print!("{}", " ".repeat(padding));

        if i < legend_lines.len() {
            println!("{}", legend_lines[i]);
        } else {
            println!();
        }
    }

    stdout().flush().unwrap();
}

fn get_piece_area(shape: &Shape) -> usize {
    shape.len()
}

fn has_unfillable_hole(
    grid: &[Vec<Option<usize>>],
    min_piece_area: usize,
    width: usize,
    height: usize,
) -> bool {
    let mut visited = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            if grid[y][x].is_none() && !visited[y][x] {
                // Found a new empty region, perform BFS
                let mut size = 0;
                let mut queue = Vec::new();
                queue.push((x, y));
                visited[y][x] = true;

                while let Some((cx, cy)) = queue.pop() {
                    size += 1;

                    // Directions: up, down, left, right
                    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                    for (dx, dy) in dirs {
                        let nx = cx as i32 + dx;
                        let ny = cy as i32 + dy;

                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            let nx = nx as usize;
                            let ny = ny as usize;
                            if grid[ny][nx].is_none() && !visited[ny][nx] {
                                visited[ny][nx] = true;
                                queue.push((nx, ny));
                            }
                        }
                    }
                }

                if size < min_piece_area {
                    // Optimized: found a hole smaller than any remaining piece
                    return true;
                }
            }
        }
    }
    false
}

fn solve_region_visualized(
    width: usize,
    height: usize,
    shapes_orientations: &HashMap<usize, Vec<Shape>>,
    piece_counts: &[usize],
) -> bool {
    // PREPARE LEGEND
    // We only want to show shapes relevant to this puzzle or all?
    // User asked for "all gift shapes", let's show all available definitions or just used ones.
    // Showing all is safer as per "available shapes".
    let legend = generate_legend(shapes_orientations, piece_counts);

    let mut pieces = Vec::new();
    // Also track min area
    let mut min_piece_area = usize::MAX;

    for (id, &count) in piece_counts.iter().enumerate() {
        if count > 0 {
            if let Some(shapes) = shapes_orientations.get(&id) {
                if let Some(first) = shapes.first() {
                    let area = get_piece_area(first);
                    if area < min_piece_area {
                        min_piece_area = area;
                    }
                }
            }
        }
        for _ in 0..count {
            pieces.push(id);
        }
    }

    if pieces.is_empty() {
        return true;
    }

    // Optimization: Sort by size descending
    pieces.sort_by_key(|&id| {
        let len = shapes_orientations
            .get(&id)
            .map(|s| s[0].len())
            .unwrap_or(0);
        Reverse(len)
    });

    let mut grid = vec![vec![None; width]; height];
    let mut steps = 0;

    fn backtrack(
        piece_idx: usize,
        pieces: &[usize],
        grid: &mut Vec<Vec<Option<usize>>>,
        shapes_orientations: &HashMap<usize, Vec<Shape>>,
        width: usize,
        height: usize,
        legend: &[String],
        steps: &mut usize,
        min_piece_area: usize,
    ) -> bool {
        // Optimization: Flood fill pruning
        // If there is any isolated hole smaller than the smallest piece (globally),
        // we can't fill it. Return false immediately.
        if has_unfillable_hole(grid, min_piece_area, width, height) {
            return false;
        }

        if piece_idx >= pieces.len() {
            draw_grid(grid, legend, *steps);
            println!("Solution found!");
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
                    // Check collision
                    for &(sx, sy) in shape {
                        if grid[py + sy as usize][px + sx as usize].is_some() {
                            fits = false;
                            break;
                        }
                    }

                    if fits {
                        // Place piece
                        for &(sx, sy) in shape {
                            grid[py + sy as usize][px + sx as usize] = Some(shape_id);
                        }

                        // Increment step
                        *steps += 1;

                        // VISUALIZATION STEP
                        draw_grid(grid, legend, *steps);
                        // Faster animation for optimized run?
                        thread::sleep(Duration::from_millis(20));

                        if backtrack(
                            piece_idx + 1,
                            pieces,
                            grid,
                            shapes_orientations,
                            width,
                            height,
                            legend,
                            steps,
                            min_piece_area,
                        ) {
                            return true;
                        }

                        // Backtrack (Remove piece)
                        for &(sx, sy) in shape {
                            grid[py + sy as usize][px + sx as usize] = None;
                        }
                    }
                }
            }
        }
        false
    }

    // Initial draw
    draw_grid(&grid, &legend, steps);
    backtrack(
        0,
        &pieces,
        &mut grid,
        shapes_orientations,
        width,
        height,
        &legend,
        &mut steps,
        min_piece_area,
    )
}

fn main() -> Result<()> {
    let lines: Vec<&str> = TEST.lines().collect();
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
                let sl = lines[i];
                if sl.trim().is_empty() {
                    break;
                }
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

    let mut shapes_orientations = HashMap::new();
    for (id, shape) in shapes {
        let set_shape: HashSet<Point> = shape.into_iter().collect();
        shapes_orientations.insert(id, get_all_orientations(&set_shape));
    }

    println!("Start Visualization...");
    thread::sleep(Duration::from_secs(1));

    // Run only for the second region (12x5) which has a known solution
    if let Some((w, h, pieces)) = regions.get(0) {
        if solve_region_visualized(*w, *h, &shapes_orientations, pieces) {
            println!("Solved!");
        } else {
            println!("No solution found.");
        }
    }

    Ok(())
}
