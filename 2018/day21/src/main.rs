use clap::Parser;
use std::collections::BTreeSet;
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
enum Operation {
    AddR,
    AddI,
    MultiR,
    MultiI,
    BitWiseAndR,
    BitWiseAndI,
    BitWiseOrR,
    BitWiseOrI,
    SetR,
    SetI,
    GreaterRI,
    GreaterIR,
    GreaterRR,
    EqualityRI,
    EqualityIR,
    EqualityRR,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    operation: Operation,
    inputa: i64,
    inputb: i64,
    output: i64,
}

impl Instruction {
    fn new(ln: &str) -> Self {
        let parts = ln.split_whitespace().collect::<Vec<_>>();
        let operation = match parts[0] {
            "addr" => Operation::AddR,
            "addi" => Operation::AddI,
            "mulr" => Operation::MultiR,
            "muli" => Operation::MultiI,
            "banr" => Operation::BitWiseAndR,
            "bani" => Operation::BitWiseAndI,
            "borr" => Operation::BitWiseOrR,
            "bori" => Operation::BitWiseOrI,
            "setr" => Operation::SetR,
            "seti" => Operation::SetI,

            "gtir" => Operation::GreaterIR,
            "gtri" => Operation::GreaterRI,
            "gtrr" => Operation::GreaterRR,

            "eqir" => Operation::EqualityIR,
            "eqri" => Operation::EqualityRI,
            "eqrr" => Operation::EqualityRR,
            _ => panic!(),
        };
        Self {
            operation,
            inputa: parts[1].parse::<i64>().unwrap(),
            inputb: parts[2].parse::<i64>().unwrap(),
            output: parts[3].parse::<i64>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Program {
    registers: Vec<i64>,
    instructions: Vec<Instruction>,
    pointer_index: usize,
    ip: i64,
    debug: bool,
}

impl Program {
    fn get_inst(&self) -> Option<&Instruction> {
        self.instructions.get(self.ip as usize)
    }

    fn apply_instruction(&mut self, inst: Instruction) {
        let output = inst.output;
        let inputa = inst.inputa;
        let inputb = inst.inputb;

        if self.debug {
            println!("Running: ");
            dbg!(&inst);
            dbg!(&self.ip);
            println!("Before: {:?}", self.registers);
        }

        // Set register to instruction pointer
        *self.registers.get_mut(self.pointer_index).unwrap() = self.ip;
        match inst.operation {
            Operation::AddR => {
                self.registers[output as usize] =
                    self.registers[inputa as usize] + self.registers[inputb as usize];
            }
            Operation::AddI => {
                self.registers[output as usize] = self.registers[inputa as usize] + inputb;
            }
            Operation::MultiR => {
                self.registers[output as usize] =
                    self.registers[inputa as usize] * self.registers[inputb as usize];
            }
            Operation::MultiI => {
                self.registers[output as usize] = self.registers[inputa as usize] * inputb;
            }
            Operation::BitWiseAndR => {
                self.registers[output as usize] =
                    self.registers[inputa as usize] & self.registers[inputb as usize];
            }
            Operation::BitWiseAndI => {
                self.registers[output as usize] = self.registers[inputa as usize] & inputb;
            }
            Operation::BitWiseOrR => {
                self.registers[output as usize] =
                    self.registers[inputa as usize] | self.registers[inputb as usize];
            }
            Operation::BitWiseOrI => {
                self.registers[output as usize] = self.registers[inputa as usize] | inputb;
            }
            Operation::SetR => {
                self.registers[output as usize] = self.registers[inputa as usize];
            }
            Operation::SetI => {
                self.registers[output as usize] = inputa;
            }
            Operation::GreaterRI => {
                self.registers[output as usize] = if self.registers[inputa as usize] > inputb {
                    1
                } else {
                    0
                };
            }
            Operation::GreaterIR => {
                self.registers[output as usize] = if inputa > self.registers[inputb as usize] {
                    1
                } else {
                    0
                };
            }
            Operation::GreaterRR => {
                self.registers[output as usize] =
                    if self.registers[inputa as usize] > self.registers[inputb as usize] {
                        1
                    } else {
                        0
                    };
            }
            Operation::EqualityRI => {
                self.registers[output as usize] = if self.registers[inputa as usize] == inputb {
                    1
                } else {
                    0
                };
            }
            Operation::EqualityIR => {
                self.registers[output as usize] = if inputa == self.registers[inputb as usize] {
                    1
                } else {
                    0
                };
            }
            Operation::EqualityRR => {
                self.registers[output as usize] =
                    if self.registers[inputa as usize] == self.registers[inputb as usize] {
                        1
                    } else {
                        0
                    };
            }
        }

        if self.debug {
            println!("After: {:?}", self.registers);
        }

        // Read register to instruction pointer
        self.ip = *self.registers.get(self.pointer_index).unwrap();
        // Incretement instruction pointer
        self.ip += 1;
        if self.ip >= self.instructions.len() as i64 {
            println!("Program to be halted after");
            dbg!(&inst);
        }
    }
}

fn get_part1(program: &Program) -> i64 {
    let mut p = program.clone();
    loop {
        if p.ip == 28 {
            // Instruction 28 is the potential trigger for the program halting
            // Program will halt if reg0 matches reg3
            // For part1, stop looking the first time we hit instruction 28
            break;
        }
        match p.get_inst() {
            None => {
                break;
            }
            Some(inst) => {
                p.apply_instruction(*inst);
            }
        }
    }
    p.registers[3]
}

fn get_part2(program: &Program) -> i64 {
    let mut p = program.clone();
    let mut seen: BTreeSet<i64> = BTreeSet::new();
    let mut part2 = 0;
    loop {
        if p.ip == 28 {
            let t = p.registers.get(3).unwrap();
            // In part2 keep looking until register has hit a loop
            // If the value of reg0 is not one of the values seen during this loop,
            // program will never halt
            if seen.contains(t) {
                break;
            }
            // The last value before the loop repeats is the one that will give the largest number
            // of iterations before halting
            part2 = *t;
            seen.insert(*t);
        }
        match p.get_inst() {
            None => {
                break;
            }
            Some(inst) => {
                p.apply_instruction(*inst);
            }
        }
    }
    part2
}

fn read_program(cont: &str) -> Program {
    let pi = cont
        .lines()
        .next()
        .unwrap()
        .split_once(" ")
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap();
    let mut instructions = Vec::new();
    for line in cont.lines().skip(1) {
        instructions.push(Instruction::new(line));
    }
    Program {
        registers: vec![0, 0, 0, 0, 0, 0],
        instructions,
        pointer_index: pi,
        ip: 0,
        debug: false,
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let program = read_program(cont);
    let part1 = get_part1(&program);
    let part2 = get_part2(&program);
    (part1, part2)
}
