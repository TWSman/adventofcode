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
    let input = cont
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let part1 = get_length(&input, 40);
    let part2 = get_length(&input, 50);
    (part1, part2)
}

fn get_length(vec: &[u32], iterations: usize) -> i32 {
    let mut v = vec.to_owned();
    println!(
        "Start with: {}",
        v.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    for i in 0..iterations {
        v = convert_list(&v);
        println!("After {} iterations: {}", i + 1, v.len());
    }
    v.len() as i32
}

fn convert_list(list: &[u32]) -> Vec<u32> {
    let mut res = Vec::new();
    let mut prev = list[0];
    let mut count = 1;
    for x in list.iter().skip(1) {
        if *x == prev {
            count += 1;
        } else {
            res.push(count);
            res.push(prev);
            prev = *x;
            count = 1;
        }
    }
    res.push(count);
    res.push(prev);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(convert_list(&[1]), [1, 1]);
        assert_eq!(convert_list(&[1, 1]), [2, 1]);
        assert_eq!(convert_list(&[2, 1]), [1, 2, 1, 1]);
        assert_eq!(convert_list(&[1, 1, 1, 2, 2, 1]), [3, 1, 2, 2, 1, 1]);
    }
}
