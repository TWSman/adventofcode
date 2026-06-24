use clap::Parser;
use std::collections::BTreeMap;
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

fn read_contents(cont: &str) -> (i64, i64) {
    let numbers = cont
        .trim()
        .split(',')
        .map(|ln| ln.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let part1 = get_answer(&numbers, 2020);
    let part2 = get_answer(&numbers, 30000000);
    (part1, part2)
}

fn get_answer(numbers: &[u64], target: usize) -> i64 {
    let mut map: BTreeMap<u64, u64> = BTreeMap::new();
    let mut prev_number = 0;
    let mut new_number = 0;
    for i in 0..target {
        if i % 100000 == 0 {
            println!("\n{}th Number:", i + 1);
        }
        new_number = if let Some(v) = numbers.get(i) {
            *v
        } else if let Some(j) = map.get(&prev_number) {
            i as u64 - j
        } else {
            0
        };
        map.insert(prev_number, i as u64);
        prev_number = new_number;
    }
    new_number as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let nums = vec![0, 3, 6];
        assert_eq!(get_answer(&nums, 4), 0);
        assert_eq!(get_answer(&nums, 5), 3);
        assert_eq!(get_answer(&nums, 6), 3);
        assert_eq!(get_answer(&nums, 7), 1);
        assert_eq!(get_answer(&nums, 8), 0);
        assert_eq!(get_answer(&nums, 9), 4);
        assert_eq!(get_answer(&nums, 10), 0);
        assert_eq!(get_answer(&nums, 2020), 436);
    }

    #[test]
    fn part2() {
        let nums = vec![0, 3, 6];
        assert_eq!(get_answer(&nums, 30000000), 175594);
    }
}
