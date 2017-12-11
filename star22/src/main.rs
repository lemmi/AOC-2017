use std::io;
use std::io::BufRead;
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone,Copy,Default,Debug)]
struct HexCube {
    x: i32,
    y: i32,
    z: i32,
}

impl HexCube {
    fn new(x: i32, y: i32, z: i32) -> HexCube {
        HexCube{x: x, y: y, z: z}
    }

    fn length(&self) -> i32 {
        (
            self.x.abs() + 
            self.y.abs() + 
            self.z.abs()
        ) / 2
    }
}

impl FromStr for HexCube {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim() {
            "n"  => Ok(HexCube::new(0,1,-1)),
            "nw" => Ok(HexCube::new(-1,1,0)),
            "sw" => Ok(HexCube::new(-1,0,1)),
            "s"  => Ok(HexCube::new(0,-1,1)),
            "se" => Ok(HexCube::new(1,-1,0)),
            "ne" => Ok(HexCube::new(1,0,-1)),
            c => Err(format!("Unexpected direction \"{}\"", c)),
        }
    }
}

impl Add for HexCube {
    type Output = HexCube;
    fn add(self, other: HexCube) -> HexCube {
        HexCube::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            )
    }
}


fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let max = line.split(",")
            .map(|v| v.parse::<HexCube>().unwrap())
            .scan(HexCube::default(), |acc, v| {
                *acc = *acc + v;
                Some(acc.length())
            }).max();

        println!("max: {}", max.unwrap());
    }
}
