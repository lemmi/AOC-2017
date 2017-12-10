use std::io;
use std::io::BufRead;

fn reverse_wrapped(s: &mut[usize], pos: usize, length: usize) {
    let mut l = pos;
    let mut r = pos + length - 1;

    while l < r {
        s.swap(l%256, r%256);

        l += 1;
        r -= 1;
    }
}

fn knot_hash(input: &[usize]) -> [usize; 256] {
    let mut hash = [0usize; 256];
    for (i, h) in hash.iter_mut().enumerate() {
        *h = i as usize;
    }

    let mut pos = 0;
    let mut skip = 0;

    for l in input {
        reverse_wrapped(&mut hash, pos, *l);

        pos += l + skip;
        skip += 1;
    }

    hash
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if line.trim().len() == 0 {
            continue;
        }
        let input: Vec<usize> = line.split(',')
            .map(|s| s.parse().expect("Not a number"))
            .collect();
        let hash = knot_hash(&input);
        for chunk in hash.chunks(32) {
            println!("{:?}", chunk);
        }
        println!("Answer: {}", hash[0] * hash[1]);
    }
}
