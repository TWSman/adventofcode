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
    let strings = cont.lines().collect::<Vec<&str>>();
    let part1 = get_part1(&strings);
    let part2 = get_part2(&strings);
    (part1, part2)
}

fn get_part1(vec: &[&str]) -> i32 {
    vec.iter().filter(|s| is_nice(s)).count() as i32
}

fn get_part2(vec: &[&str]) -> i32 {
    vec.iter().filter(|s| is_nice_part2(s)).count() as i32
}

fn is_nice(str: &str) -> bool {
    let vowel_count = str.chars().filter(|c| "aeiou".contains(*c)).count();
    if vowel_count < 3 {
        return false;
    }
    let double_letters = str.chars().zip(str.chars().skip(1)).any(|(a, b)| a == b);
    if !double_letters {
        return false;
    }
    if str.contains("ab") || str.contains("cd") || str.contains("pq") || str.contains("xy") {
        return false;
    }
    true
}

fn is_nice_part2(str: &str) -> bool {
    let mut contains_pair = false;
    for i in 0..str.len() - 1 {
        let pair = &str[i..i + 2];
        if str[i + 2..].contains(pair) {
            contains_pair = true;
            break;
        }
    }
    if !contains_pair {
        return false;
    }

    let mut contains_repeat = false;
    for i in 0..str.len() - 2 {
        if str.chars().nth(i) == str.chars().nth(i + 2) {
            contains_repeat = true;
            break;
        }
    }
    if !contains_repeat {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part2() {
        assert!(is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_part2("xxyxx"));
        assert!(!is_nice_part2("uurcxstgmygtbstg"));
        assert!(!is_nice_part2("ieodomkazucvgmuy"));
    }
}
