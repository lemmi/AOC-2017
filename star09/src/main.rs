use std::io;
use std::io::BufRead;

fn main() {
    let mut pos = 0isize;
    let mut count = 0isize;
    let stdin = io::stdin();
    let mut table: Vec<isize> = stdin.lock()
        .lines()
        .map(
            |s| s.expect("Oo?")
                .parse()
                .expect("Not a number!")
            )
        .collect();

    while 0 <= pos && pos < table.len() as isize{
        let t = table[pos as usize];
        table[pos as usize] += 1;
        pos += t;
        count += 1;
    }

    println!("{} steps taken", count);
}
