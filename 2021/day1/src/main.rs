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
    list.windows(2).filter(|w| w[1] > w[0]).count() as i64
}
fn get_part2(list: &[i64]) -> i64 {
    // Check when sliding sum of size 3 is increasing
    // i.e. we should have w[3] + w[2] + w[1] > w[2] + w[1] + w[0]
    // which is the same as w[3] > w[0]
    list.windows(4).filter(|w| w[3] > w[0]).count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(read_contents(&a).0, 7);
        assert_eq!(read_contents(&a).1, 5);
    }
}
