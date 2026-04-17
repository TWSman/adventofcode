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
    Copy(Address, Address),
    Jump(Address, Address),
    Increment(Address),
    Decrease(Address),
    Toggle(Address),
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
            "tgl" => Self::Toggle(Address::new(words[1])),
            _ => panic!(),
        }
    }
}

fn get_part2(program: &[Instruction]) -> i32 {
    let mut diff = None;
    for start in [6, 7, 8, 9] {
        // The outputs seem to differ from n! by a constant amount, so we can just calculate that constant and apply it to the answer for 12.
        let factorial = (1..=start).product::<i32>();
        let res = get_answer(program, start);
        if diff.is_none() {
            diff = Some(res - factorial);
        } else {
            // Check that the pattern holds, this does not hold for the example program
            assert_eq!(res - factorial, diff.unwrap());
        }
        println!("Start: {start}, Result: {res}");
    }
    (1..=12).product::<i32>() + diff.unwrap()
}

fn get_answer(program: &[Instruction], start: i32) -> i32 {
    let mut program: Vec<Instruction> = program.to_vec();
    let mut ind: i32 = 0;
    let mut registers = std::collections::HashMap::new();
    registers.insert('a', start);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);
    loop {
        if ind >= i32::try_from(program.len()).unwrap() {
            break;
        }
        let inst = program.get(ind as usize).unwrap();
        match inst {
            Instruction::Increment(Address::Register(c)) => {
                //println!("{ind}: Increment {c}");
                *registers.get_mut(c).unwrap() += 1;
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
            Instruction::Toggle(a) => {
                let x = match a {
                    Address::Number(n) => *n,
                    Address::Register(cc) => *registers.get(cc).unwrap(),
                };
                let target_ind = ind + x;
                //println!("{ind}: Toggling instruction at index {target_ind}: ");
                let target_inst = program.get(target_ind as usize);
                let new_inst = match target_inst {
                    None => {
                        //println!("(out of bounds)");
                        ind += 1;
                        continue;
                    }
                    Some(Instruction::Increment(x)) => Instruction::Decrease(*x),
                    Some(Instruction::Decrease(x)) | Some(Instruction::Toggle(x)) => {
                        Instruction::Increment(*x)
                    }
                    Some(Instruction::Copy(a, b)) => Instruction::Jump(*a, *b),
                    Some(Instruction::Jump(a, b)) => Instruction::Copy(*a, *b),
                };
                //println!(" to {:?} ", new_inst);
                if let Some(x) = program.get_mut(target_ind as usize) {
                    *x = new_inst;
                }
            }
            _ => {}
        }
        ind += 1;
    }
    *registers.get(&'a').unwrap()
}

fn read_contents(cont: &str) -> (i32, i32) {
    let program: Vec<Instruction> = cont.lines().map(Instruction::new).collect();
    let part1 = get_answer(&program, 7);
    let part2 = get_part2(&program);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";
        let program: Vec<Instruction> = a.lines().map(Instruction::new).collect();
        assert_eq!(get_answer(&program, 0), 3);
    }
}
