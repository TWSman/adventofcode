use clap::Parser;
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

#[derive(Debug, Clone)]
enum Operation {
    DealWithIncrement(usize),
    DealIntoNewStack,
    Cut(isize),
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

    fn apply(list: &[usize], op: &Operation) -> Vec<usize> {
        let list_size = list.len();
        dbg!(&list_size);
        let mut output = (0..list_size).collect::<Vec<usize>>();
        match op {
            Operation::DealIntoNewStack => return list.iter().rev().cloned().collect::<Vec<_>>(),
            Operation::DealWithIncrement(n) => {
                let mut i = 0;
                for v in list {
                    output[(i * n) % list_size] = *v;
                    i += 1;
                }
            }
            Operation::Cut(n) => {
                for (i,v) in list.iter().enumerate() {
                    // Element (i + N - n) % N is set to the value of element i
                    let new_index = (i + list_size) as isize - *n;
                    output[new_index as usize % list_size] = *v;
                }
            }
        }
        output
    }
}

fn apply_list(operations: &[Operation], s:usize) -> Vec<usize> {
    let mut ls = (0..s).collect::<Vec<_>>();
    for op in operations {
        println!("{:?}", &ls);
        ls = Operation::apply(&ls, op);
    }
    dbg!(&ls);
    ls
}

fn get_part1(operations: &[Operation]) -> i64 {
    apply_list(operations, 10007).iter().position(|&x| x == 2019).unwrap() as i64
}

fn get_part2(operations: &[Operation]) -> i64 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1a() {
        let a ="deal with increment 7
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
}
