use clap::Parser;
use std::fs;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();
    read_file(&args.input);
}

const CRATEMOVER: i32 = 9001; // 9000 for part1, 9001 for part2

fn read_file(filename: &str) {
    let stack_count: usize = 9;
    let mut stacks: Vec<Vec<char>> = (0..stack_count).map(|_| {Vec::<char>::new()}).collect();
    dbg!(&stacks);
    let re1: Regex = Regex::new(r"\[[A-Z]\]").unwrap();
    let re2: Regex = Regex::new(r"1   2   3   4   5   6   7   8   9").unwrap();
    let re3: Regex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let contents: Vec<String> = fs::read_to_string(filename)
        .unwrap() // Panic on errors
        .lines() // Split the string into an iterator
        .map(String::from) // Make each slice into a string
        .collect(); // Collect them in a vector
    for ln in contents {
        if re1.is_match(&ln) {
            let spl = ln.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>();
            for (i,sp) in spl.iter().enumerate() {
                let c = sp.chars().nth(1).unwrap();
                if c == ' ' {
                    continue
                } else {
                    stacks[i].push(c)
                }
            }
            continue;
        }
        if re2.is_match(&ln) {
            for s in &mut stacks {
                s.reverse()
            }
            println!("Stacks are initialized");
            continue;
        }
        let Some(res) = re3.captures(&ln) else { continue };
        let count = res[1].parse::<usize>().unwrap();
        let from = res[2].parse::<usize>().unwrap() - 1;
        let to = res[3].parse::<usize>().unwrap() - 1;
        println!("Move from index {} to index {}", from, to);
        if CRATEMOVER == 9000 {
            for _ in 0..count {
                let a = stacks[from].pop().unwrap();
                stacks[to].push(a);
            }
        } else {
            let mut tmp = Vec::<char>::new();
            for _ in 0..count {
                let a = stacks[from].pop().unwrap();
                tmp.push(a);
            }
            for _ in 0..count {
                let a = tmp.pop().unwrap();
                stacks[to].push(a);
            }
        }
        //dbg!(res);
    }

    let lasts: String = stacks.iter().map(|v| { v[v.len()-1]}).collect();
    println!("{}", lasts);

}
