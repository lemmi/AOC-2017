use std::ops;
use std::cmp;

// (y, x) ---->
//  |
//  |
//  |
//  v

#[derive(Debug,Copy,Clone)]
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

impl ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        self + (-other)
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

fn map_from_odd(base: i32) -> i32 {
    let t = 2*base + 1;
    t*t
}
fn map_from_even(base: i32) -> i32 {
    4*base*base
}
fn map_from_spiral(p: &Point) -> i32 {
    if p.y == p.x && p.y >= 0 {
        map_from_odd(p.y)
    } else if p.y == p.x - 1 && p.y < 0 {
        map_from_even(p.y)
    } else if p.y - p.x < 0 {
        let base = cmp::max(-p.y, p.x);
        let origin = map_from_odd(base-1);
        let remain = Point{y: base, x: base} - *p;
        origin + remain.y.abs() + remain.x.abs()
    } else if p.y - p.x >= 0 {
        let base = cmp::max(p.y, -p.x);
        let origin = map_from_even(base);
        let remain = -Point{y: base+1, x: base} - *p;
        origin + remain.y.abs() + remain.x.abs()
    } else {
        unreachable!("We shouldn't be here");
    }
}

fn main() {
    let mut v: Vec<i32> = vec![1];
    let p: Vec<Point> = (2..10).map(|x| map_to_spiral(x)).collect();
    
    for i in 2..100 {
        let mut sum = 0;

        let coords = map_to_spiral(i);
        for neigh in &p {
            let t = map_from_spiral(&(coords + *neigh));
            if t < i {
                let part = v[(t - 1) as usize];
                sum += part;
            }
        }

        v.push(sum);
        println!("{:3}: {:6}, ({:?})", i, sum, coords);
        if sum > 289326 {
            break;
        }
    }
}
