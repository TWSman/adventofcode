use clap::Parser;
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
    let lists = cont
        .lines()
        .map(|ln| ln.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let part1 = lists.iter().filter(|ls| valid_part1(ls)).count() as i64;
    let part2 = lists.iter().filter(|ls| valid_part2(ls)).count() as i64;
    (part1, part2)
}

fn valid_part1(list: &Vec<&str>) -> bool {
    let mut set = BTreeSet::new();
    for w in list.iter() {
        if set.contains(w) {
            return false;
        }
        set.insert(*w);
    }
    true
}

fn valid_part2(list: &Vec<&str>) -> bool {
    let mut set = BTreeSet::new();
    for w in list.iter() {
        let mut sorted = w.chars().collect::<Vec<_>>();
        sorted.sort_unstable();
        let sorted_str = sorted.into_iter().collect::<String>();
        if set.contains(&sorted_str) {
            return false;
        }
        set.insert(sorted_str);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa";

        assert_eq!(read_contents(&a).0, 2);
    }

    #[test]
    fn part2() {
        assert_eq!(read_contents(&"abcde fghij").1, 1);
        assert_eq!(read_contents(&"abcde xyz ecdab").1, 0);
        assert_eq!(read_contents(&"a ab abc abd abf abj").1, 1);
        assert_eq!(read_contents(&"iiii oiii ooii oooi oooo").1, 1);
        assert_eq!(read_contents(&"oiii ioii iioi iiio").1, 0);
    }
}
