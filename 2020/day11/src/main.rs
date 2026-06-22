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
    grid.print_grid();
    let part1 = get_answer(&grid, false);
    let part2 = get_answer(&grid, true);
    (part1, part2)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Object {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec2D, Object>,
}

impl Grid {
    fn print_grid(&self) {
        let min_x = self.grid.keys().map(|v| v.x).min().unwrap() - 1;
        let max_x = self.grid.keys().map(|v| v.x).max().unwrap() + 1;
        let min_y = self.grid.keys().map(|v| v.y).min().unwrap() - 1;
        let max_y = self.grid.keys().map(|v| v.y).max().unwrap() + 1;

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let l = Vec2D { x, y };
                match self.grid.get(&l) {
                    Some(Object::Occupied) => {
                        print!("{}", '#'.to_string().red().on_black());
                    }
                    Some(Object::Empty) => {
                        print!("{}", 'L'.to_string().white().on_black());
                    }
                    Some(Object::Floor) => {
                        print!("{}", '.'.to_string().white().on_black());
                    }
                    None => {
                        print!("{}", ".".white().on_white());
                    }
                }
            }
            println!();
        }
    }

    fn evolve(&mut self, part2: bool) -> bool {
        let mut changed = false;
        let mut new_grid = self.grid.clone();
        let all_dir = [
            AllDir::N,
            AllDir::NE,
            AllDir::E,
            AllDir::SE,
            AllDir::S,
            AllDir::SW,
            AllDir::W,
            AllDir::NW,
        ];
        let threshold = if part2 { 5 } else { 4 };
        for (loc, state) in self.grid.iter() {
            if *state == Object::Floor {
                continue;
            }
            let occupied = if part2 {
                let mut count = 0;
                for dir in all_dir {
                    let mut tmp = *loc;
                    loop {
                        tmp = tmp + dir.get_dir_true_vec();
                        match self.grid.get(&tmp).unwrap_or(&Object::Empty) {
                            Object::Empty => {
                                break;
                            }
                            Object::Occupied => {
                                count += 1;
                                break;
                            }
                            Object::Floor => {
                                continue;
                            }
                        }
                    }
                }
                count
            } else {
                all_dir
                    .iter()
                    .filter(|dir| {
                        self.grid
                            .get(&(*loc + dir.get_dir_true_vec()))
                            .unwrap_or(&Object::Floor)
                            == &Object::Occupied
                    })
                    .count()
            };
            match state {
                Object::Empty if occupied == 0 => {
                    changed = true;
                    new_grid.insert(*loc, Object::Occupied);
                }
                Object::Occupied if occupied >= threshold => {
                    changed = true;
                    new_grid.insert(*loc, Object::Empty);
                }
                _ => {}
            }
        }
        self.grid = new_grid;
        changed
    }
}

fn read_grid(cont: &str) -> Grid {
    let grid = cont
        .lines()
        .enumerate()
        .fold(BTreeMap::new(), |mut grid, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let obj = match c {
                    '#' => Object::Occupied,
                    'L' => Object::Empty,
                    '.' => Object::Floor,
                    c => panic!("Unknown character: {c} in grid"),
                };
                grid.insert(
                    Vec2D {
                        x: x as i64,
                        y: -(y as i64),
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
    let mut i = 0;
    loop {
        i += 1;
        if i > 1_000 {
            break;
        }
        let res = grid.evolve(part2);
        if !res {
            println!("No changes after round {i}");
            grid.print_grid();
            return grid
                .grid
                .iter()
                .filter(|(_i, c)| **c == Object::Occupied)
                .count() as i64;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let grid = read_grid(a);
        grid.print_grid();
        assert_eq!(get_answer(&grid, false), 37);
    }

    #[test]
    fn part2() {
        let a = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let grid = read_grid(a);
        grid.print_grid();
        assert_eq!(get_answer(&grid, true), 26);
    }
}
