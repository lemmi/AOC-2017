use std::io;
use std::io::BufRead;
use std::cmp;

fn main() {
    let mut sum = 0i32;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let v: Vec<i32> = line.unwrap().split_whitespace().map(|s| s.parse::<i32>().expect("Not a number!")).collect();
        let mut iter = v.iter();
        for x1 in v.iter() {
            if iter.next() == None { break; }
            for x2 in iter.clone() {
                let (x1, x2): (i32, i32) = (*cmp::max(x1, x2), *cmp::min(x1,x2));
                let div = x1 / x2;
                if div > 0 {
                    if div * x2 == x1 {
                        sum += div;
                    }
                }
            }
        }
    }

    println!("sum: {}", sum);
}
