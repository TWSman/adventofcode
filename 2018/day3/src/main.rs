use clap::Parser;
use regex::Regex;
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
    let claims = cont.lines().map(Claim::new).collect::<Vec<_>>();
    let part1 = get_part1(&claims);
    let part2 = get_part2(&claims);
    (part1, part2)
}

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn new(ln: &str) -> Self {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let caps = re.captures(ln).unwrap();
        Self {
            id: caps[1].parse::<usize>().unwrap(),
            left: caps[2].parse::<usize>().unwrap(),
            top: caps[3].parse::<usize>().unwrap(),
            width: caps[4].parse::<usize>().unwrap(),
            height: caps[5].parse::<usize>().unwrap(),
        }
    }

    fn min_x(&self) -> usize {
        self.left + 1
    }

    fn max_x(&self) -> usize {
        self.left + self.width
    }

    fn min_y(&self) -> usize {
        self.top + 1
    }

    fn max_y(&self) -> usize {
        self.top + self.height
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        !(x < self.min_x() || y < self.min_y() || x > self.max_x() || y > self.max_y())
    }

    fn overlap(&self, other: &Claim) -> bool {
        !(other.max_x() < self.min_x()
            || other.max_y() < self.min_y()
            || other.min_x() > self.max_x()
            || other.min_y() > self.max_y())
    }
}

fn get_part1(claims: &[Claim]) -> i32 {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 1000;
    let mut min_y = 1000;
    for l in claims {
        if l.min_x() < min_x {
            min_x = l.min_x();
        }
        if l.min_y() < min_y {
            min_y = l.min_y();
        }

        if l.max_x() > max_x {
            max_x = l.max_x();
        }
        if l.max_y() > max_y {
            max_y = l.max_y();
        }
    }
    let mut sum = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            if claims.iter().filter(|c| c.contains(x, y)).count() > 1 {
                sum += 1;
            }
        }
    }
    sum
}

fn get_part2(claims: &[Claim]) -> i32 {
    for (i, claim1) in claims.iter().enumerate() {
        let mut no_overlap = true;
        for (j, claim2) in claims.iter().enumerate() {
            if j == i {
                continue;
            }
            if claim1.overlap(claim2) {
                no_overlap = false;
            }
        }
        if no_overlap {
            return claim1.id as i32;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
        assert_eq!(read_contents(&a).0, 4);
    }

    #[test]
    fn part2() {
        let a = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
        assert_eq!(read_contents(&a).1, 3);
    }
}
