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
        .lines()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let part1 = get_part1(&list);
    let part2 = get_part2(&list);
    (part1, part2)
}

fn get_part1(list: &[i64]) -> i64 {
    for (i, a) in list.iter().enumerate() {
        for (j, b) in list.iter().enumerate() {
            if j <= i {
                continue;
            }
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    0
}
fn get_part2(list: &[i64]) -> i64 {
    for (i, a) in list.iter().enumerate() {
        for (j, b) in list.iter().enumerate() {
            if j <= i {
                continue;
            }
            for (k, c) in list.iter().enumerate() {
                if k <= j {
                    continue;
                }
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "1721
979
366
299
675
1456";
        assert_eq!(read_contents(&a).0, 514579);
    }

    #[test]
    fn part2() {
        let a = "1721
979
366
299
675
1456";
        assert_eq!(read_contents(&a).1, 241861950);
    }
}
