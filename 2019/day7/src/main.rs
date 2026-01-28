use clap::Parser;
use std::fs;
use intcode::*;
use std::time::Instant;
use itertools::Itertools;

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
    let mut max_res = 0;
    let settings: Vec<i64> = vec![0,1,2,3,4];
    for input_sequence in settings.iter().permutations(settings.len()) {
        let res = try_sequence(program, &input_sequence);
        if res > max_res {
            println!("Found new max {res} with sequence {:?}", input_sequence);
            max_res = res;
        }
    }
    max_res
}

fn get_part2(program: &Program) -> i64 {
    let mut max_res = 0;
    let settings: Vec<i64> = vec![5,6,7,8,9];
    for input_sequence in settings.iter().permutations(settings.len()) {
        let res = try_sequence_feedback(program, &input_sequence);
        if res > max_res {
            println!("Found new max {res} with sequence {:?}", input_sequence);
            max_res = res;
        }
    }
    max_res
}

fn try_sequence_feedback(program: &Program, phase_settings: &[&i64]) -> i64 {
    let mut programs = vec![program.clone(); 5];
    for (p, setting) in programs.iter_mut().zip(phase_settings.iter()) {
        // Phase settings is always the first input
        p.add_input(**setting);
    }
    let mut input_signal = 0;
    let mut e_out = 0;

    loop {
        let mut res = None;
        for p in programs.iter_mut() {
            // Next input is the previous output, or 0 for the first amplifier
            p.add_input(input_signal);
            res = p.run();
            if res.is_none() {
                // Program has halted
                break;
            }
            input_signal = res.unwrap();
        }
        if res.is_none() {
            // Program has halted
            break;
        }
        e_out = input_signal;
    }
    e_out
}

fn try_sequence(p: &mut Program, phase_settings: &[&i64]) -> i64 {
    let mut input_signal = 0;
    for setting in phase_settings {
        // Return program to initial state
        p.reset();

        // First input is the phase setting
        p.add_input(**setting);
        // Second input is the previous output, or 0 for the first amplifier
        p.add_input(input_signal);
        p.run_until_stop();
        input_signal = p.get_output(-1);
    }
    input_signal
}


fn read_contents(cont: &str) -> (i64, i64) {
    let vals = cont.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut program = Program::from_list(vals);
    program.set_verbose(0);
    let part1 = get_part1(&mut program.clone());
    let part2 = get_part2(&program);
    (part1, part2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1(){
        let mut p = Program::from_list(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]);
        p.set_verbose(0);
        assert_eq!(try_sequence(&mut p, &[&4,&3,&2,&1,&0]), 43210);
        assert_eq!(get_part1(&mut p), 43210);

        let mut p = Program::from_list(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]);
        p.set_verbose(0);
        assert_eq!(try_sequence(&mut p, &[&0,&1,&2,&3,&4]), 54321);
        assert_eq!(get_part1(&mut p), 54321);

        let mut p = Program::from_list(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]);
        p.set_verbose(0);
        assert_eq!(try_sequence(&mut p, &[&1,&0,&4,&3,&2]), 65210);
        assert_eq!(get_part1(&mut p), 65210);
    }

    #[test]
    fn part2() {
        let mut p = Program::from_list(vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]);
        p.set_verbose(0);
        assert_eq!(try_sequence_feedback(&mut p, &[&9,&8,&7,&6,&5]), 139629729);
        assert_eq!(get_part2(&mut p), 139629729);

        let mut p = Program::from_list(vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54, -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4, 53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]);
        p.set_verbose(0);
        assert_eq!(try_sequence_feedback(&mut p, &[&9,&7,&8,&5,&6]), 18216);
        assert_eq!(get_part2(&mut p), 18216);
    }
}
