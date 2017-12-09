use std::io;
use std::io::BufRead;

#[derive(Copy,Clone,Debug)]
enum State {
    Normal,
    NormalIgnore,
    Garbage,
    GarbageIgnore,
}
impl Default for State {
    fn default() -> State {
        State::Normal
    }
}

#[derive(Copy,Clone,Default,Debug)]
struct Parser {
    score: u32,
    depth: u32,
    garbage: u32,
    state: State,
}

impl Parser {
    fn consume(&mut self, c: char) -> Option<String> {
        match self.state {
            State::Normal => {
                match c {
                    '{' => {
                        self.depth += 1;
                        self.score += self.depth;
                    },
                    '}' => {
                        self.depth -= 1;
                    },
                    ',' => {},
                    '!' => {
                        self.state = State::NormalIgnore;
                    },
                    '<' => {
                        self.state = State::Garbage;
                    },
                    c => return Some(format!("Unexpected char \"{}\"", c)),
                }
            },
            State::Garbage => {
                match c {
                    '>' => self.state = State::Normal,
                    '!' => self.state = State::GarbageIgnore,
                    _ => self.garbage += 1,
                }
            },
            State::NormalIgnore => self.state = State::Normal,
            State::GarbageIgnore => self.state = State::Garbage,
        }
        None
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if line.trim().len() == 0 {
            continue;
        }
        let mut p = Parser::default();
        for c in line.chars() {
            p.consume(c);
        }
        println!("{:?}", p);
    }
}
