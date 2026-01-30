use clap::Parser;
use std::fs;
use intcode::*;
use std::time::Instant;
use shared::Dir;
use std::collections::BTreeMap;
use colored::Colorize;
use std::io::{self, Write};
use rand::Rng;

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
    println!("Part 1 answer is {}", res);  
    //println!("Part 2 answer is {}", res.1);  

    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}



#[derive(PartialEq, Eq)]
enum Object {
    Wall,
    Empty,
    Oxygen,
}


fn wait_for_enter() {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn print_grid(grid: &BTreeMap<(i64, i64), Object>, robot_loc: (i64,i64)) {
    let min_x = grid.keys().map(|(x, _)| *x).min().unwrap() - 2;
    let max_x = grid.keys().map(|(x, _)| *x).max().unwrap() + 2;
    let min_y = grid.keys().map(|(_, y)| *y).min().unwrap() - 2;
    let max_y = grid.keys().map(|(_, y)| *y).max().unwrap() + 2;
    //dbg!(&robot_loc);

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if (x,y) == robot_loc {
                print!("{}", "*".blue());
                continue;
            }
            if (x,y) == (0,0) {
                print!("{}", "o".red());
                continue;
            }
            match grid.get(&(x,y)) {
                Some(&Object::Wall) => {
                    print!("{}", "#".blue().on_blue());
                },
                Some(&Object::Oxygen) => {
                    print!("{}", "#".red().on_white());
                }
                Some(&Object::Empty) => {
                    print!("{}", ".".black());
                },
                None => {
                    print!("{}", ".".white().on_white());
                }
            }
        }
        println!();
    }
}

fn get_part1(program: &mut Program) -> i64 {
    let mut grid: BTreeMap::<(i64,i64), Object> = BTreeMap::new();
    let mut loc = (0,0);
    program.set_verbose(0);

    let mut dir = Dir::N;
    //let mut rng = rand::rng();
    let mut i = 0;
    let mut target_found = false;
    let mut steps_to_target: Vec<(i64,i64)> = vec![];
    let mut result = 0;
    loop {
        let res = program.run(None);
        match res {
            ProgramState::WaitingForInput => {
                // First try to turn right, then straight, then left
                let mut dir_cand = None;
                let mut target_pos = (0,0);
                for d in [dir.cw(), dir, dir.ccw(), dir.opposite()] {
                    let t = match d {
                        Dir::N => (loc.0, loc.1 + 1),
                        Dir::S => (loc.0, loc.1 - 1),
                        Dir::W => (loc.0 - 1, loc.1),
                        Dir::E => (loc.0 + 1, loc.1),
                    };
                    if grid.get(&t).is_none() || grid.get(&t) == Some(&Object::Empty) {
                        //println!("Unknown/empty position at {:?}, trying to explore", t);
                        dir_cand = Some(d);
                        target_pos = t;
                        break;
                    }
                }

                match dir_cand {
                    Some(Dir::N) => program.add_input(1),
                    Some(Dir::S) => program.add_input(2),
                    Some(Dir::W) => program.add_input(3),
                    Some(Dir::E) => program.add_input(4),
                    None => panic!("No direction to explore"),
                }
                i += 1;

                let res = program.run(None);
                match res {
                    ProgramState::Output(0) => {
                        // Hit a wall
                        grid.insert(target_pos, Object::Wall);
                        if i % 10000 == 0  {
                            print_grid(&grid, loc);
                            wait_for_enter();
                        }
                    },
                    ProgramState::Output(1) => {
                        dir = dir_cand.unwrap();
                        grid.insert(target_pos, Object::Empty);
                        loc = target_pos;
                        if i % 10000 == 0 {
                            print_grid(&grid, loc);
                            wait_for_enter();
                        }
                        if steps_to_target.contains(&loc) {
                            //println!("Already been to {:?}, truncating steps", loc);
                            let ind = steps_to_target.iter().position(|&x| x == loc).unwrap();
                            // Truncate steps_to_target to ind
                            steps_to_target.resize(ind + 1, (0,0));
                            assert!(&steps_to_target.contains(&loc));
                        } else {
                            println!("New location {:?}, adding to steps", loc);
                            steps_to_target.push(loc);
                        }
                        //dbg!(&steps_to_target);
                        //dbg!(&steps_to_target.len());
                        if loc == (0,0) && target_found {
                            println!("Back to start");
                            print_grid(&grid, loc);
                            wait_for_enter();
                            break;
                        }
                    },
                    ProgramState::Output(2) => {
                        dir = dir_cand.unwrap();
                        grid.insert(target_pos, Object::Oxygen);
                        print_grid(&grid, loc);
                        wait_for_enter();
                        loc = target_pos;
                        target_found = true;
                        result = 1 + steps_to_target.len() as i64;
                    },
                    _ => break,
                        }
                    }
            _ => panic!("Unexpected state"),
        }
    }
    result
}

fn read_contents(cont: &str) -> i64 {
    let vals = cont.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let p = Program::from_list(vals.clone());
    let part1 = get_part1(&mut p.clone());
    part1
}
