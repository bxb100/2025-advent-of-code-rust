use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const TEST2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>()
            .join("\n");
        let (adj, name_to_id) = parse_graph(&input);
        // Helper to safely get ID or return None if node doesn't exist
        let start = name_to_id.get("you").unwrap();
        let end = name_to_id.get("out").unwrap();

        let count = count_paths(*start, *end, &adj);
        Ok(count as usize)
    }

    assert_eq!(5, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>()
            .join("\n");
        let (adj, name_to_id) = parse_graph(&input);
        let get_id = |name: &str| name_to_id.get(name).copied();

        if let (Some(svr), Some(out), Some(dac), Some(fft)) =
            (get_id("svr"), get_id("out"), get_id("dac"), get_id("fft"))
        {
            // Check Path A: svr -> dac -> fft -> out
            let path_a = count_paths(svr, dac, &adj)
                * count_paths(dac, fft, &adj)
                * count_paths(fft, out, &adj);

            // Check Path B: svr -> fft -> dac -> out
            let path_b = count_paths(svr, fft, &adj)
                * count_paths(fft, dac, &adj)
                * count_paths(dac, out, &adj);

            return Ok(path_a as usize + path_b as usize);
        }
        Err(anyhow!(
            "无组合 {:?}, {:?}, {:?}, {:?}",
            get_id("svr"),
            get_id("out"),
            get_id("dac"),
            get_id("fft")
        ))
    }

    assert_eq!(2, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

/// Memoized DFS to count paths from src to target
fn count_paths(src: usize, target: usize, adj: &Vec<Vec<usize>>) -> u64 {
    // Memoization cache: Stores known path counts for each node index
    let mut memo: Vec<Option<u64>> = vec![None; adj.len()];
    dfs(src, target, adj, &mut memo)
}

fn dfs(u: usize, target: usize, adj: &Vec<Vec<usize>>, memo: &mut Vec<Option<u64>>) -> u64 {
    // Base case: We reached the target
    if u == target {
        return 1;
    }
    // Check cache
    if let Some(count) = memo[u] {
        return count;
    }

    let mut total_paths = 0;
    for &v in &adj[u] {
        total_paths += dfs(v, target, adj, memo);
    }

    // Cache and return
    memo[u] = Some(total_paths);
    total_paths
}

/// Parses the input string into an Adjacency List (Vec<Vec<usize>>)
/// Returns the graph and the name-to-id mapping.
fn parse_graph(input: &str) -> (Vec<Vec<usize>>, HashMap<String, usize>) {
    let mut name_to_id: HashMap<String, usize> = HashMap::new();
    let mut next_id = 0;

    // First pass: Assign IDs to all nodes
    // We need to parse lines to find all unique names first
    let mut temp_edges: Vec<(&str, Vec<&str>)> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        let src_name = parts[0].trim();
        let neighbors: Vec<&str> = parts[1].split_whitespace().collect();

        if !name_to_id.contains_key(src_name) {
            name_to_id.insert(src_name.to_string(), next_id);
            next_id += 1;
        }
        for &n in &neighbors {
            if !name_to_id.contains_key(n) {
                name_to_id.insert(n.to_string(), next_id);
                next_id += 1;
            }
        }
        temp_edges.push((src_name, neighbors));
    }

    // Build Adjacency List
    let mut adj = vec![Vec::new(); next_id];
    for (src, neighbors) in temp_edges {
        let u = name_to_id[src];
        for n in neighbors {
            let v = name_to_id[n];
            adj[u].push(v);
        }
    }

    (adj, name_to_id)
}
