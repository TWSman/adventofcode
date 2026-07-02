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
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (String, i64) {
    let vec = cont
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let part1 = get_part1(&vec, 100);
    let part2 = get_part2(&vec);
    (part1, part2)
}

fn print_order(nxt_cup: &BTreeMap<u32, u32>, start_cup: u32) {
    let mut x = start_cup;
    let n = nxt_cup.len();
    for _ in 0..20.min(n) {
        print!("{}, ", x);
        x = *nxt_cup.get(&x).unwrap();
    }
    if n > 20 {
        print!("...");
    }
    println!();
}

fn run_game(nxt_cup: &mut BTreeMap<u32, u32>, start_cup: u32, rounds: usize) {
    let mut pickup = [0, 0, 0];
    let n1 = nxt_cup.len() as u32;

    print!("Initial order: ");
    print_order(nxt_cup, start_cup);

    let mut current_cup = start_cup;
    for i in 0..rounds {
        if (i + 1) % 1_000_000 == 0 {
            println!("Round: {}", i + 1);
        }
        let mut x = current_cup;
        for p in &mut pickup {
            x = *nxt_cup.get(&x).unwrap();
            *p = x;
        }
        let mut destination_cup = current_cup - 1;
        if destination_cup == 0 {
            destination_cup = n1;
        }
        while pickup.contains(&destination_cup) {
            destination_cup -= 1;
            if destination_cup == 0 {
                destination_cup = n1;
            }
        }

        // Remove pickup, by connecting current to the next one
        let next = *nxt_cup.get(&x).unwrap();
        nxt_cup.insert(current_cup, next);

        // Add pickup after destination
        let tmp = *nxt_cup.get(&destination_cup).unwrap();
        nxt_cup.insert(destination_cup, pickup[0]);
        nxt_cup.insert(pickup[2], tmp);

        current_cup = *nxt_cup.get(&current_cup).unwrap();
    }
}

fn get_part1(vec: &Vec<u32>, rounds: usize) -> String {
    let mut nxt_cup = BTreeMap::new();
    let n = vec.len();
    let mut prev = None;
    let current_cup = vec[0];
    for v in vec {
        if let Some(x) = prev {
            nxt_cup.insert(x, *v);
        }
        prev = Some(*v);
    }
    nxt_cup.insert(prev.unwrap(), current_cup);
    run_game(&mut nxt_cup, current_cup, rounds);

    print!("Final order after 1: ");
    print_order(&nxt_cup, 1);

    let mut x = 1;
    let mut out_vec = Vec::new();
    for _ in 1..n {
        x = *nxt_cup.get(&x).unwrap();
        out_vec.push(x);
    }
    out_vec.iter().map(|c| format!("{}", c)).collect::<String>()
}

fn get_part2(vec: &Vec<u32>) -> i64 {
    let mut nxt_cup = BTreeMap::new();
    let n = vec.len();
    let mut prev = None;
    let current_cup = vec[0];

    for v in vec {
        if let Some(x) = prev {
            nxt_cup.insert(x, *v);
        }
        prev = Some(*v);
    }
    for v in (n + 1)..=1_000_000 {
        if let Some(x) = prev {
            nxt_cup.insert(x, v as u32);
        }
        prev = Some(v as u32);
    }
    nxt_cup.insert(prev.unwrap(), current_cup);

    run_game(&mut nxt_cup, current_cup, 10_000_000);

    print!("Final order after 1: ");
    print_order(&nxt_cup, 1);

    let a = *nxt_cup.get(&1).unwrap();
    let b = *nxt_cup.get(&a).unwrap();
    a as i64 * b as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "389125467";
        let vec = a
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(get_part1(&vec, 10), "92658374");
        assert_eq!(get_part1(&vec, 100), "67384529");
    }

    #[test]
    fn part2() {
        let a = "389125467";
        let vec = a
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(get_part2(&vec), 149245887792);
    }
}
