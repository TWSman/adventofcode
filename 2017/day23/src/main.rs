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
    Sub(char, Address),
    Multiply(char, Address),
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
            "sub" => Self::Sub(words[1].chars().next().unwrap(), Address::new(words[2])),
            "mul" => Self::Multiply(words[1].chars().next().unwrap(), Address::new(words[2])),
            "jnz" => Self::Jump(Address::new(words[1]), Address::new(words[2])),
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
    mul_count: i64,
    ind_counts: BTreeMap<i64, i64>,
}

enum ProgramState {
    Running,
    Terminated,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        let mut registers = BTreeMap::new();
        for c in 'a'..='h' {
            registers.insert(c, 0);
        }
        Self {
            instructions: instructions.clone(),
            ind: 0,
            loop_count: 0,
            registers,
            length: instructions.len().try_into().unwrap(),
            mul_count: 0,
            ind_counts: BTreeMap::new(),
        }
    }

    fn insert_value(&mut self, reg: char, val: i64) {
        self.registers.insert(reg, val);
    }

    fn run(&mut self) -> ProgramState {
        if self.ind >= self.length || self.ind < 0 {
            return ProgramState::Terminated;
        }
        self.ind_counts
            .entry(self.ind)
            .and_modify(|c| *c += 1)
            .or_insert(1);
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
            Instruction::Sub(reg, a) => {
                let b = *match a {
                    Address::Number(n) => n,
                    Address::Register(cc) => self.registers.get(cc).unwrap(),
                };
                *self.registers.get_mut(reg).unwrap() -= b;
            }
            Instruction::Multiply(reg, a) => {
                let b = *match a {
                    Address::Number(n) => n,
                    Address::Register(cc) => self.registers.get(cc).unwrap(),
                };
                self.mul_count += 1;
                *self.registers.get_mut(reg).unwrap() *= b;
            }
            Instruction::Jump(a, b) => {
                let x = match a {
                    Address::Number(n) => n,
                    Address::Register(cc) => self.registers.get(cc).unwrap(),
                };
                if *x != 0 {
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
    let mut program = Program::new(instructions.to_vec());
    // Runs until state is something other than running
    while let ProgramState::Running = program.run() {}
    program.mul_count
}

fn get_part2(instructions: &[Instruction]) -> i64 {
    // The given program is counting how many composite numbers there are inside the given
    // range with a step of 17
    let mut program = Program::new(instructions.to_vec());
    program.insert_value('a', 1);
    // Run the setup steps
    loop {
        if program.ind == 8 {
            break;
        }
        let _ = program.run();
    }
    let start = program.registers[&'b'];
    let end = program.registers[&'c'];
    (start..=end)
        .step_by(17)
        .filter(|i| is_composite(*i))
        .count() as i64
}

fn is_composite(n: i64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
fn get_part2_full(instructions: &[Instruction]) -> i64 {
    // Will take a huge number of steps with given input
    println!("Running part 2");
    let mut program = Program::new(instructions.to_vec());
    program.insert_value('a', 1);

    loop {
        if program.loop_count % 100_000_000 == 0 {
            println!("Loop count: {}, ind: {}", program.loop_count, program.ind);
        }
        //if program.ind == 24 {
        //    dbg!(&program.registers);
        //    return 0;
        //}
        if program.loop_count > 1_000_000_000 {
            println!("Too many loops, terminating");
            dbg!(&program.registers);
            break;
        }
        match program.run() {
            ProgramState::Running => continue,
            ProgramState::Terminated => break,
        }
    }
    program.registers.get(&'h').unwrap().to_owned()
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
    fn prime() {
        assert!(!is_composite(17));
        assert!(is_composite(125909));
        assert!(!is_composite(126011));
        assert!(is_composite(125));
    }
}
