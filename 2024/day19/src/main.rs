use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::fmt::Display;
use core::fmt;


enum Stripe {
    RED, //r
    WHITE, // w
    BLUE, // u
    GREEN, // g
    BLACK // b
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}


fn read_stuff(cont: &str) -> (Vec<&str>, Vec<&str>) {
    let first_line = cont.lines().next().unwrap();
    let towels = first_line.split(", ").collect::<Vec<&str>>();
    let targets = cont.lines().skip(2).collect::<Vec<&str>>();
    (towels, targets)
}

fn read_contents(cont: &str) -> (i64, i64) {
    let (towels,targets) = read_stuff(cont);
    let stuff = targets.iter().enumerate().filter_map(|(i,t)| {
        //println!("Checking target {}/{}", i, targets.len());
        match check_target(&t, &towels){
            0 => None,
            x => Some(x),
        }
    }).collect::<Vec<i64>>();
    (stuff.iter().count() as i64, stuff.iter().sum::<i64>() as i64)
}

fn check_target(target: &str, towels: &Vec<&str>) -> i64 {
    let mut heads: Vec<usize> = Vec::new();
    heads.push(0);
    let mut count = 0;
    loop {
        let head = match heads.pop() {
            Some(val) => val,
            None => break,
        };
        let substring = &target[head..];
        if substring.is_empty() {
            continue;
        }
        for towel in towels {
            if substring.starts_with(towel) {
                let new_head = head + towel.len();
                if new_head == target.len() {
                    count += 1;
                } else {
                    heads.push(new_head);
                }
            }
        }
    }
    count
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

    #[test]
    fn target() {
        let a = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let (towels,_) = read_stuff(&a);
        assert!(check_target("brwrr", &towels) > 0);
        assert!(check_target("bggr", &towels) > 0);
        assert!(check_target("gbbr", &towels) > 0);
        assert!(check_target("rrbgbr", &towels) > 0);
        assert_eq!(check_target("ubwu", &towels), 0);
        assert!(check_target("bwurrg", &towels) > 0);
        assert!(check_target("brgr", &towels) > 0);
        assert_eq!(check_target("bbrgwb", &towels), 0);
    }

}
