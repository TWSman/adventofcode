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

fn read_contents(cont: &str) -> (String, String) {
    let cont = cont.trim();
    dbg!(&cont);
    let part1 = get_part1(cont);
    let part2 = get_part2(cont);

    (part1, part2)
}

fn get_part1(door: &str) -> String {
    let mut j = 0;
    let target = "00000";
    let mut output = String::new();
    loop {
        let inp = format!("{}{}", door, j);
        let hash = format!("{:x}", md5::compute(&inp));
        if j % 1000000 == 0 {
            println!("Checking {inp}");
        }
        j += 1;
        if hash.starts_with(target) {
            println!("Found match with {inp}");
            output.push(hash.chars().nth(5).unwrap());
            if output.len() == 8 {
                break;
            }
        }
    }
    output
}

fn get_part2(door: &str) -> String {
    let mut j = 0;
    let target = "00000";
    let mut output: Vec<char> = vec!['_'; 8];
    loop {
        let inp = format!("{}{}", door, j);
        let hash = format!("{:x}", md5::compute(&inp));
        j += 1;
        if hash.starts_with(target) {
            println!("Found match with {inp}");
            let c = hash.chars().nth(5).unwrap().to_digit(16).unwrap() as usize;
            let c2 = hash.chars().nth(6).unwrap();

            if c < 8 && output[c] == '_' {
                output[c] = c2;
            } else {
                continue;
            }
            println!("{:?}", &output);

            if !output.contains(&'_') {
                // All positions filled, stop
                break;
            }
        }
    }
    output.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_part1(&"abc"), "18f47a30");
    }

    #[test]
    fn part2() {
        assert_eq!(get_part2(&"abc"), "05ace8e3");
    }
}
