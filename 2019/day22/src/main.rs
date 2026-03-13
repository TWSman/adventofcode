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
    DealWithIncrement(i128),
    DealIntoNewStack,
    Cut(i128),
    Identity,
}

#[memoize]
fn get_gcd(a: i128, b: i128) -> (i128, i128, i128, i128) {
    // Extended Euclidean Algorithm to find the greatest common divisor of a and b
    // And also the multiplicative inverse of a mod b
    // returns (t, s, gcd, a_inv) such that t*a + s*b = gcd
    // a * a_inv = 1 mod b
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }
    let a_inv = (old_s + b) % b;
    (old_s, old_t, old_r, a_inv)
}

fn solve_modular(a: i128, m: i128, b: i128) -> i128 {
    let (_x, _y, _gcd, a_inv) = get_gcd(a, m);

    let t = (b * a_inv) % m;
    t % m
}

impl Operation {
    fn new(ln: &str) -> Self {
        if ln.starts_with("deal into new stack") {
            Operation::DealIntoNewStack
        } else if ln.starts_with("deal with increment ") {
            let splits = ln.split(" ").collect::<Vec<_>>();
            let n = splits.last().unwrap().parse::<i128>().unwrap();
            Operation::DealWithIncrement(n)
        } else if ln.starts_with("cut ") {
            let splits = ln.split(" ").collect::<Vec<_>>();
            let n = splits.last().unwrap().parse::<i128>().unwrap();
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
                    output[(i * *n as usize) % list_size] = *v;
                }
            }
            Operation::Cut(n) => {
                for (i, v) in list.iter().enumerate() {
                    // Element (i + N - n) % N is set to the value of element i
                    let new_index = (i + list_size) as i128 - *n;
                    output[new_index as usize % list_size] = *v;
                }
            }
            Operation::Identity => return list.to_vec(),
        }
        output
    }

    fn combine(first: &Self, other: &Self, list_size: usize) -> Vec<Operation> {
        // Get the combination
        match (first, other) {
            (Operation::Cut(n), Operation::Cut(m)) if (n + m) % list_size as i128 == 0 => {
                vec![Operation::Identity]
            }
            (Operation::Cut(n), Operation::Cut(m)) => {
                vec![Operation::Cut((n + m) % list_size as i128)]
            }

            (Operation::DealIntoNewStack, Operation::DealIntoNewStack) => vec![Operation::Identity],
            (Operation::DealWithIncrement(n), Operation::DealWithIncrement(m)) => {
                vec![Operation::DealWithIncrement((n * m) % list_size as i128)]
            }
            (Operation::Identity, op) | (op, Operation::Identity) => vec![*op],
            (Operation::Cut(n), Operation::DealIntoNewStack) => {
                vec![Operation::DealIntoNewStack, Operation::Cut(-n)]
            }
            (Operation::Cut(n), Operation::DealWithIncrement(m)) => vec![
                Operation::DealWithIncrement(*m),
                Operation::Cut((n * *m) % list_size as i128),
            ],
            (Operation::DealIntoNewStack, Operation::DealWithIncrement(n)) => vec![
                Operation::DealWithIncrement(*n),
                Operation::DealIntoNewStack,
                Operation::Cut((-1 + *n) % list_size as i128),
            ],
            _ => vec![],
        }
    }

    // Get the target index, of start
    fn get_index(&self, start_ind: i128, list_size: i128) -> i128 {
        match self {
            Operation::DealIntoNewStack => list_size - 1 - start_ind,
            Operation::Cut(n) => ((start_ind + list_size) - *n + list_size) % list_size,
            Operation::DealWithIncrement(n) => (start_ind * n) % list_size,
            Operation::Identity => start_ind,
        }
    }

    fn index_inverse(&self, end_ind: i128, list_size: i128) -> i128 {
        match self {
            // Inversion is symmetric
            Operation::DealIntoNewStack => list_size - 1 - end_ind,
            Operation::Cut(n) => {
                let n = *n;
                ((end_ind + list_size) + n + list_size) % list_size
            }
            Operation::DealWithIncrement(n) => {
                let n = *n;
                solve_modular(n, list_size, end_ind)
            }
            Operation::Identity => end_ind,
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
    dbg!(&operations.len());
    let operations = reorder_operations(operations, 10007);
    dbg!(&operations.len());
    let start_index = 2019;
    let mut ind = start_index;
    for op in operations {
        ind = op.get_index(ind, 10007);
    }
    ind as i64
}

fn reorder_operations(operations: &[Operation], list_size: usize) -> Vec<Operation> {
    let mut reordered = operations.to_vec();

    let mut loop_count = 0;
    // Simplify the list of operations by repeatedly combining adjacent operations and inverting pairs of operations until no more changes can be made
    loop {
        loop_count += 1;
        let mut changed = false;
        let n = reordered.len();

        // Then try to combine pairs of operations
        for i in 1..n {
            let first = reordered[i - 1];
            let second = reordered[i];
            match Operation::combine(&first, &second, list_size).as_slice() {
                [Operation::Identity] => {
                    changed = true;
                    reordered.remove(i);
                    reordered.remove(i - 1);
                    break;
                }
                [op1] => {
                    changed = true;
                    reordered[i - 1] = *op1;
                    reordered.remove(i);
                    break;
                }
                [op1, op2] => {
                    changed = true;
                    reordered[i - 1] = *op1;
                    reordered[i] = *op2;
                    break;
                }
                [op1, op2, op3] => {
                    changed = true;
                    reordered[i - 1] = *op1;
                    reordered[i] = *op2;
                    reordered.insert(i + 1, *op3);
                    break;
                }
                _ => continue,
            }
        }
        if !changed {
            break;
        }
    }
    println!("Reordering took {} loops", loop_count);
    reordered
}

fn combine_operations(
    operations_a: &[Operation],
    operations_b: &[Operation],
    list_size: usize,
) -> Vec<Operation> {
    let mut combo = operations_a.to_owned();
    combo.extend(operations_b.iter());
    reorder_operations(&combo, list_size)
}

fn flatten_operations(
    operations: &[Operation],
    list_size: usize,
    loop_count: usize,
) -> Vec<Operation> {
    let operations = reorder_operations(operations, list_size);
    let mut combo = vec![Operation::Identity];
    let mut running = operations.to_owned();
    let mut t = loop_count;
    let mut div;
    let mut bin = String::new();
    while t > 0 {
        (t, div) = (t / 2, t % 2);
        bin += if div == 0 { "0" } else { "1" };
        if div == 1 {
            combo = combine_operations(&combo, &running, list_size);
        }

        running = combine_operations(&running, &running, list_size);
    }
    assert_eq!(
        bin.chars().rev().collect::<String>(),
        format!("{:b}", loop_count)
    );
    combo
}

fn get_part2(operations: &[Operation]) -> i64 {
    let deck_size: i128 = 119_315_717_514_047;
    let loop_count: i128 = 101_741_582_076_661;
    let operations = flatten_operations(operations, deck_size as usize, loop_count as usize);
    dbg!(&operations.len());

    // We need to find the index of 2020 in the final list
    let target_ind = 2020;

    // Get operations in reverse order
    let op_inv = operations.iter().rev().collect::<Vec<_>>();

    let mut t = target_ind;
    for op in op_inv {
        t = op.index_inverse(t, deck_size);
    }
    t as i64
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
    fn reorder() {
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
        let op_a = get_operations(a);
        assert_eq!(apply_list(&op_a, 10), [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);

        let b = "deal with increment 3
deal into new stack
cut 2";
        let op_b = get_operations(b);
        assert_eq!(apply_list(&op_a, 10), apply_list(&op_b, 10));

        let op_reorder = reorder_operations(&op_a, 10);
        assert_eq!(op_reorder, op_b);
    }

    #[test]
    fn reorder_tmp() {
        let a = "deal with increment 7
cut 4
cut -2
deal with increment 7
deal with increment 7";
        let op_a = get_operations(a);
        let b = "deal with increment 3
cut 8";

        let op_b = get_operations(b);
        let op_reorder = reorder_operations(&op_a, 10);
        assert_eq!(op_reorder, op_b);
    }

    // Combination rules:
    // Cut x
    // Cut y
    // -> Cut x + y
    //
    // Deal with increment x
    // Deal with increment y
    // -> Deal with increment x * y
    //
    // Cut x
    // Deal into new stack
    // ->
    // Deal into new stack
    // Cut -x
    //
    // Cut x
    // Deal with increment y
    // ->
    // Deal with increment y
    // Cut x * y
    //
    //
    // deal into new stack
    // deal with increment x
    // ->
    // deal with increment x
    // deal into new stack
    // cut x - 1

    #[test]
    fn combine() {
        let b = "deal with increment 3
cut 8";
        let op_b = get_operations(b);
        let target = "deal with increment 9
cut 2";
        let op_target = get_operations(target);

        assert_eq!(combine_operations(&op_b, &op_b, 10), op_target);
    }

    #[test]
    fn combos() {
        let op_cut = Operation::Cut(6);
        let op_cut2 = Operation::Cut(-2);
        let op_cut_combo = Operation::combine(&op_cut, &op_cut2, 10)[0];
        let op_cut_inv = Operation::Cut(-6);
        let op_deal = Operation::DealIntoNewStack;

        for i in 0..10 {
            assert_eq!(
                op_cut.get_index(op_deal.get_index(i, 10), 10),
                op_deal.get_index(op_cut_inv.get_index(i, 10), 10),
            );
            assert_eq!(op_cut.get_index(op_cut_inv.get_index(i, 10), 10), i);

            assert_eq!(
                op_cut.get_index(op_cut2.get_index(i, 10), 10),
                op_cut_combo.get_index(i, 10),
            );
        }

        let n = 7;
        let op_deal = Operation::DealWithIncrement(n);
        let op_rev = Operation::DealIntoNewStack;
        let op_cut = Operation::Cut(1 - n);
        for i in 0..10 {
            assert_eq!(
                op_deal.get_index(op_rev.get_index(i, 10), 10),
                op_rev.get_index(op_cut.get_index(op_deal.get_index(i, 10), 10), 10),
            );
        }
    }

    #[test]
    fn gcd() {
        assert_eq!(get_gcd(11, 7), (2, -3, 1, 2));
        assert_eq!(get_gcd(7, 11), (-3, 2, 1, 8));
    }

    #[test]
    fn modular() {
        assert_eq!(solve_modular(7, 11, 3), 2);
    }
}
