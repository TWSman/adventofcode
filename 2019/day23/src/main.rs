use clap::Parser;
use intcode::*;
use std::fs;
use std::time::Instant;
use std::collections::BTreeMap;

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
    let mut programs: BTreeMap<usize, Program> = BTreeMap::new();
    let mut queues: BTreeMap<usize, Vec<(i64, i64)>> = BTreeMap::new();
    for i in 0..50 {
        let mut p = program.clone();
        p.add_input(i as i64);
        p.set_verbose(0);
        programs.insert(i, p);
        queues.insert(i, Vec::new());
    }

    let mut loop_count = 0;
    loop {
        loop_count += 1;
        println!("Loop {}", loop_count);
        for i in 0..50 {
            let p = programs.get_mut(&i).unwrap();
            let queue = queues.get_mut(&i).unwrap();
            let mut outputs = Vec::new();
            if queue.is_empty() {
                p.add_input(-1);
            }
            while !queue.is_empty() {
                let (x,y) = queue.pop().unwrap();
                p.add_input(x);
                p.add_input(y);
            }
            loop {
                match p.run(None) {
                    ProgramState::Output(output) => outputs.push(output),
                    ProgramState::WaitingForInput => break,
                    _ => panic!("Unexpected state"),
                }
            }
            for chunk in outputs.chunks(3) {
                println!("Output from {} to {}: {}, {}", i, chunk[0], chunk[1], chunk[2]);
                let dest = chunk[0] as usize;
                let x = chunk[1];
                let y = chunk[2];
                if dest == 255 {
                    println!("Got output to 255: {}, {}", x, y);
                    println!("After {loop_count} loops");
                    return y;
                }
                queues.get_mut(&dest).unwrap().push((x,y));
            }
        }
    }
}

fn get_part2(program: &Program) -> i64 {
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
