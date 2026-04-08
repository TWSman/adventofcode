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

fn read_contents(cont: &str) -> (i64, i64) {
    let part1 = get_part1(cont);
    let part2 = get_part2(cont);
    (part1, part2)
}

#[allow(dead_code)]
fn decompress(str: &str) -> String {
    let chars = str.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut output = String::new();
    let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    while !chars[i..].is_empty() {
        if chars[i] == '(' {
            let cap = re.captures(&str[i..]).unwrap();
            let l = cap[0].len();
            let len = cap[1].parse::<usize>().unwrap();
            let count = cap[2].parse::<usize>().unwrap();
            let j = i + l;
            let sub = &str[j..j + len];
            i = j + len;
            for _ in 0..count {
                output.push_str(sub);
            }
        } else {
            output.push(chars[i]);
            i += 1;
        }
    }
    output
}

fn decompress_length(str: &str, part2: bool) -> i64 {
    let chars = str.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut output_len = 0;
    let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    while !chars[i..].is_empty() {
        if chars[i] == '(' {
            let cap = re.captures(&str[i..]).unwrap();
            let l = cap[0].len();
            let len = cap[1].parse::<usize>().unwrap();
            let count = cap[2].parse::<usize>().unwrap();
            let j = i + l;
            i = j + len;
            if part2 {
                output_len += (count as i64) * decompress_length(&str[j..j + len], true);
            } else {
                output_len += (len * count) as i64;
            }
        } else {
            output_len += 1;
            i += 1;
        }
    }
    output_len
}

fn get_part1(cont: &str) -> i64 {
    cont.lines().map(|ln| decompress_length(ln, false)).sum()
}

fn get_part2(cont: &str) -> i64 {
    cont.lines().map(|ln| decompress_length(ln, true)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(decompress("ADVENT"), "ADVENT");
        assert_eq!(decompress_length("ADVENT", false), 6);

        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress_length("A(1x5)BC", false), 7);

        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress_length("(3x3)XYZ", false), 9);

        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress_length("A(2x2)BCD(2x2)EFG", false), 11);

        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress_length("(6x1)(1x3)A", false), 6);

        assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
        assert_eq!(decompress_length("X(8x2)(3x3)ABCY", false), 18);
    }
    #[test]
    fn part2() {
        assert_eq!(decompress_length("(3x3)XYZ", true), 9);
        assert_eq!(decompress_length("X(8x2)(3x3)ABCY", true), 20);
        assert_eq!(
            decompress_length("(27x12)(20x12)(13x14)(7x10)(1x12)A", true),
            241920
        );
        assert_eq!(
            decompress_length(
                "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
                true
            ),
            445
        );
    }
}
