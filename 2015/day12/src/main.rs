use clap::Parser;
use regex::Regex;
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
    let part1 = get_part1(cont);
    let part2 = get_part2(cont);
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

fn get_part2(vec: &str) -> i32 {
    let re = Regex::new(r#"\{["\-\w\d:,]*\}"#).unwrap();
    let re2 = Regex::new(r#"\[["\-\w\d,]*\]"#).unwrap();
    let mut vec = vec.to_string();
    loop {
        if let Some(cap) = re.captures(&vec) {
            let ca = cap.get(0).unwrap();
            let res = analyze_dict(&cap[0]);
            vec = vec.replace(ca.as_str(), &res.to_string());
            continue;
        }
        if let Some(cap) = re2.captures(&vec) {
            let ca = cap.get(0).unwrap();
            let res = analyze_list(&cap[0]);
            vec = vec.replace(ca.as_str(), &res.to_string());
            continue;
        }
        println!("{vec}");
        return vec.trim().parse::<i32>().unwrap();
    }
}

fn analyze_dict(str: &str) -> i32 {
    let mut sum: i32 = 0;
    assert!(!str.contains('[')); // All of these should have been removed
    for a in str
        .strip_prefix("{")
        .unwrap()
        .strip_suffix("}")
        .unwrap()
        .split(',')
    {
        let (_i, b) = a.split_once(':').unwrap();
        if b == "\"red\"" {
            return 0;
        }
        sum += b.parse::<i32>().unwrap_or(0);
    }
    sum
}

fn analyze_list(str: &str) -> i32 {
    let mut sum: i32 = 0;
    assert!(!str.contains('{')); // All of these should have been removed
    for a in str
        .strip_prefix("[")
        .unwrap()
        .strip_suffix("]")
        .unwrap()
        .split(',')
    {
        sum += a.parse::<i32>().unwrap_or(0);
    }
    sum
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
    fn test_analyze_list() {
        assert_eq!(analyze_list(&"[1,2,3]"), 6);
        assert_eq!(analyze_list(&r#"[1,"red",5]"#), 6);
    }

    #[test]
    fn test_analyze_dict() {
        assert_eq!(analyze_dict(&r#"{"c":"red","b":2}"#), 0);
        assert_eq!(analyze_dict(&r#"{"c":3,"b":2}"#), 5);
    }

    #[test]
    fn part2() {
        assert_eq!(get_part2(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(get_part2("[1,2,3]"), 6);
        assert_eq!(get_part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(get_part2(r#"[1,"red",5]"#), 6);
    }
}
