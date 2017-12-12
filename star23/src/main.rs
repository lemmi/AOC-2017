use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn parse_edges(s: &str) -> Result<(u32, Vec<u32>), String> {
    let splitted: Vec<_> = s.split(" <-> ").collect();
    if splitted.len() != 2 {
        return Err(format!("Wrong number of separators: {}", splitted.len()));
    }
    let node = splitted[0].parse().or(Err("Not a number!"))?;
    let mut connected = Vec::new();
    for other in splitted[1].split(",")
            .map(|s| s.trim())
            .map(|s| s.parse::<u32>()) {
                let other = other.or(Err("Not a number!"))?;
                connected.push(other);
            }
    Ok((node,connected))
}

fn group(edges: &HashMap<u32, Vec<u32>>, start: u32) -> HashSet<u32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(start);

    while let Some(cur) = queue.pop_front() {
        if !visited.insert(cur) {
            continue;
        }
        let candidates = &edges[&cur];
        queue.extend(candidates);
    }
    visited
}

fn main() {
    let stdin = io::stdin();
    let mut edges = HashMap::new();
    for (node, connected) in stdin.lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| parse_edges(&l).expect("Invalid line")) {
            edges.insert(node, connected);
        }

    let visited = group(&edges, 0u32);
    println!("Visited {} nodes", visited.len());
}
