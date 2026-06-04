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
    inputa: i32,
    inputb: i32,
    output: i32,
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
            inputa: parts[1].parse::<i32>().unwrap(),
            inputb: parts[2].parse::<i32>().unwrap(),
            output: parts[3].parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Program {
    registers: Vec<i32>,
    instructions: Vec<Instruction>,
    pointer_index: usize,
    ip: i32,
    ind_counts: BTreeMap<i32, i64>, // Counts how many timse each instruction is run
}

impl Program {
    fn get_inst(&self) -> Option<&Instruction> {
        self.instructions.get(self.ip as usize)
    }

    fn apply_instruction(&mut self, inst: Instruction) {
        let output = inst.output;
        let inputa = inst.inputa;
        let inputb = inst.inputb;

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
                self.registers[output as usize] = if self.registers[inputa as usize] > inputb  {
                    1
                } else {
                    0
                };
            }
            Operation::GreaterIR => {
                self.registers[output as usize] = if inputb > self.registers[inputa as usize] {
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
                self.registers[output as usize] = if inputb == self.registers[inputa as usize] {
                    1
                } else {
                    0
                };
            }
            Operation::EqualityIR => {
                self.registers[output as usize] = if self.registers[inputb as usize] == inputa {
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
        // Read register to instruction pointer
        self.ip = *self.registers.get(self.pointer_index).unwrap();
        // Incretement instruction pointer
        self.ip += 1;
    }
}

fn get_short(program: &Program) -> i64 {
    let mut program = program.clone();
    loop {
        if program.ip == 2 {
            dbg!(&program.registers);
            let a = program.registers.get(4).unwrap();
            return get_divisor_sum(*a as i64);
        }
        match program.get_inst() {
            None => break,
            Some(inst) => {
                program.apply_instruction(*inst);
            }
        }
    }
    0
}

fn get_divisor_sum(a: i64) -> i64 {
    // Main loop of the input Program is getting the sum of divisors for the given number
    (1..=a).filter(|i| a % i == 0).sum()
}

fn get_full(program: &Program) -> i64 {
    let mut program = program.clone();
    let mut loop_count: i64 = 0;
    loop {
        loop_count += 1;
        if loop_count % 10_000_000 == 0 {
            println!("loop: {loop_count}");
        }
        if loop_count > 1_000_000_000 {
            break;
        }
        program
            .ind_counts
            .entry(program.ip)
            .and_modify(|c| *c += 1)
            .or_insert(1);

        match program.get_inst() {
            None => break,
            Some(inst) => {
                program.apply_instruction(*inst);
            }
        }
    }
    program.registers[0] as i64
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
        ind_counts: BTreeMap::new(),
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let program = read_program(cont);

    let part1 = get_full(&program);
    let part1_short = get_short(&program);
    // Use part1 to check that the short version works
    assert_eq!(part1, part1_short);

    let mut program = program.clone();
    *program.registers.get_mut(0).unwrap() = 1;
    let part2 = get_short(&program);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";
        let program = read_program(&a);
        assert_eq!(get_full(&program), 6);
    }

    #[test]
    fn part2() {
        assert_eq!(get_divisor_sum(12), 28);
        assert_eq!(get_divisor_sum(60), 168);
        assert_eq!(get_divisor_sum(970), 1764);
    }
}
