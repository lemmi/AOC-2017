extern crate aoc;
use std::io;
use aoc::input::lines;
use aoc::hash::Knot;
use aoc::graph::Implicit;

use std::str::FromStr;

fn popcnt(k: &Knot) -> u32 {
    k.iter().fold(0, |sum, c| sum + c.count_ones())
}

fn explode_u8(u: u8) -> [bool; 8] {
    let mut ret = [false; 8];
    for bit in 0..8 {
        let bitmask = 0x80 >> bit;
        ret[bit] = u & bitmask > 0
    }
    ret
}
fn explode(k: &Knot) -> Vec<bool> {
    let mut ret = Vec::with_capacity(128);
    for c in k.iter() {
        let bits = explode_u8(*c);
        ret.extend(&bits);
    }
    ret
}
fn grid(key: &str) -> Vec<bool> {
    let mut ret = Vec::new();
    for line in 0..128 {
        let line_key = format!("{}-{}", key, line);
        let k = Knot::from_str(&line_key).unwrap();
        ret.extend(explode(&k));
    }
    ret
}

fn clamp(x: isize) -> Option<usize> {
    if x >= 0 && x < 128 {
        Some(x as usize)
    } else {
        None
    }
}

fn map_grid_id(grid: &Vec<bool>, x: isize, y: isize) -> Option<usize> {
    let idx = clamp(y)? * 128 + clamp(x)?;
    if grid[idx] {
        Some(idx)
    } else {
        None
    }
}

fn neighbours(grid: &Vec<bool>, id: usize) -> [Option<usize>;4] {
    let mut ret = [None; 4];
    let x = (id % 128) as isize;
    let y = (id / 128) as isize;

    ret[0] = map_grid_id(grid, x + 1, y); 
    ret[1] = map_grid_id(grid, x - 1, y); 
    ret[2] = map_grid_id(grid, x, y + 1); 
    ret[3] = map_grid_id(grid, x, y - 1); 

    ret
}

fn build_graph(grid: &Vec<bool>) -> Implicit<usize> {
    let mut g = Implicit::new();

    for pos in 0..grid.len() {
        if !grid[pos] {
            continue;
        }
        g.insert(pos, None);
        for neigh in &neighbours(grid, pos) {
            g.insert(pos, *neigh);
        }
    }

    g
}

fn printgrid(grid: &Vec<bool>) {
    for chunk in grid.chunks(128) {
        let s: String = chunk.iter().map(|&b| if b { '#' } else { '.' }).collect();
        println!("{}", s);
    }
}

fn main() {
    let stdin = io::stdin();
    for key in lines(&stdin) {
        let g = grid(&key);
        printgrid(&g);
        let graph = build_graph(&g);
        println!("Number of regions: {}", graph.num_groups());
    }
}
