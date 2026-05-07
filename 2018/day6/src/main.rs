use clap::Parser;
use colored::Colorize;
use shared::Vec2D;
use std::collections::BTreeMap;
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
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i64, i64) {
    let grid = read_grid(cont);
    let part1 = get_part1(&grid);
    let part2 = get_part2(&grid, 10_000);
    (part1, part2)
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec2D, char>,
}

impl Grid {
    fn print_grid(&self) {
        let min_x = self.grid.keys().map(|v| v.x).min().unwrap() - 1;
        let max_x = self.grid.keys().map(|v| v.x).max().unwrap() + 1;
        let min_y = self.grid.keys().map(|v| v.y).min().unwrap() - 1;
        let max_y = self.grid.keys().map(|v| v.y).max().unwrap() + 1;

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                match self.grid.get(&Vec2D { x, y }) {
                    Some(c) => {
                        print!("{}", c.to_string().red().on_black());
                    }
                    None => {
                        let closest = self.get_closest(Vec2D { x, y });
                        match closest {
                            Some((_v, c)) => {
                                print!("{}", c.to_string().white().on_black());
                            }
                            None => {
                                print!("{}", ".".white().on_black());
                            }
                        }
                    }
                }
            }
            println!();
        }
    }

    fn print_saferegion(&self, max_sum: i64) {
        let min_x = self.grid.keys().map(|v| v.x).min().unwrap() - 1;
        let max_x = self.grid.keys().map(|v| v.x).max().unwrap() + 1;
        let min_y = self.grid.keys().map(|v| v.y).min().unwrap() - 1;
        let max_y = self.grid.keys().map(|v| v.y).max().unwrap() + 1;

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let v = Vec2D { x, y };
                match self.grid.get(&v) {
                    Some(c) => {
                        print!("{}", c.to_string().red().on_black());
                    }
                    None => {
                        let dist: i64 = self.grid.keys().map(|v2| v.manhattan(v2)).sum();
                        if dist < max_sum {
                            print!("{}", "#".red().on_black());
                        } else {
                            print!("{}", ".".white().on_black());
                        }
                    }
                }
            }
            println!();
        }
    }

    fn get_closest(&self, loc: Vec2D) -> Option<(Vec2D, char)> {
        let mut min_dist = 99999;
        let mut min_loc = None;
        for (v, c) in &self.grid {
            let d = v.manhattan(&loc);
            if d < min_dist {
                min_dist = d;
                min_loc = Some((*v, *c));
            } else if d == min_dist {
                min_loc = None;
            }
        }
        min_loc
    }
}

fn read_grid(cont: &str) -> Grid {
    let mut grid = BTreeMap::new();
    let mut ind = b'A';
    for line in cont.lines() {
        let (x, y) = line.split_once(',').unwrap();
        grid.insert(
            Vec2D::new(x.parse::<i64>().unwrap(), y.trim().parse::<i64>().unwrap()),
            ind as char,
        );

        ind += 1;
        if ind == b'A' + 26 {
            ind = b'a';
        }
    }
    Grid { grid }
}

fn get_part1(grid: &Grid) -> i64 {
    let mut infinite: BTreeSet<char> = BTreeSet::new();
    let min_x = grid.grid.keys().map(|v| v.x).min().unwrap() - 1;
    let max_x = grid.grid.keys().map(|v| v.x).max().unwrap() + 1;
    let min_y = grid.grid.keys().map(|v| v.y).min().unwrap() - 1;
    let max_y = grid.grid.keys().map(|v| v.y).max().unwrap() + 1;
    grid.print_grid();
    for y in min_y..=max_y {
        if let Some(c) = grid.get_closest(Vec2D { x: min_x, y }) {
            infinite.insert(c.1);
        }

        if let Some(c) = grid.get_closest(Vec2D { x: max_x, y }) {
            infinite.insert(c.1);
        }
    }
    for x in min_x..=max_x {
        if let Some(c) = grid.get_closest(Vec2D { x, y: min_y }) {
            infinite.insert(c.1);
        }

        if let Some(c) = grid.get_closest(Vec2D { x, y: max_y }) {
            infinite.insert(c.1);
        }
    }
    let mut areas: BTreeMap<char, i64> = BTreeMap::new();
    for x in (min_x + 1)..max_x {
        for y in (min_y + 1)..max_y {
            let closest = grid.get_closest(Vec2D { x, y });
            match closest {
                Some((_, c)) if !infinite.contains(&c) => *areas.entry(c).or_default() += 1,
                _ => {}
            }
        }
    }
    *areas.values().max().unwrap()
}

fn get_part2(grid: &Grid, max_sum: i64) -> i64 {
    let min_x = grid.grid.keys().map(|v| v.x).min().unwrap() - 1;
    let max_x = grid.grid.keys().map(|v| v.x).max().unwrap() + 1;
    let min_y = grid.grid.keys().map(|v| v.y).min().unwrap() - 1;
    let max_y = grid.grid.keys().map(|v| v.y).max().unwrap() + 1;
    let mut safe = 0;
    grid.print_saferegion(max_sum);
    for x in (min_x + 1)..max_x {
        for y in (min_y + 1)..max_y {
            let v = Vec2D { x, y };
            let dist: i64 = grid.grid.keys().map(|v2| v.manhattan(v2)).sum();
            if dist < max_sum {
                safe += 1;
            }
        }
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let grid = read_grid(a);
        assert_eq!(get_part1(&grid), 17);
    }

    #[test]
    fn part2() {
        let a = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let grid = read_grid(a);
        assert_eq!(get_part2(&grid, 32), 16);
    }
}
