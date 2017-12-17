extern crate aoc;

use std::str;
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

fn do_moves(v: &str, moves: &str) -> (Vec<u8>, Vec<u8>) {
    let from = Vec::from(v);
    let mut positional_swaps = from.clone();
    let mut renames = from.clone();

    for dance_move in moves.split(',') {
        match dance_move.split_at(1) {
            ("s",s) => {
                let shift: usize = s.parse().unwrap();
                positional_swaps = rotate(positional_swaps, shift);
            },
            ("x",x) => {
                let (x,y) = pos_pair(x);
                positional_swaps.swap(x,y);
            },
            ("p",p) => {
                let (xname,yname) = ref_pair(p);
                let x = renames.iter().position(|&x| x == xname).unwrap();
                let y = renames.iter().position(|&y| y == yname).unwrap();
                renames.swap(x,y);
            },
            (e,_) => {
                println!("Unknown move \"{}\"", e);
                break;
            },
        }
    }
    (compile(&from, &positional_swaps), renames)
}

fn compile(from: &Vec<u8>, to: &Vec<u8>) -> Vec<u8> {
    let mut ret = Vec::new();
    for c in to {
       let y = from.iter().position(|y| y == c).unwrap();
       ret.push(y as u8);
    }
    ret
}

fn do_swaps(state: &Vec<u8>, program: &Vec<u8>) -> Vec<u8>{
    program.iter().map(|&pos| state[pos as usize]).collect()
}

fn do_renames(state: &Vec<u8>, program: &Vec<u8>) -> Vec<u8>{
    state.iter().map(|&c| program[(c as usize) - ('a' as usize)]).collect()
}

fn main() {
    let stdin = io::stdin();
    for moves in input::lines(&stdin) {
        let from = String::from("abcdefghijklmnop");

        let (mut swaps, mut renames) = do_moves(&from, &moves);

        let mut t = from.into_bytes();
        let mut rounds = 1_000_000_000;

        while rounds > 0 {
            if rounds & 1 > 0 {
                t = do_swaps(&t, &swaps);
                t = do_renames(&t, &renames);
            }
            
            swaps = do_swaps(&swaps, &swaps);
            renames = do_renames(&renames, &renames);

            rounds >>= 1;
        }

        println!("Final position: {}", String::from_utf8(t).unwrap());
    }
}
