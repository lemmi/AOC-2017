extern crate aoc;
use std::io;
use aoc::input::lines;
use aoc::hash::Knot;
use std::str::FromStr;

fn popcnt(k: &Knot) -> u32 {
    k.iter().fold(0, |sum, c| sum + c.count_ones())
}

fn main() {
    let stdin = io::stdin();
    for key in lines(&stdin) {
        let mut sum = 0u32;
        for line in 0..128 {
            let line_key = format!("{}-{}", key, line);
            let k = Knot::from_str(&line_key).unwrap();
            let cnt = popcnt(&k);
            println!("{} {} {}", k, line_key, cnt);
            sum += cnt;
        }
        println!("Number of bits: {}", sum);
    }
}
