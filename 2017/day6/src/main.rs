use clap::Parser;
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

fn read_contents(cont: &str) -> (i32, i32) {
    let blocks = cont
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let (part1, blocks2) = redistribute(&blocks);
    let (part2, _) = redistribute(&blocks2);
    (part1, part2)
}

fn redistribute(ls: &[i32]) -> (i32, Vec<i32>) {
    let mut vec = ls
        .iter()
        .enumerate()
        .map(|(i, v)| (i, *v))
        .collect::<Vec<_>>();
    let mut seen = BTreeSet::new();
    println!("{vec:?}");
    let n = vec.len();
    let mut loops = 0;
    loop {
        let mut sorted = vec.clone();
        let str = vec.iter().map(|(_a, b)| b.to_string()).collect::<String>();
        if seen.contains(&str) {
            println!("Seen {str} before");
            break;
        }
        loops += 1;
        seen.insert(str);

        //sorted.sort_by(|a, b| a.0.cmp(&b.0));
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        let (ind, count) = sorted[0];
        *vec.get_mut(ind).unwrap() = (ind, 0);
        for i in 1..=count {
            let next_ind = (ind + i as usize) % n;
            vec.get_mut(next_ind).unwrap().1 += 1;
        }
    }
    // Return the current state of the vector, which will be used for part 2
    let part2 = vec.iter().map(|(_a, b)| *b).collect::<Vec<_>>();
    (loops, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "0 2 7 0";
        assert_eq!(read_contents(&a).0, 5);
    }

    #[test]
    fn part2() {
        let a = "0 2 7 0";
        assert_eq!(read_contents(&a).1, 4);
    }
}
