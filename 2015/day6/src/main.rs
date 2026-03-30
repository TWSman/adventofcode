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
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i32, i32) {
    let lines = cont.lines().map(Line::new).collect::<Vec<_>>();
    dbg!(&lines.len());
    let part1 = get_part1(&lines);
    let part2 = get_part2(&lines);
    (part1, part2)
}

#[derive(Debug, PartialEq, Eq)]
struct Area {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

impl Area {
    fn new(s: &str) -> Self {
        let mut splits = s.split_whitespace();
        let mut start = splits.next().unwrap().split(',');
        splits.next();
        let mut end = splits.next().unwrap().split(',');
        Area {
            start_x: start.next().unwrap().parse().unwrap(),
            start_y: start.next().unwrap().parse().unwrap(),
            end_x: end.next().unwrap().parse().unwrap(),
            end_y: end.next().unwrap().parse().unwrap(),
        }
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        self.start_x <= x && x <= self.end_x && self.start_y <= y && y <= self.end_y
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Instruction {
    fn new(s: &str) -> Self {
        match s {
            "turn on" => Instruction::TurnOn,
            "turn off" => Instruction::TurnOff,
            "toggle" => Instruction::Toggle,
            _ => panic!("Unknown instruction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    instruction: Instruction,
    area: Area,
}

impl Line {
    fn new(s: &str) -> Self {
        let split = s.split_whitespace().collect::<Vec<_>>();
        let (instruction, area_str) = if split.len() == 4 {
            (
                Instruction::new(split[0]),
                format!("{} {} {}", split[1], split[2], split[3]),
            )
        } else {
            (
                Instruction::new(format!("{} {}", split[0], split[1]).as_str()),
                format!("{} {} {}", split[2], split[3], split[4]),
            )
        };

        Line {
            instruction,
            area: Area::new(&area_str),
        }
    }
}

fn get_part1(vec: &[Line]) -> i32 {
    let mut lights_on = 0;
    for x in 0..=999 {
        for y in 0..999 {
            let mut stat = 0;
            for line in vec {
                if !line.area.contains(x, y) {
                    continue;
                }
                match line.instruction {
                    Instruction::TurnOn => stat = 1,
                    Instruction::TurnOff => stat = 0,
                    Instruction::Toggle => stat = 1 - stat,
                }
            }
            if stat == 1 {
                lights_on += 1;
            }
        }
    }
    lights_on
}

fn get_part2(vec: &[Line]) -> i32 {
    let mut total_brightness = 0;
    for x in 0..=999 {
        for y in 0..999 {
            let mut stat = 0;
            for line in vec {
                if !line.area.contains(x, y) {
                    continue;
                }
                match line.instruction {
                    Instruction::TurnOn => stat += 1,
                    Instruction::TurnOff => stat = 0.max(stat - 1),
                    Instruction::Toggle => stat += 2,
                }
            }
            total_brightness += stat;
        }
    }
    total_brightness
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        assert_eq!(Area::new("0,0 through 999,999").end_y, 999);
    }

    #[test]
    fn line() {
        assert_eq!(Line::new("turn on 0,0 through 999,999").area.end_y, 999);
        assert_eq!(Line::new("toggle 0,0 through 999,0").area.end_y, 0);
        assert_eq!(Line::new("toggle 0,0 through 999,0").instruction, Instruction::Toggle);
    }
}
