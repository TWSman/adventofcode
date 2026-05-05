use clap::Parser;
use std::fs;
use std::time::Instant;
use std::collections::BTreeSet;

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
    let list = cont
        .lines()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let part1 = list.iter().sum();
    let part2 = get_part2(&list);
    (part1, part2)
}

fn get_part2(list: &[i32]) -> i32 {
    let mut seen = BTreeSet::new();
    seen.insert(0);
    let mut freq = 0;
    loop {
        for l in list {
            freq += l;
            if seen.contains(&freq) {
                return freq;
            }
            seen.insert(freq);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "+1
-2
+3
+1";
        assert_eq!(read_contents(&a).0, 3);
    }

    #[test]
    fn part2() {
        let a = "+1
-2
+3
+1";
        assert_eq!(read_contents(&a).1, 2);
    }
}
