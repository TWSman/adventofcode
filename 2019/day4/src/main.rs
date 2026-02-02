use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}


fn read_contents(cont: &str) -> (i64, i64) {
    let ranges: Vec<(i64,i64)> = cont.lines().map(|l| {
        let (a,b)= l.split_once('-').unwrap();
        let start = a.parse::<i64>().unwrap();
        let end = b.parse::<i64>().unwrap();
        (start, end)
    }).collect::<Vec<(i64,i64)>>();

    let (start, end) = ranges[0];
    let part1 = (start..=end).filter(|n| check_part1(*n)).count() as i64;
    let part2 = (start..=end).filter(|n| check_part2(*n)).count() as i64;
    (part1, part2)
}


fn check_part1(num: i64) -> bool {
    let size = num.to_string().len();
    let s = (0..size).map(|i| {
        let divisor = 10_i64.pow((size - i - 1) as u32);
        (num / divisor) % 10
    }).collect::<Vec<i64>>();
    let mut has_double = false;
    let mut prev = -1;
    for curr in s {
        if curr == prev {
            has_double = true;
        }
        if curr < prev {
            return false;
        }
        prev = curr;
    }
    has_double
}

fn check_part2(num: i64) -> bool {
    // Part2 has a slightly different criteria for doubles
    let size = num.to_string().len();
    let s = (0..size).map(|i| {
        let divisor = 10_i64.pow((size - i - 1) as u32);
        (num / divisor) % 10
    }).collect::<Vec<i64>>();
    let mut has_double = false;
    let mut prev = -1;
    let mut successives: Vec<u32> = vec![];
    for curr in s {
        if curr < prev {
            return false;
        }
        if prev == curr {
            successives.push(curr as u32);
        } else if successives.len() == 1 {
            has_double = true;
            successives.clear();
        } else {
            successives.clear();
        }
        prev = curr;
    }
    if successives.len() == 1 {
        has_double = true;
    }
    has_double
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert!(check_part1(111111));
        assert!(!check_part1(223450));
        assert!(!check_part1(123789));

        assert!( check_part2(112233));
        assert!(!check_part2(123444));
        assert!( check_part2(111122));
    }
}
