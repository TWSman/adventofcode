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

fn read_contents(cont: &str) -> (String, usize) {
    let grid = read_grid(cont);
    grid.print_grid();
    let (part1, part2) = get_answer(&grid);
    (part1, part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Object {
    Empty,
    Vertical,
    Horizontal,
    Corner,
    Poi(char),
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec2D, Object>,
    entrance: Vec2D,
}

impl Grid {
    fn print_grid(&self) {
        let min_x = self.grid.keys().map(|v| v.x).min().unwrap();
        let max_x = self.grid.keys().map(|v| v.x).max().unwrap();
        let min_y = self.grid.keys().map(|v| v.y).min().unwrap();
        let max_y = self.grid.keys().map(|v| v.y).max().unwrap();

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                match self.grid.get(&Vec2D { x, y }) {
                    Some(Object::Horizontal) => {
                        print!("{}", "-".blue().on_black());
                    }
                    Some(Object::Vertical) => {
                        print!("{}", "|".blue().on_black());
                    }
                    Some(Object::Corner) => {
                        print!("{}", "+".blue().on_black());
                    }
                    Some(&Object::Empty) => {
                        print!("{}", "b".black().on_black());
                    }
                    Some(&Object::Poi(c)) => {
                        print!("{}", c.to_string().yellow().on_black());
                    }
                    None => {
                        print!("{}", ".".white().on_white());
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
                    ' ' => Object::Empty,
                    '|' => Object::Vertical,
                    '-' => Object::Horizontal,
                    '+' => Object::Corner,
                    c => Object::Poi(c),
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
    let entrance = *grid
        .iter()
        .find(|(pos, obj)| pos.y == 0 && **obj == Object::Vertical)
        .unwrap()
        .0;
    Grid { grid, entrance }
}

fn get_answer(grid: &Grid) -> (String, usize) {
    let pos = grid.entrance;
    let mut output = String::new();
    let mut queue = Vec::new();
    queue.push((pos, Dir::S, 1));
    let mut max_steps = 0;
    loop {
        if queue.is_empty() {
            break;
        }
        let (pos, dir, steps) = queue.pop().unwrap();
        if steps > max_steps {
            max_steps = steps;
        }
        let new_pos = pos + dir.get_dir_true_vec();
        match grid.grid.get(&new_pos) {
            Some(Object::Empty) => {
                continue;
            }
            Some(Object::Poi(c)) => {
                output.push(*c);
            }
            Some(Object::Horizontal) | Some(Object::Vertical) => {
                // Just keep going
            }
            Some(Object::Corner) => {
                let left_dir = dir.ccw();
                let right_dir = dir.cw();
                queue.push((new_pos, left_dir, steps + 1));
                queue.push((new_pos, right_dir, steps + 1));
            }
            None => {
                continue;
            }
        }
        queue.push((new_pos, dir, steps + 1));
    }
    (output, max_steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";

        let grid = read_grid(a);
        grid.print_grid();
        assert_eq!(get_answer(&grid).1, 38);
    }

    #[test]
    fn part2() {
        let a = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";

        let grid = read_grid(a);
        grid.print_grid();
        assert_eq!(get_answer(&grid).1, 38);
    }
}
