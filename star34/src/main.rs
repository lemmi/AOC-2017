extern crate aoc;
use aoc:: input;
use std::io;

fn main() {
    let stdin = io::stdin();

    for line in input::lines(&stdin) {
        let steps = line.parse::<usize>().unwrap();

        let mut len = 1;
        let mut pos1 = 0;
        let mut pos = 0;

        for val in 1..50_000_001 {
            pos = 1 + (pos + steps) % len;
            len += 1;
            if pos == 1 {
                pos1 = val;
            }
        }

        println!("{}", pos1)
    }
}
