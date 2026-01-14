use clap::Parser;
use std::fs;
use std::fmt::Display;
use core::fmt;
use shared::Dir;
use regex::Regex;
use colored::*;
use std::io::{self, Write};

fn wait_for_enter() {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<Object>>,
    start: (i64, i64),
    rows: Vec<RowColInfo>,
    columns: Vec<RowColInfo>,
}


impl Map {
    fn print_map(&self) {
        for ln in &self.grid {
            println!("{}", ln.iter().map(|m| match m {
            Object::Wall => "#".normal().to_string(),
            Object::Empty => ".".normal().to_string(),
            Object::Void => " ".to_string(),
            Object::Visited(dir) => dir.get_char().to_string().blue().to_string(),
            Object::Loc => "X".to_string(),
            }).collect::<String>());
        }
    }

    fn print_loc(&self, state: &State) {
        let mut grid = self.grid.clone();
        let visited = grid.iter().rposition(|v| v.iter().any(|o| matches!(o, Object::Visited(_))));
        let max_i = if let Some(v) = visited {
            grid.len().min(v + 5)
        } else {
            grid.len() 
        };

        assert!(state.x >= 0);
        assert!(state.y >= 0);
        let x = state.x as usize;
        let y = state.y as usize;
        grid[y][x] = Object::Loc;
        for (j,ln) in grid[..max_i].iter().enumerate() {
            println!("{}", ln.iter().enumerate().map(|(i,m)| match m {
                Object::Wall => "#".blue().to_string(),
                Object::Empty => if (i == state.x as usize) || (j == state.y as usize) {
                    ".".red().to_string()
                } else {
                    ".".normal().to_string()
                    },
                Object::Void => " ".to_string(),
                Object::Visited(dir) => dir.get_char().to_string().magenta().to_string(),
                Object::Loc => state.direction.get_char().to_string().red().to_string(),
            }).collect::<String>());
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct RowColInfo {
    valid_length: usize,
    start_index: usize, // First nonvoid
    end_index: usize,  // Last nonvoid
    last_wall_index: Option<usize>,
    first_wall_index: Option<usize>,
}

impl RowColInfo {
    fn new(v: &[Object]) -> Self {
        let first_nonvoid = v.iter().position(|o| *o != Object::Void).unwrap();
        let last_nonvoid = v.iter().rposition(|o| *o != Object::Void).unwrap();


        let v = v.iter().filter(|o| **o != Object::Void).collect::<Vec<&Object>>();

        let first_wall = v.iter().position(|o| **o == Object::Wall);
        let last_wall  = v.iter().rposition(|o| **o == Object::Wall);

        let valid_length = 1 + last_nonvoid - first_nonvoid;
        Self {
            valid_length,
            start_index: first_nonvoid,
            end_index: last_nonvoid,
            first_wall_index: first_wall,
            last_wall_index: last_wall,
        }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Object {
    Wall,
    Empty,
    Void,
    Loc,
    Visited(Dir),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Order {
    Move(i64),
    TurnLeft,
    TurnRight,
    End,
}

impl Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Order::Move(n) => write!(f, "Move {}", n),
            Order::TurnLeft => write!(f, "Turn Left"),
            Order::TurnRight => write!(f, "Turn Right"),
            Order::End => write!(f, "End"),
        }
    }
}

impl Object {
    fn new(c: char) -> Self {
        match c {
            '.' => Object::Empty,
            '#' => Object::Wall,
            ' ' => Object::Void,
            _ => panic!("Unknown character '{c}'"),
        }
    }
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn read_map(cont: &str) -> (Map, Vec<Order>) {
    let grid: Vec<Vec<Object>> = cont.lines().filter(|ln| !ln.is_empty() && !ln.chars().next().unwrap().is_ascii_digit()).map(|ln| {
            ln.chars().map(move |c| {
                Object::new(c)
            }).collect::<Vec<Object>>()
        }).collect::<Vec<Vec<Object>>>();

    let max_width = grid.iter().map(|v| v.len()).max().unwrap();
    let row_info: Vec<RowColInfo> = grid.iter().map(|v| RowColInfo::new(v)).collect();
    let column_info = (0..max_width).map(|x| {
        RowColInfo::new(&grid.iter().map(|ln| ln.get(x).unwrap_or(&Object::Void).to_owned()).collect::<Vec<Object>>())

    }).collect::<Vec<RowColInfo>>();
    let orders = read_orders(cont.lines().find(|ln| !ln.is_empty() && ln.chars().next().unwrap().is_ascii_digit()).unwrap());
    let start_x = grid[0].iter().position(|o| *o == Object::Empty).unwrap() as i64;
    (Map {grid, start: (start_x, 0), rows: row_info, columns: column_info}, orders)
}

fn read_orders(line: &str) -> Vec<Order> {
    let re = Regex::new(r"(\d+)([LR]|\z)").unwrap();
    let mut orders: Vec<Order> = Vec::new();
    for res in re.captures_iter(line) {
        let num = res[1].parse::<i64>().unwrap();
        orders.push(Order::Move(num));
        let ord = res[0].chars().last().unwrap();
        match ord {
            'R' => orders.push(Order::TurnRight),
            'L' => orders.push(Order::TurnLeft),
            _ => orders.push(Order::End)
        }
    }
    orders
}


#[derive(Debug, Clone)]
struct State {
    x: i64,
    y: i64,
    direction: Dir,
}


fn get_password(state: State) -> i64 {
    println!("Row: {}", state.y + 1);
    println!("Column: {}", state.x + 1);
    println!("Direction: {}", state.direction);
    1000 * (state.y + 1) + 4 * (state.x + 1) + match state.direction {
        Dir::E => 0,
        Dir::S => 1,
        Dir::W => 2,
        Dir::N => 3,
    }
}

fn get_part1(map: &Map, orders: &[Order], verbose: bool) -> i64 {
    let mut map: Map = map.clone();
    let mut state = State {
        x: map.start.0,
        y: map.start.1,
        direction: Dir::E,
    };

    for (i,ord) in orders.iter().enumerate() {
        if verbose {
            println!();
            println!();
            println!("---------");
            println!("Step {i}:");
            map.print_loc(&state);
            println!("Now at {} {} going {:?}", state.x, state.y, state.direction);
            println!("Next order: {}", ord);
        }
        assert!(
            map.grid[state.y as usize][state.x as usize] != Object::Wall && 
            map.grid[state.y as usize][state.x as usize] != Object::Void
        );
        map.grid[state.y as usize][state.x as usize] = Object::Visited(state.direction);
        match ord {
            Order::TurnLeft => state.direction = state.direction.ccw(),
            Order::TurnRight => state.direction = state.direction.cw(),
            Order::Move(move_steps) => {
                let (vec, info) = if state.direction == Dir::W || state.direction == Dir::E {
                    (map.grid[state.y as usize].clone(),
                    map.rows[state.y as usize])
                }
                else {
                    (map.grid.iter().map(|ln| ln.get(state.x as usize).unwrap_or(&Object::Void).to_owned()).collect::<Vec<Object>>(),
                    map.columns[state.x as usize])
                };
                let valid_vec = vec.iter().filter(|o| **o != Object::Void).collect::<Vec<&Object>>();

                // Location in valid vector
                let loc_in_valid_vec = if state.direction == Dir::E || state.direction == Dir::W {
                    state.x as usize - info.start_index
                } else {
                    state.y as usize - info.start_index
                };
                if state.direction == Dir::E || state.direction == Dir::S {
                    // Moving to higher index 
                    if info.last_wall_index.is_some() {
                        // Position in the valid vector
                        let next_wall = valid_vec[loc_in_valid_vec..].iter().position(|o| **o == Object::Wall);
                        let next_wall = if let Some(v) = next_wall {
                            loc_in_valid_vec + v
                        } else {
                            // Wrap around
                            valid_vec.len() + info.first_wall_index.unwrap()
                        };

                        let max_distance = next_wall as i64 - loc_in_valid_vec as i64 - 1;
                        let move_steps = max_distance.min(*move_steps);

                        if move_steps == 0 {
                            // Hit a wall right away
                            continue;
                        }

                        let new_loc = (loc_in_valid_vec as i64 + move_steps).rem_euclid(info.valid_length as i64);

                        if state.direction == Dir::S {
                            state.y = new_loc + info.start_index as i64;
                        } else {
                            state.x = new_loc + info.start_index as i64;
                        }
                    } else {
                        let new_loc = (loc_in_valid_vec as i64 + move_steps).rem_euclid(info.valid_length as i64);
                        if state.direction == Dir::S {
                            state.y = new_loc + info.start_index as i64;
                        } else {
                            state.x = new_loc + info.start_index as i64;
                        }
                    }
                } else if info.last_wall_index.is_some() {
                    // Find the last index in the vec before start_i that is a wall
                    let next_wall = valid_vec[0..loc_in_valid_vec].iter().rposition(|o| **o == Object::Wall);

                    let next_wall = if let Some(v) = next_wall {
                        v as i64
                    } else {
                        // Wrap around
                        info.last_wall_index.unwrap() as i64 - info.valid_length as i64
                    };
                    assert!(next_wall < loc_in_valid_vec as i64);
                    let max_distance = loc_in_valid_vec as i64 - next_wall - 1;
                    let move_steps = max_distance.min(*move_steps);
                    if move_steps == 0 {
                        continue;
                    }

                    let new_loc = (loc_in_valid_vec as i64 - move_steps).rem_euclid(info.valid_length as i64);

                    if state.direction == Dir::N {
                        state.y = new_loc + info.start_index as i64;
                    } else {
                        state.x = new_loc + info.start_index as i64;
                    }
                } else {
                    let new_loc = (loc_in_valid_vec as i64 - move_steps).rem_euclid(info.valid_length as i64);
                    if state.direction == Dir::N {
                        state.y = new_loc + info.start_index as i64;
                    } else {
                        state.x = new_loc + info.start_index as i64;
                    }
                }
                map.grid[state.y as usize][state.x as usize] = Object::Visited(state.direction);
            }
            Order::End => break,
        }

    }
    get_password(state)
}

fn read_contents(cont: &str) -> (i64, i64) {
    let (map, orders) = read_map(cont);

    let part1 = get_part1(&map, &orders, false);

    (part1,0)

}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

        assert_eq!(read_contents(&a).0, 6032);
    }

}
