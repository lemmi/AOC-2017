extern crate aoc;
use aoc:: input;
use std::io;

fn main() {
    let stdin = io::stdin();

    for line in input::lines(&stdin) {
        let steps = line.parse::<usize>().unwrap();

        let mut v = vec![0];
        let mut pos = 0;

        for val in 1..2018 {
            pos = (pos + 1 + steps) % v.len();
            v.insert(pos, val);
            println!("{} {:?}", val, v);
        }

        println!("{}", v[(pos+1) % v.len()]);
    }
}
