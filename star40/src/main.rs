extern crate aoc;

use std::io;
use std::ops::Add;
use std::ops::Sub;
use std::cmp::Ordering;
use std::str::FromStr;
use aoc::input;

#[derive(Copy,Clone,Eq,PartialEq,Default,Debug)]
struct V3 {
    x: i64,
    y: i64,
    z: i64,
}


impl V3 {
    fn new(x:i64, y:i64, z:i64) -> V3 {
        V3{x,y,z}
    }

    fn length(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Sub for V3 {
    type Output = V3;

    fn sub(self, other: V3) -> V3 {
        V3::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}
impl Add for V3 {
    type Output = V3;

    fn add(self, other: V3) -> V3 {
        V3::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Ord for V3 {
    fn cmp(&self, other: &V3) -> Ordering {
        self.length().cmp(&other.length())
    }
}
impl PartialOrd for V3 {
    fn partial_cmp(&self, other: &V3) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for V3 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<i64> = s.trim_matches(&['<','>',' '][..]).split(',').map(|n| n.parse::<i64>().unwrap()).collect();
        if v.len() != 3 {
            Err("Invalid number of elements")
        } else {
            Ok(V3::new(v[0],v[1],v[2]))
        }
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Default,Debug)]
struct Particle {
    pos: V3,
    vel: V3,
    acc: V3,
}

impl Particle {
    fn new(pos: V3, vel: V3, acc: V3) -> Particle {
        Particle{pos,vel,acc}
    }

    fn step(&self) -> Particle {
        let vel = self.vel + self.acc;
        let pos = self.pos + vel;
        Particle::new(pos,vel,self.acc)
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Particle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Particle {
    fn cmp(&self, other: &Particle) -> Ordering {
        self.acc.cmp(&other.acc)
            .then(self.vel.cmp(&other.vel))
            .then(self.pos.cmp(&other.pos))
    }
}

impl FromStr for Particle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<V3> = s.split(", ")
            .map(|n| n.split('=').skip(1).next().unwrap())
            .map(|p| p.parse::<V3>().unwrap()).collect();
        if v.len() != 3 {
            Err("Invalid number of elements")
        } else {
            Ok(Particle::new(v[0],v[1],v[2]))
        }
    }
}

fn step(particles: &mut Vec<Particle>) {
    for p in particles.iter_mut() {
        *p = p.step();
    }
}

fn remove_collitions(particles: &Vec<Particle>) -> (Vec<Particle>, Vec<Particle>) {
    particles.iter().partition(|&p1| particles.iter().filter(|&p2| p1.pos == p2.pos).count() == 1)
}

fn dist_matrix(particles: &Vec<Particle>) -> Vec<i64> {
    let mut ret = Vec::with_capacity(particles.len()*particles.len());
    for (i,p) in particles.iter().enumerate() {
        ret.extend(particles.iter().skip(i+1).map(|p2| (p.pos - p2.pos).length()))
    }
    ret
}

fn main() {
    let stdin = io::stdin();

    let mut particles: Vec<Particle> = input::lines(&stdin).map(|l| l.parse().unwrap()).collect();
    let mut last_dists = dist_matrix(&particles);

    loop {
        println!("Particles left: {}", particles.len());
        step(&mut particles);
        let distance = dist_matrix(&particles);
        if last_dists.len() == distance.len() && distance.iter().ge(last_dists.iter()) {
            break;
        }
        let (saved, removed) = remove_collitions(&particles);
        last_dists = distance;
        particles = saved;
        println!("{:?} {}", removed, last_dists.iter().min().unwrap());

    }

    println!();
    println!("Particles left: {}", particles.len());
}
