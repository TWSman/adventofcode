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

type Beam = BTreeMap<(i64, i64), i64>;

fn print_grid(beam: &Beam, scale: i64) {
    let min_x = beam.keys().map(|(x, _)| *x).min().unwrap() / scale - 2;
    let max_x = beam.keys().map(|(x, _)| *x).max().unwrap() / scale + 2;
    let min_y = beam.keys().map(|(_, y)| *y).min().unwrap() / scale - 2;
    let max_y = beam.keys().map(|(_, y)| *y).max().unwrap() / scale + 2;

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            match beam.get(&(x * scale, y * scale)) {
                Some(2) => {
                    print!("{}", "O".red().on_blue());
                }
                Some(1) => {
                    print!("{}", "#".blue().on_blue());
                }
                Some(0) => {
                    print!("{}", ".".black().on_black());
                }
                _ => {
                    print!("{}", ".".white().on_white());
                }
            }
        }
        println!();
    }
}

fn get_part1(program: &Program) -> i64 {
    let mut p = program.clone();
    p.set_verbose(0);

    let mut beam = Beam::new();
    for x in 0..50 {
        for y in 0..50 {
            let (out, xx, yy) = check_coord(program, x, y);
            beam.insert((xx, yy), out);
        }
    }
    print_grid(&beam, 1);
    beam.values().filter(|&&v| v == 1).count() as i64
}

fn check_coord(program: &Program, x: i64, y: i64) -> (i64, i64, i64) {
    let mut p = program.clone();
    p.set_verbose(0);
    p.reset();
    p.add_input(x as i128);
    p.add_input(y as i128);
    loop {
        let res = p.run(None);
        match res {
            ProgramState::Stopped => {
                println!("Program stopped");
                panic!("Program stopped unexpectedly");
            }
            ProgramState::Running => {
                println!("Running");
            }
            ProgramState::WaitingForInput => panic!("Program waiting for input unexpectedly"),
            ProgramState::Output(out) => return (out as i64, x, y),
            ProgramState::Unknown => panic!("Program in unknown state"),
        }
    }
}

fn get_part2(program: &Program, target_width: i64) -> i64 {
    let mut p = program.clone();
    p.set_verbose(0);
    let mut beam = Beam::new();
    let mut x = 100; // initial guess for the distance between the start of the beam and the end of the beam in x direction
    loop {
        if x > 1500 {
            println!("Reached x = {}, giving up", x);
            break;
        }
        let (found, outmap) = check_distance(&mut p, x, target_width);
        for ((xx, yy), out) in outmap {
            beam.insert((xx, yy), out);
        }
        if let Some(y) = found {
            println!("Found potential end of beam at x = {}, y = {}", x, y);
            break;
        } else {
            x *= 2;
        }
    }
    let mut a = x / 2;
    let mut b = x;
    let mut y = 0;
    loop {
        if b - a <= 1 {
            println!("Found potential end of beam at x = {}", a);
            x = b;
            break;
        }
        let mid = (a + b) / 2;
        let (found, outmap) = check_distance(&mut p, mid, target_width);
        for ((xx, yy), out) in outmap {
            beam.insert((xx, yy), out);
        }
        if let Some(val) = found {
            println!("Found potential end of beam at x = {}, y = {}", mid, y);
            b = mid;
            y = val;
        } else {
            a = mid;
        }
    }

    let check_x = x - target_width + 1;
    let check_y = y + target_width - 1;

    for xx in (check_x - 10)..=(x + 10) {
        for yy in (y - 10)..=(check_y + 10) {
            let (out, xx, yy) = check_coord(program, xx, yy);
            beam.insert((xx, yy), out);
        }
    }

    for xx in check_x..=x {
        for yy in y..=check_y {
            assert_eq!(check_coord(program, xx, yy).0, 1);
            beam.insert((xx, yy), 2);
        }
    }
    println!("Found closest point of square at x = {}, y = {}", x, y);
    check_x * 10_000 + y
}

fn check_distance(program: &mut Program, x: i64, target_width: i64) -> (Option<i64>, Beam) {
    let mut outmap = BTreeMap::new();
    let mut found: Option<i64> = None;
    for y in x..(2 * x) {
        // Beam seems to be at a bit over 45 degree angle, so we can start searching for the end of the beam in y direction from x
        let (out, xx, yy) = check_coord(program, x, y);
        outmap.insert((xx, yy), out);

        let check_y = y + 15;
        let (out2, xx, yy) = check_coord(program, x, check_y);
        outmap.insert((xx, yy), out2);
        if found.is_none() && out == 1 {
            //println!("Found start of beam at ({}, {})", x, y);
            found = Some(y);
            let (out, xx, yy) = check_coord(program, x, y + target_width);
            outmap.insert((xx, yy), out);
            if out == 0 {
                //println!("Beam is not wide enough at x = {}", x);
                return (None, outmap);
            }
            for yy in y..(y + target_width) {
                outmap.insert((x, yy), out);
            }
            continue;
        }
    }
    let start_y = found.unwrap();
    for y in start_y..(start_y + target_width * 2) {
        let check_x = x - target_width + 1;
        let check_y = y + target_width - 1;
        let (out, xx, yy) = check_coord(program, check_x, check_y); // Check towards positive x
        outmap.insert((xx, yy), out);

        if out == 0 {
            //println!("Beam is not wide enough at x = {}, y = {} - {}", x, y, check_y);
            continue;
        }
        for xx in check_x..=x {
            for yy in y..=check_y {
                assert_eq!(check_coord(program, xx, yy).0, 1);
                //outmap.insert((xx,yy), 2 as i64);
            }
        }
        outmap.insert((x, y), 2);
        return (Some(y), outmap);
    }
    (None, outmap)
}

fn read_contents(cont: &str) -> (i64, i64) {
    let vals = cont
        .split(",")
        .map(|s| s.trim().parse::<i128>().unwrap())
        .collect::<Vec<_>>();

    let p = Program::from_list(vals.clone());
    let part1 = get_part1(&p);
    let part2 = get_part2(&p, 100);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    // No tests for this one
    // All logic is very much tied to the given program.
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1, 1);
    }
}
