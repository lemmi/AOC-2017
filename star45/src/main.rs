extern crate aoc;
use std::io;
use std::str::FromStr;
use aoc::input;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Func {
    Add,
    Sub,
    Jgz,
    Jnz,
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
            "sub" => Ok(Func::Sub),
            "jgz" => Ok(Func::Jgz),
            "jnz" => Ok(Func::Jnz),
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
                if c >= 'a' && c <= 'h' {
                    return Ok(Op::Reg((c as usize) - ('a' as usize)))
                }
            }
        }
        Err(String::from("Invalid register name"))
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

struct Vm {
    regs: [i64; 8],
    pc: usize,
    prog: Vec<Instruction>,

    mul_count: usize,
    running: bool,
}

impl Vm {
    fn new() -> Vm {
        Vm {
            regs: [0i64; 8],
            pc: 0usize,
            prog: Vec::new(),

            mul_count: 0usize,
            running: true,
        }
    }

    fn fetch(&self) -> Option<Instruction> {
        self.prog.get(self.pc).cloned()
    }

    fn set(&mut self, idx: Op, val: i64) {
        match idx {
            Op::Reg(i) => self.regs[i] = val,
            _ => panic!("wtf"),
        }
    }
    fn reg(&self, idx: Op) -> Option<i64> {
        match idx {
            Op::Imm(i) => Some(i),
            Op::Reg(i) => self.regs.get(i).cloned(),
            Op::None => None,
        }
    }
    fn apply(&mut self, ins: Instruction) {
        let o1 = self.reg(ins.o1).unwrap();
        let o2 = self.reg(ins.o2);
        self.pc += 1;

        println!("{:?}, o1: {:?}, o2: {:?}", ins, o1, o2);

        match ins.f {
            Func::Add => {
                self.set(ins.o1, o1 + o2.unwrap());
            },
            Func::Sub => {
                self.set(ins.o1, o1 - o2.unwrap());
            },
            Func::Jnz => {
                if o1 != 0 {
                    self.pc = ((self.pc as isize) + (o2.unwrap() as isize) - 1) as usize;
                }
            },
            Func::Jgz => {
                if o1 > 0 {
                    self.pc = ((self.pc as isize) + (o2.unwrap() as isize) - 1) as usize;
                }
            },
            Func::Mod => {
                self.set(ins.o1, o1 % o2.unwrap());
            },
            Func::Mul => {
                self.mul_count += 1;
                self.set(ins.o1, o1 * o2.unwrap());
            },
            Func::Rcv => {
                /*if o1 != 0 {
                    let val = self.last_freq;
                    self.set(ins.o1, val);
                    println!("{}", val);
                    self.running = false;
                }*/
            },
            Func::Set => {
                self.set(ins.o1, o2.unwrap());
            },
            Func::Snd => {/*
                self.last_freq = o1;
            */},
            
        }
    }
    fn step(&mut self) -> bool {
        if let Some(ins) = self.fetch() {
            self.apply(ins);
        } else {
            self.running = false;
        }
        self.running
    }
}


fn main() {
    let stdin = io::stdin();

    let mut vm = Vm::new();
    vm.prog = input::lines(&stdin).map(|l| l.parse::<Instruction>().unwrap()).collect();

    while vm.step(){};

    println!("Number of mul ops: {}", vm.mul_count);
}
