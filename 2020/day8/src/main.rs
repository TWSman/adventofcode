use clap::Parser;
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
enum Instruction {
    Accumulate(i64),
    Jump(i64),
    NoOp(i64),
}

impl Instruction {
    fn new(ln: &str) -> Self {
        let words = ln.split_whitespace().collect::<Vec<_>>();
        match words[0] {
            "nop" => Self::NoOp(words[1].parse::<i64>().unwrap()),
            "acc" => Self::Accumulate(words[1].parse::<i64>().unwrap()),
            "jmp" => Self::Jump(words[1].parse::<i64>().unwrap()),
            _ => panic!(),
        }
    }
}

fn get_part1(instructions: &[Instruction]) -> i64 {
    let mut seen = Vec::new();
    let mut acc = 0;
    let mut ind: i64 = 0;
    loop {
        if seen.contains(&ind) {
            break;
        }
        seen.push(ind);
        let inst = instructions.get(ind as usize).unwrap();
        match inst {
            Instruction::NoOp(_) => {
                ind += 1;
                continue;
            }
            Instruction::Accumulate(v) => {
                acc += v;
                ind += 1;
                continue;
            }
            Instruction::Jump(j) => {
                ind += j;
                continue;
            }
        }
    }
    acc
}

fn get_part2(instructions: &[Instruction]) -> i64 {
    let n = instructions.len();
    for i in (0..n).rev() {
        if let Instruction::Accumulate(_) = instructions[i] {
            continue;
        }
        let mut inst2 = instructions.to_owned();
        match instructions[i] {
            Instruction::NoOp(v) => {
                inst2[i] = Instruction::Jump(v);
            }
            Instruction::Jump(v) => {
                inst2[i] = Instruction::NoOp(v);
            }
            _ => panic!(),
        }
        match run_to_end(&inst2) {
            None => continue,
            Some(v) => {
                println!("Replaced index {i} out of {n}");
                return v;
            }
        }
    }
    0
}

fn run_to_end(instructions: &[Instruction]) -> Option<i64> {
    // Run the program until the next operation would be just outside the instruction vector
    // Return None If the program loops or tries to access other indices outside the program
    let n = instructions.len() as i64;
    let mut seen = Vec::new();
    let mut acc = 0;
    let mut ind: i64 = 0;
    loop {
        if seen.contains(&ind) {
            return None;
        }
        if ind == n {
            return Some(acc);
        }
        if ind > n {
            return None;
        }
        seen.push(ind);
        let inst = instructions.get(ind as usize).unwrap();
        match inst {
            Instruction::NoOp(_) => {
                ind += 1;
                continue;
            }
            Instruction::Accumulate(v) => {
                acc += v;
                ind += 1;
                continue;
            }
            Instruction::Jump(j) => {
                ind += j;
                continue;
            }
        }
    }
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
        let a = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(read_contents(&a).0, 5);
    }

    #[test]
    fn part2() {
        let a = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(read_contents(&a).1, 8);
    }
}
