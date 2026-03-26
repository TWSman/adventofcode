use clap::Parser;

use shared::Dir;
use shared::Vec2D;
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

fn read_contents(cont: &str) -> (i32, i32) {
    let moves = cont
        .chars()
        .filter_map(|c| if c != '\n' { Some(Dir::new(c)) } else { None })
        .collect::<Vec<_>>();
    let part1 = get_part1(&moves);
    let part2 = get_part2(&moves);
    (part1, part2)
}

fn get_part1(vec: &[Dir]) -> i32 {
    let mut moves_vec = vec
        .iter()
        .map(|&d| d.get_dir_true_vec())
        .collect::<Vec<_>>();
    let zero = Vec2D { x: 0, y: 0 };
    moves_vec.insert(0, zero); // Otherwise starting point wouldn't be counted
    moves_vec
        .iter()
        .scan(zero, |pos, mv| {
            *pos = *pos + *mv;
            Some(*pos)
        })
        .collect::<BTreeSet<_>>()
        .len() as i32
}

fn get_part2(vec: &[Dir]) -> i32 {
    let moves_vec = vec
        .iter()
        .map(|&d| d.get_dir_true_vec())
        .collect::<Vec<_>>();
    let mut visited = BTreeSet::new();
    let mut pos_santa = Vec2D { x: 0, y: 0 };
    let mut pos_robot = Vec2D { x: 0, y: 0 };
    visited.insert(pos_santa);
    for (i, mv) in moves_vec.iter().enumerate() {
        if i % 2 == 0 {
            pos_santa = pos_santa + *mv;
            visited.insert(pos_santa);
        } else {
            pos_robot = pos_robot + *mv;
            visited.insert(pos_robot);
        }
    }
    visited.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(read_contents(">").0, 2);
        assert_eq!(read_contents("^>v<").0, 4);
        assert_eq!(read_contents("^v^v^v^v^v").0, 2);
    }

    #[test]
    fn part2() {
        assert_eq!(read_contents("^v").1, 3);
        assert_eq!(read_contents("^>v<").1, 3);
        assert_eq!(read_contents("^v^v^v^v^v").1, 11);
    }
}
