use std::io;
use std::io::BufRead;

fn main() {
    let mut sum = 0i32;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut min = i32::max_value();
        let mut max = i32::min_value();
        for num in line.unwrap().split_whitespace().map(|s| s.parse::<i32>()) {
            match num {
                Ok(x) => {
                    if x < min {
                        min = x;
                    }
                    if x > max {
                        max = x;
                    }
                },
                Err(error) => panic!(error),
            }
        }
        if max < min {
            panic!("{} < {}?", max, min);
        }
        sum += max - min;
    }

    println!("sum: {}", sum);
}
