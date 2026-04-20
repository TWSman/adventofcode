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
    println!("Part 1 answer is {}", res);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Address {
    Number(i32),
    Register(char),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    Copy(Address, Address),
    Jump(Address, Address),
    Increment(Address),
    Decrease(Address),
    Transmit(Address),
}

impl Address {
    fn new(ln: &str) -> Self {
        if let Ok(num) = ln.parse::<i32>() {
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
            "cpy" => Self::Copy(Address::new(words[1]), Address::new(words[2])),
            "inc" => Self::Increment(Address::new(words[1])),
            "dec" => Self::Decrease(Address::new(words[1])),
            "jnz" => Self::Jump(Address::new(words[1]), Address::new(words[2])),
            "out" => Self::Transmit(Address::new(words[1])),
            _ => panic!(),
        }
    }
}

fn get_part1(program: &[Instruction]) -> i32 {
    let mut i = 0;
    loop {
        if i > 10_000 {
            break;
        }
        if i % 10 == 0 {
            println!("Trying with start value {i}");
        }
        if check_start(program, i) {
            return i;
        }
        i += 1;
    }
    0
}

fn check_start(program: &[Instruction], start: i32) -> bool {
    let program: Vec<Instruction> = program.to_vec();
    let mut states = std::collections::HashSet::new();
    let mut ind: i32 = 0;
    let mut registers = std::collections::HashMap::new();
    let mut loop_count = 0;
    let mut output_count = 0;
    registers.insert('a', start);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);
    let mut prev = None;
    loop {
        if ind >= i32::try_from(program.len()).unwrap() {
            break;
        }
        loop_count += 1;
        let output = format!(
            "{ind}_a{}_b{}_c{}_d{}",
            registers.get(&'a').unwrap(),
            registers.get(&'b').unwrap(),
            registers.get(&'c').unwrap(),
            registers.get(&'d').unwrap(),
        );
        if states.contains(&output) {
            println!("Loop detected: {output} after {loop_count} loops and {output_count} outputs");
            return true;
        }
        states.insert(output);
        let inst = program.get(ind as usize).unwrap();
        match inst {
            Instruction::Increment(Address::Register(c)) => {
                //println!("{ind}: Increment {c}");
                *registers.get_mut(c).unwrap() += 1;
            }
            Instruction::Transmit(a) => {
                let x = match a {
                    Address::Number(n) => *n,
                    Address::Register(cc) => *registers.get(cc).unwrap(),
                };
                //println!("Transmit: {x}, ");
                output_count += 1;
                if prev.is_none() {
                    prev = Some(x);
                } else if prev.unwrap() == x {
                    //println!("Same output twice in a row");
                    return false;
                } else {
                    prev = Some(x);
                }
            }
            Instruction::Decrease(Address::Register(c)) => {
                //println!("{ind}: Decrease {c}");
                *registers.get_mut(c).unwrap() -= 1;
            }
            Instruction::Copy(a, Address::Register(c)) => {
                let b = match a {
                    Address::Number(n) => *n,
                    Address::Register(cc) => *registers.get(cc).unwrap(),
                };
                //println!("{ind}: Copying {b} to {c}");
                *registers.get_mut(c).unwrap() = b;
            }
            Instruction::Jump(a, b) => {
                let aa = match a {
                    Address::Number(n) => *n,
                    Address::Register(cc) => *registers.get(cc).unwrap(),
                };
                let bb = match b {
                    Address::Number(n) => *n,
                    Address::Register(cc) => *registers.get(cc).unwrap(),
                };
                //println!("{ind}: Jumping {bb} steps if {aa} is not zero");
                if aa != 0 {
                    ind += bb;
                    continue;
                }
            }
            _ => {}
        }
        ind += 1;
    }
    false
}

fn read_contents(cont: &str) -> i32 {
    let program: Vec<Instruction> = cont.lines().map(Instruction::new).collect();
    get_part1(&program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "cpy 2 a
cpy 1 a
dec a
dec a";
        let program: Vec<Instruction> = a.lines().map(Instruction::new).collect();
        assert_eq!(check_start(&program, 0), false);
    }
}
