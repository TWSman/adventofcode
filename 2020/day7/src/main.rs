use clap::Parser;
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

#[derive(Debug, Clone)]
struct Bag {
    name: String,
    children: Vec<(i64, String)>,
    parents: Vec<String>,
}

impl Bag {
    fn new(ln: &str) -> Self {
        let (a, b) = ln.split_once(" bags contain ").unwrap();
        let name = a.to_string();
        let mut children = Vec::new();
        if b != "no other bags." {
            for child in b.strip_suffix(".").unwrap().split(',') {
                let (count, rest) = child.trim().split_once(' ').unwrap();
                let count = count.parse::<i64>().unwrap();
                children.push((
                    count,
                    rest.replace(" bags", "").replace(" bag", "").to_string(),
                ));
            }
        }
        Self {
            name,
            children,
            parents: Vec::new(),
        }
    }
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

fn read_contents(cont: &str) -> (i64, i64) {
    let vec = cont.lines().map(Bag::new).collect::<Vec<_>>();
    let mut map = vec
        .iter()
        .map(|bag| (bag.name.clone(), bag.clone()))
        .collect::<BTreeMap<_, _>>();
    for bag in vec.iter() {
        for child in &bag.children {
            map.get_mut(&child.1)
                .unwrap()
                .parents
                .push(bag.name.clone());
        }
    }
    let part1 = get_part1(&map);
    let part2 = get_part2(&map);
    (part1, part2)
}

fn get_part1(map: &BTreeMap<String, Bag>) -> i64 {
    let start = "shiny gold";
    let mut queue = vec![start];
    let mut potential = BTreeSet::new();
    loop {
        if queue.is_empty() {
            break;
        }
        let node = queue.pop().unwrap();
        let bag = map.get(node).unwrap();
        for parent in &bag.parents {
            if potential.contains(parent) {
                continue;
            }
            potential.insert(parent.clone());
            queue.push(parent);
        }
    }
    potential.len() as i64
}

fn get_part2(bags: &BTreeMap<String, Bag>) -> i64 {
    let mut forest = BagForest {
        bags: bags.clone(),
        totals: BTreeMap::new(),
    };
    forest.get("shiny gold")
}

struct BagForest {
    bags: BTreeMap<String, Bag>,
    totals: BTreeMap<String, i64>,
}

impl BagForest {
    fn get(&mut self, key: &str) -> i64 {
        if let Some(v) = self.totals.get(key) {
            return *v;
        }
        let children = self.bags.get(key).unwrap().children.clone();
        let mut sum = 0;
        for (count, child) in children {
            sum += count * (1 + self.get(&child));
        }
        self.totals.insert(key.to_string(), sum);
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(read_contents(&a).0, 4);
        assert_eq!(read_contents(&a).1, 32);
    }

    #[test]
    fn part2() {
        let a = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(read_contents(&a).1, 126);
    }
}
