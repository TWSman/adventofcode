use clap::Parser;
use memoize::memoize;
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
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i64, i64) {
    let operations = get_operations(cont);
    let part1 = get_part1(&operations);
    let part2 = get_part2(&operations);
    (part1, part2)
}

fn get_operations(cont: &str) -> Vec<Operation> {
    cont.lines().map(Operation::new).collect::<Vec<_>>()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
enum Operation {
    DealWithIncrement(usize),
    DealIntoNewStack,
    Cut(isize),
}

#[memoize]
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64, i64) {
    //  returns (t, s, gcd) such that t*a + s*b = gcd
    let mut s = 0;
    let mut old_s = 1;
    let mut r = b;
    let mut old_r = a;

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }

    let bezout_t = if b != 0 { (old_r - old_s * a) / b } else { 0 };
    let a_inv = (old_s + b) % b;
    (old_s, bezout_t, old_r, a_inv)
}

fn solve_modular(a: i64, m: i64, b: i64) -> i64 {
    let (_x, _y, _gcd, a_inv) = extended_gcd(a, m);

    let t = (b as i128 * a_inv as i128) % m as i128;
    let n = t % m as i128;
    n as i64
}

impl Operation {
    fn new(ln: &str) -> Self {
        if ln.starts_with("deal into new stack") {
            Operation::DealIntoNewStack
        } else if ln.starts_with("deal with increment ") {
            let splits = ln.split(" ").collect::<Vec<_>>();
            let n = splits.last().unwrap().parse::<usize>().unwrap();
            Operation::DealWithIncrement(n)
        } else if ln.starts_with("cut ") {
            let splits = ln.split(" ").collect::<Vec<_>>();
            let n = splits.last().unwrap().parse::<isize>().unwrap();
            Operation::Cut(n)
        } else {
            panic!("Unknown operation: {}", ln);
        }
    }

    fn apply(&self, list: &[usize]) -> Vec<usize> {
        let list_size = list.len();
        let mut output = (0..list_size).collect::<Vec<usize>>();
        match self {
            Operation::DealIntoNewStack => return list.iter().rev().cloned().collect::<Vec<_>>(),
            Operation::DealWithIncrement(n) => {
                for (i, v) in list.iter().enumerate() {
                    output[(i * n) % list_size] = *v;
                }
            }
            Operation::Cut(n) => {
                for (i, v) in list.iter().enumerate() {
                    // Element (i + N - n) % N is set to the value of element i
                    let new_index = (i + list_size) as isize - *n;
                    output[new_index as usize % list_size] = *v;
                }
            }
        }
        output
    }

    // Get the target index, of start
    fn get_index(&self, start_ind: usize, list_size: usize) -> usize {
        match self {
            Operation::DealIntoNewStack => list_size - 1 - start_ind,
            Operation::Cut(n) => {
                ((start_ind + list_size) as isize - *n + list_size as isize) as usize % list_size
            }

            Operation::DealWithIncrement(n) => (start_ind * n) % list_size,
        }
    }

    fn index_inverse(&self, end_ind: i64, list_size: i64) -> i64 {
        match self {
            // Inversion is symmetric
            Operation::DealIntoNewStack => list_size - 1 - end_ind,
            Operation::Cut(n) => {
                let n = *n as i64;
                ((end_ind + list_size) + n + list_size) % list_size
            }
            Operation::DealWithIncrement(n) => {
                let n = *n as i64;
                solve_modular(n, list_size, end_ind)
            }
        }
    }
}

fn get_index_inverse(op: Operation, end_ind: i64, list_size: i64) -> i64 {
    match op {
        // Inversion is symmetric
        Operation::DealIntoNewStack => list_size - 1 - end_ind,
        Operation::Cut(n) => {
            let n = n as i64;
            ((end_ind + list_size) + n + list_size) % list_size
        }
        Operation::DealWithIncrement(n) => {
            let n = n as i64;
            solve_modular(n, list_size, end_ind)
        }
    }
}

fn apply_list(operations: &[Operation], s: usize) -> Vec<usize> {
    let mut ls = (0..s).collect::<Vec<_>>();
    for op in operations {
        ls = op.apply(&ls);
    }
    ls
}

fn get_part1(operations: &[Operation]) -> i64 {
    let start_index = 2019;
    let mut ind = start_index;
    for op in operations {
        ind = op.get_index(ind, 10007);
    }
    ind as i64
}

fn get_part2(operations: &[Operation]) -> i64 {
    let deck_size: i64 = 119_315_717_514_047;
    let loop_count: i64 = 101_741_582_076_661;
    // We need to find the index of 2020 in the final list
    let target_ind = 2020;
    let mut new_target = target_ind;
    // let max_loops = 1_000_000_000; // 1e9 was not enought to find a repeat
    let max_loops = 10000;

    let mut prev_target = target_ind;

    // Get operations in reverse order
    let op_inv = operations.iter().rev().collect::<Vec<_>>();
    let mut loop_ind = 0;
    loop {
        loop_ind += 1;
        if loop_ind > max_loops {
            println!(
                "Looped {} times without finding the period. Something is wrong.",
                max_loops
            );
            break;
        }
        for op in op_inv.iter() {
            //let tmp = new_target;
            new_target = op.index_inverse(new_target, deck_size);

            // Double check that the inverse is correct
            //assert_eq!(op.get_index(new_target as usize, deck_size as usize) as i64, tmp);
        }
        if new_target == target_ind {
            println!("After inverse loop {}: {}", loop_ind, new_target);
            println!(
                "This means that the shuffle is periodic with period {}",
                loop_ind
            );
            println!(
                "loop_ind / loop_count = {}",
                loop_ind as f64 / loop_count as f64
            );
            break;
        }
        if loop_ind % 5_000_000 == 0 {
            println!("After inverse loop {}: {}", loop_ind, new_target);
            println!(
                "Index changed by {} from previous loop",
                (new_target - prev_target + deck_size) % deck_size
            );
        }
        prev_target = new_target;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1a() {
        let a = "deal with increment 7
deal into new stack
deal into new stack";

        let op = get_operations(a);
        assert_eq!(apply_list(&op, 10), [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn part1b() {
        let a = "cut 6
deal with increment 7
deal into new stack";

        let op = get_operations(a);
        assert_eq!(apply_list(&op, 10), [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn part1c() {
        let a = "deal with increment 7
deal with increment 9
cut -2";
        let op = get_operations(a);
        dbg!(&op);
        assert_eq!(apply_list(&op, 10), [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn part1d() {
        let a = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        let op = get_operations(a);
        dbg!(&op);
        assert_eq!(apply_list(&op, 10), [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn tmp() {
        let start_list = (0..10).collect::<Vec<_>>();
        let operations = vec![
            Operation::Cut(6),
            Operation::Cut(-6),
            Operation::DealWithIncrement(7),
            Operation::DealIntoNewStack,
        ];
        for op in operations {
            let ls = apply_list(&[op.clone()], 11);
            println!("{:?}", &start_list);
            println!("{:?}", &ls);
            for i in 0..11 {
                assert_eq!(ls[dbg!(op.get_index(i, 11))], i);

                let inv = op.index_inverse(i as i64, 11);
                println!("end j: {}", &i);
                println!("start i: {}", &inv);
                assert_eq!(ls[i] as i64, inv);
            }
        }
    }

    #[test]
    fn gcd() {
        assert_eq!(extended_gcd(11, 7), (2, -3, 1, 2));
        assert_eq!(extended_gcd(7, 11), (-3, 2, 1, 8));
    }

    #[test]
    fn modular() {
        assert_eq!(solve_modular(7, 11, 3), 2);
    }
}
