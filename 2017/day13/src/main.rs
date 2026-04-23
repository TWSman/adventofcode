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

fn read_contents(cont: &str) -> (i32, i32) {
    let mut layers: BTreeMap<i32, i32> = BTreeMap::new();
    for line in cont.lines() {
        let (a, b) = line.split_once(':').unwrap();
        layers.insert(a.parse().unwrap(), b.trim().parse().unwrap());
    }
    let part1 = get_severity(&layers, 0);
    let part2 = get_part2(&layers);
    (part1, part2)
}

fn get_severity(layers: &BTreeMap<i32, i32>, start_time: i32) -> i32 {
    let mut severity = 0;
    for (depth, width) in layers {
        let time = depth + start_time;
        let loc = time % (2 * width - 2);
        if loc == 0 {
            // Caught in this layer
            //println!("Caught at layer {} with width {}", depth, width);
            if start_time > 0 {
                return 1;
            }
            severity += depth * width;
        }
    }
    severity
}

fn get_part2(layers: &BTreeMap<i32, i32>) -> i32 {
    // Check different start times until we find one that works
    let mut start_time = 0;
    loop {
        start_time += 1;
        if start_time % 100_000 == 0 {
            println!("Trying start time {}", start_time);
        }
        if get_severity(layers, start_time) == 0 {
            return start_time;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(read_contents(&a).0, 24);
    }

    #[test]
    fn part2() {}
}
