use clap::Parser;
use colored::Colorize;
use shared::Dir;
use shared::Vec2D;
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
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i64, i64) {
    let grid = read_grid(cont);
    let part1 = get_part1(&grid, 10000);
    let part2 = get_part2(&grid, 10_000_000);
    (part1, part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Object {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec2D, Object>,
    center: Vec2D,
}

impl Grid {
    fn print_grid(&self, virus: &Virus) {
        let min_x = self.grid.keys().map(|v| v.x).min().unwrap() - 1;
        let max_x = self.grid.keys().map(|v| v.x).max().unwrap() + 1;
        let min_y = self.grid.keys().map(|v| v.y).min().unwrap() - 1;
        let max_y = self.grid.keys().map(|v| v.y).max().unwrap() + 1;

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if virus.loc == (Vec2D { x, y }) {
                    match self.grid.get(&Vec2D { x, y }) {
                        Some(Object::Infected) => {
                            print!("{}", virus.dir.get_char().to_string().red().on_black());
                        }
                        Some(Object::Clean) | None => {
                            print!("{}", virus.dir.get_char().to_string().yellow().on_black());
                        }
                        Some(Object::Flagged) => {
                            print!("{}", virus.dir.get_char().to_string().blue().on_black());
                        }
                        Some(Object::Weakened) => {
                            print!("{}", virus.dir.get_char().to_string().magenta().on_black());
                        }
                    }
                } else {
                    match self.grid.get(&Vec2D { x, y }) {
                        Some(Object::Infected) => {
                            print!("{}", "#".red().on_black());
                        }
                        Some(Object::Clean) | None => {
                            print!("{}", ".".white().on_black());
                        }
                        Some(Object::Flagged) => {
                            print!("{}", "F".blue().on_black());
                        }
                        Some(Object::Weakened) => {
                            print!("{}", "W".magenta().on_black());
                        }
                    }
                }
            }
            println!();
        }
    }
}

fn read_grid(cont: &str) -> Grid {
    let mut grid = BTreeMap::new();
    let mut max_x = 0;
    let mut min_y = 0;
    for (y, line) in cont.lines().enumerate() {
        min_y = min_y.min(-(y as i64));
        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x as i64);
            let obj = match c {
                '#' => Object::Infected,
                '.' => Object::Clean,
                _ => panic!("Unknown char in input: {}", c),
            };
            grid.insert(
                Vec2D {
                    x: x as i64,
                    y: -(y as i64),
                },
                obj,
            );
        }
    }
    let center = Vec2D {
        x: max_x / 2,
        y: min_y / 2,
    };
    Grid { grid, center }
}

struct Virus {
    loc: Vec2D,
    dir: Dir,
}

impl Virus {
    fn new() -> Self {
        Virus {
            loc: Vec2D { x: 0, y: 0 },
            dir: Dir::N,
        }
    }

    fn step(&mut self) {
        self.loc = self.loc + self.dir.get_dir_true_vec();
    }
}

fn get_part1(grid: &Grid, steps: usize) -> i64 {
    let mut grid = grid.clone();
    let mut virus = Virus::new();
    virus.loc = grid.center;
    grid.print_grid(&virus);
    let mut activations = 0;
    for _ in 0..steps {
        match grid.grid.get(&virus.loc) {
            Some(Object::Infected) => {
                virus.dir = virus.dir.cw();
                grid.grid.insert(virus.loc, Object::Clean);
            }
            _ => {
                virus.dir = virus.dir.ccw();
                grid.grid.insert(virus.loc, Object::Infected);
                activations += 1;
            }
        }
        virus.step();
    }
    grid.print_grid(&virus);
    activations
}

fn get_part2(grid: &Grid, steps: usize) -> i64 {
    let mut grid = grid.clone();
    let mut virus = Virus::new();
    virus.loc = grid.center;
    grid.print_grid(&virus);
    let mut activations = 0;
    for i in 0..steps {
        if i % 500_000 == 0 {
            println!("Step {} / {}", i + 1, steps);
        }
        match grid.grid.get(&virus.loc) {
            Some(Object::Weakened) => {
                //println!("Weakened, not turning");
                grid.grid.insert(virus.loc, Object::Infected);
                activations += 1;
            }
            Some(Object::Infected) => {
                //println!("Infected, turning right");
                virus.dir = virus.dir.cw();
                grid.grid.insert(virus.loc, Object::Flagged);
            }
            Some(Object::Flagged) => {
                //println!("Flagged, going back");
                virus.dir = virus.dir.opposite();
                grid.grid.insert(virus.loc, Object::Clean);
            }
            _ => {
                //println!("Clean, turning left");
                virus.dir = virus.dir.ccw();
                grid.grid.insert(virus.loc, Object::Weakened);
            }
        }
        virus.step();
        if steps < 20 {
            grid.print_grid(&virus);
        }
    }
    //grid.print_grid(&virus);
    activations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "..#
#..
...";
        let grid = read_grid(a);
        assert_eq!(get_part1(&grid, 7), 5);
        assert_eq!(get_part1(&grid, 70), 41);
        assert_eq!(get_part1(&grid, 10_000), 5587);
    }

    #[test]
    fn part2() {
        let a = "..#
#..
...";

        let grid = read_grid(a);
        assert_eq!(get_part2(&grid, 7), 1);
        assert_eq!(get_part2(&grid, 100), 26);
        assert_eq!(get_part2(&grid, 10_000_000), 2511944);
    }
}
