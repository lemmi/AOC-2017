extern crate aoc;

use aoc::input;
use std::io;


fn rotate(v: Vec<u8>, r: usize) -> Vec<u8> {
    let l = v.len();
    v.into_iter().cycle().skip(l-r).take(l).collect()
}

fn pos_pair(s: &str) -> (usize, usize) {
    let mut pos = s.split("/").map(|x| x.parse::<usize>().unwrap());
    (pos.next().unwrap(), pos.next().unwrap())
}
fn ref_pair(s: &str) -> (u8, u8) {
    let mut pos = s.split("/");
    (pos.next().unwrap().as_bytes()[0], pos.next().unwrap().as_bytes()[0])
}

fn main() {
    let stdin = io::stdin();
    for line in input::lines(&stdin) {
        let mut positions = String::from("abcdefghijklmnop").into_bytes();
        for dance_move in line.split(',') {
            match dance_move.split_at(1) {
                ("s",s) => {
                    let shift: usize = s.parse().unwrap();
                    positions = rotate(positions,shift);
                },
                ("x",x) => {
                    let (x,y) = pos_pair(x);
                    positions.swap(x,y);
                },
                ("p",p) => {
                    let (xname,yname) = ref_pair(p);
                    let x = positions.iter().position(|&x| x == xname).unwrap();
                    let y = positions.iter().position(|&y| y == yname).unwrap();
                    positions.swap(x,y);
                },
                (e,_) => {
                    println!("Unknown move \"{}\"", e);
                    break;
                },

            }
            println!("{}: {:?}", dance_move, String::from_utf8(positions.clone()).unwrap());
        }

        println!("Final position: {:?}", String::from_utf8(positions).unwrap());
    }
}
