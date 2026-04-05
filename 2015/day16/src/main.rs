use clap::Parser;
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
    let res = read_contents(&contents);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

// Target is:
// children: 3
// cats: 7
// samoyeds: 2
// pomeranians: 3
// akitas: 0
// vizslas: 0
// goldfish: 5
// trees: 3
// cars: 2
// perfumes: 1

#[derive(Debug)]
struct AuntSue {
    name: String,
    values: BTreeMap<String, usize>,
}

impl AuntSue {
    fn new(str: &str) -> Self {
        let (name, res) = str.split_once(':').unwrap();
        let mut out = Self {
            name: name.to_string(),
            values: BTreeMap::new(),
        };
        for a in res.trim().split(',') {
            let (key, value) = a.trim().split_once(':').unwrap();
            let val = value.trim().parse::<usize>().unwrap();
            out.values.insert(key.to_string(), val);
        }
        out
    }

    fn compare(&self, target: &Self) -> bool {
        for (key, value) in &self.values {
            if target.values.get(key).unwrap() != value {
                return false;
            }
        }
        true
    }

    fn compare_part2(&self, target: &Self) -> bool {
        for (key, value) in &self.values {
            match key {
                c if c == "cats" || c == "trees" => {
                    // self must have greater than target,
                    // i.e. target must not have greater than or equal
                    if target.values.get(key).unwrap() >= value {
                        return false;
                    }
                }
                c if c == "pomeranians" || c == "goldfish" => {
                    // self must have less than target,
                    // i.e. target must not have less than or equal
                    if target.values.get(key).unwrap() <= value {
                        return false;
                    }
                }
                _ if target.values.get(key).unwrap() != value => {
                    return false;
                }
                _ => {}
            }
        }
        true
    }
}

fn read_contents(cont: &str) -> (usize, usize) {
    let aunts = cont.lines().map(AuntSue::new).collect::<Vec<_>>();
    let target_aunt = AuntSue::new(
        "target: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1",
    );
    dbg!(&target_aunt);
    println!("{} aunts", aunts.len());
    let part1 = get_part1(&target_aunt, &aunts);
    let part2 = get_part2(&target_aunt, &aunts);
    (part1, part2)
}

fn get_part1(target: &AuntSue, aunts: &[AuntSue]) -> usize {
    for (i, aunt) in aunts.iter().enumerate() {
        if aunt.compare(target) {
            println!("Found match for part 1");
            dbg!(&aunt);
            return i + 1;
        }
    }
    0
}

fn get_part2(target: &AuntSue, aunts: &[AuntSue]) -> usize {
    for (i, aunt) in aunts.iter().enumerate() {
        if aunt.compare_part2(target) {
            println!("Found match for part 2");
            dbg!(&aunt);
            return i + 1;
        }
    }
    0
}
