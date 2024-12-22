use clap::Parser;
use std::fs;
use std::collections::VecDeque;
use std::collections::BTreeSet;

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


fn read_contents(cont: &str) -> (i64, i64) {
    let starts = cont.lines().map(|m| {
        m.parse().unwrap()
    }).collect::<Vec<i64>>();

    let part1 = starts.iter().map(|s| get_part1(*s, 2000)).sum();
    let part2 = get_part2(&starts);
    (part1, part2)
}

fn get_next(secret: i64) -> i64 {
    let tmp = prune(mix(secret, secret * 64));
    let tmp2 = prune(mix(tmp / 32, tmp));
    prune(mix(tmp2 * 2048, tmp2))
}


fn get_price(secret: i64) -> i8 {
    (secret % 10) as i8
}


fn get_best_sequence(prices: &Vec<i8>) -> Vec<VecDeque<i8>> {
    let mut best_price = 0;
    let mut prev: Option<i8> = None;
    let mut sec: VecDeque<i8> = VecDeque::new();
    let mut best_seqs = Vec::new();
    for p in prices {
        match prev {
            None => {
                prev = Some(*p);
                continue
            },
            Some(v) => {
                sec.push_back(p - v)
            }
        }
        if sec.len() > 4 {
            sec.pop_front();
            if p == &best_price {
                best_seqs.push(sec.clone());
            }
            if p > &best_price {
                best_price = *p;
                best_seqs = vec![sec.clone()];
            }
        }
        prev = Some(*p);
    }
    best_seqs
}

fn get_check(seq: &VecDeque<i8>) -> (i8,i8,i8,i8) {
    assert!(seq.len() == 4);
    (seq[0], seq[1], seq[2], seq[3])
}

fn get_part2(secrets: &[i64]) -> i64 {
    let mut best_price = 0;
    let prices = get_price_sequences(secrets, 2000);

    let mut checked_sequences: BTreeSet<(i8,i8,i8,i8)> = BTreeSet::new();

    // Assume that the best sequence is the best sequence for one of the price sequences
    let best_sequences = prices.iter().flat_map(get_best_sequence).collect::<Vec<_>>();
    let n = best_sequences.len();
    println!("{} candidates", n);
    for (i,seq) in best_sequences.iter().enumerate() {
        let check = get_check(seq);
        if checked_sequences.contains(&check) {
            continue
        } else {
            checked_sequences.insert(check);
        }
        if i % 100 == 0 {
            println!("{} / {}", i, n);
            println!("Best price is now: {}", best_price);
        }
        let res = prices.iter().map(|p| check_part2(p, seq) as i64).sum::<i64>();
        if res > best_price {
            best_price = res;
            println!("i: {i}, Best price is now: {}", best_price);
        }
    }

    // Loop over all possible sequences
    //for x in 0..19_i64.pow(4) {
    //    println!("{} / {}", x, 19_i64.pow(4));
    //    let check_seq: VecDeque<i8> = [
    //        (x % 19_i64 - 9) as i8,
    //        ((x / 19_i64) % 19_i64 - 9) as i8,
    //        ((x / 19_i64.pow(2) ) % 19_i64 - 9) as i8,
    //        ((x / 19_i64.pow(3) ) % 19_i64 - 9) as i8,
    //    ].into();
    //    // dbg!(&check_seq);
    //    let res = prices.iter().map(|p| check_part2(p, &check_seq) as i64).sum::<i64>();
    //    if res > best_price {
    //        best_price = res;
    //    }
    //}
    //best_price
    best_price
}

fn get_price_sequences(secrets: &[i64], rec: usize) -> Vec<Vec<i8>> {
    secrets.iter().map(|s| {
        let mut out = *s;
        let mut prices: Vec<i8> = Vec::new();
        for _ in 0..rec {
            out = get_next(out);
            prices.push(get_price(out));
        }
        prices
    }).collect()
}


fn get_part1(secret: i64, rec: usize) -> i64 {
    let mut out = secret;
    for _ in 0..rec {
        out = get_next(out);
    }
    out
}

fn check_part2(prices: &Vec<i8>, target_seq: &VecDeque<i8>) -> i8 {
    let mut prev: Option<i8> = None;
    let mut sec: VecDeque<i8> = VecDeque::new();
    for p in prices {
        match prev {
            None => {
                prev = Some(*p);
                continue
            },
            Some(v) => {
                sec.push_back(p - v)
            }
        }
        if sec.len() > 4 {
            sec.pop_front();
        }
        if (sec.len() == 4)  & (sec == *target_seq) {
            return *p
        }
        prev = Some(*p);
    }
    0
}

fn mix(a: i64, b: i64) -> i64{
    a ^ b
}

fn prune(a: i64) -> i64 {
    a % 2_i64.pow(24)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "1
10
100
2024";

        assert_eq!(read_contents(&a).0, 37327623);

        let b = "1
2
3
2024";
        assert_eq!(read_contents(&b).1, 23);

    }

    #[test]
    fn test_rec() {
        assert_eq!(get_part1(1, 2000), 8685429);
    }


    #[test]
    fn test_next() {
        assert_eq!(get_next(123), 15887950);
        assert_eq!(get_next(15887950), 16495136);
    }

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_price() {
        assert_eq!(get_price(16495136), 6);
        assert_eq!(get_price(15887950), 0);
    }

    #[test]
    fn part2() {
        let secrets = vec![1,2,3,2024];
        let prices = get_price_sequences(&secrets, 2000);

        assert_eq!(check_part2(&prices[0], &vec![-2,1,-1,3].into()), 7);
        assert_eq!(check_part2(&prices[1], &vec![-2,1,-1,3].into()), 7);
        assert_eq!(check_part2(&prices[2], &vec![-2,1,-1,3].into()), 0);
        assert_eq!(check_part2(&prices[3], &vec![-2,1,-1,3].into()), 9);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }
}
