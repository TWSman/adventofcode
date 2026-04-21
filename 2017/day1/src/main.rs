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

fn read_contents(cont: &str) -> (i64, i64) {
    let list = cont
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let part1 = get_part1(&list);
    let part2 = get_part2(&list);
    (part1, part2)
}

fn get_part2(list: &[u32]) -> i64 {
    let mut sum = 0;
    let len = list.len();
    assert_eq!(len % 2, 0);
    for (i, n) in list.iter().enumerate() {
        if n == &list[(i + len / 2) % len] {
            sum += *n as i64;
        }
    }
    sum
}

fn get_part1(list: &[u32]) -> i64 {
    let mut sum = 0;
    let mut prev = list.last().unwrap();
    for n in list {
        if n == prev {
            sum += *n as i64;
        }
        prev = n;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(read_contents("1122").0, 3);
        assert_eq!(read_contents("1111").0, 4);
        assert_eq!(read_contents("1234").0, 0);
        assert_eq!(read_contents("91212129").0, 9);
    }

    #[test]
    fn part2() {
        assert_eq!(read_contents("1212").1, 6);
        assert_eq!(read_contents("1221").1, 0);
        assert_eq!(read_contents("123425").1, 4);
        assert_eq!(read_contents("123123").1, 12);
        assert_eq!(read_contents("12131415").1, 4);
    }
}
