use clap::Parser;
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
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i64, i64) {
    let part1 = get_part1(cont);
    let part2 = get_part2(cont);
    (part1, part2)
}

fn get_part1(cont: &str) -> i64 {
    let mut sum = 0;
    let mut group = BTreeSet::new();
    for line in cont.lines() {
        if line.is_empty() {
            sum += group.len();
            group = BTreeSet::new();
            continue;
        }
        for c in line.chars() {
            group.insert(c);
        }
    }
    sum += group.len();
    sum as i64
}

fn get_part2(cont: &str) -> i64 {
    let mut sum = 0;
    let mut group = BTreeMap::new();
    let mut count = 0;
    for line in cont.lines() {
        if line.is_empty() {
            sum += group.values().filter(|c| **c == count).count();
            group = BTreeMap::new();
            count = 0;
            continue;
        }
        count += 1;
        for c in line.chars() {
            *group.entry(c).or_insert(0) += 1;
        }
    }
    sum += group.values().filter(|c| **c == count).count();
    sum as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(read_contents(&a).0, 11);
    }

    #[test]
    fn part2() {
        let a = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(read_contents(&a).1, 6);
    }
}
