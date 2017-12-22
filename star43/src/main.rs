extern crate aoc;

use std::io;
use aoc::input;

fn parse_node(c: char) -> Result<bool,&'static str> {
    match c {
        '#' => Ok(true),
        '.' => Ok(false),
        _ => Err("Invalid character"),
    }
}

fn from_origin(g: &Vec<Vec<bool>>, rel: (isize, isize)) -> (usize, usize) {
    let height = g.len();
    let width = g[0].len();

    let offy = (height / 2) as isize;
    let offx = (width / 2) as isize;

    let y = (offy + rel.0) as usize;
    let x = (offx + rel.1) as usize;

    (y,x)
}
fn expand_grid(init: &Vec<Vec<bool>>, size: usize) -> Vec<Vec<bool>>{
    let height = init.len();
    let width = init.iter().map(|r| r.len()).max().unwrap();
    let mheight = 2*size + height;
    let mwidth = 2*size + width;

    let mut ret = vec![vec![false;mwidth];mheight];

    for (i, r) in init.iter().enumerate() {
        ret[size + i][size..(size + r.len())].copy_from_slice(&r);
    }

    ret
}

fn window(g: &Vec<Vec<bool>>) -> (usize, usize, usize, usize) {
    let miny = g.iter().position(|r| r.iter().any(|&c| c)).unwrap();
    let maxy = g.iter().rposition(|r| r.iter().any(|&c| c)).unwrap()+1;

    let minx = g[miny..maxy].iter().map(|r| r.iter().position(|&c| c).unwrap()).min().unwrap();
    let maxx = g[miny..maxy].iter().map(|r| r.iter().rposition(|&c| c).unwrap()).max().unwrap()+1;

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

fn print_grid(g: &Vec<Vec<bool>>, (miny, maxy, minx, maxx): (usize, usize, usize, usize)) {
    for r in &g[miny..maxy] {
        println!("{}", r[minx..maxx].iter().map(|&c| if c { '#' } else { '.' }).collect::<String>());
    }
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
    fn turn(&self, infected: bool) -> Carrier {
        let dir = if infected {
            (self.dir.1, -self.dir.0)
        } else {
            (-self.dir.1, self.dir.0)
        };
        Carrier {
            pos: self.pos,
            dir,
        }
    }
}


fn main() {
    let stdin = io::stdin();
    let init: Result<Vec<Vec<bool>>,_> = input::lines(&stdin).map(|l| l.chars().map(|c| parse_node(c)).collect()).collect();
    let init = init.unwrap_or_else(|e| panic!(e));

    let iterations = 10_000;
    let mut infections = 0usize;
    let mut grid = expand_grid(&init, 100);
    let mut carrier = Carrier::default();
    let mut win = window(&grid);
    print!("\x1B[2J");
    for _ in 0..iterations {
        print!("\x1B[H");

        let (y, x) = from_origin(&grid, carrier.pos);

        win = window_update(win, (y, x));
        print_grid(&grid, win);
        println!("Number of infections: {}", infections);

        let infected = grid[y][x];
        if !infected {
            infections += 1;
        }
        carrier = carrier.turn(infected);
        grid[y][x] = !infected;
        carrier = carrier.mv();

        //std::thread::sleep(std::time::Duration::from_secs(1));
    }

    print!("\x1B[H");
    print_grid(&grid, win);

    println!("Number of infections: {}", infections);
}
