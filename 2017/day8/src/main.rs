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

#[derive(Debug)]
struct Instruction {
    target: String,
    operation: Operation,
    check_register: String,
    check: Check,
}

impl Instruction {
    fn new(ln: &str) -> Self {
        let parts = ln.split(' ').collect::<Vec<_>>();
        Self {
            target: parts[0].to_string(),
            operation: match parts[1] {
                "inc" => Operation::Increase(parts[2].parse::<i32>().unwrap()),
                "dec" => Operation::Decrease(parts[2].parse::<i32>().unwrap()),
                _ => panic!("Unknown operation {}", parts[1]),
            },
            // parts[3] is just 'if'
            check_register: parts[4].to_string(),
            check: Check::new(parts[5], parts[6].parse::<i32>().unwrap()),
        }
    }
}

#[derive(Debug)]
enum Check {
    Larger(i32),
    Smaller(i32),
    LargerOrEqual(i32),
    SmallerOrEqual(i32),
    Equal(i32),
    NotEqual(i32),
}

impl Check {
    fn new(str: &str, val: i32) -> Self {
        match str {
            ">" => Self::Larger(val),
            "<" => Self::Smaller(val),
            ">=" => Self::LargerOrEqual(val),
            "<=" => Self::SmallerOrEqual(val),
            "==" => Self::Equal(val),
            "!=" => Self::NotEqual(val),
            _ => panic!("Unknown check {}", str),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Increase(i32),
    Decrease(i32),
}

fn read_contents(cont: &str) -> (i32, i32) {
    let instructions = cont.lines().map(Instruction::new).collect::<Vec<_>>();
    dbg!(&instructions);
    get_answer(&instructions)
}

fn get_answer(list: &[Instruction]) -> (i32, i32) {
    let mut max_val: i32 = 0;
    let mut registers = BTreeMap::new();
    for inst in list {
        let target = &inst.target;
        if !registers.contains_key(target) {
            registers.insert(target.clone(), 0);
        }
        let check = inst.check_register.clone();
        if !registers.contains_key(&check) {
            registers.insert(check.clone(), 0);
        }
        let check_val = registers.get(&check).unwrap();
        let action = match inst.check {
            Check::Larger(val) => *check_val > val,
            Check::Smaller(val) => *check_val < val,
            Check::LargerOrEqual(val) => *check_val >= val,
            Check::SmallerOrEqual(val) => *check_val <= val,
            Check::Equal(val) => *check_val == val,
            Check::NotEqual(val) => *check_val != val,
        };
        if action {
            let target_val = registers.get(target).unwrap();
            let new_val = match inst.operation {
                Operation::Increase(val) => *target_val + val,
                Operation::Decrease(val) => *target_val - val,
            };
            if *target_val > max_val {
                max_val = *target_val;
            }
            registers.insert(target.clone(), new_val);
        }
        dbg!(&registers);
    }
    (*registers.values().max().unwrap(), max_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!(read_contents(&a).0, 1);
    }

    #[test]
    fn part2() {
        let a = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!(read_contents(&a).1, 10);
    }
}
