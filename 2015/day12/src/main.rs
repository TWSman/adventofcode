use clap::Parser;
use std::fs;
use std::time::Instant;
use regex::Regex;

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
    let part1 = get_part1(&cont);
    let part2 = get_part2(&cont);;
    (part1, part2)
}

fn get_part1(vec: &str) -> i32 {
    let re = Regex::new(r"-?\d+").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(vec) {
        sum += cap[0].parse::<i32>().unwrap();
    }
    sum
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Structure {
    Array,
    Dict,
}

fn get_part2(vec: &str) -> i32 {
    let typ = if vec.chars().next().unwrap() == '[' {
        Structure::Array
    } else {
        Structure::Dict
    };
    let mut sum = 0;
    let mut i = 0;
    let mut stack = 0;
    let mut red = false;
    let mut accu = String::new();
    if typ == Structure::Array {
        loop {
            i += 1;
            if i > vec.len() - 2 {
                break;
            }
            let c = vec.chars().nth(i).unwrap();
            dbg!(&c);
            accu += &c.to_string();
            if c == '{' || c == '[' {
                stack += 1;
            } else if c == '}' || c == ']' {
                stack -= 1;
            }
            if stack < 0 {
                panic!("Stack should never be negative");
            }
        }
    }

    dbg!(&typ);
    0

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_part1("[1,2,3]"), 6);
        assert_eq!(get_part1(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(get_part1("[[[3]]]"), 3);
        assert_eq!(get_part1(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(get_part1(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(get_part1("[]"), 0);
        assert_eq!(get_part1("{}"), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(get_part2("[1,2,3]"), 6);
        assert_eq!(get_part2(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(get_part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(get_part2(r#"[1,"red",5]"#), 0);
    }
}
