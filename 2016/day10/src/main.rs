use clap::Parser;
use regex::Regex;
use std::collections::BTreeMap;
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
    let res = read_contents(&contents, (17, 61));
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_instructions(cont: &str) -> (BTreeMap<i32, usize>, BTreeMap<usize, (i32, i32)>) {
    let re_value = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let re_bot = regex::Regex::new(
        r"bot (\d+) gives low to (output|bot) (\d+) and high to (output|bot) (\d+)",
    )
    .unwrap();
    let mut values_to = BTreeMap::new();
    let mut bots_to = BTreeMap::new();
    for ln in cont.lines() {
        if ln.contains("value") {
            let caps = re_value.captures(ln).unwrap();
            let value = caps[1].parse::<i32>().unwrap();
            let bot = caps[2].parse::<usize>().unwrap();
            if values_to.contains_key(&value) {
                panic!("Bot {} already has a value", bot);
            }
            values_to.insert(value, bot);
        } else {
            let caps = re_bot.captures(ln).unwrap();
            let bot = caps[1].parse::<usize>().unwrap();
            let mut low_to = caps[3].parse::<i32>().unwrap();
            if caps[2] == *"output" {
                low_to = -low_to - 1;
            }
            let mut high_to = caps[5].parse::<i32>().unwrap();
            if caps[4] == *"output" {
                high_to = -high_to - 1;
            }
            bots_to.insert(bot, (low_to, high_to));
        }
    }
    (values_to, bots_to)
}

fn read_contents(cont: &str, target: (i32, i32)) -> (i64, i64) {
    let (values_to, bots_to) = read_instructions(cont);
    let part1 = run(&values_to, &bots_to, target, false);
    let part2 = run(&values_to, &bots_to, target, true);
    (part1, part2)
}

fn run(
    values_to: &BTreeMap<i32, usize>,
    bots_to: &BTreeMap<usize, (i32, i32)>,
    check: (i32, i32),
    part2: bool,
) -> i64 {
    // Bot id and its chips
    let mut bots: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    // Output id and chips there
    let mut outputs: BTreeMap<i32, i32> = BTreeMap::new();
    for (value, bot) in values_to.iter() {
        bots.entry(*bot as i32).or_default().push(*value);
    }
    loop {
        let mut actions = vec![];
        for (bot, vec) in bots.iter_mut() {
            if vec.len() == 2 {
                //println!("Bot {} has chips {:?} and is comparing them", bot, vec);
                let bot_instructions = bots_to.get(&(*bot as usize)).unwrap();
                vec.sort();
                let high = vec.pop().unwrap();
                let low = vec.pop().unwrap();
                assert!(low < high);
                if low == check.0 && high == check.1 && !part2 {
                    //println!("Bot {} is comparing {} and {}", bot, low, high);
                    return *bot as i64;
                }
                let low_target = bot_instructions.0;
                let high_target = bot_instructions.1;
                actions.push((low, low_target));
                actions.push((high, high_target));
                break;
            }
        }
        //println!("{} actions", actions.len());
        for action in &actions {
            let value = action.0;
            let target = action.1;
            //println!("{value} goes to {} {}", if target < 0 { "output" } else { "bot" }, target.abs());
            if target < 0 {
                let output_id = -target - 1;
                if outputs.contains_key(&output_id) {
                    panic!();
                }
                outputs.insert(output_id, value);
            } else {
                bots.entry(target).or_default().push(value);
            }
        }
        if actions.is_empty() {
            break;
        }
    }
    if part2 {
        (outputs.get(&0).unwrap_or(&0)
            * outputs.get(&1).unwrap_or(&0)
            * outputs.get(&2).unwrap_or(&0)) as i64
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";
        assert_eq!(read_contents(&a, (2, 5)).0, 2);
        assert_eq!(read_contents(&a, (2, 5)).1, 30);
    }
}
