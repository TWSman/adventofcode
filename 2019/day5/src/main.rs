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

fn get_result(vals: Vec<i64>, input: i64) -> i64 {
    let mut p = Program::from_list(vals);
    p.add_input(input);
    p.run_until_stop();
    p.get_output(-1)
}


fn read_contents(cont: &str) -> (i64, i64) {
    let vals = cont.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let part1 = get_result(vals.clone(), 1);
    let part2 = get_result(vals.clone(), 5);
    (part1, part2)
}


