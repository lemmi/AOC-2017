extern crate aoc;

use std::fmt;
use std::convert;
use std::io;
use aoc::input;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl fmt::Display for NodeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            NodeState::Clean    => '.',
            NodeState::Weakened => 'W',
            NodeState::Infected => '#',
            NodeState::Flagged  => 'F',
        };
        write!(f, "{}", c)
    }
}

impl convert::From<char> for NodeState {
    fn from(c: char) -> Self {
        match c {
            '.' => NodeState::Clean,
            'W' => NodeState::Weakened,
            '#' => NodeState::Infected,
            'F' => NodeState::Flagged,
            _ => panic!("Unexpected character for NodeState"),
        }
    }
}

impl NodeState {
    fn visit(&self) -> NodeState {
        match *self {
            NodeState::Clean    => NodeState::Weakened,
            NodeState::Weakened => NodeState::Infected,
            NodeState::Infected => NodeState::Flagged,
            NodeState::Flagged  => NodeState::Clean,
        }
    }
}

fn from_origin(g: &Vec<Vec<NodeState>>, rel: (isize, isize)) -> (usize, usize) {
    let height = g.len();
    let width = g[0].len();

    let offy = (height / 2) as isize;
    let offx = (width / 2) as isize;

    let y = (offy + rel.0) as usize;
    let x = (offx + rel.1) as usize;

    (y,x)
}
fn expand_grid(init: &Vec<Vec<NodeState>>, size: usize) -> Vec<Vec<NodeState>>{
    let height = init.len();
    let width = init.iter().map(|r| r.len()).max().unwrap();
    let mheight = 2*size + height;
    let mwidth = 2*size + width;

    let mut ret = vec![vec![NodeState::Clean; mwidth];mheight];

    for (i, r) in init.iter().enumerate() {
        ret[size + i][size..(size + r.len())].copy_from_slice(&r);
    }

    ret
}

fn window(g: &Vec<Vec<NodeState>>) -> (usize, usize, usize, usize) {
    let miny = g.iter().position(|r| r.iter().any(|&c| c != NodeState::Clean)).unwrap();
    let maxy = g.iter().rposition(|r| r.iter().any(|&c| c != NodeState::Clean)).unwrap()+1;

    let minx = g[miny..maxy].iter().map(|r| r.iter().position(|&c| c != NodeState::Clean).unwrap()).min().unwrap();
    let maxx = g[miny..maxy].iter().map(|r| r.iter().rposition(|&c| c != NodeState::Clean).unwrap()).max().unwrap()+1;

    (miny, maxy, minx, maxx)
}

fn window_update((miny, maxy, minx, maxx): (usize, usize, usize, usize), (posy, posx): (usize, usize)) -> (usize, usize, usize, usize) {
    (
        miny.min(posy),
        maxy.max(posy+1),
        minx.min(posx),
        maxx.max(posx+1),
        )
}

fn print_grid(g: &Vec<Vec<NodeState>>, (miny, maxy, minx, maxx): (usize, usize, usize, usize)) {
    for r in &g[miny..maxy] {
        for c in &r[minx..maxx] {
            print!("{}", c);
        }
        println!();
    }
}
fn need_expand(g: &Vec<Vec<NodeState>>, (miny, maxy, minx, maxx): (usize, usize, usize, usize)) -> bool {
    miny == 0 || maxy == g.len() || minx == 0 || maxx == g[0].len()
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
struct Carrier {
    pos: (isize, isize),
    dir: (isize, isize),
}

impl Default for Carrier {
    fn default() -> Self {
        Carrier {
            pos: (0,0),
            dir: (-1,0),
        }
    }
}

impl Carrier {
    fn mv(&self) -> Carrier {
        Carrier {
            pos: (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1),
            dir: self.dir,
        }
    }
    fn turn(&self, state: &NodeState) -> Carrier {
        let dir = match *state {
            NodeState::Clean    => (-self.dir.1, self.dir.0),
            NodeState::Weakened => self.dir,
            NodeState::Infected => (self.dir.1, -self.dir.0),
            NodeState::Flagged  => (-self.dir.0, -self.dir.1),
        };
        Carrier {
            pos: self.pos,
            dir,
        }
    }
}


fn main() {
    let stdin = io::stdin();
    let init: Vec<Vec<NodeState>> = input::lines(&stdin).map(|l| l.chars().map(|c| NodeState::from(c)).collect()).collect();

    let iterations = 10_000_000;
    let mut infections = 0usize;
    let mut grid = expand_grid(&init, init.len());
    let mut carrier = Carrier::default();
    let mut win = window(&grid);
    print!("\x1B[2J");
    for i in 0..iterations {
        if need_expand(&grid, win) {
            grid = expand_grid(&grid, 100);
            win = window(&grid);
        }
        let (y, x) = from_origin(&grid, carrier.pos);
        let state = grid[y][x];
        let next_state = state.visit();

        win = window_update(win, (y, x));
        if i % 1_000_000 == 0 {
            println!("Number of infections: {} {:?}", infections, win);
        }

        if next_state == NodeState::Infected {
            infections += 1;
        }
        carrier = carrier.turn(&state);
        grid[y][x] = next_state;
        carrier = carrier.mv();
    }

    println!("Number of infections: {}", infections);
}
