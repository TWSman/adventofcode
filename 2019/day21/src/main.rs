use clap::Parser;
use intcode::*;
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
    let mut input_str = "OR A J\n".to_string(); // J should now be A
    input_str += "AND B J\n"; // J should now be A AND B
    input_str += "AND C J\n"; // J should now be A AND C
    input_str += "NOT J J\n"; // J should now be ~(A AND C) = ~A OR ~C
    input_str += "AND D J\n"; // J should now be D AND ~(A AND C)
    input_str += "WALK\n";

    // J = D && (!(A AND B AND C)) // jump if target is not empty, and either A or B or C is empty

    println!("Input program:\n{}", input_str);

    let input_as_ascii = input_str.chars().map(|c| c as i64).collect::<Vec<i64>>();

    p.set_verbose(0);
    for o in input_as_ascii {
        p.add_input(o as i128);
    }
    p.run_until_stop();
    // Chech final output
    let final_output = p.get_output(-1);
    if final_output > 127 {
        let n = p.get_outputs();
        let output = n[..n.len() - 1]
            .iter()
            .map(|&o| o as u8 as char)
            .collect::<String>();
        println!("Program prints:\n{}", output);

        println!("Final output: {}", final_output);
        return final_output.try_into().unwrap();
    }
    let output = p.get_outputs_ascii();
    println!("{}", output);
    0
}

fn get_part2(program: &Program) -> i64 {
    let mut p = program.clone();

    // First segment checks that we can continue moving after the jump, either by a further jump, or
    // by taking a step. Thus either the next tile must have ground, or the next jump target must
    // have ground
    // i.e. (E OR H)
    let mut input_str = "OR E J\n".to_string(); // J = E
    input_str += "OR H J\n"; // J = E OR H

    // Next segment checks that jumping actually makes sense, i.e. there is an empty space between
    // current space and target space
    // i.e. ~A OR ~B OR ~C
    input_str += "OR A T\n"; // T = A
    input_str += "AND B T\n"; // T = A AND B
    input_str += "AND C T\n"; // T = A AND B AND C
    input_str += "NOT T T\n"; // T = ~(A AND B AND C) = ~A OR ~B OR ~C
    input_str += "AND T J\n"; // 
    // (~A OR ~B OR ~C) AND (E OR H)

    // Finally check that target space is safe to jump to, i.e. D is true
    input_str += "AND D J\n";
    // (~A OR ~B OR ~C) AND (E OR H) AND D
    input_str += "RUN\n";

    println!("Input program:\n{}", input_str);

    let input_as_ascii = input_str.chars().map(|c| c as i64).collect::<Vec<i64>>();

    p.set_verbose(0);
    for o in input_as_ascii {
        p.add_input(o as i128);
    }
    p.run_until_stop();
    // Chech final output
    let final_output = p.get_output(-1);
    if final_output > 127 {
        // If the output is not valid ASCII,
        let n = p.get_outputs();
        let output = n[..n.len() - 1]
            .iter()
            .map(|&o| o as u8 as char)
            .collect::<String>();
        println!("Program prints:\n{}", output);

        println!("Final output: {}", final_output);
        return final_output.try_into().unwrap();
    }
    let output = p.get_outputs_ascii();
    println!("{}", output);
    0
}

fn read_contents(cont: &str) -> (i64, i64) {
    let vals = cont
        .split(",")
        .map(|s| s.trim().parse::<i128>().unwrap())
        .collect::<Vec<_>>();

    let p = Program::from_list(vals.clone());
    let part1 = get_part1(&p);
    let part2 = get_part2(&p);
    (part1, part2)
}
