use clap::Parser;
use std::fs;
use regex::Regex;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[derive(Debug)]
struct Range {
    min: i32,
    max: i32
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        if (self.max >= other.max) & (self.min <= other.min) {
            true
        } else if (other.max >= self.max) & (other.min <= self.min) {
            true
        } else {
            false
        }
    }

    fn overlaps(&self, other: &Range) -> bool {
        if (self.max <= other.max) & (self.max >= other.min) {
            true
        //} else if (other.max >= self.max) & (other.min <= self.min) {
        } else if (other.max <= self.max) & (other.max >= self.min) {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range() {
        let ra1 = Range {min: 3, max: 10};
        let ra2 = Range {min: 4, max: 5};
        let ra3 = Range {min: 7, max: 12};
        assert!(ra1.contains(&ra2));
        assert!(ra2.contains(&ra1));
        assert!(!ra1.contains(&ra3));

        assert!(ra3.overlaps(&ra1));
        assert!(ra1.overlaps(&ra3));

        assert!(!ra3.overlaps(&ra2));
        assert!(!ra2.overlaps(&ra3));

        let ra4 = Range {min: 1, max: 1};
        let ra5 = Range {min: 1, max: 1};
        assert!(ra4.contains(&ra5));
        assert!(ra5.contains(&ra4));
    }
}

fn main() {
    let args = Args::parse();
    read_file(&args.input);
}

fn read_file(filename: &str) {
    let re: Regex = Regex::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();
    let contents: Vec<String> = fs::read_to_string(filename)
        .unwrap() // Panic on errors
        .lines() // Split the string into an iterator
        .map(String::from) // Make each slice into a string
        .collect(); // Collect them in a vector
    let mut contain_count = 0;
    let mut overlap_count = 0;
    for ln in contents {
        println!("{}", ln);
        let Some(res) = re.captures(&ln) else { return };
        let a_min = res[1].parse::<i32>().unwrap();
        let a_max = res[2].parse::<i32>().unwrap();
        let b_min = res[3].parse::<i32>().unwrap();
        let b_max = res[4].parse::<i32>().unwrap();
        let ran1 = Range {min: a_min, max: a_max};
        let ran2 = Range {min: b_min, max: b_max};
        if ran1.contains(&ran2) {
            contain_count += 1
        }
        if ran1.overlaps(&ran2) {
            overlap_count += 1
        }
    }

    println!("{} containments", contain_count);
    println!("{} overlaps", overlap_count);
}
