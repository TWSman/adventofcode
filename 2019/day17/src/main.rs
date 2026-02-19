use clap::Parser;
use colored::Colorize;
use intcode::*;
use shared::Dir;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Object {
    Scaffold,
    Empty,
    Robot(Dir),
    Intersection,
}

type Grid = BTreeMap<(i64, i64), Object>;

fn print_grid(grid: &Grid) {
    let min_x = grid.keys().map(|(x, _)| *x).min().unwrap() - 2;
    let max_x = grid.keys().map(|(x, _)| *x).max().unwrap() + 2;
    let min_y = grid.keys().map(|(_, y)| *y).min().unwrap() - 2;
    let max_y = grid.keys().map(|(_, y)| *y).max().unwrap() + 2;

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            match grid.get(&(x, y)) {
                Some(&Object::Scaffold) => {
                    print!("{}", "#".blue().on_black());
                }
                Some(&Object::Empty) => {
                    print!("{}", ".".black().on_black());
                }
                Some(&Object::Intersection) => {
                    print!("{}", "O".red().on_black());
                }
                Some(&Object::Robot(dir)) => {
                    let c = match dir {
                        Dir::N => '^',
                        Dir::S => 'v',
                        Dir::E => '>',
                        Dir::W => '<',
                    };
                    print!("{}", c.to_string().yellow().on_black());
                }
                None => {
                    print!("{}", ".".white().on_white());
                }
            }
        }
        println!();
    }
}

fn get_grid(program: &mut Program) -> Grid {
    program.set_verbose(0);
    program.run_until_stop();
    let output = program.get_outputs_ascii();
    read_grid(&output)
}

