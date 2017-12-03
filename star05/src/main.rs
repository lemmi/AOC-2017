use std::io;
use std::io::BufRead;
use std::ops;

// (y, x) ---->
//  |
//  |
//  |
//  v

#[derive(Debug)]
struct Point {
    y: i32,
    x: i32,
}

impl ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            y: self.y+other.y,
            x: self.x+other.x,
        }
    }
}

impl ops::Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point{
            y: -self.y,
            x: -self.x,
        }
    }
}

fn map_to_spiral(n:i32) -> Point {
    if n < 0 {
        return Point{y:0, x:0};
    }

    let base = (n as f32).sqrt().floor() as i32;
    let remain = n - base*base;
    let odd = base & 1 > 0;

    let origin = if odd {
        Point{y:base / 2, x:base / 2}
    } else {
        Point{y:base / 2, x:base / 2 - 1}
    };

    let coord = if remain == 0 {
        origin
    } else if remain <= base {
        let offset = Point{y: 1-remain, x: 1};
        origin + offset
    } else if remain <= base * 2 {
        let offset = Point{y: -base, x: 1};
        origin + offset + Point{y: 0, x: 1-(remain - base)}
    } else {
        unreachable!("We shouldn't be here");
    };

    if odd {
        coord
    } else {
        -coord
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let n: i32 = line.expect("Oo?").parse().expect("Not a number!");
        let coords = map_to_spiral(n);
        println!("location: {:?}", coords);
        println!("distance: {:?}", coords.y.abs() + coords.x.abs());

    }
}
