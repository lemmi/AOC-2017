use std::io;
use std::io::BufRead;
use std::collections::BTreeSet;

#[derive(Copy,Clone,PartialEq,Eq,Debug,PartialOrd,Ord,Default)]
struct State {
    banks: [u8; 16],
}

impl From<Vec<u8>> for State {
    fn from(s: Vec<u8>) -> State {
        let mut ret = State::default();
        for (from, to) in s.iter().zip(ret.banks.iter_mut()) {
            *to = *from;
        }
        ret
    }
}

impl State {
    fn find_max(&self) -> usize {
        let mut max = self.banks[0];
        let mut max_index = 0usize;

        for (i, bank) in self.banks.iter().enumerate() {
            if *bank > max {
                max = *bank;
                max_index = i;
            }
        }

        max_index
    }
}

impl Iterator for State {
    type Item = State;

    fn next(&mut self) -> Option<State> {
        let mut idx = self.find_max();
        let mut n = self.banks[idx];

        self.banks[idx] = 0;
        while n > 0 {
            idx = (idx + 1) % self.banks.len();
            self.banks[idx] += 1;
            n -= 1;
        }

        Some(self.clone())
    }
}

fn main() {
    let stdin = io::stdin();
    let start: Vec<u8> = stdin.lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(
            |s| s.parse()
                .expect("Not a number!")
            )
        .collect();

    let start = State::from(start);

    let mut states = BTreeSet::new();
    let mut history = Vec::new();
    states.insert(start);
    history.push(start);

    for state in start {
        println!("{:?}", state);
        history.push(state);
        if !states.insert(state) {
            break;
        }
    }
    let last = history.last().unwrap();
    let pos = history.iter().position(|s| *s == *last).unwrap();
    println!("Found duplicate! {}", states.len() - pos);

}
