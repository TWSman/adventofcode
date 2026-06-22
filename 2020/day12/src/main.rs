use clap::Parser;
use shared::Dir;
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
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i64, i64) {
    let instructions = cont.lines().map(Instruction::new).collect::<Vec<_>>();
    dbg!(&instructions);
    let part1 = get_answer(&instructions, false);
    let part2 = get_answer(&instructions, true);
    (part1, part2)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Instruction {
    Cardinal(Dir, i64),
    TurnLeft,
    TurnRight,
    Turn180,
    Forward(i64),
}

impl Instruction {
    fn new(ln: &str) -> Self {
        let c = &ln[..1];
        let rest = ln[1..].parse::<i64>().unwrap();
        match c {
            "R" if rest == 90 => Instruction::TurnRight,
            "R" if rest == 180 => Instruction::Turn180,
            "R" if rest == 270 => Instruction::TurnLeft,
            "L" if rest == 90 => Instruction::TurnLeft,
            "L" if rest == 180 => Instruction::Turn180,
            "L" if rest == 270 => Instruction::TurnRight,
            "N" => Instruction::Cardinal(Dir::N, rest),
            "E" => Instruction::Cardinal(Dir::E, rest),
            "S" => Instruction::Cardinal(Dir::S, rest),
            "W" => Instruction::Cardinal(Dir::W, rest),
            "F" => Instruction::Forward(rest),
            _ => panic!(),
        }
    }
}

struct State {
    dir: Dir,
    loc: Vec2D,
    waypoint: Vec2D, // Relative to ship location
}

impl State {
    fn new() -> Self {
        State {
            dir: Dir::E,
            loc: Vec2D { x: 0, y: 0 },
            waypoint: Vec2D { x: 10, y: 1 },
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::TurnRight => {
                self.dir = self.dir.cw();
            }
            Instruction::TurnLeft => {
                self.dir = self.dir.ccw();
            }
            Instruction::Turn180 => {
                self.dir = self.dir.opposite();
            }
            Instruction::Cardinal(dir, steps) => {
                self.loc = self.loc + dir.get_dir_true_vec() * steps;
            }
            Instruction::Forward(steps) => {
                self.loc = self.loc + self.dir.get_dir_true_vec() * steps;
            }
        }
    }

    fn apply2(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::TurnRight => {
                self.waypoint = Vec2D {
                    x: self.waypoint.y,
                    y: -self.waypoint.x,
                };
            }
            Instruction::TurnLeft => {
                self.waypoint = Vec2D {
                    x: -self.waypoint.y,
                    y: self.waypoint.x,
                };
            }
            Instruction::Turn180 => {
                self.waypoint = Vec2D {
                    x: -self.waypoint.x,
                    y: -self.waypoint.y,
                };
            }
            Instruction::Cardinal(dir, steps) => {
                self.waypoint = self.waypoint + dir.get_dir_true_vec() * steps;
            }
            Instruction::Forward(steps) => {
                self.loc = self.loc + self.waypoint * steps;
            }
        }
    }
}

fn get_answer(instructions: &[Instruction], part2: bool) -> i64 {
    let mut state = State::new();
    for inst in instructions {
        if part2 {
            state.apply2(inst);
        } else {
            state.apply(inst);
        }
    }
    state.loc.manhattan(&Vec2D { x: 0, y: 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "F10
N3
F7
R90
F11";
        assert_eq!(read_contents(&a).0, 25);
    }

    #[test]
    fn part2() {
        let a = "F10
N3
F7
R90
F11";
        assert_eq!(read_contents(&a).1, 286);
    }
}
