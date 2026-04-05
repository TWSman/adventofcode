use clap::Parser;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
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
    let res = read_contents(&contents, 150);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str, target: i32) -> (i32, i32) {
    let buckets = cont
        .lines()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    println!("{} buckets", buckets.len());
    get_answer(&buckets, target)
}

fn get_answer(buckets: &[i32], target: i32) -> (i32, i32) {
    let n = buckets.len();
    let mut valid = 0;

    let mut counts: BTreeMap<usize, i32> = BTreeMap::new();

    // There are 2 ^ n options to check (each bucket is either used or not)
    let option_count = 2_i32.pow(u32::try_from(n).unwrap());
    println!("{option_count} options");
    for opt in 0..option_count {
        // println!("{opt:b}");
        let mut o = opt;
        let mut x_sum = 0;
        let mut c;
        let mut buckets_used = 0;
        for bucket in buckets {
            (c, o) = (o % 2, o / 2);
            if c == 1 {
                x_sum += bucket;
                buckets_used += 1;
            }
        }
        if x_sum != target {
            // If the sum does not equal this is not a valid option
            continue;
        }
        if let Entry::Vacant(e) = counts.entry(buckets_used) {
            e.insert(1);
        } else {
            *counts.get_mut(&buckets_used).unwrap() += 1;
        }
        valid += 1;
    }
    dbg!(&counts);
    let min_buckets = counts.keys().min().unwrap();
    (valid, *counts.get(min_buckets).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "20
15
10
5
5";
        assert_eq!(read_contents(&a, 25).0, 4);
        assert_eq!(read_contents(&a, 25).1, 3);
    }
}
