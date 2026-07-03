use clap::Parser;
use shared::Vec2D;
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

fn read_contents(cont: &str) -> (i64, i64) {
    let list = cont.lines().map(Instruction::new).collect::<Vec<_>>();
    let part1 = get_part1(&list);
    let part2 = get_part2(&list);
    (part1, part2)
}

enum Direction {
    Down,
    Up,
    Forward,
}

struct Instruction {
    dir: Direction,
    steps: i64,
}

impl Instruction {
    fn new(ln: &str) -> Self {
        let (a, b) = ln.split_once(" ").unwrap();
        let steps = b.parse::<i64>().unwrap();
        let dir = match a {
            "down" => Direction::Down,
            "up" => Direction::Up,
            "forward" => Direction::Forward,
            _ => panic!(),
        };

        Self { dir, steps }
    }

    fn to_vec(&self) -> Vec2D {
        match self.dir {
            // Positive y is downwards
            Direction::Down => Vec2D {
                x: 0,
                y: self.steps,
            },
            Direction::Up => Vec2D {
                x: 0,
                y: -self.steps,
            },
            Direction::Forward => Vec2D {
                x: self.steps,
                y: 0,
            },
        }
    }
}

struct Submarine {
    pos: Vec2D,
    aim_vec: Vec2D,
}

impl Submarine {
    fn new() -> Self {
        Self {
            pos: Vec2D { x: 0, y: 0 },
            aim_vec: Vec2D { x: 1, y: 0 },
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        let d = instruction.steps;
        match instruction.dir {
            // Positive y is downwards
            Direction::Down => self.aim_vec.y += d,
            Direction::Up => self.aim_vec.y -= d,
            Direction::Forward => {
                self.pos = self.pos + self.aim_vec * d;
            }
        }
    }
}

fn get_part1(list: &[Instruction]) -> i64 {
    let mut submarine = Submarine::new();
    for inst in list {
        submarine.pos = submarine.pos + inst.to_vec();
    }
    submarine.pos.x * submarine.pos.y
}

fn get_part2(list: &[Instruction]) -> i64 {
    let mut submarine = Submarine::new();
    for inst in list {
        submarine.apply(inst);
    }
    submarine.pos.x * submarine.pos.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(read_contents(&a).0, 150);
    }

    #[test]
    fn part2() {
        let a = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(read_contents(&a).1, 900);
    }
}
