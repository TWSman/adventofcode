use clap::Parser;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
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
    let res = read_contents(&contents, None);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

#[derive(Debug, Clone)]
struct Transform {
    start: String,
    end: String,
}

impl Transform {
    fn new(ln: &str) -> Self {
        let (start, end) = ln.split_once(" => ").unwrap();
        Self {
            start: start.to_string(),
            end: end.to_string(),
        }
    }
}

fn read_contents(cont: &str, start: Option<&str>) -> (usize, i32) {
    let transforms = cont
        .lines()
        .filter(|ln| ln.contains("=>"))
        .map(Transform::new)
        .collect::<Vec<_>>();
    println!("{} possible transformations", transforms.len());
    let start = if let Some(s) = start {
        s
    } else {
        &cont.lines().last().unwrap().to_string()
    };
    let part1 = get_part1(start, &transforms);
    let part2 = get_part2(start, &transforms);
    (part1, part2)
}

fn get_part1(input: &str, transforms: &Vec<Transform>) -> usize {
    let mut results: BTreeSet<String> = BTreeSet::new();
    for transform in transforms {
        let start = &transform.start;
        for (i, _c) in input.match_indices(start) {
            let mut new_string = input.to_string();
            new_string.replace_range(i..(i + start.len()), &transform.end);
            results.insert(new_string);
        }
    }
    results.len()
}

fn get_part2(target: &str, transforms: &Vec<Transform>) -> i32 {
    // Task is to find how many steps it takes to go from "e" to target
    // This logic starts from target and tries to reach "e"
    let mut queue = PriorityQueue::new();
    let prio = i32::try_from(target.len()).unwrap();
    queue.push((target.to_string(), 0), Reverse(prio));
    loop {
        if queue.is_empty() {
            return 0;
        }
        let ((input, n), _prio) = queue.pop().unwrap();
        let mut results: BTreeSet<String> = BTreeSet::new();
        for transform in transforms {
            let end = &transform.end;
            for (i, _c) in input.match_indices(end) {
                let mut new_string = input.clone();
                new_string.replace_range(i..(i + end.len()), &transform.start);
                if new_string == "e" {
                    return n + 1;
                }
                results.insert(new_string.clone());
            }
        }
        for res in results {
            let prio = i32::try_from(res.len()).unwrap() - n - 1;
            queue.push((res, n + 1), Reverse(prio));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "H => HO
H => OH
O => HH";
        assert_eq!(read_contents(&a, Some("HOH")).0, 4);
        assert_eq!(read_contents(&a, Some("HOHOHO")).0, 7);
    }

    #[test]
    fn part2() {
        let a = "e => H
e => O
H => HO
H => OH
O => HH";
        assert_eq!(read_contents(&a, Some("HOH")).1, 3);
        assert_eq!(read_contents(&a, Some("HOHOHO")).1, 6);
    }
}
