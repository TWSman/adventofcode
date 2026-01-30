use clap::Parser;
use std::fs;
use intcode::*;
use std::time::Instant;
use std::collections::BTreeMap;
use colored::Colorize;
use std::io;

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

#[derive(Debug, PartialEq, Eq)]
enum Object {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Object {
    fn new(id: i64) -> Self {
        match id {
            0 => Object::Empty,
            1 => Object::Wall,
            2 => Object::Block,
            3 => Object::Paddle,
            4 => Object::Ball,
            _ => panic!("Unknown object id {}", id),
        }
    }
}

fn print_grid(grid: &BTreeMap<(usize, usize), Object>) {
    let min_x = grid.keys().map(|(x, _)| *x).min().unwrap();
    let max_x = grid.keys().map(|(x, _)| *x).max().unwrap();
    let min_y = grid.keys().map(|(_, y)| *y).min().unwrap();
    let max_y = grid.keys().map(|(_, y)| *y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match grid.get(&(x,y)) {
                Some(&Object::Empty) => { print!("{}", ".".black()); },
                Some(&Object::Wall) => { print!("{}", "#".white().on_white()); },
                Some(&Object::Block) => { print!("{}", "#".blue().on_blue()); },
                Some(&Object::Ball) => { print!("{}", "O".red()); },
                Some(&Object::Paddle) => { print!("{}", "_".blue()); },
                _ => {   print!("{}", ".".black()); }
            }
        }
        println!();
    }
}

fn get_part1(program: &mut Program) -> i64 {
    program.run_until_stop();
    let output = program.get_outputs();
    let mut grid: BTreeMap<(usize,usize), Object> = BTreeMap::new();
    for r in output.chunks(3) {
        let x = r[0];
        let y = r[1];
        let tile_id = r[2];
        let obj = Object::new(tile_id);
        grid.insert((x as usize, y as usize), obj);
    }
    print_grid(&grid);
    grid.values().filter(|v| **v == Object::Block).count() as i64
}


enum Choice {
    Left,
    Center,
    Right,
    Quit,
}

fn read_input() -> Option<Choice> {
    loop {
        println!("Choose (L, C, or R) or q for quit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        return match input.trim().to_uppercase().as_str() {
            "L" => Some(Choice::Left),
            "C" => Some(Choice::Center),
            "R" => Some(Choice::Right),
            "Q" => Some(Choice::Quit),
            _ => {
                println!("Try again.\n");
                continue;
            },
        }
    }
}


fn get_part2(program: &mut Program, read_user_input: bool) -> i64 {
    let inp = vec![];
    for i in inp {
        program.add_input(i);
    }
    program.set_index(0, 2); // Setting memory address 0 to 2, starts free play mode
    program.set_verbose(0);
    let mut grid: BTreeMap<(usize,usize), Object> = BTreeMap::new();
    let mut output: Vec<i64> = vec![];
    let mut score = 0;
    let mut ball_x: i64 = 0;
    let mut paddle_x: i64 = 0;
    loop {
        let res = program.run(None); // Run until action is needed
        match res {
            ProgramState::Output(out) => {
                // Program returned an output
                output.push(out);
            }
            ProgramState::Stopped => {
                println!("Program returned: STOP");
                break;
            }
            ProgramState::WaitingForInput => {
                // Program is waiting for input
                if read_user_input {
                    // Manual mode
                    println!("Score: {}", score);
                    print_grid(&grid);
                    match read_input() {
                        Some(Choice::Right) => {
                            program.add_input(1);
                        }
                        Some(Choice::Left) => {
                            program.add_input(-1);
                        }
                        Some(Choice::Center) => {
                            program.add_input(0);
                        }
                        Some(Choice::Quit) => break,
                        None => break,
                    }
                    continue;
                } 
                // In automatic mode move the paddle always towards the ball position
                if ball_x == paddle_x {
                    program.add_input(0);
                } else if ball_x < paddle_x {
                    program.add_input(-1);
                } else {
                    program.add_input(1);
                }
                continue;
            }
            _ => {
                panic!("Unexpected program state {:?}", res);
            }
        }

        if output.len() == 3 {
            // Outputs come in triplets, Either defining a tile with coordinates or the score
            let x = output[0];
            let y = output[1];
            if (x,y) == (-1, 0) {
                score = output[2];
                output.clear();
                continue;
            }
            let tile_id = output[2];
            let obj = Object::new(tile_id);
            if obj == Object::Ball {
                ball_x = x;
            }
            if obj == Object::Paddle {
                paddle_x = x;
            }
            grid.insert((x as usize, y as usize), obj);
            output.clear();
        }
    }
    print_grid(&grid);
    println!("Needed {} moves to beat the game.", program.get_input_pointer());
    println!("Score: {}", score);
    score
}

fn read_contents(cont: &str) -> (i64, i64) {
    let vals = cont.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut p = Program::from_list(vals.clone());
    p.set_verbose(0);
    let part1  = get_part1(&mut p.clone());
    let part2 = get_part2(&mut p.clone(), true);
    (part1, part2)
}