fn read_grid(cont: &str) -> Grid {
    cont.lines()
        .enumerate()
        .fold(BTreeMap::new(), |mut grid, (y, line)| {
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

fn get_part1(grid: &mut Grid) -> i64 {
    find_intersections(grid);
    print_grid(grid);
    grid.iter()
        .filter(|(_, obj)| **obj == Object::Intersection)
        .map(|((x, y), _)| x * -y)
        .sum()
}

fn get_part2(program: &Program, grid: &Grid) -> i64 {
    let mut program = program.clone();
    let path = find_path(grid);
    let (blocks, output) = locate_blocks(&path);
    let output = format!(
        "{}\n{}\n{}\n{}\nn\n",
        output, blocks[0], blocks[1], blocks[2]
    );
    let output_as_ascii = output.chars().map(|c| c as i64).collect::<Vec<i64>>();
    // Set the first memory address to 2 to wake up the robot
    program.set_index(0, 2);
    program.set_verbose(0);

    println!("Input to give:\n{}##############\n\n", output);
    for o in output_as_ascii {
        program.add_input(o);
    }
    program.run_until_stop();
    *program.get_outputs().last().unwrap()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Move {
    Forward(i64),
    TurnLeft,
    TurnRight,
}

impl Move {
    fn stringify(&self) -> String {
        match self {
            Move::Forward(n) => n.to_string(),
            Move::TurnLeft => "L".to_string(),
            Move::TurnRight => "R".to_string(),
        }
    }
}

fn locate_blocks(path: &[Move]) -> (Vec<String>, String) {
    let path_string = path
        .iter()
        .map(|m| m.stringify())
        .collect::<Vec<String>>()
        .join(",")
        + ",";
    assert!(
        path[0] == Move::TurnRight || path[0] == Move::TurnLeft,
        "Path should start with a turn"
    );
    let path = path
        .chunks(2)
        .map(|chunk| {
            assert!(chunk.len() == 2, "Expected pairs of turn and forward moves");
            let turn = chunk[0];
            let forward = chunk[1];
            assert!(
                matches!(turn, Move::TurnRight | Move::TurnLeft),
                "Expected a turn move"
            );
            assert!(
                matches!(forward, Move::Forward(_)),
                "Expected a forward move"
            );
            (turn, forward)
        })
        .collect::<Vec<(Move, Move)>>();

    let mut candidates_front: BTreeMap<String, usize> = BTreeMap::new();
    let mut current_string = String::new();
    for j in 0..path.len() {
        let m = path.get(j).unwrap();
        current_string += &format!("{},{},", m.0.stringify(), m.1.stringify());
        if current_string.len() > 21 {
            continue;
        }
        let tmp = path_string.matches(&current_string).count();
        if tmp > 1 && current_string.len() > 6 {
            candidates_front.insert(
                current_string.clone(),
                path_string.matches(&current_string).count(),
            );
        }
    }

    let mut current_string = String::new();
    let mut candidates_back = BTreeMap::new();
    for j in (0..path.len()).rev() {
        let m = path.get(j).unwrap();
        current_string = format!("{},{},{}", m.0.stringify(), m.1.stringify(), current_string);
        if current_string.len() > 21 {
            continue;
        }
        let tmp = path_string.matches(&current_string).count();
        if tmp > 1 && current_string.len() > 6 {
            candidates_back.insert(
                current_string.clone(),
                path_string.matches(&current_string).count(),
            );
        }
    }

    for cf in &candidates_front {
        for cb in &candidates_back {
            let candidate_string = path_string.replace(cf.0, "|").replace(cb.0, "|");
            let remaining = candidate_string
                .split("|")
                .filter(|s| !s.is_empty())
                .collect::<BTreeSet<&str>>();
            if remaining.len() == 1 {
                println!("Found solution");
                let a = cf.0.clone();
                let b = cb.0.clone();
                let c = remaining.into_iter().next().unwrap();
                let mut output = path_string.replace(&a, "A,");
                output = output.replace(&b, "B,");
                output = output.replace(c, "C,").trim_end_matches(',').to_string();
                return (
                    vec![
                        a.trim_end_matches(",").to_string(),
                        b.trim_end_matches(",").to_string(),
                        c.trim_end_matches(",").to_string(),
                    ],
                    output,
                );
            }
        }
    }
    (Vec::new(), String::new())
}

fn find_intersections(grid: &mut Grid) {
    let cloned = grid.clone();
    for ((x, y), obj) in cloned.iter() {
        if *obj != Object::Scaffold {
            continue;
        }
        let neighbors = [(x + 1, *y), (x - 1, *y), (*x, y + 1), (*x, y - 1)];
        if neighbors
            .iter()
            .all(|n| cloned.get(n) == Some(&Object::Scaffold))
        {
            grid.insert((*x, *y), Object::Intersection);
        }
    }
}

fn find_path(grid: &Grid) -> Vec<Move> {
    let mut output_grid = grid.clone();
    let start = grid
        .iter()
        .find(|(_, obj)| matches!(obj, Object::Robot(_)))
        .unwrap();
    let mut loc = (
        start.0.0,
        start.0.1,
        match start.1 {
            Object::Robot(Dir::N) => Dir::N,
            Object::Robot(Dir::S) => Dir::S,
            Object::Robot(Dir::E) => Dir::E,
            Object::Robot(Dir::W) => Dir::W,
            _ => panic!("Expected robot"),
        },
    );
    let mut steps = 0;
    let mut path = Vec::new();
    loop {
        //wait_for_enter();
        // Check if we can continue
        output_grid.insert((loc.0, loc.1), Object::Robot(loc.2));
        let forward_loc = match loc.2 {
            Dir::N => (loc.0, loc.1 + 1),
            Dir::S => (loc.0, loc.1 - 1),
            Dir::E => (loc.0 + 1, loc.1),
            Dir::W => (loc.0 - 1, loc.1),
        };
        match grid.get(&forward_loc) {
            None => {}
            Some(Object::Empty) => {}
            Some(_) => {
                loc.0 = forward_loc.0;
                loc.1 = forward_loc.1;
                //println!("Move forward to {:?}", (loc.0, loc.1));
                steps += 1;
                continue;
            }
        }
        //println!("Can't move forward, trying to turn");
        let cw_dir = loc.2.cw();
        let ccw_dir = loc.2.ccw();
        let cw_loc = match cw_dir {
            Dir::N => (loc.0, loc.1 + 1),
            Dir::S => (loc.0, loc.1 - 1),
            Dir::E => (loc.0 + 1, loc.1),
            Dir::W => (loc.0 - 1, loc.1),
        };
        match grid.get(&cw_loc) {
            None => {}
            Some(Object::Empty) => {}
            Some(_) => {
                loc.2 = cw_dir;
                //println!("Turn right at {:?}", (loc.0, loc.1));
                if steps > 0 {
                    path.push(Move::Forward(steps));
                }
                path.push(Move::TurnRight);
                steps = 0;
                continue;
            }
        }

        let ccw_loc = match ccw_dir {
            Dir::N => (loc.0, loc.1 + 1),
            Dir::S => (loc.0, loc.1 - 1),
            Dir::E => (loc.0 + 1, loc.1),
            Dir::W => (loc.0 - 1, loc.1),
        };

        match grid.get(&ccw_loc) {
            None => {}
            Some(Object::Empty) => {}
            Some(_) => {
                loc.2 = ccw_dir;
                //println!("Move left at {:?}", (loc.0, loc.1));
                if steps > 0 {
                    path.push(Move::Forward(steps));
                }
                path.push(Move::TurnLeft);
                steps = 0;
                continue;
            }
        }
        //println!("Found end at {:?}", (loc.0, loc.1));
        path.push(Move::Forward(steps));
        break;
    }
    print_grid(&output_grid);

    for m in &path {
        print!("{},", m.stringify());
    }
    println!();
    path
}

fn read_contents(cont: &str) -> (i64, i64) {
    let vals = cont
        .split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let p = Program::from_list(vals.clone());
    let mut grid = get_grid(&mut p.clone());
    let part1 = get_part1(&mut grid);
    let part2 = get_part2(&p, &grid);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";

        let grid = read_grid(a);

        print_grid(&grid);
        //  Should take 4 minutes to fill with oxygen
        assert_eq!(get_part1(&mut grid.clone()), 76);
        //assert_eq!(get_part1(&grid), 4)
    }

    #[test]
    fn part2() {
        let a = "#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......";
        let grid = read_grid(a);
        let path = find_path(&grid);
        assert_eq!(
            path,
            vec![
                Move::TurnRight,
                Move::Forward(8),
                Move::TurnRight,
                Move::Forward(8),
                Move::TurnRight,
                Move::Forward(4),
                Move::TurnRight,
                Move::Forward(4),
                Move::TurnRight,
                Move::Forward(8),
                Move::TurnLeft,
                Move::Forward(6),
                Move::TurnLeft,
                Move::Forward(2),
                Move::TurnRight,
                Move::Forward(4),
                Move::TurnRight,
                Move::Forward(4),
                Move::TurnRight,
                Move::Forward(8),
                Move::TurnRight,
                Move::Forward(8),
                Move::TurnRight,
                Move::Forward(8),
                Move::TurnLeft,
                Move::Forward(6),
                Move::TurnLeft,
                Move::Forward(2),
            ]
        );
        let (blocks, output) = locate_blocks(&path);
        assert!(blocks.len() == 3);
        // This is one possible solution
        assert!(blocks.contains(&"R,8,R,8".to_string()));
        assert!(blocks.contains(&"R,4,R,4".to_string()));
        assert!(blocks.contains(&"R,8,L,6,L,2".to_string()));
        assert_eq!(output, "A,C,B,C,A,B");

        // Another one would be:
        // R,8,R,8,
        // R,4,R,4,R,8,
        // L,6,L,2,
    }
}
