use clap::Parser;
use colored::Colorize;
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
    let enhancements = cont.lines().map(Enhancement::new).collect::<Vec<_>>();

    let mut rules = BTreeMap::new();
    for rule in enhancements {
        for input in &rule.input_list {
            rules.insert(input.clone(), rule.output.clone());
        }
    }
    let part1 = evolve(&rules, 5);
    let part2 = evolve(&rules, 18);
    (part1, part2)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Enhancement {
    input_list: BTreeSet<String>,
    output: Vec<Vec<char>>,
}

impl Enhancement {
    fn new(s: &str) -> Self {
        let mut parts = s.split(" => ");
        let input = parts.next().unwrap();
        let output = parts.next().unwrap();
        let mut a = BTreeMap::new();
        for (y, row) in input.split("/").enumerate() {
            for (x, c) in row.chars().enumerate() {
                a.insert((x, y), c == '#');
            }
        }
        let size = if a.len() == 4 { 2 } else { 3 };

        let mut input_list = Vec::new();
        for _ in 0..8 {
            input_list.push(vec![' '; size * size]);
        }
        for y in 0..size {
            for x in 0..size {
                let c = if a[&(x, y)] { '#' } else { '.' };
                let (i_col, i_row) = (x, y);
                input_list[0][i_row * size + i_col] = c;
                // Flip horizontally
                input_list[1][i_row * size + size - 1 - i_col] = c;

                // Rotate clockwise once
                let (i_col, i_row) = (size - 1 - i_row, i_col);
                input_list[2][i_row * size + i_col] = c;
                // Flip horizontally
                input_list[3][i_row * size + size - 1 - i_col] = c;

                // Second rotation
                let (i_col, i_row) = (size - 1 - i_row, i_col);
                input_list[4][i_row * size + i_col] = c;
                // Flip horizontally
                input_list[5][i_row * size + size - 1 - i_col] = c;

                // Third rotation
                let (i_col, i_row) = (size - 1 - i_row, i_col);
                input_list[6][i_row * size + i_col] = c;
                // Flip horizontally
                input_list[7][i_row * size + size - 1 - i_col] = c;
            }
        }
        Self {
            input_list: input_list
                .iter()
                .map(|s| s.iter().collect::<String>())
                .collect::<BTreeSet<_>>(),
            output: output.split("/").map(|row| row.chars().collect()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<(usize, usize), char>,
    max_x: usize,
    min_x: usize,
    max_y: usize,
    min_y: usize,
    width: usize,
}

impl Grid {
    fn print_grid(&self) {
        let min_x = self.min_x;
        let max_x = self.max_x;
        let min_y = self.min_y;
        let max_y = self.max_y;

        println!();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match self.grid.get(&(x, y)) {
                    Some('#') => {
                        print!("{}", "#".red().on_black());
                    }
                    Some('.') => {
                        print!("{}", ".".red().on_white());
                    }
                    None => {
                        print!("{}", ".".white().on_white());
                    }
                    _ => unreachable!(),
                }
            }
            println!();
        }
    }

    fn new(grid: BTreeMap<(usize, usize), char>) -> Self {
        let min_x = grid.keys().map(|v| v.0).min().unwrap();
        let max_x = grid.keys().map(|v| v.0).max().unwrap();
        let min_y = grid.keys().map(|v| v.1).min().unwrap();
        let max_y = grid.keys().map(|v| v.1).max().unwrap();
        assert_eq!(max_x - min_x + 1, max_y - min_y + 1);
        let width = 1 + max_x - min_x;
        assert!(width % 3 == 0 || width % 2 == 0);
        Self {
            grid,
            min_x,
            max_x,
            min_y,
            max_y,
            width,
        }
    }

    fn evolve(&self, rules: &BTreeMap<String, Vec<Vec<char>>>) -> Self {
        assert_eq!(self.min_x, 0);
        assert_eq!(self.min_y, 0);
        let mut new_grid = BTreeMap::new();
        let block_size = if self.width.is_multiple_of(2) { 2 } else { 3 };
        let block_count = self.width / block_size;
        for block_y in 0..block_count {
            for block_x in 0..block_count {
                let mut block = String::new();
                for y in 0..block_size {
                    for x in 0..block_size {
                        let pos = (block_x * block_size + x, block_y * block_size + y);
                        let c = *self.grid.get(&pos).unwrap();
                        block.push(c);
                    }
                }
                let rule_output = rules.get(&block).unwrap();
                let output_size = rule_output.len();
                for (y, row) in rule_output.iter().enumerate() {
                    for (x, &c) in row.iter().enumerate() {
                        let pos = (block_x * output_size + x, block_y * output_size + y);
                        new_grid.insert(pos, c);
                    }
                }
            }
        }
        Grid::new(new_grid)
    }
}

fn evolve(rules: &BTreeMap<String, Vec<Vec<char>>>, steps: usize) -> i64 {
    let mut grid = BTreeMap::new();
    grid.insert((0, 0), '.');
    grid.insert((0, 1), '.');
    grid.insert((0, 2), '#');

    grid.insert((1, 0), '#');
    grid.insert((1, 1), '.');
    grid.insert((1, 2), '#');

    grid.insert((2, 0), '.');
    grid.insert((2, 1), '#');
    grid.insert((2, 2), '#');
    let mut grid = Grid::new(grid);
    grid.print_grid();
    for _ in 0..steps {
        grid = grid.evolve(rules);
        if grid.width <= 16 {
            grid.print_grid();
        } else {
            println!("Grid is now {}x{}", grid.width, grid.width);
        }
    }
    //grid.print_grid();
    grid.grid.values().filter(|&&v| v == '#').count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        let enhancements = a.lines().map(Enhancement::new).collect::<Vec<_>>();

        let mut rules = BTreeMap::new();
        for rule in enhancements {
            for input in &rule.input_list {
                rules.insert(input.clone(), rule.output.clone());
            }
        }
        assert_eq!(evolve(&rules, 2), 12);
    }

    #[test]
    fn enhancement() {
        let en = Enhancement::new("##./.../... => ..../..../..../....");
        assert_eq!(en.input_list.len(), 8);
        assert!(en.input_list.contains(&"##.......".to_string()));
        assert!(en.input_list.contains(&".##......".to_string()));
        assert!(en.input_list.contains(&"#..#.....".to_string()));
        assert!(en.input_list.contains(&"..#..#...".to_string()));
        assert!(en.input_list.contains(&".......##".to_string()));
        assert!(en.input_list.contains(&"......##.".to_string()));
        assert!(en.input_list.contains(&"...#..#..".to_string()));
        assert!(en.input_list.contains(&".....#..#".to_string()));
        // Output should be 4x4
        assert!(en.output.len() == 4);
        assert!(en.output[0].len() == 4);
        //assert!(en.input_list.contains(&"### #.. #..".to_string()));
        //assert!(en.input_list.contains(&"### #.. #..".to_string()));
    }
}
