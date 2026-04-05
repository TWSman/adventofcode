use clap::Parser;
use colored::Colorize;
use shared::AllDir;
use shared::Vec2D;
use std::collections::BTreeMap;
use std::fs;
use std::time::Instant;
use strum::IntoEnumIterator;

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
    let res = read_contents(&contents, 100);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Light {
    On,
    Off,
}

fn read_grid(cont: &str) -> BTreeMap<Vec2D, Light> {
    cont.lines()
        .enumerate()
        .fold(BTreeMap::new(), |mut grid, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let obj = match c {
                    '#' => Light::On,
                    '.' => Light::Off,
                    c => panic!("Unknown character: {c} in grid"),
                };
                grid.insert(
                    Vec2D {
                        x: i64::try_from(x).unwrap(),
                        y: -i64::try_from(y).unwrap(),
                    },
                    obj,
                );
            });
            grid
        })
}

fn print_grid(grid: &BTreeMap<Vec2D, Light>) {
    let min_x = grid.keys().map(|v| v.x).min().unwrap() - 1;
    let max_x = grid.keys().map(|v| v.x).max().unwrap() + 1;
    let min_y = grid.keys().map(|v| v.y).min().unwrap() - 1;
    let max_y = grid.keys().map(|v| v.y).max().unwrap() + 1;

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            match grid.get(&Vec2D { x, y }) {
                Some(Light::On) => {
                    print!("{}", "#".blue().on_black());
                }
                Some(&Light::Off) => {
                    print!("{}", ".".white().on_black());
                }
                None => {
                    print!("{}", ".".white().on_white());
                }
            }
        }
        println!();
    }
}

fn get_answer(grid: &BTreeMap<Vec2D, Light>, steps: usize, part2: bool) -> usize {
    let mut grid = grid.clone();
    let min_x = grid.keys().map(|v| v.x).min().unwrap();
    let max_x = grid.keys().map(|v| v.x).max().unwrap();
    let min_y = grid.keys().map(|v| v.y).min().unwrap();
    let max_y = grid.keys().map(|v| v.y).max().unwrap();
    if part2 {
        grid.insert(Vec2D { x: min_x, y: min_y }, Light::On);
        grid.insert(Vec2D { x: min_x, y: max_y }, Light::On);
        grid.insert(Vec2D { x: max_x, y: min_y }, Light::On);
        grid.insert(Vec2D { x: max_x, y: max_y }, Light::On);
    }
    for _ in 0..steps {
        let mut new_grid = grid.clone();
        for (vec, ll) in &mut new_grid {
            if part2 && (vec.x == min_x || vec.x == max_x) && (vec.y == min_y || vec.y == max_y) {
                continue;
            }
            let mut l = 0;
            for dir in AllDir::iter() {
                if grid.get(&(*vec + dir.get_dir_true_vec())) == Some(&Light::On) {
                    l += 1;
                }
            }
            if ll == &Light::On {
                if l != 2 && l != 3 {
                    *ll = Light::Off;
                }
            } else if l == 3 {
                *ll = Light::On;
            }
        }
        grid = new_grid;
    }
    print_grid(&grid);
    grid.values().filter(|l| l == &&Light::On).count()
}

fn read_contents(cont: &str, steps: usize) -> (usize, usize) {
    let grid = read_grid(cont);
    let part1 = get_answer(&grid, steps, false);
    let part2 = get_answer(&grid, steps, true);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
        assert_eq!(read_contents(&a, 4).0, 4);
    }

    #[test]
    fn part2() {
        let a = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
        assert_eq!(read_contents(&a, 5).1, 17);
    }
}
