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
    Half(char),
    Triple(char),
    Increment(char),
    JumpIfOne((char, i32)),
    JumpIfEven((char, i32)),
    Jump(i32),
}

impl Instruction {
    fn new(ln: &str) -> Self {
        let words = ln.split_whitespace().collect::<Vec<_>>();
        match words[0] {
            "hlf" => Self::Half(words[1].chars().next().unwrap()),
            "tpl" => Self::Triple(words[1].chars().next().unwrap()),
            "inc" => Self::Increment(words[1].chars().next().unwrap()),
            "jmp" => Self::Jump(words[1].parse::<i32>().unwrap()),
            "jio" => Self::JumpIfOne((
                words[1].chars().next().unwrap(),
                words[2].parse::<i32>().unwrap(),
            )),
            "jie" => Self::JumpIfEven((
                words[1].chars().next().unwrap(),
                words[2].parse::<i32>().unwrap(),
            )),
            _ => panic!(),
        }
    }
}

fn get_answer(program: &[Instruction], part2: bool) -> i32 {
    // In part 2 a starts from 1
    let mut a: i32 = i32::from(part2);
    let mut b: i32 = 0;
    let mut ind: i32 = 0;
    loop {
        if ind >= i32::try_from(program.len()).unwrap() {
            break;
        }
        let inst = program.get(ind as usize).unwrap();
        match inst {
            Instruction::Half('a') => {
                a /= 2;
            }
            Instruction::Half('b') => {
                b /= 2;
            }
            Instruction::Increment('a') => {
                a += 1;
            }
            Instruction::Increment('b') => {
                b += 1;
            }
            Instruction::Triple('a') => {
                a *= 3;
            }
            Instruction::Triple('b') => {
                b *= 3;
            }
            Instruction::Jump(c) => {
                ind += c;
                continue;
            }
            Instruction::JumpIfOne(('a', c)) => {
                if a == 1 {
                    ind += c;
                    continue;
                }
            }
            Instruction::JumpIfOne(('b', c)) => {
                if b == 1 {
                    ind += c;
                    continue;
                }
            }
            Instruction::JumpIfEven(('a', c)) => {
                if a % 2 == 0 {
                    ind += c;
                    continue;
                }
            }
            Instruction::JumpIfEven(('b', c)) => {
                if b % 2 == 0 {
                    ind += c;
                    continue;
                }
            }
            _ => panic!(),
        }
        ind += 1;
    }
    b
}

fn read_contents(cont: &str) -> (i32, i32) {
    let program: Vec<Instruction> = cont.lines().map(Instruction::new).collect();
    let part1 = get_answer(&program, false);
    let part2 = get_answer(&program, true);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "inc a
jio a, +2
tpl a
inc a";
        assert_eq!(read_contents(&a).0, 0);

        let a = "inc b
jio b, +2
tpl b
inc b";
        assert_eq!(read_contents(&a).0, 2);
    }
}
