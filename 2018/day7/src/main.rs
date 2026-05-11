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

#[derive(Debug)]
struct Step {
    name: char,
    pre: Vec<char>,
    duration: usize,
}

impl Step {
    fn new(name: char) -> Self {
        Self {
            name,
            pre: Vec::new(),
            duration: (name as u8 - b'A' + 61) as usize,
        }
    }

    fn insert_pre(&mut self, name: char) {
        self.pre.push(name);
    }
}

fn read_contents(cont: &str) -> (String, i32) {
    let steps = read_steps(cont);
    let part1 = get_part1(&steps);
    let part2 = get_part2(&steps, 5);
    (part1, part2)
}

fn read_steps(cont: &str) -> BTreeMap<char, Step> {
    let mut steps = BTreeMap::new();
    for line in cont.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        // Before step
        let a = parts[1].chars().next().unwrap();
        // After step
        let b = parts[7].chars().next().unwrap();

        steps.entry(a).or_insert_with(|| Step::new(a));
        steps.entry(b).or_insert_with(|| Step::new(b)).insert_pre(a);
    }
    steps
}

fn get_part1(steps: &BTreeMap<char, Step>) -> String {
    let mut finished: Vec<char> = Vec::new();
    loop {
        let mut possible: BTreeSet<char> = BTreeSet::new();
        for (c, step) in steps {
            if finished.contains(c) {
                // This step is already finished
                continue;
            }
            let mut use_this = true;
            for pre in step.pre.iter() {
                if !finished.contains(pre) {
                    use_this = false;
                    break;
                }
            }
            if use_this {
                possible.insert(*c);
            }
        }
        finished.push(*possible.iter().next().unwrap());
        if finished.len() == steps.len() {
            break;
        }
    }

    finished.iter().collect::<String>()
}

fn get_part2(steps: &BTreeMap<char, Step>, workers: usize) -> i32 {
    let mut inprogress: BTreeMap<char, usize> = BTreeMap::new();
    let mut finished: BTreeSet<char> = BTreeSet::new();
    let mut time = 0;
    loop {
        for (i, t) in inprogress.iter_mut() {
            *t -= 1;
            if *t == 0 {
                println!("{}: {} is finished", time, i);
                finished.insert(*i);
            }
        }
        if finished.len() == steps.len() {
            return time;
        }
        time += 1;

        inprogress.retain(|_i, t| *t > 0);

        let mut possible: Vec<&Step> = Vec::new();
        for (c, step) in steps {
            if finished.contains(c) {
                // This has already been finished
                continue;
            }
            if inprogress.contains_key(c) {
                continue;
            }
            let mut use_this = true;
            for pre in step.pre.iter() {
                if !finished.contains(pre) {
                    use_this = false;
                    break;
                }
            }
            if use_this {
                possible.push(step);
            }
        }
        possible.sort_by_key(|s| s.name);
        for step in possible.iter() {
            if inprogress.len() < workers {
                println!("{}: Start {}", time, step.name);
                inprogress.insert(step.name, step.duration);
            }
        }
        //dbg!(&inprogress);
        if time > 20000 {
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        assert_eq!(read_contents(&a).0, "CABDFE");
    }

    #[test]
    fn part2() {
        let a = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let mut steps = read_steps(a);
        for (_c, step) in steps.iter_mut() {
            step.duration -= 60;
        }
        assert_eq!(get_part2(&steps, 2), 15);
    }
}
