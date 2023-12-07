use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use itertools::Itertools;
use std::cmp::Ordering;
// use regex::Regex;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
}


#[derive(Debug, Clone, PartialEq)]
struct HandType {
}

impl HandType {
    pub const FIVEOFAKIND: i32 = 7;
    pub const FOUROFAKIND: i32 = 6;
    pub const FULLHOUSE: i32 = 5;
    pub const THREEOFAKIND: i32 = 4;
    pub const TWOPAIRS: i32 = 3;
    pub const ONEPAIR: i32 = 2;
    pub const HIGHCARD: i32 = 1;
}

#[derive(Debug)]
struct Hand {
    hand: Vec<u32>,
    hand_type: i32,
    bid: i64,
}

enum HandKind {
    Part1,
    Part2,
}

impl Hand {
    fn new1(input: &str) -> Hand {
        let split: Vec<&str> = input.split_whitespace().collect();
        let hand = split[0];
        let bid = split[1].parse::<i64>().expect("Should be a number");
        let cc: Vec<u32> = hand.chars().map(|m| {
            match m {
                x if x.is_numeric() => x.to_digit(10).unwrap(),
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Should not happen"),
            }
        }).collect();

        let mut hand_map: BTreeMap<u32, u32> = BTreeMap::new();
        for c in &cc {
            match hand_map.get(&c) {
                Some(count) => {hand_map.insert(*c, count+1);}
                None =>{hand_map.insert(*c, 1);}
            }
        }
        let mut count_b: Vec<&u32> = hand_map.iter().map(|(k,v)| v).collect::<Vec<&u32>>();
        count_b.sort();
        let hand_type: i32 = match count_b.len() {
            5 => HandType::HIGHCARD, // All must be 1
            4 => HandType::ONEPAIR, // 1 pair, 4 others
            // 2 pairs, or 3 of a kind
            3 => if *count_b[2] == 3 {
                HandType::THREEOFAKIND 
            } else {
                HandType::TWOPAIRS 
            }
            2 => if *count_b[1] == 4 {
                HandType::FOUROFAKIND
            } else {
                HandType::FULLHOUSE
            },
            1 => HandType::FIVEOFAKIND,
            _ => panic!("Should not happen"),
        };
        Hand {hand: cc, hand_type: hand_type, bid}
    }
    fn new2(input: &str) {
        let split: Vec<&str> = input.split_whitespace().collect();
        let hand = split[0];
        let bid = split[1].parse::<i64>().expect("Should be a number");
        let cc: Vec<u32> = hand.chars().map(|m| {
            match m {
                x if x.is_numeric() => x.to_digit(10).unwrap(),
                'T' => 10,
                'J' => 0,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Should not happen"),
            }
        }).collect();

        let mut hand_map: BTreeMap<u32, u32> = BTreeMap::new();
        for c in &cc {
            match hand_map.get(&c) {
                Some(count) => {hand_map.insert(*c, count+1);}
                None =>{hand_map.insert(*c, 1);}
            }
        }
        let mut count_b: Vec<&u32> = hand_map.iter().map(|(k,v)| v).collect::<Vec<&u32>>();
        count_b.sort();
        let hand_type: i32 = match count_b.len() {
            5 => HandType::HIGHCARD, // All must be 1
            4 => HandType::ONEPAIR, // 1 pair, 4 others
            // 2 pairs, or 3 of a kind
            3 => if *count_b[2] == 3 {
                HandType::THREEOFAKIND 
            } else {
                HandType::TWOPAIRS 
            }
            2 => if *count_b[1] == 4 {
                HandType::FOUROFAKIND
            } else {
                HandType::FULLHOUSE
            },
            1 => HandType::FIVEOFAKIND,
            _ => panic!("Should not happen"),
        };
        Hand {hand: cc, hand_type: hand_type, bid}
    }

    // Return true if self outranks the other
    fn outranks(&self, other: &Hand) -> bool {
        if self.hand_type == other.hand_type {
            for i in 0..5 {
                if self.hand[i] == other.hand[i] {
                    continue;
                } else {
                    return self.hand[i] > other.hand[i];
                }
            }
            false
        } else {
            self.hand_type > other.hand_type
        }
    } 
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut hand_list: Vec<Hand> = vec![];
    for ln in cont.lines() {
        let h = Hand::new1(&ln);
        let n = hand_list.len();
        hand_list.push(h);
    }
    
    hand_list.sort_by(|a,b| match a.outranks(b) {
        true => Ordering::Greater,
        _ => Ordering::Less,
    });
    let n = hand_list.len();
    dbg!(&hand_list);
    for i in 0..(n-1) {
        if !hand_list[i+1].outranks(&hand_list[i]) {
            println!();
            println!("i+1 should outrank i");
            dbg!(i);
            dbg!(&hand_list[i+1]);
            dbg!(&hand_list[i]);
            panic!("LIST NOT IN ORDER");
        }
    }

    let sum = hand_list.iter().enumerate().map(|(i, h)| {(i as i64 +1)*h.bid}).sum();
    (sum,0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(read_contents(&a).0, 6440);
    }

    #[test]
    fn hand() {
        let h1 = Hand::new1("32T3K 765");
        let h2 = Hand::new1("T55J5 684");
        let h3 = Hand::new1("KK677 28");
        let h4 = Hand::new1("KTJJT 220");
        let h5 = Hand::new1("QQQJA 483");
        let h6 = Hand::new1("AAAKK 111");
        let h7 = Hand::new1("AAAAK 111");
        let h8 = Hand::new1("AAAAA 111");
        assert_eq!(h2.hand_type, HandType::THREEOFAKIND);
        assert_eq!(h1.hand_type, HandType::ONEPAIR);
        assert_eq!(h3.hand_type, HandType::TWOPAIRS);
        assert_eq!(h4.hand_type, HandType::TWOPAIRS);
        assert_eq!(h5.hand_type, HandType::THREEOFAKIND);
        assert_eq!(h6.hand_type, HandType::FULLHOUSE);
        assert_eq!(h7.hand_type, HandType::FOUROFAKIND);
        assert_eq!(h8.hand_type, HandType::FIVEOFAKIND);

        assert!(h2.outranks(&h1));
        assert!(h2.outranks(&h3));
        assert!(h3.outranks(&h4));
        assert!(h5.outranks(&h2));
    }
}
