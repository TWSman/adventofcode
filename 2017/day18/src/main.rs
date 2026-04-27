use clap::Parser;
use std::collections::BTreeMap;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let start = Instant::now();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Address {
    Number(i64),
    Register(char),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    Set(char, Address),
    Send(Address),
    Add(char, Address),
    Multiply(char, Address),
    Modulo(char, Address),
    Recover(char),
    Jump(Address, Address),
}

impl Address {
    fn new(ln: &str) -> Self {
        if let Ok(num) = ln.parse::<i64>() {
            Self::Number(num)
        } else {
            Self::Register(ln.chars().next().unwrap())
        }
    }
}

impl Instruction {
    fn new(ln: &str) -> Self {
        let words = ln.split_whitespace().collect::<Vec<_>>();
        match words[0] {
            "set" => Self::Set(words[1].chars().next().unwrap(), Address::new(words[2])),
            "snd" => Self::Send(Address::new(words[1])),
            "add" => Self::Add(words[1].chars().next().unwrap(), Address::new(words[2])),
            "mul" => Self::Multiply(words[1].chars().next().unwrap(), Address::new(words[2])),
            "mod" => Self::Modulo(words[1].chars().next().unwrap(), Address::new(words[2])),
            "rcv" => Self::Recover(words[1].chars().next().unwrap()),
            "jgz" => Self::Jump(Address::new(words[1]), Address::new(words[2])),
            _ => panic!(),
        }
    }
}

struct Program {
    instructions: Vec<Instruction>,
    length: i64,
    registers: BTreeMap<char, i64>,
    loop_count: i64,
    ind: i64,
    part2: bool,
    input: Vec<i64>,
}

enum ProgramState {
    Running,
    Waiting,
    Send(i64),
    Terminated,
}

impl Program {
    fn new(instructions: Vec<Instruction>, part2: bool) -> Self {
        Self {
            instructions: instructions.clone(),
            ind: 0,
            part2,
            loop_count: 0,
            input: Vec::new(),
            registers: BTreeMap::new(),
            length: instructions.len().try_into().unwrap(),
        }
    }

    fn insert_value(&mut self, reg: char, val: i64) {
        self.registers.insert(reg, val);
    }

    fn add_input(&mut self, val: i64) {
        self.input.push(val);
    }

    fn run(&mut self) -> ProgramState {
        if self.ind >= self.length || self.ind < 0 {
            return ProgramState::Terminated;
        }
        self.loop_count += 1;
        let inst = self.instructions.get(self.ind as usize).unwrap();
        match inst {
            Instruction::Set(reg, a) => {
                let b = match a {
                    Address::Number(n) => n,
                    Address::Register(cc) => self.registers.get(cc).unwrap(),
                };
                self.registers.insert(*reg, *b);
            }
            Instruction::Send(a) => {
                let b = match a {
                    Address::Number(n) => n,
                    Address::Register(cc) => self.registers.get(cc).unwrap(),
                };
                self.ind += 1;
                return ProgramState::Send(*b);
            }
            Instruction::Add(reg, a) => {
                let b = *match a {
                    Address::Number(n) => n,
                    Address::Register(cc) => self.registers.get(cc).unwrap(),
                };
                if !self.registers.contains_key(reg) {
                    self.registers.insert(*reg, 0);
                }
                *self.registers.get_mut(reg).unwrap() += b;
            }
            Instruction::Multiply(reg, a) => {
                let b = *match a {
                    Address::Number(n) => n,
                    Address::Register(cc) => self.registers.get(cc).unwrap(),
                };
                if !self.registers.contains_key(reg) {
                    self.registers.insert(*reg, 0);
                }
                *self.registers.get_mut(reg).unwrap() *= b;
            }
            Instruction::Modulo(reg, b) => {
                let x = *match b {
                    Address::Number(n) => n,
                    Address::Register(cc) => self.registers.get(cc).unwrap(),
                };
                if !self.registers.contains_key(reg) {
                    self.registers.insert(*reg, 0);
                }
                let val = self.registers.get(reg).unwrap();
                *self.registers.get_mut(reg).unwrap() = val % x;
            }
            Instruction::Recover(reg) => {
                if !self.registers.contains_key(reg) {
                    self.registers.insert(*reg, 0);
                }
                if self.part2 {
                    // In part2 we try read from the input, and if it's empty we wait, otherwise we read the value into the register
                    if self.input.is_empty() {
                        return ProgramState::Waiting;
                    }
                    let val = self.input.remove(0);
                    self.registers.insert(*reg, val);
                } else {
                    // In part 1 we just check if the value in the register is nonzero, and if so we recover the last sound
                    let x = self.registers.get(reg).unwrap();
                    if *x != 0 {
                        return ProgramState::Waiting;
                    }
                }
            }
            Instruction::Jump(a, b) => {
                let x = match a {
                    Address::Number(n) => n,
                    Address::Register(cc) => self.registers.get(cc).unwrap(),
                };
                if *x > 0 {
                    let offset = match b {
                        Address::Number(n) => n,
                        Address::Register(cc) => self.registers.get(cc).unwrap(),
                    };
                    self.ind += offset;
                    return ProgramState::Running;
                }
            }
        }
        self.ind += 1;
        ProgramState::Running
    }
}

fn get_part1(instructions: &[Instruction]) -> i64 {
    let mut prev_sound = None;
    let mut program = Program::new(instructions.to_vec(), false);
    loop {
        match program.run() {
            ProgramState::Running => continue,
            ProgramState::Waiting => return prev_sound.unwrap(),
            ProgramState::Send(val) => prev_sound = Some(val),
            ProgramState::Terminated => panic!("Program terminated without recovering a sound"),
        }
    }
}

fn get_part2(instructions: &[Instruction]) -> i64 {
    let mut program0 = Program::new(instructions.to_vec(), true);
    let mut program1 = Program::new(instructions.to_vec(), true);
    program0.insert_value('p', 0);
    program1.insert_value('p', 1);
    let mut waiting0 = false;
    let mut waiting1 = false;
    let mut output0 = Vec::new();
    let mut output1 = Vec::new();
    loop {
        match program0.run() {
            ProgramState::Running => waiting0 = false,
            ProgramState::Waiting => {
                waiting0 = true;
            }
            ProgramState::Send(val) => {
                program1.add_input(val);
                output0.push(val);
            }
            ProgramState::Terminated => break,
        }
        match program1.run() {
            ProgramState::Running => waiting1 = false,
            ProgramState::Waiting => {
                waiting1 = true;
            }
            ProgramState::Send(val) => {
                program0.add_input(val);
                output1.push(val);
            }
            ProgramState::Terminated => break,
        }
        if waiting0 && waiting1 {
            break;
        }
    }
    output1.len() as i64
}

fn read_contents(cont: &str) -> (i64, i64) {
    let program: Vec<Instruction> = cont.lines().map(Instruction::new).collect();
    let part1 = get_part1(&program);
    let part2 = get_part2(&program);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        let program: Vec<Instruction> = a.lines().map(Instruction::new).collect();
        assert_eq!(get_part1(&program), 4);
    }

    #[test]
    fn part2() {
        let a = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
        let program: Vec<Instruction> = a.lines().map(Instruction::new).collect();
        assert_eq!(get_part2(&program), 3);
    }
}
