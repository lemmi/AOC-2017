use std::fmt;
use std::collections::HashMap;

type CellType = bool;
type State = char;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
struct Effect {
    value: CellType,
    dir: Direction,
    state: State,
}

impl Effect {
    fn with(value: CellType, dir: Direction, state: State) -> Effect {
        Effect{value, dir, state}
    }
}

struct Turing {
    tape_left: Vec<CellType>,
    tape_right: Vec<CellType>,
    transitions: HashMap<(State, CellType), Effect>,
    state: State,
    value: CellType,
}

impl fmt::Display for Turing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State: {}\n", self.state)?;
        

        for left in self.tape_left.iter() {
            write!(f, " {} ", *left as u8)?;
        }

        write!(f, "[{}]", if self.value { "1" } else { "0" })?;

        for right in self.tape_right.iter().rev() {
            write!(f, " {} ", *right as u8)?;
        }
        Ok(())
    }
}

impl Turing {
    fn new() -> Turing {
        Turing {
            tape_left: Vec::new(),
            tape_right: Vec::new(),
            transitions: HashMap::new(),
            state: '\0',
            value: false,
        }
    }

    fn with_start(state: State) -> Turing {
        Turing {
            state: state,
            .. Turing::new()
        }
    }

    fn step(&mut self) -> bool {
        match self.get_effect() {
            None => false,
            Some(e) => {
                self.apply_effect(e);
                true
            }
        }
    }

    fn get_effect(&self) -> Option<Effect> {
        self.transitions.get(&(self.state, self.value)).cloned()
    }

    fn mv(&mut self, dir: Direction) {
        match dir {
            Direction::Left => {
                self.tape_right.push(self.value);
                self.value = self.tape_left.pop().unwrap_or_default();
            }
            Direction::Right => {
                self.tape_left.push(self.value);
                self.value = self.tape_right.pop().unwrap_or_default();
            }
        }
    }

    fn apply_effect(&mut self, effect: Effect) {
        self.value = effect.value;
        self.mv(effect.dir);
        self.state = effect.state;
    }

    fn checksum(&self) -> usize {
        self.value as usize
            + self.tape_left.iter().map(|&c| c as usize).sum::<usize>()
            + self.tape_right.iter().map(|&c| c as usize).sum::<usize>()
    }
}

fn main() {
    let mut tm = Turing::with_start('A');

    tm.transitions.insert(('A',  false), Effect::with(true  ,Direction::Right, 'B'));
    tm.transitions.insert(('A',  true ), Effect::with(false ,Direction::Left , 'C'));
    tm.transitions.insert(('B',  false), Effect::with(true  ,Direction::Left , 'A'));
    tm.transitions.insert(('B',  true ), Effect::with(true  ,Direction::Left , 'D'));
    tm.transitions.insert(('C',  false), Effect::with(true  ,Direction::Right, 'D'));
    tm.transitions.insert(('C',  true ), Effect::with(false ,Direction::Right, 'C'));
    tm.transitions.insert(('D',  false), Effect::with(false ,Direction::Left , 'B'));
    tm.transitions.insert(('D',  true ), Effect::with(false ,Direction::Right, 'E'));
    tm.transitions.insert(('E',  false), Effect::with(true  ,Direction::Right, 'C'));
    tm.transitions.insert(('E',  true ), Effect::with(true  ,Direction::Left , 'F'));
    tm.transitions.insert(('F',  false), Effect::with(true  ,Direction::Left , 'E'));
    tm.transitions.insert(('F',  true ), Effect::with(true  ,Direction::Right, 'A'));

    for i in 0..12_172_063 {
        if i % 1_000_000 == 0 {
            println!("{}", tm);
        }
        tm.step();
    }

    println!("{}", tm);
    println!("Checksum: {}", tm.checksum());
}
