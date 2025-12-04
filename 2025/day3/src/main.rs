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
    let ranges: Vec<Vec<u32>> = cont.lines().map(|m| {
        m.chars().map(|c| {
            c.to_digit(10).unwrap()
        }).collect()
            }).collect();

    let part1 = ranges.iter().map(|l| get_part2(l, 2)).sum();
    let part2 = ranges.iter().map(|l| get_part2(l, 12)).sum();
    (part1, part2)
}

fn get_part1(vals: &[u32]) -> i64 {
    // Solves for part2
    let mut largest: u32 = 0;
    let mut next: u32 = 0;
    for (pos,v) in vals.iter().with_position() {
        if v > &largest && pos != Position::Last  {
            largest = *v;
            next = 0;
        } else if v > &next {
            next = *v;
        }
    }
    let s = largest.to_string();
    (s + &next.to_string()).parse::<i64>().expect("Must be a valid string")
}

fn get_part2(input: &[u32], target_len: usize) -> i64 {
    // Generalized version. Solves for part2, but can be also used for part1
    let n = input.len();
    let mut output: Vec<char> = Vec::new();
    let mut start_ind = 0;
    for j in 0..target_len {
        let mut max_val = 0;
        let mut max_ind = 0;
        let start = start_ind;
        let end = n - (target_len-j);
        for (i, v) in input.iter().enumerate().take(end + 1).skip(start) {
            if v > &max_val {
                max_val = *v;
                max_ind = i;
            }
        }
        output.push(char::from_digit(max_val, 10).expect("Should ork"));
        start_ind = max_ind + 1;
    }
    output.iter().cloned().collect::<String>().parse::<i64>().expect("Should work")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(read_contents(&a).0, 357);
        assert_eq!(read_contents(&a).1, 3121910778619);
    }
}
