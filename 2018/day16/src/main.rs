use clap::Parser;
use itertools::Itertools;
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
    BitwiseAndR,
    BitwiseAndI,
    BitwiseOrR,
    BitwiseOrI,
    SetR,
    SetI,
    GreaterRI,
    GreaterIR,
    GreaterRR,
    EqualityRI,
    EqualityIR,
    EqualityRR,
}

impl Operation {
    fn apply(&self, i: &Instruction) -> Vec<i32> {
        let mut outputs = i.before.clone();
        assert!(i.inputa <= 3);
        assert!(i.output <= 3);
        match self {
            Operation::AddR => {
                outputs[i.output as usize] =
                    outputs[i.inputa as usize] + outputs[i.inputb as usize];
            }
            Operation::AddI => {
                outputs[i.output as usize] = outputs[i.inputa as usize] + i.inputb;
            }
            Operation::MultiR => {
                outputs[i.output as usize] =
                    outputs[i.inputa as usize] * outputs[i.inputb as usize];
            }
            Operation::MultiI => {
                outputs[i.output as usize] = outputs[i.inputa as usize] * i.inputb;
            }
            Operation::BitwiseAndR => {
                outputs[i.output as usize] =
                    outputs[i.inputa as usize] & outputs[i.inputb as usize];
            }
            Operation::BitwiseAndI => {
                outputs[i.output as usize] = outputs[i.inputa as usize] & i.inputb;
            }
            Operation::BitwiseOrR => {
                outputs[i.output as usize] =
                    outputs[i.inputa as usize] | outputs[i.inputb as usize];
            }
            Operation::BitwiseOrI => {
                outputs[i.output as usize] = outputs[i.inputa as usize] | i.inputb;
            }
            Operation::SetR => {
                outputs[i.output as usize] = outputs[i.inputa as usize];
            }
            Operation::SetI => {
                outputs[i.output as usize] = i.inputa;
            }
            Operation::GreaterRI => {
                outputs[i.output as usize] = if outputs[i.inputa as usize] > i.inputb  {
                    1
                } else {
                    0
                };
            }
            Operation::GreaterIR => {
                outputs[i.output as usize] = if i.inputa > outputs[i.inputb as usize] {
                    1
                } else {
                    0
                };
            }
            Operation::GreaterRR => {
                outputs[i.output as usize] =
                    if outputs[i.inputa as usize] > outputs[i.inputb as usize] {
                        1
                    } else {
                        0
                    };
            }
            Operation::EqualityRI => {
                outputs[i.output as usize] = if i.inputb == outputs[i.inputa as usize] {
                    1
                } else {
                    0
                };
            }
            Operation::EqualityIR => {
                outputs[i.output as usize] = if outputs[i.inputb as usize] == i.inputa {
                    1
                } else {
                    0
                };
            }
            Operation::EqualityRR => {
                outputs[i.output as usize] =
                    if outputs[i.inputa as usize] == outputs[i.inputb as usize] {
                        1
                    } else {
                        0
                    };
            }
        }
        outputs
    }
}

struct Instruction {
    operation: i32,
    inputa: i32,
    inputb: i32,
    output: i32,
    before: Vec<i32>,
    after: Vec<i32>,
}

fn get_part1(instructions: &[Instruction]) -> i64 {
    instructions
        .iter()
        .filter(|inst| check_instruction(inst).len() >= 3)
        .count() as i64
}

fn check_instruction(instruction: &Instruction) -> Vec<Operation> {
    let mut possible = Vec::new();
    for operation in [
        Operation::AddR,
        Operation::AddI,
        Operation::MultiR,
        Operation::MultiI,
        Operation::BitwiseAndR,
        Operation::BitwiseAndI,
        Operation::BitwiseOrR,
        Operation::BitwiseOrI,
        Operation::SetR,
        Operation::SetI,
        Operation::GreaterRI,
        Operation::GreaterIR,
        Operation::GreaterRR,
        Operation::EqualityRI,
        Operation::EqualityIR,
        Operation::EqualityRR,
    ] {
        if instruction.after == operation.apply(instruction) {
            possible.push(operation);
        }
    }
    possible
}

fn get_mapping(instructions: &[Instruction]) -> BTreeMap<i32, Operation> {
    let mut possible = BTreeMap::new();
    for i in 0..16 {
        possible.insert(
            i,
            vec![
                Operation::AddR,
                Operation::AddI,
                Operation::MultiR,
                Operation::MultiI,
                Operation::BitwiseAndR,
                Operation::BitwiseAndI,
                Operation::BitwiseOrR,
                Operation::BitwiseOrI,
                Operation::SetR,
                Operation::SetI,
                Operation::GreaterRI,
                Operation::GreaterIR,
                Operation::GreaterRR,
                Operation::EqualityRI,
                Operation::EqualityIR,
                Operation::EqualityRR,
            ],
        );
    }
    for instruction in instructions {
        //println!("Checking op code {}", instruction.operation);
        let tmp = possible.get_mut(&instruction.operation).unwrap();
        if tmp.len() == 1 {
            continue;
        }
        if tmp.is_empty() {
            panic!();
        }
        let pos = check_instruction(instruction);
        tmp.retain(|t| pos.contains(t));
    }
    let keys = possible.keys().copied().collect::<Vec<_>>();
    loop {
        let mut fixed = true;
        for k in &keys {
            let pos = possible.get(k).unwrap().clone();
            if pos.is_empty() {
                panic!();
            }
            if pos.len() > 1 {
                fixed = false;
                continue;
            }
            // Pos len is now 1
            let t = pos.first().unwrap();
            for k2 in &keys {
                if k == k2 {
                    continue;
                }
                possible.get_mut(k2).unwrap().retain(|t2| t2 != t);
            }
        }
        if fixed {
            break;
        }
    }
    possible
        .iter()
        .map(|(i, vec)| {
            assert_eq!(vec.len(), 1);
            (*i, *vec.first().unwrap())
        })
        .collect::<BTreeMap<_, _>>()
}

