use clap::Parser;
use core::fmt;
use std::fmt::Display;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn length(&self) -> i64 {
        self.end - self.start + 1
    }

    fn new(ln: &str) -> Self {
        let parts: Vec<&str> = ln.split('-').collect();
        if parts.len() != 2 {
            panic!("Invalid range: {}", ln);
        }
        Range {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        }
    }

    fn remove_blacklist(&self, blacklist: &Range) -> Vec<Range> {
        //println!("Removing blacklist {} from range {}", blacklist, self);
        if blacklist.end < self.start || blacklist.start > self.end {
            //println!("No Overlap");
            return vec![*self];
        }
        if blacklist.start <= self.start && blacklist.end >= self.end {
            //println!("Fully covered");
            return vec![];
        }

        if blacklist.start > self.start && blacklist.end < self.end {
            //println!("Blacklist is in the middle");
            return vec![
                Range {
                    start: self.start,
                    end: blacklist.start - 1,
                },
                Range {
                    start: blacklist.end + 1,
                    end: self.end,
                },
            ];
        }

        if blacklist.start <= self.start {
            //println!("Remove start");
            assert!(blacklist.end >= self.start);
            return vec![Range {
                start: blacklist.end + 1,
                end: self.end,
            }];
        }
        if blacklist.end >= self.end {
            //println!("Remove end");
            assert!(blacklist.start <= self.end);
            return vec![Range {
                start: self.start,
                end: blacklist.start - 1,
            }];
        }
        panic!("Uncovered case");
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:03} - {:03}", self.start, self.end)
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut ranges: Vec<Range> = cont.lines().map(Range::new).collect::<Vec<_>>();
    ranges.sort_by_key(|r| r.start);
    let whitelist = get_whitelist(&ranges);
    let part1 = whitelist.first().unwrap().start;
    let part2 = whitelist.iter().map(|r| r.length()).sum();

    (part1, part2)
}

fn get_whitelist(blacklists: &Vec<Range>) -> Vec<Range> {
    let max_ip = 4294967295;
    let mut accepted_ranges = vec![Range {
        start: 0,
        end: max_ip,
    }];
    for blacklist in blacklists {
        let mut new_accepted_ranges = vec![];
        for accepted in &accepted_ranges {
            new_accepted_ranges.extend(accepted.remove_blacklist(blacklist));
        }
        accepted_ranges = new_accepted_ranges;
    }
    accepted_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "5-8
0-2
4-7";
        assert_eq!(read_contents(&a).0, 3);
    }

    #[test]
    fn range() {
        let range = Range { start: 10, end: 20 };
        let a = Range { start: 21, end: 25 };
        let b = Range { start: 1, end: 9 };
        assert_eq!(
            range.remove_blacklist(&a),
            vec![Range { start: 10, end: 20 }]
        );
        assert_eq!(
            range.remove_blacklist(&b),
            vec![Range { start: 10, end: 20 }]
        );

        let c = Range { start: 13, end: 18 };
        assert_eq!(
            range.remove_blacklist(&c),
            vec![Range { start: 10, end: 12 }, Range { start: 19, end: 20 }]
        );

        let d = Range { start: 13, end: 22 };
        assert_eq!(
            range.remove_blacklist(&d),
            vec![Range { start: 10, end: 12 }]
        );

        let d = Range { start: 7, end: 13 };
        assert_eq!(
            range.remove_blacklist(&d),
            vec![Range { start: 14, end: 20 }]
        );

        let e = Range { start: 12, end: 12 };
        assert_eq!(
            range.remove_blacklist(&e),
            vec![Range { start: 10, end: 11 }, Range { start: 13, end: 20 }]
        );

        let f = Range { start: 20, end: 22 };
        assert_eq!(
            range.remove_blacklist(&f),
            vec![Range { start: 10, end: 19 }]
        );

        let g = Range { start: 9, end: 10 };
        assert_eq!(
            range.remove_blacklist(&g),
            vec![Range { start: 11, end: 20 }]
        );

        let h = Range { start: 10, end: 12 };
        assert_eq!(
            range.remove_blacklist(&h),
            vec![Range { start: 13, end: 20 }]
        );

        let i = Range { start: 18, end: 20 };
        assert_eq!(
            range.remove_blacklist(&i),
            vec![Range { start: 10, end: 17 }]
        );
    }
}
