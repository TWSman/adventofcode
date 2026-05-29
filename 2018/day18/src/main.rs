use clap::Parser;
use colored::Colorize;
use shared::AllDir;
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
    let part1 = get_answer(&grid, 10);
    let part2 = get_answer(&grid, 1_000_000_000);
    (part1, part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Object {
    Open,
    Tree,
    LumberYard,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec2D, Object>,
    max_x: i64,
    min_x: i64,
    max_y: i64,
    min_y: i64,
}

impl Grid {
    fn print_grid(&self) {
        let min_x = self.min_x - 1;
        let max_x = self.max_x + 1;
        let min_y = self.min_y - 1;
        let max_y = self.max_y + 1;

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                match self.grid.get(&Vec2D { x, y }) {
                    Some(Object::LumberYard) => {
                        print!("{}", "#".red().on_black());
                    }
                    Some(Object::Open) => {
                        print!("{}", ".".white().on_black());
                    }
                    Some(Object::Tree) => {
                        print!("{}", "|".blue().on_black());
                    }
                    None => {
                        print!("{}", " ".white().on_white());
                    }
                }
            }
            println!();
        }
    }

    fn get_hash(&self) -> String {
        let min_x = self.min_x;
        let max_x = self.max_x;
        let min_y = self.min_y;
        let max_y = self.max_y;

        let mut out = String::new();
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let c = match self.grid.get(&Vec2D { x, y }) {
                    Some(Object::LumberYard) => '#',
                    Some(Object::Open) => '.',
                    Some(Object::Tree) => '|',
                    None => {
                        panic!();
                    }
                };
                out.push(c);
            }
        }
        out
    }

    fn get_score(&self) -> i64 {
        (self
            .grid
            .iter()
            .filter(|(_, o)| *o == &Object::Tree)
            .count()
            * self
                .grid
                .iter()
                .filter(|(_, o)| *o == &Object::LumberYard)
                .count()) as i64
    }

    fn evolve(&mut self) {
        let dirs = [
            AllDir::N,
            AllDir::S,
            AllDir::W,
            AllDir::E,
            AllDir::NW,
            AllDir::NE,
            AllDir::SW,
            AllDir::SE,
        ];
        let mut new_grid = self.grid.clone();
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let loc = Vec2D { x, y };
                match self.grid.get(&loc) {
                    None => panic!(),
                    Some(Object::Open) => {
                        let c = dirs
                            .iter()
                            .filter(|d| {
                                let l = loc + d.get_dir_true_vec();
                                self.grid.get(&l) == Some(&Object::Tree)
                            })
                            .count();
                        if c >= 3 {
                            new_grid.insert(loc, Object::Tree);
                        }
                    }
                    Some(Object::Tree) => {
                        let c = dirs
                            .iter()
                            .filter(|d| {
                                let l = loc + d.get_dir_true_vec();
                                self.grid.get(&l) == Some(&Object::LumberYard)
                            })
                            .count();
                        if c >= 3 {
                            new_grid.insert(loc, Object::LumberYard);
                        }
                    }
                    Some(Object::LumberYard) => {
                        let c_yard = dirs
                            .iter()
                            .filter(|d| {
                                let l = loc + d.get_dir_true_vec();
                                self.grid.get(&l) == Some(&Object::LumberYard)
                            })
                            .count();
                        let c_tree = dirs
                            .iter()
                            .filter(|d| {
                                let l = loc + d.get_dir_true_vec();
                                self.grid.get(&l) == Some(&Object::Tree)
                            })
                            .count();
                        if (c_yard == 0) | (c_tree == 0) {
                            new_grid.insert(loc, Object::Open);
                        }
                    }
                }
            }
        }
        self.grid = new_grid;
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
                '#' => Object::LumberYard,
                '.' => Object::Open,
                '|' => Object::Tree,
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

    let min_x = grid.keys().map(|v| v.x).min().unwrap();
    let max_x = grid.keys().map(|v| v.x).max().unwrap();
    let min_y = grid.keys().map(|v| v.y).min().unwrap();
    let max_y = grid.keys().map(|v| v.y).max().unwrap();
    Grid {
        grid,
        max_x,
        min_x,
        max_y,
        min_y,
    }
}

fn get_answer(grid: &Grid, steps: usize) -> i64 {
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    let mut grid = grid.clone();
    grid.print_grid();
    let mut i = 0;
    loop {
        i += 1;
        grid.evolve();
        if i == steps {
            break;
        }
        let hash = grid.get_hash();
        match seen.get(&hash) {
            Some(j) => {
                let loop_length = i - j;
                println!("{i}: State already seen at step {j}, loop length is {loop_length}");
                println!(
                    "{} steps remaining, remainder {}",
                    steps - i,
                    (steps - i) % loop_length
                );
                grid.print_grid();
                if (steps - i).is_multiple_of(loop_length) {
                    break;
                }
            }
            None => {
                seen.insert(hash, i);
                continue;
            }
        }
    }
    grid.get_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        let grid = read_grid(a);
        assert_eq!(get_part1(&grid, 10), 1147);
        assert_eq!(get_part2(&grid, 10), 1147);
    }

    #[test]
    fn part2() {
        let a = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        let grid = read_grid(a);
        assert_eq!(get_part2(&grid, 1_000_000_000), 1147);
    }
}
