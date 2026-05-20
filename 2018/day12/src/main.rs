use clap::Parser;
use colored::Colorize;
use std::collections::BTreeMap;
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
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut rules: RuleMap = BTreeMap::new();
    let mut vec = Vec::new();
    for line in cont.lines() {
        if line.starts_with("initial") {
            let state = line.split_once(':').unwrap().1.trim();
            vec = state.chars().map(|c| c == '#').collect::<Vec<_>>();
            continue;
        }
        if line.contains('>') {
            let rule = Rule::new(line);
            rules.insert(rule.input, rule.out);
        }
    }
    dbg!(&rules);
    dbg!(&vec);

    let part1 = get_sum(&vec, &rules, 20);
    let part2 = get_sum(&vec, &rules, 50_000_000_000);
    (part1, part2)
}

type RuleMap = BTreeMap<(bool, bool, bool, bool, bool), bool>;

#[derive(Debug)]
struct Rule {
    input: (bool, bool, bool, bool, bool),
    out: bool,
}

impl Rule {
    fn new(ln: &str) -> Self {
        let (a, b) = ln.split_once("=>").unwrap();
        let input = a.trim().chars().map(|c| c == '#').collect::<Vec<_>>();
        let out = b.trim().starts_with('#');
        assert_eq!(input.len(), 5);
        Self {
            input: (input[0], input[1], input[2], input[3], input[4]),
            out,
        }
    }
}

fn get_hash(plants: &BTreeSet<i64>) -> String {
    let min_plant = *plants.iter().min().unwrap();
    let max_plant = *plants.iter().max().unwrap();
    let mut output = String::new();
    for x in min_plant..=max_plant {
        if plants.contains(&x) {
            output.push('#');
        } else {
            output.push('.');
        }
    }
    output
}

fn get_sum(input: &[bool], rules: &RuleMap, steps: usize) -> i64 {
    let mut plants: BTreeSet<i64> = BTreeSet::new();
    for (i, t) in input.iter().enumerate() {
        if *t {
            plants.insert(i as i64);
        }
    }
    let mut seen = BTreeMap::new();
    print_plants(&plants, -5, (input.len() * 2) as i64);

    for i in 0..steps {
        let hash = get_hash(&plants);
        let start_plant = plants.iter().min().unwrap() - 2;
        if let Some((prev_start, prev_loop)) = seen.get(&hash) {
            assert_eq!(i - prev_loop, 1);
            assert_eq!(start_plant - prev_start, 1);

            let steps_remaining = steps - i;
            println!(
                "state already seen at step {}, starting from {}",
                prev_loop, prev_start
            );
            return plants.iter().sum::<i64>() + (plants.len() as i64) * steps_remaining as i64;
        }
        seen.insert(hash, (start_plant, i));
        let max_plant = plants.iter().max().unwrap() + 2;
        let mut new_plants: BTreeSet<i64> = BTreeSet::new();

        for t in start_plant..=max_plant {
            let input = (
                plants.contains(&(t - 2)),
                plants.contains(&(t - 1)),
                plants.contains(&(t)),
                plants.contains(&(t + 1)),
                plants.contains(&(t + 2)),
            );
            if let Some(true) = rules.get(&input) {
                new_plants.insert(t);
            }
        }
        plants = new_plants;

        print_plants(&plants, -5, (input.len() * 2) as i64);
    }
    println!();
    plants.iter().sum()
}

fn print_plants(plants: &BTreeSet<i64>, min_val: i64, max_val: i64) {
    for x in min_val..=max_val {
        if plants.contains(&x) {
            print!("{}", "#".red().on_black());
        } else {
            print!("{}", ".".white().on_black());
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
        assert_eq!(read_contents(&a).0, 325);
    }

    #[test]
    fn part2() {}
}
