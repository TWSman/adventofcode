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

#[derive(Debug)]
struct Node {
    metadata: Vec<u32>,
    children: Vec<Node>,
}

impl Node {
    fn new(ll: &[u32]) -> (Self, Vec<u32>) {
        let n_kids = ll[0] as usize;
        let n_metadata = ll[1] as usize;
        let mut rest = ll[2..].to_vec();
        let mut children = Vec::new();
        for _ in 0..n_kids {
            let tmp = Node::new(&rest);
            rest = tmp.1;
            children.push(tmp.0);
        }
        (
            Node {
                metadata: rest[..n_metadata].to_vec(),
                children,
            },
            rest[n_metadata..].to_vec(),
        )
    }

    fn metadata_sum(&self) -> u32 {
        self.children.iter().map(|c| c.metadata_sum()).sum::<u32>()
            + self.metadata.iter().sum::<u32>()
    }

    fn get_value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .map(|i| {
                    if *i as usize > self.children.len() {
                        0
                    } else {
                        self.children.get((i - 1) as usize).unwrap().get_value()
                    }
                })
                .sum()
        }
    }
}

fn read_contents(cont: &str) -> (u32, u32) {
    let tree = read_tree(cont);
    let part1 = tree.metadata_sum();
    let part2 = tree.get_value();
    (part1, part2)
}

fn read_tree(cont: &str) -> Node {
    let nums = cont
        .split_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    Node::new(&nums).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(read_contents(&a).0, 138);
    }

    #[test]
    fn part2() {
        let a = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(read_contents(&a).1, 66);
    }
}
