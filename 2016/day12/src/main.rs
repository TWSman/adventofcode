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
enum Address {
    Number(i32),
    Register(char),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    Copy(Address, char),
    Increment(char),
    Decrease(char),
    Jump(Address, i32),
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
            "cpy" => Self::Copy(Address::new(words[1]), words[2].chars().next().unwrap()),
            "inc" => Self::Increment(words[1].chars().next().unwrap()),
            "dec" => Self::Decrease(words[1].chars().next().unwrap()),
            "jnz" => Self::Jump(Address::new(words[1]), words[2].parse::<i32>().unwrap()),
            _ => panic!(),
        }
    }
}

fn get_answer(program: &[Instruction], part2: bool) -> i32 {
    let mut ind: i32 = 0;
    let mut registers = std::collections::HashMap::new();
    registers.insert('a', 0);
    registers.insert('b', 0);
    if part2 {
        // In part 2 c starts from 1
        registers.insert('c', 1);
    } else {
        registers.insert('c', 0);
    }
    registers.insert('d', 0);
    loop {
        if ind >= i32::try_from(program.len()).unwrap() {
            break;
        }
        let inst = program.get(ind as usize).unwrap();
        match inst {
            Instruction::Increment(c) => {
                *registers.get_mut(c).unwrap() += 1;
            }
            Instruction::Decrease(c) => {
                *registers.get_mut(c).unwrap() -= 1;
            }
            Instruction::Copy(a, c) => {
                let b = match a {
                    Address::Number(n) => *n,
                    Address::Register(cc) => *registers.get(cc).unwrap(),
                };
                *registers.get_mut(c).unwrap() = b;
            }
            Instruction::Jump(a, c) => {
                let b = match a {
                    Address::Number(n) => *n,
                    Address::Register(cc) => *registers.get(cc).unwrap(),
                };
                if b != 0 {
                    ind += c;
                    continue;
                }
            }
        }
        ind += 1;
    }
    *registers.get(&'a').unwrap()
}

fn read_contents(cont: &str) -> (i32, i32) {
    let program: Vec<Instruction> = cont.lines().map(Instruction::new).collect();
    dbg!(&program);
    let part1 = get_answer(&program, false);
    let part2 = get_answer(&program, true);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";
        assert_eq!(read_contents(&a).0, 42);
    }
}
