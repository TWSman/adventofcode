use clap::Parser;
use std::fs;
use intcode::*;
use std::time::Instant;
use shared::Dir;
use std::collections::BTreeMap;
use colored::Colorize;

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

struct Robot {
    loc: (i64, i64),
    direction: Dir,
}

impl Robot {
    fn new() -> Self {
        Robot {
            loc: (0, 0),
            direction: Dir::N,
        }
    }

    fn take_one_step(&mut self) {
        match self.direction {
            Dir::N => {
                self.loc.1 += 1;
            },
            Dir::S => {
                self.loc.1 -= 1;
            },
            Dir::W => {
                self.loc.0 -= 1;
            },
            Dir::E => {
                self.loc.0 += 1;
            },
        }
    }

    fn turn_left(&mut self) {
        self.direction = self.direction.ccw();
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.cw();
    }
}


#[derive(PartialEq, Eq)]
enum Color {
    White,
    Black
}

fn print_grid(grid: &BTreeMap<(i64, i64), Color>) {
    let min_x = grid.keys().map(|(x, _)| *x).min().unwrap() - 2;
    let max_x = grid.keys().map(|(x, _)| *x).max().unwrap() + 2;
    let min_y = grid.keys().map(|(_, y)| *y).min().unwrap() - 2;
    let max_y = grid.keys().map(|(_, y)| *y).max().unwrap() + 2;

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if grid.get(&(x,y)) == Some(&Color::White) {
                print!("{}", "#".white().on_white());
            } else {
                print!("{}", ".".black());
            }
        }
        println!();
    }
}

fn get_part1(program: &mut Program, part2: bool) -> i64 {
    let mut robot = Robot::new();
    let mut grid: BTreeMap<(i64,i64), Color> = BTreeMap::new();
    if part2 {
        grid.insert((0,0), Color::White);
    }
    loop {
        //dbg!(&robot.loc);
        // Undefined colors are black
        let current_color = grid.get(&robot.loc).unwrap_or(&Color::Black);
        match current_color { 
            Color::Black => program.add_input(0),
            Color::White => program.add_input(1),
        }
        let res = program.run(None); // Run until receiving an output

        // First output defines the color to paint
        match res {
            // 1 indicates white
            Some(1) => {
                grid.insert(robot.loc, Color::White);
            },
            Some(0) => {
                grid.insert(robot.loc, Color::Black);
            },
            None => {
                break;
            },
            _ => panic!("Unexpected output"),
        }

        let res = program.run(None); // Run until receiving an output
        // First output defines the color to paint
        match res {
            // 1 indicates white
            Some(1) => {
                robot.turn_right();
            },
            Some(0) => {
                robot.turn_left();
            },
            None => {
                break;
            },
            _ => panic!("Unexpected output"),
        }
        robot.take_one_step();
    }
    print_grid(&grid);
    grid.len() as i64
}

fn read_contents(cont: &str) -> i64 {
    let vals = cont.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let p = Program::from_list(vals.clone());
    let part1 = get_part1(&mut p.clone(), false);
    get_part1(&mut p.clone(), true);
    part1
}
