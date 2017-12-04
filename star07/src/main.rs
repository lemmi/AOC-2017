use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn check_pass(pw: &String) -> bool {
    let mut set = HashSet::new();
    for word in pw.split_whitespace() {
        if set.insert(word) == false {
            return false
        }
    }
    true
}

fn main() {
    let mut num_valid = 0u32;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Oo?");
        
        if line.trim().len() == 0 {
            continue;
        }

        let valid = check_pass(&line);

        println!("{:5}: {}", valid, line);
        if valid {
            num_valid += 1;
        }
    }
    println!("Number of valid passwords: {}", num_valid);
}
