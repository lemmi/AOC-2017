extern crate aoc;

use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

use aoc::input;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Func {
    Add,
    Jgz,
    Mod,
    Mul,
    Rcv,
    Set,
    Snd,
}
impl FromStr for Func {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Func::Add),
            "jgz" => Ok(Func::Jgz),
            "mod" => Ok(Func::Mod),
            "mul" => Ok(Func::Mul),
            "rcv" => Ok(Func::Rcv),
            "set" => Ok(Func::Set),
            "snd" => Ok(Func::Snd),
            e => Err(format!("Invalid Instruction \"{}\"", e)),
        }
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Op {
    None,
    Imm(i64),
    Reg(usize),
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Ok(Op::None);
        }
        if let Ok(n) = s.parse::<i64>() {
            return Ok(Op::Imm(n));
        }
        if s.len() == 1 {
            if let Ok(c) = s.parse::<char>() {
                if c >= 'A' && c <= 'z' {
                    return Ok(Op::Reg((c as usize) - ('A' as usize)))
                }
            }
        }
        Err(String::from("Invalid register name"))
    }
}

impl Op {
    fn must_reg(self) -> Result<usize,&'static str> {
        if let Op::Reg(i) = self {
            Ok(i)
        } else {
            Err("Expected register")
        }
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
struct Instruction {
    f: Func,
    o1: Op,
    o2: Op,
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        Ok(Instruction{
            f: tokens.next().unwrap_or("").parse::<Func>()?,
            o1: tokens.next().ok_or(String::from("Expected operand"))?.parse::<Op>()?,
            o2: tokens.next().unwrap_or("").parse::<Op>()?,
        })
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum State {
    Running,
    Receiving,
    Sending(i64),
}

struct Vm {
    regs: [i64; 2*26],
    pc: usize,
    prog: Vec<Instruction>,

    state: State,

    sent: usize,
    inbox: VecDeque<i64>,
}

impl Vm {
    fn new(prog: &Vec<Instruction>, id: usize) -> Vm {
        let mut ret = Vm {
            regs: [0i64; 2*26],
            pc: 0usize,
            prog: prog.clone(),

            state: State::Running,

            sent: 0usize,
            inbox: VecDeque::new(),
        };
        ret.set(Op::from_str("p").unwrap(), id as i64).unwrap();
        ret
    }

    fn fetch(&self) -> Result<Instruction, &'static str> {
        self.prog.get(self.pc).cloned().ok_or("Invalid register")
    }

    fn set(&mut self, idx: Op, val: i64) -> Result<(),&'static str> {
        match idx {
            Op::Reg(i) => {self.regs[i] = val; Ok(())},
            _ => Err("Target isn't a register")
        }
    }
    fn load_reg(&self, idx: Op) -> Result<i64,&'static str> {
        let idx = idx.must_reg()?;
        self.regs.get(idx).cloned().ok_or("Invalid register")
    }
    fn load(&self, idx: Op) -> Result<i64,&'static str> {
        match idx {
            Op::Imm(i) => Ok(i),
            Op::Reg(_) => self.load_reg(idx),
            Op::None => Err("Invalid Register"),
        }
    }
    fn apply(&mut self, ins: Instruction) -> Result<(),&'static str> {
        self.pc += 1;

        println!("{:?} PC: {} {:?}", self.state, self.pc, ins);

        match ins.f {
            Func::Add => {
                let o1 = self.load_reg(ins.o1)?;
                let o2 = self.load(ins.o2)?;
                self.set(ins.o1, o1 + o2)?;
            },
            Func::Jgz => {
                let o1 = self.load(ins.o1)?;
                let o2 = self.load(ins.o2)?;
                if o1 > 0 {
                    self.pc = ((self.pc as isize) + (o2 as isize) - 1) as usize;
                }
            },
            Func::Mod => {
                let o1 = self.load_reg(ins.o1)?;
                let o2 = self.load(ins.o2)?;
                self.set(ins.o1, o1 % o2)?;
            },
            Func::Mul => {
                let o1 = self.load_reg(ins.o1)?;
                let o2 = self.load(ins.o2)?;
                self.set(ins.o1, o1 * o2)?;
            },
            Func::Rcv => {
                if let Some(val) = self.inbox.pop_front() {
                    self.set(ins.o1, val)?;
                } else {
                    self.state = State::Receiving;
                    self.pc -= 1;
                }
            },
            Func::Set => {
                let o2 = self.load(ins.o2)?;
                self.set(ins.o1, o2)?;
            },
            Func::Snd => {
                let o1 = self.load(ins.o1)?;
                self.state = State::Sending(o1);
                self.sent += 1;
            },
        }
        Ok(())
    }
    fn step(&mut self) -> Result<(), &'static str> {
        let ins = self.fetch()?;
        self.apply(ins)
    }
}

fn other(id: usize) -> usize {
    1 - id
}

fn main() {
    let stdin = io::stdin();

    let prog = input::lines(&stdin).map(|l| l.parse::<Instruction>().unwrap()).collect();
    let mut vms = [Vm::new(&prog, 0), Vm::new(&prog, 1)];

    let mut scheduler = VecDeque::new();
    scheduler.push_back(0);
    scheduler.push_back(1);

    while let Some(id) = scheduler.pop_front() {
        print!("{}, {:2}: ", scheduler.len(), id);
        if let Err(e) = vms[id].step() {
            println!("Vm {}: {}", id, e);
            break;
        }
        if let State::Sending(val) = vms[id].state {
            vms[other(id)].inbox.push_back(val);
            if vms[other(id)].state == State::Receiving {
                vms[other(id)].state = State::Running;
                scheduler.push_back(other(id));
            }
            vms[id].state = State::Running;
        }
        if vms[id].state == State::Running {
            scheduler.push_back(id);
        }
    }

    for (id, vm) in vms.iter().enumerate() {
        println!("Vm {} sent {} messages", id, vm.sent);
    }
}
