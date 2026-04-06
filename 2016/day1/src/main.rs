use clap::Parser;
use shared::Dir;
use shared::Vec2D;
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

#[derive(Debug)]
enum Instruction {
    Left(i64),
    Right(i64),
}

impl Instruction {
    fn new(ln: &str) -> Self {
        let ln = ln.trim();
        match ln.chars().next() {
            Some('L') => Instruction::Left(ln[1..].parse::<i64>().unwrap()),
            Some('R') => Instruction::Right(ln[1..].parse::<i64>().unwrap()),
            _ => panic!(),
        }
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let inst = cont.split(',').map(Instruction::new).collect::<Vec<_>>();
    let part1 = get_part1(&inst);
    let part2 = get_part2(&inst);
    (part1, part2)
}

fn get_part1(vec: &[Instruction]) -> i64 {
    let mut loc = Vec2D::new(0, 0);
    let mut dir = Dir::N;
    for inst in vec {
        match inst {
            Instruction::Left(c) => {
                dir = dir.ccw();
                loc = loc + dir.get_dir_true_vec() * *c;
            }
            Instruction::Right(c) => {
                dir = dir.cw();
                loc = loc + dir.get_dir_true_vec() * *c;
            }
        }
    }
    loc.manhattan(&Vec2D::new(0, 0))
}

fn get_part2(vec: &[Instruction]) -> i64 {
    let mut visited = BTreeSet::new();
    let mut loc = Vec2D::new(0, 0);
    visited.insert(loc);
    let mut dir = Dir::N;
    for inst in vec {
        let c = match inst {
            Instruction::Left(c) => {
                dir = dir.ccw();
                *c
            }
            Instruction::Right(c) => {
                dir = dir.cw();
                *c
            }
        };
        for _ in 0..c {
            loc = loc + dir.get_dir_true_vec();
            if visited.contains(&loc) {
                println!("Already visited {loc:?}");
                return loc.manhattan(&Vec2D::new(0, 0));
            }
            visited.insert(loc);
        }
    }
    loc.manhattan(&Vec2D::new(0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "R2, L3";
        assert_eq!(read_contents(&a).0, 5);

        let a = "R2, R2, R2";
        assert_eq!(read_contents(&a).0, 2);

        let a = "R5, L5, R5, R3";
        assert_eq!(read_contents(&a).0, 12);
    }

    #[test]
    fn part2() {
        let a = "R8, R4, R4, R8";
        assert_eq!(read_contents(&a).1, 4);
    }
}
