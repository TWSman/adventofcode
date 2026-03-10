use clap::Parser;
use colored::Colorize;
use intcode::*;
use std::collections::BTreeMap;
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
    println!("Execution lasted {:.2?}", elapsed);
}


fn get_part1(program: &Program) -> i64 {
    let mut p = program.clone();
    let mut input_str = "NOT D T\n".to_string(); // Check that jump target is not empty, set to T
    input_str += "NOT T J\n"; // Move ~T to J, if D was true, J should now be true
    input_str += "NOT A T\n"; // If A is empty, set T to true
    input_str += "AND T J\n"; // If A is empty, and D was true, J should still be true
    // J should now be D && !A
    
    input_str += "NOT E T\n"; // If C is empty, set T to true
    // T should now be !C
    input_str += "AND D T\n";
    // T should now be !C AND D
    input_str += "OR T J\n";
    input_str += "WALK\n";

    println!("Input program:\n{}", input_str);

    let input_as_ascii = input_str.chars().map(|c| c as i64).collect::<Vec<i64>>();

    p.set_verbose(0);
    for o in input_as_ascii {
        p.add_input(o);
    }
    p.run_until_stop();
    // Chech final output
    let final_output = p.get_output(-1);
    if final_output > 127 {
        println!("Final output: {}", final_output);
        return final_output;
    }
    let output = p.get_outputs_ascii();
    println!("{}", output);
    0
}

fn get_part2(program: &Program) -> i64 {
    // WIP
    let mut p = program.clone();
    let mut input_str = "NOT D T\n".to_string(); // Check that jump target is not empty, set to T
    input_str += "NOT T J\n"; // Move ~T to J, if D was true, J should now be true
    input_str += "NOT A T\n"; // If A is empty, set T to true
    input_str += "AND T J\n"; // If A is empty, and D was true, J should still be true
    // J should now be D && !A
    
    input_str += "NOT C T\n"; // If C is empty, set T to true
    // T should now be !C
    input_str += "AND D T\n";
    // T should now be !C AND D
    input_str += "OR T J\n";
    input_str += "RUN\n";

    println!("Input program:\n{}", input_str);

    let input_as_ascii = input_str.chars().map(|c| c as i64).collect::<Vec<i64>>();

    p.set_verbose(0);
    for o in input_as_ascii {
        p.add_input(o);
    }
    p.run_until_stop();
    // Chech final output
    let final_output = p.get_output(-1);
    if final_output > 127 {
        println!("Final output: {}", final_output);
        return final_output;
    }
    let output = p.get_outputs_ascii();
    println!("{}", output);
    0
}


fn read_contents(cont: &str) -> (i64, i64) {
    let vals = cont
        .split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let p = Program::from_list(vals.clone());
    let part1 = get_part1(&p);
    let part2 = get_part2(&p);
    (part1, part2)
}

