use clap::Parser;
use std::fs;
use shared::Dir;
use regex::Regex;

use day22::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents, 50);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn read_contents(cont: &str, side_length: usize) -> (i64, i64) {
    let (map, orders) = read_map(cont);

    let part1 = get_part1(&map, &orders, false);

    let cube = Cube::new(&map, side_length);
    let part2 = get_part2(&cube, &orders, true);

    (part1, part2)

}

fn read_map(cont: &str) -> (Map, Vec<Order>) {
    let grid: Vec<Vec<Object>> = cont.lines().filter(|ln| !ln.is_empty() && !ln.chars().next().unwrap().is_ascii_digit()).map(|ln| {
            ln.chars().map(move |c| {
                Object::new(c)
            }).collect::<Vec<Object>>()
        }).collect::<Vec<Vec<Object>>>();

    let max_width = grid.iter().map(|v| v.len()).max().unwrap();
    let row_info: Vec<RowColInfo> = grid.iter().map(|v| RowColInfo::new(v)).collect();
    dbg!(&max_width);
    let column_info = (0..max_width).map(|x| {
        RowColInfo::new(&grid.iter().map(|ln| ln.get(x).unwrap_or(&Object::Void).to_owned()).collect::<Vec<Object>>())

    }).collect::<Vec<RowColInfo>>();
    let orders = read_orders(cont.lines().find(|ln| !ln.is_empty() && ln.chars().next().unwrap().is_ascii_digit()).unwrap());
    let start_col = grid[0].iter().position(|o| *o == Object::Empty).unwrap() as i64;
    (Map {grid, start: (start_col, 0), rows: row_info, columns: column_info}, orders)
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

fn get_turn_left(state: &State3D) -> Dir3D {
    match (state.direction, state.face) {
        (Dir3D::E, Face::Top) => Dir3D::N,
        (Dir3D::W, Face::Top) => Dir3D::S,
        (Dir3D::N, Face::Top) => Dir3D::W,
        (Dir3D::S, Face::Top) => Dir3D::E,

        (Dir3D::E, Face::Bottom) => Dir3D::S,
        (Dir3D::W, Face::Bottom) => Dir3D::N,
        (Dir3D::N, Face::Bottom) => Dir3D::E,
        (Dir3D::S, Face::Bottom) => Dir3D::W,

        (Dir3D::E,    Face::Front) => Dir3D::Up,
        (Dir3D::W,    Face::Front) => Dir3D::Down,
        (Dir3D::Up,   Face::Front) => Dir3D::W,
        (Dir3D::Down, Face::Front) => Dir3D::E,

        (Dir3D::E,    Face::Back) => Dir3D::Down,
        (Dir3D::W,    Face::Back) => Dir3D::Up,
        (Dir3D::Up,   Face::Back) => Dir3D::E,
        (Dir3D::Down, Face::Back) => Dir3D::W,

        (Dir3D::N,    Face::Right) => Dir3D::Up,
        (Dir3D::S,    Face::Right) => Dir3D::Down,
        (Dir3D::Up,   Face::Right) => Dir3D::S,
        (Dir3D::Down, Face::Right) => Dir3D::N,

        (Dir3D::N,    Face::Left) => Dir3D::Down,
        (Dir3D::S,    Face::Left) => Dir3D::Up,
        (Dir3D::Up,   Face::Left) => Dir3D::N,
        (Dir3D::Down, Face::Left) => Dir3D::S,
        _ => {
            dbg!(&state);
            panic!("Should not happen");
        }

    }
}

fn get_new_face(x: i64, y: i64, z: i64, max_val: i64) -> Face {
    if x == 0 {
        Face::Left
    } else if x == max_val {
        Face::Right
    } else if y == 0 {
        Face::Front
    } else if y == max_val {
        Face::Back
    } else if z == 0 {
        Face::Bottom
    } else if z == max_val {
        Face::Top
    } else {
        panic!("Coordinate {}, {}, {} is not on any face", x, y, z);
    }
}

fn get_wrapped_direction(face: Face) -> Dir3D {
    match face {
        Face::Top => Dir3D::Down, // Only way is down
        Face::Bottom => Dir3D::Up,
        Face::Front => Dir3D::N,
        Face::Back => Dir3D::S,
        Face::Left => Dir3D::E,
        Face::Right => Dir3D::W,
    }
}

fn get_part2(cube: &Cube, orders: &[Order], verbose: bool) -> i64 {
    // We start from top left corner of the front face, going east
    //
    let edge_coord = cube.side_length as i64 + 1;
    let mut state = State3D {
        x: 1,
        y: cube.side_length as i64,
        z: edge_coord,
        face: Face::Top,
        direction: Dir3D::E,
    };

    for (i,ord) in orders.iter().enumerate() {
        if verbose {
            println!();
            println!();
            println!("---------");
            println!("Step {i}:");
            println!("Now at x: {} y: {} z: {} ({:?} face) going {:?}", state.x, state.y, state.z, state.face, state.direction);
            println!("Next order: {}", ord);
        }
        match ord {
            Order::TurnLeft => {
                state.direction = get_turn_left(&state);
                continue;
            }
            Order::TurnRight => {
                state.direction = get_turn_left(&state).opposite();
                continue;
            }
            Order::Move(move_count) => {
                dbg!(&move_count);
                for _step in 0..(*move_count as usize) {
                    let current_coord = match state.direction {
                        Dir3D::N | Dir3D::S => state.y,
                        Dir3D::E | Dir3D::W => state.x,
                        Dir3D::Up | Dir3D::Down => state.z,
                    };

                    let new_coord = current_coord + match state.direction {
                        Dir3D::N | Dir3D::E | Dir3D::Up => 1,
                        _ => -1,
                    };

                    let mut new_direction = state.direction;
                    let mut new_face = state.face;
                    let mut new_loc = match state.direction {
                        Dir3D::N | Dir3D::S => (state.x, new_coord, state.z, state.face),
                        Dir3D::E | Dir3D::W => (new_coord, state.y, state.z, state.face),
                        Dir3D::Up | Dir3D::Down => (state.x, state.y, new_coord, state.face),
                    };
                    if new_coord == 0 || new_coord == edge_coord {
                        println!("Wrapping around");
                        dbg!(&new_loc);
                        dbg!(&state);
                        new_direction = get_wrapped_direction(state.face);
                        dbg!(&new_direction);
                        match new_direction {
                            Dir3D::E => new_loc.0 += 1,
                            Dir3D::W => new_loc.0 -= 1,

                            Dir3D::N => new_loc.1 += 1,
                            Dir3D::S => new_loc.1 -= 1,

                            Dir3D::Up => new_loc.2 += 1,
                            Dir3D::Down => new_loc.2 -= 1,
                        }
                        new_face = get_new_face(new_loc.0, new_loc.1, new_loc.2, edge_coord);
                        dbg!(&new_face);
                    };
                    dbg!(&new_loc);
                    new_loc.3 = new_face;
                    if cube.walls.contains(&new_loc) {
                        // We hit a wall
                        println!("Hit a wall at {}, {}, {}", new_loc.0, new_loc.1, new_loc.2);
                        break; // Break from movement loop
                    }
                    state.x = new_loc.0;
                    state.y = new_loc.1;
                    state.z = new_loc.2;
                    state.direction = new_direction;
                    state.face = new_face;
                }
            }
            Order::End => {
                break;
            }
        }
    }
    cube.get_password(&state)
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

        assert_eq!(read_contents(&a, 4).0, 6032);
        assert_eq!(read_contents(&a, 4).1, 5031);
    }

    #[test]
    fn part2() {
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
        let (map, orders) = read_map(a);
        let cube = Cube::new(&map, 4);
        // Top face: 
        //  1234 x
        // 4..v# // After 1 move and turn (10R) (3,4,5)
        // 3.#..
        // 2#...
        // 1....
        // y
        assert!(cube.walls.contains(&(4,4,5, Face::Top)));
        assert!(cube.walls.contains(&(2,3,5, Face::Top)));
        assert!(cube.walls.contains(&(1,2,5, Face::Top)));
        
        assert_eq!(cube.face_info.get(&Face::Top).unwrap().dir_col, Axis::PosX);
        assert_eq!(cube.face_info.get(&Face::Top).unwrap().dir_row, Axis::NegY);


        // Front face: y = 0
        //  1234 x
        // 4...#
        // 3#.>. // After 2 moves and turns (5L) (3, 0, 3)
        // 2....
        // 1..#.
        // z
        //
        assert_eq!(cube.face_info.get(&Face::Front).unwrap().dir_col, Axis::PosX);
        assert_eq!(cube.face_info.get(&Face::Front).unwrap().dir_row, Axis::NegZ);
        assert!(cube.walls.contains(&(4,0,4, Face::Front)));
        assert!(cube.walls.contains(&(1,0,3, Face::Front)));
        assert!(cube.walls.contains(&(3,0,1, Face::Front)));

        // Bottom face: z = 0
        //  1234 x
        // 1...#
        // 2....
        // 3.#v.  // After 4 moves and turns (10L) (3, 3, 0)
        // 4....
        // y
        //
        assert_eq!(cube.face_info.get(&Face::Bottom).unwrap().dir_col, Axis::PosX);
        assert_eq!(cube.face_info.get(&Face::Bottom).unwrap().dir_row, Axis::PosY);
        assert!(cube.walls.contains(&(4,1,0, Face::Bottom)));
        assert!(cube.walls.contains(&(2,3,0, Face::Bottom)));


        // Right face: x = 5
        //  1234 z
        // 1....
        // 2.#..
        // 3..<. // After 3 moves and turns (5R) (5, 3,3)
        // 4..#.
        // y
        //
        //
        assert_eq!(cube.face_info.get(&Face::Right).unwrap().dir_col, Axis::PosZ);
        assert_eq!(cube.face_info.get(&Face::Right).unwrap().dir_row, Axis::PosY);
        assert!(cube.walls.contains(&(5,2,2, Face::Right)));
        assert!(cube.walls.contains(&(5,4,3, Face::Right)));
        // Left face (x = 0)
        //
        //  4321 y
        // 4..^. // After last move (0, 2, 4)
        // 3..^. // After 6 moves and turns (5L) (0, 2,3)
        // 2...#
        // 1....
        // z
        assert_eq!(cube.face_info.get(&Face::Left).unwrap().dir_col, Axis::NegY);
        assert_eq!(cube.face_info.get(&Face::Left).unwrap().dir_row, Axis::NegZ);
        assert!(cube.walls.contains(&(0,1,2, Face::Left)));

        // Back face (y = 5)
        // 
        //  4321 x
        // 4...#
        // 3.>.. // After 5 moves and turns (4R) (3,5,3)
        // 2..#.
        // 1....
        // z
        //
        assert_eq!(cube.face_info.get(&Face::Back).unwrap().dir_col, Axis::NegX);
        assert_eq!(cube.face_info.get(&Face::Back).unwrap().dir_row, Axis::NegZ);
        assert!(cube.walls.contains(&(1,5,4, Face::Back)));
        assert!(cube.walls.contains(&(2,5,2, Face::Back)));
        let part2 = get_part2(&cube, &orders, true);
        // Moves should be:
        // 3,4,5
        // 3,0,3
        // 5,3,3
        // 3,3,0
        // 3,5,3
        // 0,2,3
        // 0,2,4
        assert_eq!(part2, 5031);
    }






    #[test]
    fn test_input() {
        // Test case that matches the layout of my input data
        let a = "     .##.......
     ..........
     ..........
     ......#..#
     ..#.......
     .....
     .....
     .....
     ..#..
     ##...
..........
....#.....
..........
..........
..........
.....
.###.
.....
.....
...#.

10R5L5R10L4R5L5";
//       x12345 z54321    
//        T##..  R....5  
//        .....  .....4  
//        .....  .....3  
//        .....  .#..#2  
//        ..#..  .....1  
//                    y
//        F....       
//        .....
//        .....
//        ..#..
//        ##...
//
//  L.... Bo...
//  ....# .....
//  ..... .....
//  ..... .....
//  ..... .....
//  Ba...
//  .###.
//  .....
//  .....
//  ...#.

        dbg!(&a);
        let (map, _orders) = read_map(a);
        let cube = Cube::new(&map, 5);
        // Top face: z = 6
        //  12345 x
        // 5.##..
        // 4.....
        // 3.....
        // 2.....
        // 1..#..
        // y
        assert!(cube.walls.contains(&(2,5,6, Face::Top)));
        assert!(cube.walls.contains(&(3,5,6, Face::Top)));
        
        assert_eq!(cube.face_info.get(&Face::Top).unwrap().dir_col, Axis::PosX);
        assert_eq!(cube.face_info.get(&Face::Top).unwrap().dir_row, Axis::NegY);

        // Right Face: x = 6
        // 
        //   54321 z
        //  5.....
        //  4.....
        //  3.....
        //  2.#..#
        //  1.....
        //  y
        //
        assert_eq!(cube.face_info.get(&Face::Right).unwrap().dir_col, Axis::NegZ);
        assert_eq!(cube.face_info.get(&Face::Right).unwrap().dir_row, Axis::NegY);
        assert!(cube.walls.contains(&(6,2,4, Face::Right)));
        assert!(cube.walls.contains(&(6,2,1, Face::Right)));

        // Front face (y = 0)
        //
        //  12345 x
        // 5.....
        // 4.....
        // 3.....
        // 2..#..
        // 1##...
        // z
        //
        assert_eq!(cube.face_info.get(&Face::Front).unwrap().dir_col, Axis::PosX);
        assert_eq!(cube.face_info.get(&Face::Front).unwrap().dir_row, Axis::NegZ);
        assert!(cube.walls.contains(&(1,0,1, Face::Front)));
        assert!(cube.walls.contains(&(2,0,1, Face::Front)));
        assert!(cube.walls.contains(&(3,0,2, Face::Front)));

        // Bottom face: z = 0
        //
        //  12345 x
        // 1.....
        // 2.....
        // 3.....
        // 4.....
        // 5.....
        // y
        //
        assert_eq!(cube.face_info.get(&Face::Bottom).unwrap().dir_col, Axis::PosX);
        assert_eq!(cube.face_info.get(&Face::Bottom).unwrap().dir_row, Axis::PosY);

        // left face: x = 0
        //  
        //  54321 z -> Bottom
        // 1.....   |
        // 2....#   v Back
        // 3.....
        // 4.....
        // 5.....
        // y
        assert_eq!(cube.face_info.get(&Face::Left).unwrap().dir_col, Axis::NegZ);
        assert_eq!(cube.face_info.get(&Face::Left).unwrap().dir_row, Axis::PosY);

        assert!(cube.walls.contains(&(0,2,1, Face::Left)));


        // Back face: (y = 6)
        //  54321 z
        // 1..... -> Bottom
        // 2.###. |
        // 3..... v Right
        // 4.....
        // 5...#.
        // x
        //
        assert_eq!(cube.face_info.get(&Face::Back).unwrap().dir_col, Axis::NegZ);
        assert_eq!(cube.face_info.get(&Face::Back).unwrap().dir_row, Axis::PosX);

        assert!(cube.walls.contains(&(2,6,4, Face::Back)));
        assert!(cube.walls.contains(&(2,6,3, Face::Back)));
        assert!(cube.walls.contains(&(2,6,2, Face::Back)));
        assert!(cube.walls.contains(&(5,6,2, Face::Back)));
    }
}
