use clap::Parser;
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

#[derive(Clone, Debug)]
struct Policy {
    a: usize,
    b: usize,
    letter: char,
    password: Vec<char>,
}

impl Policy {
    fn new(ln: &str) -> Self {
        let parts = ln.split_whitespace().collect::<Vec<_>>();
        let (mn, mx) = parts[0].split_once('-').unwrap();
        let letter = parts[1].chars().next().unwrap();
        Self {
            a: mn.parse::<usize>().unwrap(),
            b: mx.parse::<usize>().unwrap(),
            letter,
            password: parts[2].chars().collect::<Vec<_>>(),
        }
    }

    fn check_part1(&self) -> bool {
        let c = self.password.iter().filter(|c| **c == self.letter).count();
        !{ c < self.a || c > self.b }
    }

    fn check_part2(&self) -> bool {
        let mut ok = 0;
        if *self.password.get(self.a - 1).unwrap() == self.letter {
            ok += 1;
        }
        if *self.password.get(self.b - 1).unwrap() == self.letter {
            ok += 1;
        }
        ok == 1
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let list = cont.lines().map(Policy::new).collect::<Vec<_>>();
    let part1 = get_part1(&list);
    let part2 = get_part2(&list);
    (part1, part2)
}

fn get_part1(list: &[Policy]) -> i64 {
    list.iter().filter(|p| p.check_part1()).count() as i64
}
fn get_part2(list: &[Policy]) -> i64 {
    list.iter().filter(|p| p.check_part2()).count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(read_contents(&a).0, 2);
    }

    #[test]
    fn part2() {
        let a = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(read_contents(&a).1, 1);
    }
}
