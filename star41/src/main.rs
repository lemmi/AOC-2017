extern crate aoc;

use std::collections::BTreeMap;
use std::fmt;
use std::cmp;
use std::io;
use aoc::input;

fn parse_cell(input: char) -> Result<bool, &'static str> {
    match input {
        '#' => Ok(true),
        '.' => Ok(false),
        _ => Err("Invalid char"),
    }
}

fn assemble_grid(grid: &[&[bool]]) -> Result<Grid, &'static str> {
    let min = grid.iter().map(|&r| r.len()).min().ok_or("Empty grid?")?;
    let max = grid.iter().map(|&r| r.len()).max().ok_or("Empty grid?")?;

    if min != max {
        return Err("Row lengths don't match");
    }

    if min != grid.len() {
        return Err("Grid is not a square");
    }

    Ok(Grid{
        size: min, 
        pixel: grid.iter().flat_map(|&r| r).cloned().collect(),
    })
}

fn parse_grid(input: &str) -> Result<Grid, &'static str> {
    let grid = input.split("/")
        .map(|l|
             l.chars()
             .map(|c| parse_cell(c))
             .collect()
            ).collect::<Result<Vec<Vec<bool>>, &'static str>>()?;

    let sliced: Vec<&[bool]> = grid.iter().map(|r| r.as_slice()).collect();
    assemble_grid(&sliced)

}

fn parse_rule(input: &str) -> Result<(Grid, Grid), &'static str> {
    let mut split = input.split(" => ");
    let from = parse_grid(split.next().ok_or("Rule is missing from")?.trim())?;
    let to = parse_grid(split.next().ok_or("Rule is missing to")?.trim())?;
    Ok((from, to))
}

#[derive(PartialEq,Eq,Clone,PartialOrd)]
struct Grid {
    size: usize,
    pixel: Vec<bool>,
}

fn clamped<T: Ord>(min: T, max: T, val: T) -> Option<T> {
    if min <= val && val < max {
        Some(val)
    } else {
        None
    }
}

impl Grid {
    fn new(size: usize) -> Grid {
        Grid {
            size: size,
            pixel: vec![false; size * size],
        }
    }
    fn get(&self, y: usize, x: usize) -> Option<bool> {
        let y = clamped(0, self.size, y)?;
        let x = clamped(0, self.size, x)?;
        self.pixel.get(y * self.size +x).cloned()
    }
    fn get_mut(&mut self, y: usize, x: usize) -> Option<&mut bool> {
        let y = clamped(0, self.size, y)?;
        let x = clamped(0, self.size, x)?;
        self.pixel.get_mut(y * self.size + x)
    }
    fn rotate(&self) -> Grid {
        let mut ret = Grid {
            size: self.size,
            pixel: vec![false; self.size * self.size],
        };

        for y in 0..self.size {
        for x in 0..self.size {
            *ret.get_mut(self.size-1-x,y).unwrap() = self.get(y,x).unwrap()
            }
        }

        ret
    }
    fn flip(&self) -> Grid {
        let mut ret = Grid {
            size: self.size,
            pixel: vec![false; self.size * self.size],
        };

        for y in 0..self.size {
            for x in 0..self.size {
                *ret.get_mut(y, self.size-1-x).unwrap() = self.get(y,x).unwrap()
            }
        }

        ret
    }
    fn canonical(&self) -> Grid {
        let can = [
            self.rotate(),
            self.rotate().rotate(),
            self.rotate().rotate().rotate(),
            self.rotate().rotate().rotate().rotate(),
            self.flip(),
            self.flip().rotate(),
            self.flip().rotate().rotate(),
            self.flip().rotate().rotate().rotate(),
        ].iter().max().unwrap().clone();
        
        can
    }

    fn slice_mut<'a>(&'a mut self, offy: usize, offx: usize, size: usize) -> Option<Vec<&'a mut[bool]>> {
        let ystart = clamped(0, self.size, offy)?;
        clamped(0, self.size, offy+size-1)?;
        let xstart = clamped(0, self.size, offx)?;
        let xend = clamped(0, self.size, offx+size-1)?;

        let mut ret = Vec::new();

        for slice in self.pixel.chunks_mut(self.size).skip(ystart).take(size) {
            ret.push(&mut slice[xstart..xend+1]);
        }

        Some(ret)
    }

    fn slice<'a>(&'a self, offy: usize, offx: usize, size: usize) -> Option<Vec<&'a[bool]>> {
        let ystart = clamped(0, self.size, offy)?;
        clamped(0, self.size, offy+size-1)?;
        let xstart = clamped(0, self.size, offx)?;
        let xend = clamped(0, self.size, offx+size-1)?;

        let mut ret = Vec::new();

        for slice in self.pixel.chunks(self.size).skip(ystart).take(size) {
            ret.push(&slice[xstart..xend+1]);
        }

        Some(ret)
    }

    fn splice(&mut self, other: &Grid, offy: usize, offx: usize) {
        for (dest, src) in self.slice_mut(offy, offx, other.size).unwrap().iter_mut().zip(other.pixel.chunks(other.size)) {
           dest.copy_from_slice(src); 
        }
    }

    fn enhance(&self, rules: &BTreeMap<Vec<bool>, Grid>) -> Grid {
        let chunksize = if self.size % 2 == 0 {
            2
        } else if self.size % 3 == 0 {
            3
        } else {
            unreachable!()
        };

        let steps = self.size / chunksize;
        let newchunksize = chunksize + 1;
        let newsize = newchunksize * steps;

        let mut ret = Grid::new(newsize);

        for offy in 0..steps {
            for offx in 0..steps {
                let sliced = self.slice(offy * chunksize, offx * chunksize, chunksize).unwrap();
                let src = assemble_grid(&sliced).unwrap().canonical();
                let enhanced = &rules[&src.pixel];
                assert_eq!(enhanced.size, newchunksize);
                ret.splice(enhanced, offy * newchunksize, offx * newchunksize);
            }
        }

        ret
    }
}

fn format_row(r: &[bool]) -> String {
    r.iter().map(|&x| if x { '#' } else { '.' }).collect::<String>()
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.pixel.chunks(self.size) {
            write!(f, "{}\n", format_row(line))?
        }
        Ok(())
    }
}

impl cmp::Ord for Grid {
    fn cmp(&self, other: &Grid) -> cmp::Ordering {
        self.size.cmp(&other.size).then(
                self.pixel.iter().cloned().cmp(
                    other.pixel.iter().cloned()
                    )
                )
    }
}

fn main() {
    let stdin = io::stdin();

    let rules: Result<Vec<_>,_> = input::lines(&stdin).map(|l| parse_rule(&l)).collect();
    let rules = rules.unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let mut ruleset = BTreeMap::new();

    for (from, to) in rules {
        println!("{}\n->\n{}", from, to);
        println!("-----------------------");

        ruleset.insert(from.canonical().pixel, to.clone());
    }

    for (k,v) in &ruleset {
        println!("{} ->\n{}", format_row(&k), v);
    }

    let mut g = parse_grid(".#./..#/###").unwrap().canonical();

    for i in 0..5 {
        g = g.enhance(&ruleset);
        println!("Step {}", i);
        println!("{}", g);
    }

    let n = g.pixel.iter().map(|&c| if c { 1 } else { 0 }).sum::<usize>();
    println!("Enabled pixels: {}", n);
}
