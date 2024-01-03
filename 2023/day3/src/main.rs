use clap::Parser;
use std::fs;
use regex::Regex;


#[derive(Debug)]
struct Number{
    row: i32,
    start: i32,
    end: i32,
    found: bool,
    value: i32,
}

impl Number{
    fn is_match(&mut self, ind: i32, line_width: i32) -> bool {
        if self.found {
            // Already a match
            return false
        }
        if ind > self.end + line_width + 1 {
            // No hope of being a match
            return false
        }
        if ind < self.start - line_width - 1 {
            // No hope of being a match
            return false
        }
        // On the same row either preceding or after
        if (ind == self.start - 1) | (ind == self.end + 1) {
            self.found = true;
            return true
        }
        let mut start_i = self.start % line_width;
        if start_i > 0 {
            start_i -= 1;
        }
        let mut end_i = self.end % line_width;
        if end_i < line_width {
            end_i += 1;
        }
        for i in [-1,1] {
            if (ind >= start_i + (self.row + i) * line_width) & (ind <= end_i + (self.row + i) * line_width) {
                self.found = true;
                return true;
            }
        }
        false
    }
}
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

fn read_contents(input: &str) -> (i32, i64) {
    // Add +1 to include new line characters
    let line_width = input.lines().next().expect("Should be at least 1 line").len() as i32 + 1;
    let re = Regex::new("[0-9]+").unwrap();
    let mut number_matches: Vec<Number> = re.captures_iter(input).map(|res|
        {
            let m = res.get(0).unwrap();
            Number {
            row: (m.start() as i32) / line_width,
            start: m.start() as i32,
            end: m.end() as i32 - 1,
            found: false,
            value: m.as_str().parse::<i32>().unwrap()}}
    ).collect();
    //dbg!(&number_matches[0]);
    let re = Regex::new("[^.0-9\\s]").unwrap();
    let mut gear_sum: i64 = 0;
    for x in re.captures_iter(input) {
        let mut gear_product: i64 = 1;
        let mut founds: usize = 0;
        let i = x.get(0).unwrap().start() as i32;
        for m in &mut number_matches {
            if m.is_match(i, line_width) && x.get(0).unwrap().as_str() == "*" {
                    dbg!(gear_product);
                    dbg!(m.value);
                    gear_product *= m.value as i64;
                    founds += 1;
                }
        }
        if founds == 2 {
            gear_sum += gear_product;
        }
    }
    let sum1 = number_matches.iter().filter_map(|m| {
        if m.found {
            Some(m.value)
        } else {
            None
        }
    }).sum();
    (sum1, gear_sum)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conts() {
        let a: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(read_contents(&a).0, 4361);
        assert_eq!(read_contents(&a).1, 467835 );
    }
}
