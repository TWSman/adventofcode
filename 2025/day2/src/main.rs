// Target: For each list of numbers define if its truly decreasing or increasing
// And check that successive differences are 1 or 2

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
    let res2: i64 = 0;
    let cont = cont.replace("\n","");
    let ranges = cont.split(',').map(|m| {
        let parts: Vec<&str> = m.split('-').collect();
        dbg!(&parts);
        Range {
            start: parts[0].parse::<i64>().unwrap(),
            end: parts[1].parse::<i64>().unwrap(),
        }
    }).collect::<Vec<Range>>();
    println!("HEY");
    dbg!(&ranges);
    let res1 = ranges.iter().map(|r| get_sum(r, 2)).sum();
    let res2 = ranges.iter().map(|r| get_sums(r)).sum();
    (res1, res2)
}

fn get_sums(input: &Range) -> i64 {
    let size_end = input.end.to_string().len();
    (2..=size_end).map(|m| get_sum(input, m as usize)).sum()
}

fn get_sum(input: &Range, split_count: usize) -> i64 {
    let mut start = input.start;
    let mut end = input.end;
    let mut size_start = start.to_string().len();
    let mut size_end = input.end.to_string().len();
    if size_start % split_count != 0 && size_end == size_start {
        // Odd sized strings cant be made up of repeating the same thing twice
        return 0;
    }
    while size_start % split_count != 0 {
        // Round start upwards
        //start = (start / 10_i64.pow((size_start / 2) as u32) + 1) * 10_i64.pow((size_start / 2) as u32);
        start = round_up(start);
        size_start = start.to_string().len();
    }
    while size_end % split_count != 0 {
        end = round_down(end);
        size_end = end.to_string().len();
    }
    assert_eq!(size_start % split_count, 0);
    assert_eq!(size_start, size_end);

    let start_binding = start.to_string();
    let start_first = start_binding.split_at(size_start / split_count).0.parse::<i64>().unwrap();
    let start_second = start_binding.split_at(size_start / split_count).1.parse::<i64>().unwrap();

    let start_candidate = if start_first >= start_second {
        start_first
    } else {
        start_first + 1
    };

    let end_binding = end.to_string();
    let end_first = end_binding.split_at(size_end / split_count).0.parse::<i64>().unwrap();
    let end_second = end_binding.split_at(size_end / split_count).1.parse::<i64>().unwrap();


    let end_candidate = if end_first <= end_second {
        // For example 111 112, final option will be 111 111
        end_first
    } else {
        // For example 111 109, final option will be 110 110
        end_first - 1
    };
    dbg!(start_candidate);
    dbg!(end_candidate);
    let res = (start_candidate..=end_candidate).map(|m| m.to_string().repeat(split_count).parse::<i64>().unwrap()).sum();
    res
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
        assert_eq!(read_contents(&a).1, 4174379265);
    }
}
