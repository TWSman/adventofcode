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
    Scaffold,
    Empty,
    Robot(Dir),
    Intersection,
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
            //if (x,y) == robot_loc {
            //    print!("{}", "*".blue());
            //    continue;
            //}
            //if (x,y) == (0,0) {
            //    print!("{}", "o".red());
            //    continue;
            //}
            match grid.get(&(x,y)) {
                Some(&Object::Scaffold) => {
                    print!("{}", "#".blue().on_black());
                },
                Some(&Object::Empty) => {
                    print!("{}", ".".black().on_black());
                },
                Some(&Object::Intersection) => {
                    print!("{}", "O".red().on_black());
                },
                Some(&Object::Robot(dir)) => {
                    let c = match dir {
                        Dir::N => '^',
                        Dir::S => 'v',
                        Dir::E => '>',
                        Dir::W => '<',
                    };
                    print!("{}", c.to_string().yellow().on_black());
                },
                None => {
                    print!("{}", ".".white().on_white());
                }
            }
        }
        println!();
    }
}

fn get_grid(program: &mut Program) -> BTreeMap::<(i64,i64), Object> {
    let grid: BTreeMap::<(i64,i64), Object> = BTreeMap::new();
    program.set_verbose(0);
    program.run_until_stop();
    let output = program.get_outputs_ascii();
    let grid = read_grid(&output);
    grid
}

fn read_grid(cont: &str) -> BTreeMap<(i64,i64), Object> {
    cont.lines().enumerate().fold(BTreeMap::new(), |mut grid, (y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let obj = match c {
                '#' => Object::Scaffold,
                '.' => Object::Empty,
                '^' => Object::Robot(Dir::N),
                'v' => Object::Robot(Dir::S),
                '<' => Object::Robot(Dir::W),
                '>' => Object::Robot(Dir::E),
                'O' => Object::Intersection,
                _ => panic!("Unknown character in grid"),
            };
            grid.insert((x as i64, -(y as i64)), obj);
        });
        grid
    })
}

fn get_part1(grid: &mut BTreeMap<(i64,i64), Object>) -> i64 {
    find_intersections(grid);
    print_grid(&grid, (0,0));
    grid.iter().filter(|(_, obj)| **obj == Object::Intersection).map(|((x,y), _)|x*-y).sum()
}

fn get_part2(program: &Program) -> i64 {
    let mut program = program.clone();
    // Set the first memory address to 2 to wake up the robot
    program.set_index(0, 2);
    0
}

fn find_intersections(grid: &mut BTreeMap<(i64,i64), Object>) {
    let cloned = grid.clone();
    for ((x,y), obj) in cloned.iter() {
        if *obj != Object::Scaffold {
            continue;
        }
        let neighbors = vec![
            (x+1, *y),
            (x-1, *y),
            (*x, y+1),
            (*x, y-1),
        ];
        if neighbors.iter().all(|n| cloned.get(n) == Some(&Object::Scaffold)) {
            grid.insert((*x,*y), Object::Intersection);
        }
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let vals = cont.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let p = Program::from_list(vals.clone());
    let mut grid = get_grid(&mut p.clone());
    let part1 = get_part1(&mut grid);
    //let part2 = get_part2(&grid);
    let part2 = 0;
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a= "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";

        let grid = read_grid(a);

        print_grid(&grid, (0,0));
        //  Should take 4 minutes to fill with oxygen
        assert_eq!(get_part1(&mut grid.clone()), 76);
        //assert_eq!(get_part1(&grid), 4)
    }
}
