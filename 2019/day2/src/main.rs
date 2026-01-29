use clap::Parser;
use std::fs;
use intcode::*;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("\n########################");  
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  

    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn get_part1(mut vals: Vec<i64>) -> i64 {
    // For part1, replace position 1 with 12 and position 2 with 2
    vals[1] = 12;
    vals[2] = 2;
    let mut p = Program::from_list(vals);
    p.run(None);
    p.get_index(0)
}

fn get_part2(vals: Vec<i64>, target: i64) -> i64 {
    let mut p = Program::from_list(vals);
    p.set_verbose(0);
    for noun in 0..100 {
        for verb in 0..100 {
            p.set_index(1, noun);
            p.set_index(2, verb);
            p.run(None);
            if p.get_index(0) == target {
                println!("Found noun {} and verb {}", noun, verb);
                return 100 * noun + verb;
            }
            p.reset();
        }
    }
    0
}

fn read_contents(cont: &str) -> (i64, i64) {
    let vals = cont.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let part1 = get_part1(vals.clone());
    let part2 = get_part2(vals.clone(), 19690720);
    (part1, part2)
}


