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

fn read_contents(cont: &str) -> (i32, i32) {
    let input = cont.trim().parse::<i32>().unwrap();
    let part1 = get_part1(input);
    let part2 = get_part2(input);
    (part1, part2)
}

fn get_part1(step: i32) -> i32 {
    let mut buffer = vec![0];
    let mut pos = 0;
    for i in 1..=2017 {
        pos = (pos + step) % i + 1;
        buffer.insert(pos as usize, i);
    }
    buffer[pos as usize + 1]
}

fn get_part2(step: i32) -> i32 {
    let mut pos = 0;
    // 0 will always be at position 0, so we only care about the value that is inserted at position 1
    // First position can only update when something is inserted there
    let mut first_pos = 0;
    for i in 1..=50_000_000 {
        pos = (pos + step) % i + 1;
        if pos == 1 {
            // If the new value is inserted at position 1, it becomes the new first position
            println!("Inserting {i} at position 1");
            first_pos = i;
        }
    }
    first_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(read_contents("3").0, 638);
    }

    #[test]
    fn part2() {
        assert_eq!(read_contents("3").1, 1222153);
    }
}
