use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::str::FromStr;

enum Dir {
    Inc,
    Dec,
}

struct Op {
    kind: Dir,
    reg: String,
    imm: i32
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() != 3 {
            return Err(String::from("Nope"));
        }

        let reg = tokens[0].to_owned();
        let kind = match tokens[1] {
            "inc" => Dir::Inc,
            "dec" => Dir::Dec,
            _ => return Err(String::from("Unknown Opcode")),
        };
        let imm = tokens[2].parse().or(Err(String::from("Not a Number")))?;
        Ok(Op{kind: kind, reg: reg, imm: imm})
    }
}

enum Cond {
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Ne,
}

struct OpCond {
    kind: Cond,
    reg: String,
    imm: i32,
}

impl FromStr for OpCond {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() != 3 {
            return Err(String::from("Nope"));
        }

        let reg = tokens[0].to_owned();
        let kind = match tokens[1] {
            "==" => Cond::Eq,
            ">=" => Cond::Ge,
            ">"  => Cond::Gt,
            "<=" => Cond::Le,
            "<"  => Cond::Lt,
            "!=" => Cond::Ne,
            err => return Err(String::from(format!("Unknown comparator \"{}\"", err))),
        };
        let imm = tokens[2].parse().or(Err(String::from("Not a Number")))?;
        Ok(OpCond{kind: kind, reg: reg, imm: imm})
    }
}


struct Ins {
    op: Op,
    cond: OpCond,
}

impl FromStr for Ins {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split("if").collect();
        if tokens.len() == 0 {
            return Err(String::from(""))
        }
        if tokens.len() != 2 {
            return Err(String::from("Nope"))
        }

        Ok(Ins{
            op: tokens[0].parse()?,
            cond: tokens[1].parse()?,
        })
    }
    
}

#[derive(Debug)]
struct MachineState {
    regs: HashMap<String, i32>,
}

impl MachineState {
    fn exec(&mut self, ins: Ins) {
        let reg = self.get(&ins.cond.reg);
        let run = match ins.cond.kind {
            Cond::Eq => reg == ins.cond.imm,
            Cond::Ge => reg >= ins.cond.imm,
            Cond::Gt => reg >  ins.cond.imm,
            Cond::Le => reg <= ins.cond.imm,
            Cond::Lt => reg <  ins.cond.imm,
            Cond::Ne => reg != ins.cond.imm,
        };
        if !run {
            return;
        }
        let reg = self.get(&ins.op.reg);
        self.set(&ins.op.reg,
            match ins.op.kind {
                Dir::Inc => reg + ins.op.imm,
                Dir::Dec => reg - ins.op.imm,
            }
        )
    }

    fn get(&mut self, key: &str) -> i32 {
        *self.regs.entry(key.to_owned()).or_insert(0)
    }
    fn set(&mut self, key: &str, val: i32) {
        self.regs.insert(key.to_owned(), val);
    }
}

fn main() {
    let stdin = io::stdin();

    let mut ms = MachineState{regs:HashMap::new()};
    let mut max = 0i32;
    for op in stdin.lock().lines()
        .map(io::Result::unwrap)
        .map(|l| l.parse::<Ins>()) {
            match op {
                Ok(op) => {
                ms.exec(op);
                println!("{:?}", ms);
            },
            Err(err) => println!("{}", err),
            };

            let m = *ms.regs.values().max().unwrap();
            if m > max {
                max = m;
            }
    }

    println!("Max register value: {}", max);
}
