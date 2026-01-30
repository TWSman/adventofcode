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

fn get_part1(program: &mut Program) -> i64 {
    program.set_verbose(0);
    program.add_input(1);
    program.run_until_stop();
    program.get_output(-1)
}

fn get_part2(program: &mut Program) -> i64 {
    program.add_input(2);
    program.run_until_stop();
    program.get_output(-1)
}


fn read_contents(cont: &str) -> (i64, i64) {
    let vals = cont.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut p = Program::from_list(vals.clone());
    let part1 = get_part1(&mut p.clone());
    let part2 = get_part2(&mut p);
    (part1, part2)
}
