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
    // 1293 is too low
    // 1350 is too low
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i32, i32) {
    let lines = cont
        .lines()
        .map(|ls| ls.to_owned())
        .collect::<Vec<String>>();
    dbg!(&lines.len());
    let part1 = get_part1(&lines);
    let part2 = get_part2(&lines);
    (part1, part2)
}

fn analyze_string(str: &str) -> i32 {
    // Calculate difference between string and decoded version
    // Decodes: \\ as \, \" as # and \x00 as the corresponding char
    let len = str.len();
    let mut actual = str.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
    let tmp = &actual.replace("\\\"", "\"");
    actual = tmp;

    let tmp = &actual.replace("\\\\", "\\");
    actual = tmp;
    let mut actual_len = actual.len();
    let mut rep_count = 0;
    for (i, _st) in actual.match_indices("\\x") {
        if i + 4 > actual.len() {
            continue;
        }
        let hex = &actual[i + 2..i + 4];
        if let Ok(_hex_val) = u8::from_str_radix(hex, 16) {
            rep_count += 1;
        }
    }
    actual_len -= rep_count * 3;
    (len - actual_len) as i32
}

fn analyze_string2(str: &str) -> i32 {
    // Its enough to count the number of quotations and slashes, and add 2 for the new quotations
    // Each quotation and slash adds one new character to the new string
    let quotas_count = str.matches('"').count();
    let slash_count = str.matches('\\').count();
    (quotas_count + slash_count + 2) as i32
}

fn get_part1(ls: &[String]) -> i32 {
    ls.iter().map(|s| analyze_string(s)).sum()
}

fn get_part2(ls: &[String]) -> i32 {
    ls.iter().map(|s| analyze_string2(s)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(analyze_string(r#""""#), 2);
        assert_eq!(analyze_string(r#""abc""#), 2);
        assert_eq!(analyze_string(r#""aaa\"aaa""#), 3);
        assert_eq!(analyze_string(r#""\x27""#), 5);
        assert_eq!(analyze_string(r#""tzckolphexfq\\\x23\xfbdqv\\\"m""#), 11);

        let inputs = [(r#""\xa8br\x8bjr\"""#, 9)];

        for (input, expected) in inputs {
            println!("\nTesting input: {}", input);
            assert_eq!(analyze_string(input), expected);
        }
    }

    #[test]
    fn part2() {
        assert_eq!(analyze_string2(r#""""#), 4);
        assert_eq!(analyze_string2(r#""abc""#), 4);
        assert_eq!(analyze_string2(r#""aaa\"aaa""#), 6);
        assert_eq!(analyze_string2(r#""\x27""#), 5);
    }
}
