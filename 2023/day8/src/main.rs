use clap::Parser;
use std::fs;
use std::collections::HashMap;
use regex::Regex;
//use std::cmp::Ordering;
use itertools::Itertools;
extern crate nom;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[derive(Debug)]
enum Move {
    Left,
    Right
}

impl Move {
    fn new(c: char) -> Move {
        match c {
            'L' => Move::Left,
            'R' => Move::Right,
            _ => panic!("Must no happen"),
        }
    }
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents, true);
    println!("Part 1 answer is {}", res);
    let res = read_contents(&contents, false);
    println!("Part 2 answer is {}", res);
}



fn read_contents(cont: &str, part1: bool) -> u64 {
    let mut left_list: HashMap<&str, &str> = HashMap::new();
    let mut right_list: HashMap<&str, &str> = HashMap::new();
    let mut lines = cont.lines();
    let moves: Vec<Move> = lines.next().unwrap().chars().map(Move::new).collect();
    for ln in lines {
        match parse_line(&ln) {
            None => {continue;},
            Some((key, left, right)) => {
                left_list.insert(&key, left);
                right_list.insert(&key, right);
                //right_list.get_mut()[key] = right;
            }
        }
    }
    let mut i: u64 = 0;
    let mut val: &str = "AAA";
    let n = moves.len();
    if part1 {
        while val != "ZZZ" {
            let m = &moves[(i as usize) % n];
            i += 1;
            val = match m {
                Move::Left => left_list[val],
                Move::Right => right_list[val],
            }
        }
    } else if false {
        let mut starts: Vec<&str> = left_list.keys().filter(|m| { m.chars().last() == Some('A')}).map(|m| {*m}).collect();
        while starts.iter().any(|m| {m.chars().last() != Some('Z')}) {
            let m = &moves[(i as usize) % n];
            i += 1;
            if i % 1000000 == 0 {
                dbg!(i);
                dbg!(&starts);
            }
            for j in 0..starts.len() {
                let s = starts[j];
                starts[j] = match m {
                    Move::Left => left_list[s],
                    Move::Right => right_list[s],
                }
            }
        }
    } else {
        let mut starts: Vec<&str> = left_list.keys().filter(|m| { m.chars().last() == Some('A')}).map(|m| {*m}).collect();

        let mut ends: Vec<&str> = left_list.keys().filter(|m| { m.chars().last() == Some('Z')}).map(|m| {*m}).collect();
        let mut targets: HashMap::<&str, &str> = HashMap::new();
        let mut counts: HashMap::<&str, u64> = HashMap::new();
        for s in &starts {
            let mut val = (*s).clone();
            let mut j: u64 = 0;
            while val.chars().last() != Some('Z') {
                let m = &moves[(j as usize) % n];
                j += 1;
                val = match m {
                    Move::Left => left_list[val],
                    Move::Right => right_list[val],
                }
            }
            targets.insert(&s, &val);
            counts.insert(&s, j);
        }
        for s in ends {
            let mut val = s.clone();
            let mut j: u64 = 0;
            while (j == 0) | (val.chars().last() != Some('Z')) {
                let m = &moves[(j as usize) % n];
                j += 1;
                val = match m {
                    Move::Left => left_list[val],
                    Move::Right => right_list[val],
                }
            }
            // If this is true, the counts will be additive
            // Thus we don't need to care about how the move cycle coincides with the loop cycle
            assert_eq!(j % (n as u64), 0);
            targets.insert(&s, &val);
            counts.insert(&s, j);
        }

        let mut new_counts: HashMap::<&str, u64> = HashMap::new();
        for s in &starts {
            let val = targets[s];
            new_counts.insert(s, counts[val]);
            dbg!(counts[s] / (n as u64));
            // If this is true it is enough to find the least common multiple
            // Otherwise the first cycle to a Z would be different from the others
            assert_eq!(counts[s], new_counts[s]);
        }
        // assert_eq!(n, 293);
        // In the input data
        // All of the counts are divisible by 293, with no other common denominators
        // i.e. for each count c / 293 is a prime number
        // The least common nominator is the product of (c/293) multiplied by 293
        i =  new_counts.values().map(|m| { m / (n as u64)}).product();
        // 293 by itself is a prime number, so to find a common multiple we need to add 293 as a
        //     factor
        i = i * (n as u64);
    }
    i as u64
}

fn parse_line(input: &str)-> Option<(&str, &str, &str)>{
    let re = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();
    let Some(res) = re.captures(input) else {return None; };
    let x: Vec<&str> = res.iter().skip(1).filter_map(|m| match m {
        Some(val) => Some(val.as_str()),
        None => None,
    }).collect();
    assert!(x.len() >= 2);
    Some((x[0], x[1], x[2]))
    //None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(read_contents(&a,true), 2);

        let b: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(read_contents(&b,true), 6);
    }

    #[test]
    fn part2() {
        let a: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(read_contents(&a, false), 6);
    }
}
