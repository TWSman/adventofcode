use clap::Parser;

use itertools::Itertools;
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
    println!("Execution lasted {elapsed:.2?}");
}

fn read_guests(cont: &str) -> (BTreeMap<String, usize>, BTreeMap<(usize, usize), i32>) {
    let mut guests: BTreeMap<String, usize> = BTreeMap::new();
    let mut happiness: BTreeMap<(usize, usize), i32> = BTreeMap::new();
    let mut i = 0;
    for line in cont.lines() {
        let words = line
            .strip_suffix(".")
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        let from = words.first().unwrap().to_string();
        let to = words.get(10).unwrap().to_string();
        let mut h = words.get(3).unwrap().parse::<i32>().unwrap();
        if words.get(2).unwrap() == &"lose" {
            h *= -1;
        }
        if !guests.contains_key(&from) {
            guests.insert(from.clone(), i);
            i += 1;
        }
        if !guests.contains_key(&to) {
            guests.insert(to.clone(), i);
            i += 1;
        }
        let from_i = guests.get(&from).unwrap();
        let to_i = guests.get(&to).unwrap();
        happiness.insert((*from_i, *to_i), h);
    }
    (guests, happiness)
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn read_contents(cont: &str) -> (i32, i32) {
    let (guests, happiness) = read_guests(cont);
    println!("{} guests", guests.len());
    println!("{} potential arrangements", factorial(guests.len()));
    let part1 = get_part1(&guests, &happiness);
    let part2 = get_part2(&guests, &happiness);
    (part1, part2)
}

fn get_part2(guests: &BTreeMap<String, usize>, happiness: &BTreeMap<(usize, usize), i32>) -> i32 {
    let mut guests = guests.clone();
    let n = guests.len();

    guests.insert("Me".to_string(), n);
    println!("Part2: {} guests", guests.len());
    println!("Part2: {} potential arrangements", factorial(guests.len()));
    get_part1(&guests, happiness)
}

fn get_part1(guests: &BTreeMap<String, usize>, happiness: &BTreeMap<(usize, usize), i32>) -> i32 {
    let mut max_happiness = i32::MIN;
    let guest_names: Vec<usize> = guests.values().copied().collect();
    let n = guest_names.len();
    for guest_list in guest_names.iter().permutations(guest_names.len()) {
        let mut total_happiness: i32 = 0;
        let first = guest_list.first().unwrap();
        let last = guest_list.get(n - 1).unwrap();
        total_happiness += *happiness.get(&(**first, **last)).unwrap_or(&0);
        total_happiness += *happiness.get(&(**last, **first)).unwrap_or(&0);
        total_happiness += guest_list
            .windows(2)
            .map(|w| {
                let from = w[0];
                let to = w[1];
                *happiness.get(&(*from, *to)).unwrap_or(&0)
                    + *happiness.get(&(*to, *from)).unwrap_or(&0)
            })
            .sum::<i32>();

        if total_happiness > max_happiness {
            max_happiness = total_happiness;
        }
    }
    max_happiness
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";
        assert_eq!(read_contents(a).0, 330);
        assert_eq!(read_contents(a).1, 286);
    }
}
