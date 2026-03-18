use clap::Parser;
use intcode::*;
use std::collections::BTreeMap;
use std::collections::VecDeque;
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
    println!("Starting part 1");
    let mut programs: BTreeMap<usize, Program> = BTreeMap::new();
    let mut queues: BTreeMap<usize, VecDeque<(i128, i128)>> = BTreeMap::new();
    for i in 0..50 {
        let mut p = program.clone();
        p.add_input(i as i128);
        p.set_verbose(0);
        programs.insert(i, p);
        queues.insert(i, VecDeque::new());
    }

    let mut loop_count = 0;
    loop {
        loop_count += 1;
        for i in 0..50 {
            let p = programs.get_mut(&i).unwrap();
            let queue = queues.get_mut(&i).unwrap();
            let mut outputs = Vec::new();
            if queue.is_empty() {
                p.add_input(-1);
            }
            while let Some((x, y)) = queue.pop_front() {
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
                let dest = chunk[0] as usize;
                let x = chunk[1];
                let y = chunk[2];
                if dest == 255 {
                    println!("After {loop_count} loops, Got output to 255: {}, {}", x, y);
                    return y as i64;
                }
                queues.get_mut(&dest).unwrap().push_back((x, y));
            }
        }
    }
}

fn get_part2(program: &Program) -> i64 {
    println!("Starting part 2");
    let mut programs: BTreeMap<usize, Program> = BTreeMap::new();
    let mut queues: BTreeMap<usize, VecDeque<(i128, i128)>> = BTreeMap::new();
    let mut nat_packet: Option<(i128, i128)> = None;
    for i in 0..50 {
        let mut p = program.clone();
        p.add_input(i as i128);
        p.set_verbose(0);
        programs.insert(i, p);
        queues.insert(i, VecDeque::new());
    }

    let mut loop_count = 0;
    let mut prev_y = 0;
    let max_loops = 100;
    let mut packets_sent = Vec::new();
    let mut idle_count = 0;
    loop {
        loop_count += 1;
        if loop_count > max_loops {
            break;
        }
        let mut got_input = 0;
        if loop_count % 1000 == 0 {
            println!("Loop {}", loop_count);
        }
        for i in 0..50 {
            let p = programs.get_mut(&i).unwrap();
            let queue = queues.get_mut(&i).unwrap();
            let mut outputs = Vec::new();
            if queue.is_empty() {
                p.add_input(-1);
            }
            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();
                p.add_input(x);
                p.add_input(y);
                got_input += 1;
            }
            loop {
                match p.run(None) {
                    ProgramState::Output(output) => outputs.push(output),
                    ProgramState::WaitingForInput => break,
                    _ => panic!("Unexpected state"),
                }
            }
            for chunk in outputs.chunks(3) {
                if idle_count > 0 {
                    println!(
                        "Output from {} to {}: {}, {}",
                        i, chunk[0], chunk[1], chunk[2]
                    );
                }
                let dest = chunk[0] as usize;
                let x = chunk[1];
                let y = chunk[2];
                if dest == 255 {
                    nat_packet = Some((x, y));
                } else {
                    queues.get_mut(&dest).unwrap().push_back((x, y));
                }
            }
        }
        if got_input == 0 {
            if let Some((x, y)) = nat_packet {
                println!(
                    "loop: {}, Sending NAT packet to 0: {}, {}",
                    loop_count, x, y
                );
                queues.get_mut(&0).unwrap().push_back((x, y));
                packets_sent.push((x, y));
                if prev_y == y {
                    println!("Got repeated Y value: {}", y);
                    return y as i64;
                }
                idle_count = 0;
                prev_y = y;
            } else {
                panic!("No output or input, but no NAT packet either");
            }
        }
    }
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
