use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::fmt::Display;
use core::fmt;

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
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn length(&self) -> i64 {
        self.end - self.start + 1
    }
}


impl Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:015} - {:015}", self.start, self.end)
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut ranges: BTreeMap<usize, Range> = BTreeMap::new();
    let mut ids: Vec<i64> = vec![];
    for (i,l) in cont.lines().enumerate() {
        let parts: Vec<&str> = l.split('-').collect();
        match parts.len() {
            2 => {
                ranges.insert(i,
                    Range {
                        start: parts[0].parse().unwrap(),
                        end: parts[1].parse().unwrap(),
                    });
            },
            1 => {
                match parts[0].parse() {
                    Ok(n) => ids.push(n),
                    Err(_) => continue,
                }
            }
            0 => continue,
            _ => panic!("Invalid line: {}", l),
        }
    }
    let part1 = ids.iter().filter(|id| {
        ranges.iter().any(|(_i, r)| {
            **id >= r.start && **id <= r.end
        })
    }).count() as i64;

    let part2 = get_part2(&ranges);

    (part1, part2)
}


fn get_part2(ranges: &BTreeMap<usize, Range>) -> i64 {
    let mut new_ranges: BTreeMap<usize, Range> = ranges.clone();

    // Star combining ranges
    let idx: Vec<usize> = new_ranges.keys().copied().collect();

    loop {
        let mut changed: bool = false;
        for i in idx.iter() {
            for j in idx.iter() {
                if i == j {
                    continue;
                }
                let r1 = match new_ranges.get(i) {
                    Some(v) => v,
                    None => continue,

                };
                let r2 = match new_ranges.get(j) {
                    Some(v) => v,
                    None => continue,

                };

                // r1 is completely before r2
                if r1.end < r2.start {
                    continue;
                }

                // r2 is completely before r1
                if r2.end < r1.start {
                    continue;
                }
                // otherwise there is overlap
                let new_start = std::cmp::min(r1.start, r2.start);
                let new_end = std::cmp::max(r1.end, r2.end);
                let new_range = Range { start: new_start, end: new_end };
                println!("Merging ranges {:} and {:} into {:}", r1, r2, new_range);

                new_ranges.remove(i);
                new_ranges.remove(j);
                new_ranges.insert(*i, new_range);
                changed = true;
            }
        }

        // There were no changes, so we can stop the loop
        if !changed {
            break;
        }
    }

    new_ranges.values().map(|r| {
        r.length()
    }).sum::<i64>()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(read_contents(&a).0, 3);
        assert_eq!(read_contents(&a).1, 14);
    }
}
