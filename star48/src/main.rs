extern crate aoc;

use std::fmt;
use std::cmp;
use std::str::FromStr;
use std::io;
use aoc::input;

type Port = u32;

#[derive(Copy,Clone,Eq,PartialEq,Default)]
struct Component (Port, Port);

impl Component {
    //fn with_ports(from: Port, to: Port) -> Component {
    //    Component(from, to)
    //}

    fn low(&self) -> Port {
        cmp::min(self.0, self.1)
    }
    fn high(&self) -> Port {
        cmp::max(self.0, self.1)
    }
    fn strength(&self) -> u32 {
        self.0 + self.1
    }
    fn can_connect(&self, other: Port) -> bool {
        self.0 == other || self.1 == other
    }
    fn get_other(&self, other: Port) -> Port {
        if other == self.0 {
            self.1
        } else {
            self.0
        }
    }
}

impl fmt::Debug for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}
impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl FromStr for Component {
    type Err = std::num::ParseIntError; 
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: Vec<_> = s.split("/").map(str::parse::<Port>).take(2).collect::<Result<_,_>>()?;
        Ok(Component(
                p.iter().min().cloned().unwrap(),
                p.iter().max().cloned().unwrap(),
                ))
    }
}

impl PartialOrd for Component {
    fn partial_cmp(&self, other: &Component) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Component {
    fn cmp(&self, other: &Component) -> cmp::Ordering {
        cmp::Ordering::Equal
            .then(self.strength().cmp(&other.strength()))
            .then(self.low().cmp(&other.low()))
            .then(self.high().cmp(&other.high()))
    }
}

fn bridge_strength(bridge: &[Component]) -> u32 {
    bridge.iter().map(Component::strength).sum()
}

fn build_bridge(components: &[Component], bridge: &[Component], connector: Port) -> (Vec<Component>, u32) {
    let mut local = bridge.to_vec();
    let mut best = bridge.to_vec();
    let mut best_strength = bridge_strength(bridge);
    for candidate in components.iter().filter(|c| c.can_connect(connector)).filter(|c| !bridge.contains(c)) {
        local.push(*candidate);
        let (b, s) = build_bridge(components, &local[..], candidate.get_other(connector));
        local.pop();
        if b.len() > best.len() || b.len() == best.len() && s > best_strength {
            best = b;
            best_strength = s;
        }
    }
    if local.len() < 5 {
        println!("{:4} {:?}", best_strength, best);
    }
    (best, best_strength)
}

fn main() {
    let stdin = io::stdin();
    let mut v: Vec<_> = input::lines(&stdin).map(|s| s.parse::<Component>().unwrap()).collect();
    v.sort();
    v.reverse();
    let (b, s) = build_bridge(&v, &[Component(0,0)], 0);
    println!();
    println!("Longest bridge: {:4} {} {:?}", s, b.len()-1, b);
}
