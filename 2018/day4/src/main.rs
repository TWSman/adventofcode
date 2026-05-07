use clap::Parser;
use regex::Regex;
use std::cmp::Ordering;
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
    let mut records = cont.lines().map(Record::new).collect::<Vec<_>>();
    records.sort_by_key(|r| r.timestamp);
    let guards = analyze_records(&records);
    let part1 = get_part1(&guards);
    let part2 = get_part2(&guards);
    (part1, part2)
}

#[derive(Debug)]
enum RecordContent {
    WakeUp,
    FallAsleep,
    Guard(usize),
}

impl RecordContent {
    fn new(ln: &str) -> Self {
        if ln.starts_with("falls") {
            Self::FallAsleep
        } else if ln.starts_with("wakes") {
            Self::WakeUp
        } else if ln.starts_with("Guard") {
            let guard = ln
                .split_whitespace()
                .nth(1)
                .unwrap()
                .strip_prefix('#')
                .unwrap()
                .parse::<usize>()
                .unwrap();
            Self::Guard(guard)
        } else {
            panic!("Unknown line: {}", ln);
        }
    }
}

#[derive(Debug)]
struct Record {
    timestamp: Timestamp,
    message: RecordContent,
}

impl Record {
    fn new(ln: &str) -> Self {
        let (a, message) = ln.split_once("]").unwrap();
        Self {
            timestamp: Timestamp::new(a),
            message: RecordContent::new(message.strip_prefix(' ').unwrap()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Timestamp {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl Timestamp {
    fn new(ln: &str) -> Self {
        let re = Regex::new(r"(\d+)-(\d+)-(\d+) (\d+):(\d+)").unwrap();
        let caps = re.captures(ln).unwrap();
        Self {
            year: caps[1].parse::<usize>().unwrap(),
            month: caps[2].parse::<usize>().unwrap(),
            day: caps[3].parse::<usize>().unwrap(),
            hour: caps[4].parse::<usize>().unwrap(),
            minute: caps[5].parse::<usize>().unwrap(),
        }
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
            .then(self.hour.cmp(&other.hour))
            .then(self.minute.cmp(&other.minute))
    }
}

struct Guard {
    minutes_asleep: BTreeMap<usize, usize>,
    total_sleep: usize,
}

impl Guard {
    fn new() -> Self {
        Guard {
            minutes_asleep: (0..60).map(|i| (i, 0)).collect::<BTreeMap<_, _>>(),
            total_sleep: 0,
        }
    }

    fn add_time(&mut self, start_minute: usize, end_minute: usize) {
        for m in start_minute..end_minute {
            self.total_sleep += 1;
            *self.minutes_asleep.get_mut(&m).unwrap() += 1;
        }
    }

    fn get_max_min(&self) -> (&usize, &usize) {
        self.minutes_asleep.iter().max_by_key(|p| p.1).unwrap()
    }
}

fn analyze_records(records: &[Record]) -> BTreeMap<usize, Guard> {
    let mut guards: BTreeMap<usize, Guard> = BTreeMap::new();
    let mut current_guard = 0;
    let mut sleep_start = 99;
    for rec in records {
        match rec.message {
            RecordContent::Guard(g) => {
                current_guard = g;
            }
            RecordContent::FallAsleep => sleep_start = rec.timestamp.minute,
            RecordContent::WakeUp => {
                assert!(sleep_start != 99);
                guards
                    .entry(current_guard)
                    .or_insert_with(Guard::new)
                    .add_time(sleep_start, rec.timestamp.minute);
            }
        }
    }
    guards
}

fn get_part1(guards: &BTreeMap<usize, Guard>) -> i32 {
    let mut max_min = 0;
    let mut result = 0;
    for (g, guard) in guards {
        if guard.total_sleep > max_min {
            result = g * guard.get_max_min().0;
            max_min = guard.total_sleep;
        }
    }
    result as i32
}

fn get_part2(guards: &BTreeMap<usize, Guard>) -> i32 {
    let mut max_min = 0;
    let mut result = 0;
    for (g, guard) in guards {
        let m = guard.get_max_min();
        if *m.1 > max_min {
            result = g * m.0;
            max_min = *m.1;
        }
    }
    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        assert_eq!(read_contents(&a).0, 240);
    }

    #[test]
    fn part2() {
        let a = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        assert_eq!(read_contents(&a).1, 4455);
    }
}
