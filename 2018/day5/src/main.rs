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
    let part1 = get_part1(cont);
    let part2 = get_part2(cont);
    (part1, part2)
}

fn get_part1(cont: &str) -> i32 {
    let polymer = cont.trim().chars().collect::<Vec<char>>();
    shorten_polymer(&polymer).len() as i32
}

fn shorten_polymer(polymer: &[char]) -> Vec<char> {
    let mut polymer = polymer.to_vec();
    loop {
        let mut to_remove = None;
        for (i, c) in polymer.iter().enumerate() {
            if i + 1 == polymer.len() {
                continue;
            }
            let next_c = polymer.get(i + 1).unwrap();
            if *c as u8 + 32 == *next_c as u8 || *next_c as u8 + 32 == *c as u8 {
                to_remove = Some(i);
            }
        }
        if to_remove.is_none() {
            break;
        }
        let i = to_remove.unwrap();
        polymer.remove(i);
        polymer.remove(i);
    }
    polymer
}

fn get_part2(cont: &str) -> i32 {
    let polymer = cont.trim().chars().collect::<Vec<char>>();
    let tmp = shorten_polymer(&polymer);
    let trimmed = tmp.iter().collect::<String>();
    let mut min_len = i32::MAX;
    for c in 'A'..='Z' {
        let tmp = trimmed.replace([c, (c as u8 + 32) as char], "");
        let l = get_part1(&tmp);
        if l < min_len {
            min_len = l
        }
    }
    min_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_part1(&"aA"), 0);
        assert_eq!(get_part1(&"abBA"), 0);
        assert_eq!(get_part1(&"abAB"), 4);
        assert_eq!(get_part1(&"aabAAB"), 6);
        assert_eq!(get_part1(&"dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn part2() {
        assert_eq!(get_part2(&"dabAcCaCBAcCcaDA"), 4);
    }
}
