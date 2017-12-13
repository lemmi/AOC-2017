use std::u32;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Copy,Clone,Debug,Default,PartialEq,Eq)]
struct Scanner {
    depth: u32,
    range: u32,
}

impl Scanner {
    fn pos(&self, t: u32) -> u32 {
        match self.range {
            0 => u32::MAX,
            1 => 0,
            r => {
                let period = (2 * r).saturating_sub(2); // TODO 
                let slot = t % period;
                slot - 2*slot.saturating_sub(r-1)
            },
        }
    }
    fn hit(&self, t: u32) -> bool {
        self.pos(t) == 0
    }
    fn hitcost(&self) -> u32 {
        self.depth * self.range
    }
    fn cost(&self, t: u32) -> Option<u32> {
        match self.hit(t) {
            true => Some(self.hitcost()),
            false => None,
        }
    }
    fn new(d:u32, r:u32) -> Scanner {
        Scanner{depth: d, range: r}
    }
}

impl FromStr for Scanner {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let mut v = s.split(':').map(|x| x.trim().parse::<u32>());
        let depth = v.next().unwrap().or(Err("Not a number"))?;
        let range = v.next().unwrap().or(Err("Not a number"))?;
        if let Some(_) = v.next() {
            Err("Too many elements")
        } else {
            Ok(Scanner::new(depth, range))
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let scanners: Vec<_> = stdin.lock().lines().map(|l| l.unwrap().parse::<Scanner>().unwrap()).collect();
    for delay in 0.. {
        let mut cost = 0u32;
        let mut caught = false;
        for s in &scanners {
            let t = s.depth + delay;
            if let Some(c) = s.cost(t) {
                cost += c;
                caught = true;
            }
        }
        if cost == 0 && !caught {
            print!("\rd:{:8} cost:{:5}", delay, cost);
            println!(" {}", caught);
            break;
        }
    }
}
