use clap::Parser;
use std::fs;
use intcode::*;
use std::time::Instant;
use shared::Dir;
use std::collections::BTreeMap;
use colored::Colorize;
use std::io::{self, Write};

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



#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
                    print!("{}", "#".red().on_red());
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

fn get_part1(program: &mut Program) -> (i64, BTreeMap<(i64,i64), Object>) {
    let mut grid: BTreeMap::<(i64,i64), Object> = BTreeMap::new();
    let mut loc = (0,0);
    program.set_verbose(0);

    let mut dir = Dir::N;
    //let mut rng = rand::rng();
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
                    if !grid.contains_key(&t) || grid.get(&t) == Some(&Object::Empty) {
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

                let res = program.run(None);
                match res {
                    // Program outputs 0, if the suggested move would hit a wall
                    ProgramState::Output(0) => {
                        grid.insert(target_pos, Object::Wall);
                    },
                    // Program outputs 1, if the suggested move ends in an empty space
                    ProgramState::Output(1) => {
                        dir = dir_cand.unwrap();
                        grid.insert(target_pos, Object::Empty);
                        loc = target_pos;
                        if steps_to_target.contains(&loc) {
                            let ind = steps_to_target.iter().position(|&x| x == loc).unwrap();

                            // Truncate steps_to_target to ind
                            //println!("Already been to {:?}, truncating steps", loc);
                            steps_to_target.resize(ind + 1, (0,0));
                            assert!(&steps_to_target.contains(&loc));
                        } else {
                            println!("New location {:?}, adding to steps", loc);
                            steps_to_target.push(loc);
                        }
                        if loc == (0,0) && target_found {
                            println!("Back to start");
                            print_grid(&grid, loc);
                            break;
                        }
                    },
                    // Program outputs 2, if the suggested move ends in the location of the oxygen system
                    ProgramState::Output(2) => {
                        //
                        dir = dir_cand.unwrap();
                        grid.insert(target_pos, Object::Oxygen);
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
    (result, grid)
}

fn get_part2(grid: &BTreeMap<(i64, i64), Object>) -> i64 {
    let start = grid.iter().find(|(_, v)| **v == Object::Oxygen).unwrap().0;
    let mut grid = grid.clone();
    let mut heads = Vec::from([*start]);
    let mut timer = 0;
    loop {
        let mut new_heads = Vec::new();
        for head in heads {
            // From each head, try to expand in each direction
            for d in [Dir::N, Dir::S, Dir::W, Dir::E] {
                let candidate = match d {
                    Dir::N => (head.0, head.1 + 1),
                    Dir::S => (head.0, head.1 - 1),
                    Dir::W => (head.0 - 1, head.1),
                    Dir::E => (head.0 + 1, head.1),
                };
                if grid.get(&candidate) == Some(&Object::Empty) {
                    // If the space is currently empty, fill it with oxygen
                    // Add it to heads for the next round
                    grid.insert(candidate, Object::Oxygen);
                    new_heads.push(candidate);
                }
            }
        }
        if timer % 30 == 0 {
            print_grid(&grid, *start);
            wait_for_enter();
        }
        if new_heads.is_empty() {
            break;
        }
        timer += 1;
        heads = new_heads;
    }
    print_grid(&grid, *start);
    timer
}

fn read_contents(cont: &str) -> (i64, i64) {
    let vals = cont.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let p = Program::from_list(vals.clone());
    let (part1, grid) = get_part1(&mut p.clone());
    let part2 = get_part2(&grid);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
        //
        //  ##   
        // #..## 
        // #.#..#
        // #.O.# 
        //  ###  
        let grid = BTreeMap::from([
            ((1,4), Object::Wall), ((2,4), Object::Wall),
            ((0,3), Object::Wall), ((1,3), Object::Empty), ((2,3), Object::Empty), ((3,3), Object::Wall), ((4,3), Object::Wall),
            ((0,2), Object::Wall), ((1,2), Object::Empty), ((2,2), Object::Wall), ((3,2), Object::Empty), ((4,2), Object::Empty), ((5,2), Object::Wall),
            ((0,1), Object::Wall), ((1,1), Object::Empty), ((2,1), Object::Oxygen), ((3,1), Object::Empty), ((4,1), Object::Wall),
            ((1,0), Object::Wall), ((2,0), Object::Wall), ((3,0), Object::Wall),
        ]);
        print_grid(&grid, (0,0));
        //  Should take 4 minutes to fill with oxygen
        assert_eq!(get_part2(&grid), 4)
    }
}
