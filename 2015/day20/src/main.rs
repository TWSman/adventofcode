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
    let input = cont.trim().parse::<i64>().unwrap();
    println!("{input} presents needed");
    let part1 = get_part1(input);
    let part2 = get_part2(input);
    (part1, part2)
}

// List of highly composite numbers
// Taken from A002182 in OEIS
// Answer to part1 should be one of these
const HCN: [i64; 40] = [
    2, 4, 6, 12, 24, 36, 48, 60, 120, 180, 240, 360, 720, 840, 1260, 1680, 2520, 5040, 7_560,
    10_080, 15_120, 20_160, 25_200, 27_720, 45_360, 50_400, 55_440, 83_160, 110_880, 166_320,
    221_760, 277_200, 332_640, 498_960, 554_400, 665_280, 720_720, 1_081_080, 1_441_440, 2_162_160,
];

fn get_part1(target: i64) -> i64 {
    for hc in HCN {
        if get_presents(hc) * 10 > target {
            return hc;
        }
    }
    0
}

fn get_part2(target: i64) -> i64 {
    for i in 0..720_720 {
        // This checks simultenously that the number is divisible by
        // 2,3,4,5,6,7,8,9, 10 and 12
        if i % 2520 != 0 {
            continue;
        }
        if get_presents2(i) * 11 > target {
            return i;
        }
    }
    0
}

fn get_presents(house: i64) -> i64 {
    // Get number of presents for the given house in part 1
    // Does not include factor of 10
    let mut sum = 1;
    for i in 2..=house {
        if house % i == 0 {
            sum += i;
        }
    }
    sum
}

fn get_presents2(house: i64) -> i64 {
    // Get number of presents for the given house in part 2
    // Does not include factor of 11
    let mut sum = 1;
    for i in 2..=house {
        if house % i == 0 && house / i <= 50 {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_presents(1), 1);
        assert_eq!(get_presents(2), 3);
        assert_eq!(get_presents(3), 4);
        assert_eq!(get_presents(4), 7);
        assert_eq!(get_presents(5), 6);
        assert_eq!(get_presents(6), 12);
        assert_eq!(get_presents(7), 8);
        assert_eq!(get_presents(8), 15);
        assert_eq!(get_presents(9), 13);
        assert_eq!(get_presents(10), 18);
        assert_eq!(get_presents(11), 12);
        assert_eq!(get_presents(12), 28);
        assert_eq!(get_presents(13), 14);
        assert_eq!(get_presents(14), 24);
    }
}
