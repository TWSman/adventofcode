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

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    speed: i32,
    interval: i32,
    rest: i32,
    points: i32,
}

impl Reindeer {
    fn new(str: &str) -> Self {
        let words = str.split_whitespace().collect::<Vec<_>>();
        let speed = words[3].parse::<i32>().unwrap();
        let interval = words[6].parse::<i32>().unwrap();
        let rest = words[13].parse::<i32>().unwrap();
        Self {
            speed,
            interval,
            rest,
            points: 0,
        }
    }

    fn get_part1(&self, time: i32) -> i32 {
        let full_seq = self.interval + self.rest;
        let n = time / full_seq;
        let n2 = time - full_seq * n;
        self.speed * self.interval * n + self.interval.min(n2) * self.speed
    }
}

fn get_part2(reindeer: &[Reindeer], time: usize) -> i32 {
    let mut reindeer = reindeer.to_owned();
    let mut distances: BTreeMap<usize, i32> = BTreeMap::new();
    for t in 1..=time {
        let mut max_dist = 0;
        for (j, r) in reindeer.iter().enumerate() {
            let dist = r.get_part1(i32::try_from(t).unwrap());
            distances.insert(j, dist);
            if dist > max_dist {
                max_dist = dist;
            }
        }
        for (j, r) in reindeer.iter_mut().enumerate() {
            if distances.get(&j).unwrap() == &max_dist {
                r.points += 1;
            }
        }
    }
    reindeer.iter().map(|r| r.points).max().unwrap()
}

fn read_contents(cont: &str) -> (i32, i32) {
    let reindeer = cont.lines().map(Reindeer::new).collect::<Vec<_>>();
    println!("{} reindeer", reindeer.len());
    let part1 = reindeer.iter().map(|r| r.get_part1(2503)).max().unwrap();
    let part2 = get_part2(&reindeer, 2503);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let reindeer = a.lines().map(Reindeer::new).collect::<Vec<_>>();
        dbg!(&reindeer);
        assert_eq!(reindeer[0].get_part1(1000), 1120);
        assert_eq!(reindeer[1].get_part1(1000), 1056);
    }

    #[test]
    fn part2() {
        let a = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let reindeer = a.lines().map(Reindeer::new).collect::<Vec<_>>();
        dbg!(&reindeer);
        assert_eq!(get_part2(&reindeer, 1000), 689);
    }
}
