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

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

fn read_contents(cont: &str) -> (i64, i64) {
    let cont = cont.replace("\n","");
    let ranges = cont.split(',').map(|m| {
        let parts: Vec<&str> = m.split('-').collect();
        Range {
            start: parts[0].parse::<i64>().unwrap(),
            end: parts[1].parse::<i64>().unwrap(),
        }
    }).collect::<Vec<Range>>();
    let res1 = ranges.iter().map(get_part1).sum();
    let res2 = ranges.iter().map(get_part2).sum();
    (res1, res2)
}

fn get_part2(input: &Range) -> i64 {
    // This isn't very efficient but it works. Runs in ~2 seconds
    let start = input.start;
    let end = input.end;
    let mut sum = 0;

    for i in start..=end {
        for split_count in 2..=(i.to_string().len()) {
            if check_valid(i, split_count) {
                // This is a valid id.
                // No need to check other split counts
                sum += i;
                break;
            }
        }
    }
    sum
}

fn check_valid(input: i64, split_count: usize) -> bool {
    // Checks if the input can be formed by repeating the same string split_count times
    let b = input.to_string();
    let n = b.len();
    if !n.is_multiple_of(split_count) {
        return false;
    }
    let split_len = n / split_count;
    let a = b.split_at(split_len).0;

    if a.repeat(split_count) == b {
        return true;
    }
    // Otherwise return false
    false
}

fn get_part1(input: &Range) -> i64 {
    // Calculate the sum of all numbers in the range that can be formed by repeating the same
    // string twice (e.g. 1212, 5656, 9999)
    let mut start = input.start;
    let mut end = input.end;
    let mut size_start = start.to_string().len();
    let mut size_end = input.end.to_string().len();
    if !size_start.is_multiple_of(2) && size_end == size_start {
        // Odd sized strings cant be made up of repeating the same thing twice
        return 0;
    }
    if !size_start.is_multiple_of(2) {
        start = round_up(start);
        size_start = start.to_string().len();
    }
    if !size_end.is_multiple_of(2) {
        end = round_down(end);
        size_end = end.to_string().len();
    }
    assert_eq!(size_start%2, 0);
    assert_eq!(size_start, size_end);

    let start_binding = start.to_string();
    let start_first = start_binding.split_at(size_start/2).0.parse::<i64>().unwrap();
    let start_second = start_binding.split_at(size_start/2).1.parse::<i64>().unwrap();

    let start_candidate = if start_first >= start_second {
        start_first
    } else {
        start_first + 1
    };

    let end_binding = end.to_string();
    let end_first = end_binding.split_at(size_end/2).0.parse::<i64>().unwrap();
    let end_second = end_binding.split_at(size_end/2).1.parse::<i64>().unwrap();


    let end_candidate = if end_first <= end_second {
        // For example 111 112, final option will be 111 111
        end_first
    } else {
        // For example 111 109, final option will be 110 110
        end_first - 1
    };
    (start_candidate..=end_candidate).map(|m| m.to_string().repeat(2).parse::<i64>().unwrap()).sum()
}

fn round_up(input: i64) -> i64 {
    let size_start = input.to_string().len();
    (input / 10_i64.pow(size_start as u32) + 1) * 10_i64.pow(size_start as u32)
}

fn round_down(input: i64) -> i64 {
    round_up(input) / 10 - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round() {
        assert_eq!(round_up(9), 10);
        assert_eq!(round_up(999), 1000);
        assert_eq!(round_up(111), 1000);
        assert_eq!(round_up(9999), 10000);
        assert_eq!(round_up(99999), 100000);

        assert_eq!(round_down(101), 99);
        assert_eq!(round_down(10001), 9999);
    }
    #[test]
    fn part1() {
        assert_eq!(read_contents("11-22").0, 33);
        assert_eq!(read_contents("95-115").0, 99);
        let a = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(read_contents(&a).0, 1227775554);
    }
    #[test]
    fn valid() {
        assert!(check_valid(111, 3));
        // Doesn't divide
        assert!(!check_valid(111, 2));

        assert!(!check_valid(121, 3));
    }

    #[test]
    fn part2() {
        assert_eq!(read_contents("11-22").1, 33);
        assert_eq!(read_contents("95-115").1, 99+111);
        
        assert_eq!(get_part2(&Range{start: 998, end: 1012}), 999 + 1010);

        assert_eq!(read_contents("998-1012").1, 999+1010);
        assert_eq!(read_contents("1188511880-1188511890").1, 1188511885);
        assert_eq!(read_contents("222220-222224").1, 222222);

        let a = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(read_contents(&a).1, 4174379265);
    }
}

