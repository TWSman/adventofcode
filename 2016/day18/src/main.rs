use clap::Parser;
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
    let res = read_contents(&contents, 40);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Trap,
    Safe,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '^' => Tile::Trap,
            '.' => Tile::Safe,
            _ => panic!("Invalid tile character"),
        }
    }

    fn char(&self) -> char {
        match self {
            Tile::Trap => '^',
            Tile::Safe => '.',
        }
    }
}

fn get_part1(row: &[Tile], rows: usize) -> usize {
    let mut current_row = row.to_vec();
    println!(
        "{}",
        current_row.iter().map(|t| t.char()).collect::<String>()
    );
    let mut safe_count = current_row.iter().filter(|t| **t == Tile::Safe).count();
    for _ in 0..(rows - 1) {
        current_row = convert_row(&current_row);
        let str = current_row.iter().map(|t| t.char()).collect::<String>();
        if rows < 100 {
            println!("{}", &str);
        }
        safe_count += current_row.iter().filter(|t| **t == Tile::Safe).count();
    }
    safe_count
}

fn convert_row(row: &[Tile]) -> Vec<Tile> {
    let mut new_row = Vec::with_capacity(row.len());
    for i in 0..row.len() {
        let left = if i == 0 {
            &Tile::Safe
        } else {
            row.get(i - 1).unwrap_or(&Tile::Safe)
        };

        let right = row.get(i + 1).unwrap_or(&Tile::Safe);
        let c = if left != right {
            Tile::Trap
        } else {
            Tile::Safe
        };
        new_row.push(c);
    }
    new_row
}

fn read_contents(cont: &str, rows: usize) -> (usize, usize) {
    let first_row = cont.trim().chars().map(Tile::new).collect::<Vec<_>>();
    let part1 = get_part1(&first_row, rows);
    let part2 = get_part1(&first_row, 400_000);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "..^^.";
        assert_eq!(read_contents(&a, 3).0, 6);

        let a = ".^^.^.^^^^";
        assert_eq!(read_contents(&a, 10).0, 38);
    }
}