fn get_part2(instructions: &[Instruction], program: &Vec<(i32, i32, i32, i32)>) -> i64 {
    let mapping = get_mapping(instructions);
    let mut registers = [0, 0, 0, 0];
    for (op, inputa, inputb, output) in program {
        let operation = mapping.get(op).unwrap();
        match operation {
            Operation::AddR => {
                registers[*output as usize] =
                    registers[*inputa as usize] + registers[*inputb as usize];
            }
            Operation::AddI => {
                registers[*output as usize] = registers[*inputa as usize] + *inputb;
            }
            Operation::MultiR => {
                registers[*output as usize] =
                    registers[*inputa as usize] * registers[*inputb as usize];
            }
            Operation::MultiI => {
                registers[*output as usize] = registers[*inputa as usize] * *inputb;
            }
            Operation::BitwiseAndR => {
                registers[*output as usize] =
                    registers[*inputa as usize] & registers[*inputb as usize];
            }
            Operation::BitwiseAndI => {
                registers[*output as usize] = registers[*inputa as usize] & *inputb;
            }
            Operation::BitwiseOrR => {
                registers[*output as usize] =
                    registers[*inputa as usize] | registers[*inputb as usize];
            }
            Operation::BitwiseOrI => {
                registers[*output as usize] = registers[*inputa as usize] | *inputb;
            }
            Operation::SetR => {
                registers[*output as usize] = registers[*inputa as usize];
            }
            Operation::SetI => {
                registers[*output as usize] = *inputa;
            }
            Operation::GreaterRI => {
                registers[*output as usize] = if registers[*inputa as usize] > *inputb {
                    1
                } else {
                    0
                };
            }
            Operation::GreaterIR => {
                registers[*output as usize] = if *inputa > registers[*inputb as usize] {
                    1
                } else {
                    0
                };
            }
            Operation::GreaterRR => {
                registers[*output as usize] =
                    if registers[*inputa as usize] > registers[*inputb as usize] {
                        1
                    } else {
                        0
                    };
            }
            Operation::EqualityRI => {
                registers[*output as usize] = if *inputb == registers[*inputa as usize] {
                    1
                } else {
                    0
                };
            }
            Operation::EqualityIR => {
                registers[*output as usize] = if registers[*inputb as usize] == *inputa {
                    1
                } else {
                    0
                };
            }
            Operation::EqualityRR => {
                registers[*output as usize] =
                    if registers[*inputa as usize] == registers[*inputb as usize] {
                        1
                    } else {
                        0
                    };
            }
        }
    }
    registers[0] as i64
}

fn read_program(cont: &str) -> Vec<(i32, i32, i32, i32)> {
    let mut iter = cont.lines();
    let mut program = Vec::new();
    loop {
        let line = iter.next();
        if line.is_none() {
            break;
        }
        if line.unwrap().starts_with("Before") {
            iter.next();
            iter.next();
            iter.next();
            continue;
        }
        if line.unwrap().is_empty() {
            continue;
        }
        program.push(
            line.unwrap()
                .split_whitespace()
                .map(|c| c.trim().parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap(),
        );
    }
    program
}

fn read_instructions(cont: &str) -> Vec<Instruction> {
    let mut inst = Vec::new();
    for mut chunk in &cont.lines().chunks(4) {
        let first = &chunk.next().unwrap();
        let second = &chunk.next().unwrap();
        let third = &chunk.next().unwrap();
        if !first.starts_with("Before") {
            break;
        }

        let before = first
            .split_once('[')
            .unwrap()
            .1
            .strip_suffix(']')
            .unwrap()
            .split(',')
            .map(|c| c.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let (operation, inputa, inputb, output) = second
            .split_whitespace()
            .map(|c| c.trim().parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        let after = third
            .split_once('[')
            .unwrap()
            .1
            .strip_suffix(']')
            .unwrap()
            .split(',')
            .map(|c| c.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        inst.push(Instruction {
            operation,
            inputa,
            inputb,
            output,
            before,
            after,
        })
    }
    inst
}

fn read_contents(cont: &str) -> (i64, i64) {
    let instructions = read_instructions(cont);
    let program = read_program(cont);
    let part1 = get_part1(&instructions);
    let part2 = get_part2(&instructions, &program);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
";
        let inst = read_instructions(&a);
        assert_eq!(check_instruction(&inst[0]).len(), 3);

        assert_eq!(
            check_instruction(&inst[0]),
            [Operation::AddI, Operation::MultiR, Operation::SetI]
        );
    }
}
