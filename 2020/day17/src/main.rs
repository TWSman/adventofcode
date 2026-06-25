use clap::Parser;
use colored::Colorize;
use shared::Vec4D;
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
    grid.print_grid();
    let part1 = get_answer(&grid, false);
    let part2 = get_answer(&grid, true);
    (part1, part2)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum State {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec4D, State>,
}

impl Grid {
    fn print_grid(&self) {
        let min_x = self.grid.keys().map(|v| v.x).min().unwrap() - 1;
        let max_x = self.grid.keys().map(|v| v.x).max().unwrap() + 1;
        let min_y = self.grid.keys().map(|v| v.y).min().unwrap() - 1;
        let max_y = self.grid.keys().map(|v| v.y).max().unwrap() + 1;
        let min_z = self.grid.keys().map(|v| v.z).min().unwrap();
        let max_z = self.grid.keys().map(|v| v.z).max().unwrap();
        let min_t = self.grid.keys().map(|v| v.t).min().unwrap();
        let max_t = self.grid.keys().map(|v| v.t).max().unwrap();
        let xwidth = max_x - min_x + 3;

        for t in min_t..=max_t {
            println!("t = {t}:");
            for z in min_z..=max_z {
                let tmp = format!("z = {z}:");
                print!("{tmp:width$}", width = xwidth as usize);
            }
            println!();

            for y in (min_y..=max_y).rev() {
                for z in min_z..=max_z {
                    for x in min_x..=max_x {
                        let l = Vec4D { x, y, z, t };
                        match self.grid.get(&l) {
                            Some(State::Active) => {
                                print!("{}", '#'.to_string().red().on_black());
                            }
                            _ => {
                                print!("{}", '.'.to_string().white().on_black());
                            }
                        }
                    }
                    print!("  ");
                }
                println!();
            }
            println!();
        }
    }

    fn evolve(&mut self, part2: bool) -> bool {
        let mut changed = false;
        let mut new_grid = self.grid.clone();
        let mut all_dir = Vec::new();
        for t in [-1, 0, 1] {
            if !part2 && t != 0 {
                continue;
            }
            for x in [-1, 0, 1] {
                for y in [-1, 0, 1] {
                    for z in [-1, 0, 1] {
                        if t == 0 && x == 0 && y == 0 && z == 0 {
                            continue;
                        }
                        all_dir.push(Vec4D { x, y, z, t });
                    }
                }
            }
        }

        let min_x = self.grid.keys().map(|v| v.x).min().unwrap() - 1;
        let max_x = self.grid.keys().map(|v| v.x).max().unwrap() + 1;
        let min_y = self.grid.keys().map(|v| v.y).min().unwrap() - 1;
        let max_y = self.grid.keys().map(|v| v.y).max().unwrap() + 1;
        let min_z = self.grid.keys().map(|v| v.z).min().unwrap() - 1;
        let max_z = self.grid.keys().map(|v| v.z).max().unwrap() + 1;
        let min_t = self.grid.keys().map(|v| v.z).min().unwrap() - 1;
        let max_t = self.grid.keys().map(|v| v.z).max().unwrap() + 1;

        for t in min_t..=max_t {
            if !part2 && t != 0 {
                continue;
            }
            for z in min_z..=max_z {
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        let loc = Vec4D { x, y, z, t };
                        let state = self.grid.get(&loc).unwrap_or(&State::Inactive);
                        let active_neighbors = all_dir
                            .iter()
                            .filter(|dir| {
                                self.grid.get(&(loc + **dir)).unwrap_or(&State::Inactive)
                                    == &State::Active
                            })
                            .count();
                        match state {
                            State::Active if !(2..=3).contains(&active_neighbors) => {
                                changed = true;
                                new_grid.insert(loc, State::Inactive);
                            }
                            State::Inactive if active_neighbors == 3 => {
                                changed = true;
                                new_grid.insert(loc, State::Active);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        self.grid = new_grid;
        changed
    }

    fn active_count(&self) -> usize {
        self.grid.values().filter(|v| **v == State::Active).count()
    }
}

fn read_grid(cont: &str) -> Grid {
    let grid = cont
        .lines()
        .enumerate()
        .fold(BTreeMap::new(), |mut grid, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let obj = match c {
                    '#' => State::Active,
                    '.' => State::Inactive,
                    c => panic!("Unknown character: {c} in grid"),
                };
                grid.insert(
                    Vec4D {
                        x: x as i64,
                        y: -(y as i64),
                        z: 0,
                        t: 0,
                    },
                    obj,
                );
            });
            grid
        });
    Grid { grid }
}

fn get_answer(grid: &Grid, part2: bool) -> i64 {
    let mut grid = grid.clone();
    for _ in 0..6 {
        grid.evolve(part2);
    }
    grid.active_count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = ".#.
..#
###";
        let grid = read_grid(a);
        grid.print_grid();
        assert_eq!(get_answer(&grid, false), 112);
    }

    #[test]
    fn part2() {
        let a = ".#.
..#
###";
        let grid = read_grid(a);
        grid.print_grid();
        assert_eq!(get_answer(&grid, true), 848);
    }
}
