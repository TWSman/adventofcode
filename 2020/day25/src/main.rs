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
    println!("Part 1 answer is {}", res);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> i64 {
    let numbers = cont
        .lines()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    get_part1(&numbers)
}

fn get_part1(list: &[i64]) -> i64 {
    let a = list[0];
    let b = list[1];
    let loop_a = find_loop_size(7, a);
    //let loop_b = find_loop_size(7, b);
    transform(b, loop_a)
}

fn find_loop_size(number: i64, target: i64) -> usize {
    let mut x = 1;
    for ls in 1.. {
        if ls > 100_000_000 {
            break;
        }
        x *= number;
        x %= 20201227;

        if x == target {
            return ls;
        }
    }
    0
}

fn transform(number: i64, loop_size: usize) -> i64 {
    let mut x = 1;
    for _ in 0..loop_size {
        x *= number;
        x %= 20201227;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trans() {
        assert_eq!(transform(7, 8), 5764801);
        assert_eq!(transform(7, 11), 17807724);
        assert_eq!(transform(5764801, 11), 14897079);
        assert_eq!(transform(17807724, 8), 14897079);
    }

    #[test]
    fn part1() {
        let a = "5764801
17807724";

        assert_eq!(find_loop_size(7, 5764801), 8);
        assert_eq!(find_loop_size(7, 17807724), 11);

        assert_eq!(read_contents(&a), 14897079);
    }
}
