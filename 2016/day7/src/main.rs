use clap::Parser;
use std::collections::BTreeSet;
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

fn read_contents(cont: &str) -> (i32, i32) {
    let addresses = cont.lines().map(Address::new).collect::<Vec<_>>();
    let part1 = get_part1(&addresses);
    let part2 = get_part2(&addresses);
    (part1, part2)
}

#[derive(Debug)]
struct Address {
    sequences: Vec<String>,
    hyper_sequences: Vec<String>,
}

impl Address {
    fn new(ln: &str) -> Self {
        let mut sequences = Vec::new();
        let mut hyper_sequences = Vec::new();
        for split in ln.split('[').collect::<Vec<_>>() {
            if !split.contains(']') {
                sequences.push(split.to_string());
                continue;
            }
            assert!(!split.contains('['));
            assert!(split.contains(']'));
            let (hyper, seq) = split.split_once(']').unwrap();
            hyper_sequences.push(hyper.to_string());
            if !seq.is_empty() {
                sequences.push(seq.to_string());
            }
        }
        Self {
            sequences,
            hyper_sequences,
        }
    }

    fn valid(&self) -> bool {
        for seq in &self.hyper_sequences {
            if contains_abba(seq) {
                return false;
            }
        }
        for seq in &self.sequences {
            if contains_abba(seq) {
                return true;
            }
        }
        false
    }

    fn valid2(&self) -> bool {
        let mut candidates = BTreeSet::new();
        for seq in &self.sequences {
            let n = seq.len();
            if n < 3 {
                continue;
            }
            for i in 0..(n - 2) {
                let a = seq.chars().nth(i).unwrap();
                let b = seq.chars().nth(i + 1).unwrap();
                let c = seq.chars().nth(i + 2).unwrap();
                if a == c && a != b {
                    candidates.insert(format!("{}{}{}", b, a, b));
                }
            }
        }
        for cand in candidates {
            for seq in &self.hyper_sequences {
                if seq.contains(&cand) {
                    return true;
                }
            }
        }
        false
    }
}

fn contains_abba(str: &str) -> bool {
    let n = str.len();
    if n < 4 {
        return false;
    }
    for i in 0..(n - 3) {
        let a = str.chars().nth(i).unwrap();
        let b = str.chars().nth(i + 1).unwrap();
        let c = str.chars().nth(i + 2).unwrap();
        let d = str.chars().nth(i + 3).unwrap();
        if a == d && b == c && a != b {
            return true;
        }
    }
    false
}

fn get_part1(rooms: &[Address]) -> i32 {
    rooms.iter().filter(|ln| ln.valid()).count() as i32
}

fn get_part2(rooms: &[Address]) -> i32 {
    rooms.iter().filter(|ln| ln.valid2()).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Address::new("abba[mnop]qrst").valid(), true);
        assert_eq!(Address::new("abcd[bddb]xyyx").valid(), false);
        assert_eq!(Address::new("aaaa[qwer]tyui").valid(), false);
        assert_eq!(Address::new("ioxxoj[asdfgh]zxcvbn").valid(), true);
    }

    #[test]
    fn part2() {
        assert_eq!(Address::new("aba[bab]xyz").valid2(), true);
        assert_eq!(Address::new("xyx[xyx]xyx").valid2(), false);
        assert_eq!(Address::new("aaa[kek]eke").valid2(), true);
        assert_eq!(Address::new("zazbz[bzb]cdb").valid2(), true);
    }
}
