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
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i32, i32) {
    let series = cont
        .chars()
        .filter_map(|c| match c {
            '(' => Some(1),
            ')' => Some(-1),
            _ => None,
        })
        .collect::<Vec<_>>();
    let part1 = get_part1(&series);
    let part2 = get_part2(&series);
    (part1, part2)
}

fn get_part1(vec: &[i32]) -> i32 {
    vec.iter().sum()
}

fn get_part2(vec: &[i32]) -> i32 {
    vec.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .position(|x| x == -1)
        .unwrap_or(usize::MAX) as i32
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(read_contents("(())").0, 0);
        assert_eq!(read_contents("(((").0, 3);
        assert_eq!(read_contents("(()(()(").0, 3);

        assert_eq!(read_contents("))(((((").0, 3);
        assert_eq!(read_contents("())").0, -1);
    }

    #[test]
    fn part2() {
        assert_eq!(read_contents(")").1, 1);
        assert_eq!(read_contents("()())").1, 5);
    }
}
