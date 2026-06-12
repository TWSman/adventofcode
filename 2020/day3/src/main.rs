use clap::Parser;
use colored::Colorize;
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
    grid.print_grid(None);
    let part1 = get_part1(&grid);
    let part2 = get_part2(&grid);
    (part1, part2)
}

#[derive(Debug, Clone)]
enum Object {
    Tree,
    Open,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec2D, Object>,
    width: i64,
}

impl Grid {
    fn get_loc(&self, loc: &Vec2D) -> Option<&Object> {
        let get_loc = Vec2D {
            x: loc.x % self.width,
            y: loc.y,
        };
        self.grid.get(&get_loc)
    }

    fn print_grid(&self, loc: Option<Vec2D>) {
        let min_x = self.grid.keys().map(|v| v.x).min().unwrap();
        let max_x = self.grid.keys().map(|v| v.x).max().unwrap() + 11;
        let min_y = self.grid.keys().map(|v| v.y).min().unwrap();
        let max_y = self.grid.keys().map(|v| v.y).max().unwrap();

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let l = Vec2D { x, y };
                match self.get_loc(&l) {
                    Some(Object::Tree) if Some(l) == loc => {
                        print!("{}", '#'.to_string().white().on_black());
                    }
                    Some(Object::Tree) => {
                        print!("{}", '#'.to_string().red().on_black());
                    }
                    Some(Object::Open) if Some(l) == loc => {
                        print!("{}", 'O'.to_string().white().on_black());
                    }
                    Some(Object::Open) => {
                        print!("{}", '.'.to_string().white().on_black());
                    }
                    None => {
                        print!("{}", ".".white().on_black());
                    }
                }
            }
            println!();
        }
    }
}

fn read_grid(cont: &str) -> Grid {
    let grid = cont
        .lines()
        .enumerate()
        .fold(BTreeMap::new(), |mut grid, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let obj = match c {
                    '#' => Object::Tree,
                    '.' => Object::Open,
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
    let width = grid.keys().map(|v| v.x).max().unwrap() + 1;
    Grid { grid, width }
}

fn get_part1(grid: &Grid) -> i64 {
    get_trees(grid, Vec2D { x: 3, y: -1 })
}

fn get_trees(grid: &Grid, dx: Vec2D) -> i64 {
    let mut loc = Vec2D { x: 0, y: 0 };
    let mut trees = 0;
    dbg!(&loc);
    grid.print_grid(Some(loc));
    loop {
        loc = loc + dx;
        match grid.get_loc(&loc) {
            Some(Object::Tree) => {
                trees += 1;
            }
            Some(Object::Open) => {}
            None => {
                break;
            }
        }
    }
    trees
}

fn get_part2(grid: &Grid) -> i64 {
    let mut multi = 1;
    for dx in [
        Vec2D { x: 1, y: -1 },
        Vec2D { x: 3, y: -1 },
        Vec2D { x: 5, y: -1 },
        Vec2D { x: 7, y: -1 },
        Vec2D { x: 1, y: -2 },
    ] {
        multi *= get_trees(grid, dx);
    }
    multi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let grid = read_grid(a);
        grid.print_grid(None);
        assert_eq!(get_part1(&grid), 7);
    }

    #[test]
    fn part2() {
        let a = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let grid = read_grid(a);
        assert_eq!(get_part2(&grid), 336);
    }
}
