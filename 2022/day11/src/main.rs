use std::fs;
use clap::Parser;
use regex::Regex;
use itertools::Itertools;
use std::collections::VecDeque;
use std::collections::BTreeMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

#[derive(Debug, Clone)]
enum Op {
    Multiply(i64),
    Square,
    Add(i64),
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: VecDeque<i64>, // list of worry numbers
    operation: Op,
    test: i64,
    success_to: usize,
    fail_to: usize,
    inspections: u64,
}

impl Monkey {
// Parse input like
// Monkey 1:
//   Starting items: 54, 65, 75, 74
//   Operation: new = old + 6
//   Test: divisible by 19
//     If true: throw to monkey 2
//     If false: throw to monkey 0
    fn new(chunk: Vec<&str>) -> Self {
        // 
        let get_id = Regex::new(r"Monkey ([0-9]*):").unwrap();
        let get_op = Regex::new(r"Operation: new = old (.*) (.*)").unwrap();
        let Some(monkey_id) = get_id.captures(chunk[0]) else { panic!() };
        let Some(res3) = get_op.captures(chunk[2]) else { panic!() };
        let operation = if &res3[2] == "old" {
            Op::Square
        } else if &res3[1] == "*"{
            Op::Multiply(res3[2].parse::<i64>().unwrap())
        } else if &res3[1] == "+"{
            Op::Add(res3[2].parse::<i64>().unwrap())
        } else {
            panic!();
        };
        let test = chunk[3][20..].trim().parse::<i64>().unwrap();
        let items = chunk[1][17..].split(',').map(|m| {
            m.trim().parse::<i64>().unwrap()
        }).collect::<VecDeque<i64>>();
        let success_to = chunk[4][29..].parse::<usize>().unwrap();
        let fail_to = chunk[5][30..].parse::<usize>().unwrap();
        Monkey {id: monkey_id[1].parse::<usize>().unwrap(), 
            items,
            operation,
            test,
            fail_to,
            success_to,
            inspections: 0,
        }
    }

    fn add_item(&mut self, item: i64) {
        self.items.push_back(item);
    }

    // Inspect all the items this monkey holds
    // Return list of items and their targets
    fn do_round(&mut self, part1: bool) -> Vec<(usize, i64)> {
        let mut output = Vec::new();
        while let Some(mut level) = self.items.pop_front() {
            self.inspections += 1;
            level = match self.operation {
                Op::Square => level * level,
                Op::Add(a) => level + a,
                Op::Multiply(a) => level * a,
            };
            if part1 {
                // This is skipped in part2
                level /= 3;
            }
            let target = if level % self.test == 0 {
                self.success_to
            } else {
                self.fail_to
            };
            output.push((target, level));
        }
        output
    }
}

fn read_contents(cont: &str) -> (u64, u64) {
    let monkeys = cont.lines().chunks(7).into_iter().map(|chnk| {
        let monkey = Monkey::new(chnk.collect::<Vec<&str>>());
        (monkey.id, monkey)
    }).collect::<BTreeMap<usize, Monkey>>();
    dbg!(&monkeys);
    let keys = monkeys.keys().copied();
    // Get part1, 20 rounds
    let mut monkeys1 = monkeys.clone();
    for _ in 0..20 {
        for k in keys.clone() {
            let m = monkeys1.get_mut(&k).unwrap();
            let res = m.do_round(true);
            for (target, item) in res {
                monkeys1.get_mut(&target).unwrap().add_item(item); 
            }
        }
    }
    let test_vals = monkeys.values().map(|m| m.test).collect::<Vec<i64>>();
    let combined_test: i64 = test_vals.iter().product();
    let part1 = get_results(&monkeys1);
    let mut monkeys2 = monkeys.clone();
    // Get part2, 10000 rounds
    for i in 0..10000 {
        if i % 100 == 0 {
            println!("{} / {}", i, 10_000);
        }
        for k in keys.clone() {
            let m = monkeys2.get_mut(&k).unwrap();
            let res = m.do_round(false);
            for (target, item) in res {
                monkeys2.get_mut(&target).unwrap().add_item(
                    // Must keep track of the modulo. Otherwise numbers would grow way too big
                    item % combined_test
                    ); 
            }
        }
    }
    let part2 = get_results(&monkeys2);
    (part1, part2)
}


fn get_results(monkeys: &BTreeMap<usize, Monkey>) -> u64 {
    // Get the top 2 highest inspection counts and multiply them
    let mut inspections = monkeys.iter().map(|(_,m)| m.inspections).collect::<Vec<u64>>();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {

        let a = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(read_contents(&a).0, 10605);
        assert_eq!(read_contents(&a).1, 2713310158);

    }
}
