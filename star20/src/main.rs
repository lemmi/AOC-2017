use std::io;
use std::io::BufRead;

fn reverse_wrapped(s: &mut[u8], pos: usize, length: usize) {
    let mut l = pos;
    let mut r = pos + length - 1;

    while l < r {
        s.swap(l%256, r%256);

        l += 1;
        r -= 1;
    }
}

fn knot_hash(input: &[u8]) -> [u8; 256] {
    let mut hash = [0u8; 256];
    for (i, h) in hash.iter_mut().enumerate() {
        *h = i as u8;
    }

    let mut pos = 0;
    let mut skip = 0;
    let postfix = [17, 31, 73, 47, 23];

    for _ in 0..64 {
        for l in input.iter().chain(&postfix) {
            let l = *l as usize;
            reverse_wrapped(&mut hash, pos, l);

            pos = (pos +l + skip) % 256;
            skip = (skip + 1) % 256;
        }
    }

    hash
}

fn make_dense(input: &[u8]) -> [u8; 16] {
    let mut ret = [0u8; 16];
    for (i, chunk) in input.chunks(16).enumerate() {
        ret[i] = chunk.iter().fold(0, |sum, x| sum ^ x);
    }
    ret
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if line.trim().len() == 0 {
            continue;
        }
        let hash = knot_hash(line.as_bytes());
        let dense = make_dense(&hash);
        for chunk in hash.chunks(32) {
            println!("{:?}", chunk);
        }
        for c in &dense {
            print!("{:02x}", c);
        }
        println!();
    }
}
