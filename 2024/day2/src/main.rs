// Target: For each list of numbers define if its truly decreasing or increasing
// And check that successive differences are 1 or 2

use clap::Parser;
use std::fs;
use itertools::{Itertools, Position};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut res1: i64 = 0;
    let mut res2: i64 = 0;
    for ln in cont.lines() {
        let res = get_part1(ln);
        res1 += res;
        res2 += get_part2(ln);
    }
    (res1, res2)
}

// Analyze a list of numbers
fn analyze_list(nums: &[i64]) -> i64 {
    let a = nums.iter().zip(nums.iter().skip(1)).with_position().fold(1, |res, a| {
        let (p, m) = a;
        let mut d = m.1 - m.0;
        if !(-3..=3).contains(&d) {
            d = 0
        }
        if p == Position::First {
            d
        } else if d * res > 0 {
            i64::abs(d) * res
        } else {
            0
        }
    });
    if a != 0 {
        1
    } else {
        0
    }
}

fn get_part1(input: &str) -> i64 {
    let nums: Vec<i64> = input.split_whitespace().map(|m| {m.parse::<i64>().unwrap() }).collect();
    analyze_list(&nums)
}

fn get_part2(input: &str) -> i64 {
    let nums: Vec<i64> = input.split_whitespace().map(|m| {m.parse::<i64>().unwrap() }).collect();
    for skip in 0..nums.len() {
        let mut nums_copy = nums.clone();
        nums_copy.remove(skip);
        if analyze_list(&nums_copy) > 0 {
            return 1
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_part1("7 6 4 2 1"), 1);
        let a = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(read_contents(&a).0, 2);
    }

    #[test]
    fn part2() {
        assert_eq!(get_part2("1 3 2 4 5"), 1);
        assert_eq!(get_part2("9 7 6 2 1"), 0);
        let a = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(read_contents(&a).1, 4);
    }

}
