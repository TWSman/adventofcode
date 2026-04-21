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
    let res = read_contents(&contents);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i32, i32) {
    let lists = cont
        .lines()
        .map(|ln| {
            ln.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let part1 = lists.iter().map(|ls| get_part1(ls)).sum::<i32>();
    let part2 = lists.iter().map(|ls| get_part2(ls)).sum::<i32>();
    (part1, part2)
}

fn get_part1(ls: &[i32]) -> i32 {
    let (min, max) = ls
        .iter()
        .fold((ls[0], ls[0]), |acc, &x| (acc.0.min(x), acc.1.max(x)));
    max - min
}

fn get_part2(ls: &[i32]) -> i32 {
    let mut sum = 0;
    for (i, a) in ls.iter().enumerate() {
        for (j, b) in ls.iter().enumerate() {
            if i == j {
                continue;
            }
            if a % b == 0 {
                sum += a / b;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "5 1 9 5
7 5 3
2 4 6 8";
        assert_eq!(read_contents(&a).0, 18);
    }

    #[test]
    fn part2() {
        let a = "5 9 2 8
9 4 7 3
3 8 6 5";
        assert_eq!(read_contents(&a).1, 9);
    }
}
