use clap::Parser;
use regex::Regex;
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
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i32, i32) {
    let mut discs = cont.lines().map(Disc::new).collect::<Vec<_>>();
    // Sorting by number of positions, so we can skip more iterations in the search
    discs.sort_by_key(|d| -(d.positions as i32));
    let part1 = get_part1(&discs);
    let mut discs_part2 = discs.clone();
    let new_disc = Disc {
        id: discs_part2.len() + 1,
        positions: 11,
        start: 0,
        x0: (discs_part2.len() + 1) % 11,
    };
    discs_part2.push(new_disc);
    discs_part2.sort_by_key(|d| -(d.positions as i32));
    let part2 = get_part1(&discs_part2);
    (part1, part2)
}

fn get_part1(discs: &[Disc]) -> i32 {
    let start = discs[0].positions - discs[0].x0;
    let step = discs[0].positions;
    (start..9999999)
        .step_by(step)
        .map(|i| {
            if discs.iter().all(|d| (d.x0 + i) % d.positions == 0) {
                return i as i32;
            }
            -1
        })
        .find(|x| *x != -1)
        .unwrap_or(-1)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Disc {
    positions: usize,
    start: usize,
    id: usize,
    x0: usize,
}

impl Disc {
    fn new(ln: &str) -> Self {
        let re =
            Regex::new("Disc #(\\d+) has (\\d+) positions; at time=0, it is at position (\\d+)")
                .unwrap();
        let id = re.captures(ln).unwrap()[1].parse::<usize>().unwrap();
        let positions = re.captures(ln).unwrap()[2].parse::<usize>().unwrap();
        let start = re.captures(ln).unwrap()[3].parse::<usize>().unwrap();
        let x0 = start + id % positions;
        Self {
            positions,
            start,
            id,
            x0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";
        assert_eq!(read_contents(&a).0, 5);
    }
}
