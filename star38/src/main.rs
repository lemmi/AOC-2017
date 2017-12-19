use std::str;
use std::io;
use std::io::Read;
use std::ops::Add;

struct Pipes {
    data: Vec<u8>,
    height: usize,
    width: usize,
}

impl Pipes {
    fn new(data: String) -> Pipes {
        let v: Vec<_> = data.lines().map(|l| l.len()+1).collect();
        let min = v.iter().min();
        let max = v.iter().max();
        if min != max {
            panic!("Expected equal widths");
        }
        let width = *min.unwrap();
        let height = v.len();
        let data = data.into_bytes();
        Pipes{
            data,
            height,
            width,
        }
    }

    fn in_range(&self, v: V2) -> Option<V2> {
        if v.y >= self.height as isize || 0 > v.y {
            None
        } else if v.x >= self.width as isize || 0 > v.x {
            None
        } else {
            Some(v)
        }
    }

    fn get(&self, v: V2) -> Option<u8> {
        let v = self.in_range(v)?;
        let idx = (v.y as usize)*self.width + v.x as usize;
        self.data.get(idx).cloned()
    }

    fn start(&self) -> Option<V2> {
        let x = self.data[0..self.width].iter().position(|&c| c == b'|')?;
        Some(V2::new(0,x as isize))
    }
}

#[derive(Copy,Clone,Default,Eq,PartialEq,Debug)]
struct V2 {
    y: isize,
    x: isize,
}

impl V2 {
    fn new(y: isize, x: isize) -> V2 {
        V2{y, x}
    }
    fn rr(self) -> V2 {
        V2::new(self.x, -self.y)
    }
    fn rl(self) -> V2 {
        V2::new(-self.x, self.y)
    }
}

impl Add for V2 {
    type Output = V2;

    fn add(self, other: V2) -> V2 {
        V2::new(self.y + other.y, self.x + other.x)
    }
}
impl<'a> Add for &'a V2 {
    type Output = V2;

    fn add(self, other: &V2) -> V2 {
        V2::new(self.y + other.y, self.x + other.x)
    }
}

fn is_alpha(c: u8) -> bool {
    c >= b'A' && c <= b'Z' || c >= b'a' && c <= b'z'
}

#[derive(Debug)]
struct Packet {
    pos: V2,
    dir: V2,
}

impl Packet {
    fn new(pos: V2, dir: V2) -> Packet {
        Packet{pos, dir}
    }

    fn step(&self, pipes: &Pipes) -> Option<Packet> {
        let dir = match pipes.get(self.pos)? {
            b'-' | b'|' => Some(self.dir),
            b'+' => {
                if let Some(_) = Packet::new(self.pos + self.dir.rl(), self.dir.rl()).step(pipes) {
                    Some(self.dir.rl())
                } else if let Some(_) = Packet::new(self.pos + self.dir.rr(), self.dir.rr()).step(pipes) {
                    Some(self.dir.rr())
                } else {
                    None
                }
            }
            c if is_alpha(c) => {
                print!("{}", c as char);
                Some(self.dir)
            },
            c => {
                None
            },
        }?;
        Some(Packet::new(self.pos + dir, dir))
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let p = Pipes::new(input);
    let mut packet = Packet::new(p.start().unwrap(), V2::new(1,0));

    for line in p.data.chunks(p.width) {
        print!("{}", str::from_utf8(line).unwrap());
    }
    let mut n = 0usize;
    while let Some(p) = packet.step(&p) {
        packet = p;
        n += 1;
    }
    println!();
    println!("{}", n);
}
