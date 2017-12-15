extern crate aoc;

use aoc::input;
use std::io;

#[derive(Copy,Clone,Debug)]
struct Generator {
    state: u64,
    fac: u64,
    mask: u64,
}

impl Iterator for Generator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        loop {
            self.state *= self.fac;
            self.state %= 2147483647;
            if self.state & self.mask == 0 {
                break
            }
        }
        Some(self.state)
    }
}

impl Generator {
    fn new(state: u64, fac: u64, mask: u64) -> Generator {
        Generator{state: state, fac: fac, mask: mask}
    }
}

fn main() {
    let stdin = io::stdin();
    let inputs: Vec<_> = input::lines(&stdin)
        .take(2)
        .map(|l| l.split_whitespace()
             .last()
             .unwrap()
             .to_owned()
             .parse::<u64>()
             .unwrap()
            ).collect();

    let gen_a = Generator::new(inputs[0], 16807, 4-1);
    let gen_b = Generator::new(inputs[1], 48271, 8-1);

    let mut count = 0u64;
    for (i, (s_a, s_b)) in gen_a.zip(gen_b).enumerate() {
        if i == 5_000_000 {
            break;
        }

        if s_a & 0xffff != s_b & 0xffff {
            continue;
        }

        count += 1;
        println!("{:6}: {:10} == {:10}", count, s_a, s_b);
    }
}
