use clap::Parser;
use colored::Colorize;
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
    println!("Part 1 answer is {}", res);
    //println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    Rectangle(i64, i64),
    RotateRow(i64, i64),
    RotateColumn(i64, i64),
}

impl Instruction {
    fn new(ln: &str) -> Self {
        let re_rect = regex::Regex::new(r"rect (\d+)x(\d+)").unwrap();
        let re_rotate = regex::Regex::new(r"rotate (row|column) [xy]=(\d+) by (\d+)").unwrap();
        if ln.contains("rect") {
            let caps = re_rect.captures(ln).unwrap();
            Self::Rectangle(
                caps[1].parse::<i64>().unwrap(),
                caps[2].parse::<i64>().unwrap(),
            )
        } else {
            let caps = re_rotate.captures(ln).unwrap();
            let ind = caps[2].parse::<i64>().unwrap();
            let by = caps[3].parse::<i64>().unwrap();
            if &caps[1] == "row" {
                Self::RotateRow(ind, by)
            } else {
                Self::RotateColumn(ind, by)
            }
        }
    }
}

fn print_grid(grid: &BTreeSet<Vec2D>) {
    let width = 50;
    let height = 6;

    for y in 0..height {
        for x in 0..width {
            if grid.contains(&Vec2D { x, y }) {
                print!("{}", ".".black().on_red());
            } else {
                print!("{}", ".".black().on_green());
            }
        }
        println!();
    }
}

fn run(instructions: &Vec<Instruction>) -> usize {
    let mut grid: BTreeSet<Vec2D> = BTreeSet::new();
    for inst in instructions {
        let keys = grid.iter().collect::<Vec<_>>();
        let mut new_grid = BTreeSet::new();
        match inst {
            Instruction::Rectangle(width, height) => {
                for k in keys.iter() {
                    new_grid.insert(Vec2D { x: k.x, y: k.y });
                }
                for y in 0..*height {
                    for x in 0..*width {
                        new_grid.insert(Vec2D { x, y });
                    }
                }
            }
            Instruction::RotateRow(row, count) => {
                for k in keys.iter() {
                    if k.y == *row {
                        new_grid.insert(Vec2D {
                            x: (k.x + count) % 50,
                            y: k.y,
                        });
                    } else {
                        new_grid.insert(Vec2D { x: k.x, y: k.y });
                    }
                }
            }
            Instruction::RotateColumn(col, count) => {
                for k in keys.iter() {
                    if k.x == *col {
                        new_grid.insert(Vec2D {
                            x: k.x,
                            y: (k.y + count) % 6,
                        });
                    } else {
                        new_grid.insert(Vec2D { x: k.x, y: k.y });
                    }
                }
            }
        }
        grid = new_grid;
    }
    println!("Part2 Answer:");
    print_grid(&grid);
    grid.len()
}

fn read_contents(cont: &str) -> usize {
    let instructions = cont.lines().map(Instruction::new).collect::<Vec<_>>();
    run(&instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";
        assert_eq!(read_contents(&a), 6);
    }
}
