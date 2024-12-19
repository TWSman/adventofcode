use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use num_format::{Locale, ToFormattedString};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

struct Analyzer {
    // Store result of check_target
    memoized: BTreeMap<String, i64>,
    towels: Vec<String>,
}

impl Analyzer {
    fn new(towels: Vec<String>) -> Self {
        Self {towels, memoized: BTreeMap::new()}
    }

    fn check_target(&mut self, target: &str) -> i64 {
        if let Some(&x) = self.memoized.get(target) {
            // We have already checked this target
            return x;
        }
        let mut count = 0;
        for towel in self.towels.clone() {
            // Check all towels for a match
            if target.starts_with(&towel) {
                // This towel exactly matches the target
                if towel.len() == target.len() {
                    count += 1;
                } else {
                    // Towel only matches the beginning of the target
                    let newsub = &target[towel.len()..];
                    let tmp = self.check_target(newsub);
                    self.memoized.insert(newsub.to_string(), tmp);
                    count += tmp;
                }
            }
        }
        count
    }
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {}", part1.to_formatted_string(&Locale::fi));
    println!("Part 2 answer is {}", part2.to_formatted_string(&Locale::fi));

    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}


fn read_stuff(cont: &str) -> (Vec<String>, Vec<&str>) {
    let first_line = cont.lines().next().unwrap();
    let towels = first_line.split(", ").map(String::from).collect::<Vec<String>>();
    let targets = cont.lines().skip(2).collect::<Vec<&str>>();
    (towels, targets)
}

fn read_contents(cont: &str) -> (i64, i64) {
    let (towels,targets) = read_stuff(cont);
    let mut analyzer = Analyzer::new(towels);
    let stuff = targets.iter().filter_map(|t| {
        match analyzer.check_target(t){
            0 => None,
            x => Some(x),
        }
    }).collect::<Vec<i64>>();
    // Part1 is the number of targets that can be made with a combination
    // Part2 is the total number of combinations that any target can be made
    (stuff.len() as i64, stuff.iter().sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(read_contents(&a).0, 6);
        assert_eq!(read_contents(&a).1, 16);
    }

}
