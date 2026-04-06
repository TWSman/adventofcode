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

fn read_contents(cont: &str) -> (usize, usize) {
    let candidates = cont
        .lines()
        .map(|ln| {
            ln.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let n = candidates.len();

    let part1 = candidates.iter().filter(is_triangle).count();

    // If the number of rows is divisible by 3, we don't have to worry about triplets wrapping
    // around from one column to the next
    assert_eq!(n % 3, 0);
    let mut candidates2 = Vec::new();
    for i in 0..(n / 3) {
        for j in 0..3 {
            let (a, b, c) = (
                candidates[i * 3][j],
                candidates[i * 3 + 1][j],
                candidates[i * 3 + 2][j],
            );
            candidates2.push(vec![a, b, c]);
        }
    }
    assert_eq!(candidates.len(), candidates2.len());

    let part2 = candidates2.iter().filter(is_triangle).count();
    (part1, part2)
}

fn is_triangle(v: &&Vec<i32>) -> bool {
    let mut v = v.to_owned().clone();
    v.sort_unstable();
    v[2] < v[1] + v[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = vec![25, 5, 10];
        assert_eq!(is_triangle(&&a), false);
        let a = vec![5, 10, 12];
        assert_eq!(is_triangle(&&a), true);
    }
}
