use std::collections::HashSet;
use clap::Parser;
use std::iter::FromIterator;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

// ( 1, 0) turns into ( 0,-1) E to S 
// ( 0,-1) turns into (-1, 0) S to W
// (-1, 0) turns into (0, 1) W to N
// (0, 1) turns into (1, 0) N to E

fn rotate_vec(tup: (i32, i32)) -> (i32, i32) {
    (tup.1, -tup.0)
}

fn sum_vec(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn get_obstacles(cont: &str) -> HashSet<(i32,i32)> {
    let obstacles: HashSet<(i32, i32)> = HashSet::from_iter(
        cont.lines().enumerate().flat_map(|(i, ln)| {
            let y = -(i as i32);
            ln.chars().enumerate().map(move |(j, c)| {
                let x = j as i32;
                match c {
                    '#' => Some((x, y)),
                    '^' => Some((x, 1 + i as i32)),
                    _ => None
                }
            })
        }).flatten()
    );
    obstacles
}

fn check_loop(obstacles: &HashSet<(i32, i32)>, height: i32, width: i32, start_position: (i32,i32)) -> bool {
    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut guard_pos= start_position;
    let mut direction: (i32,i32) = (0,1); // Going north, i.e positive y
    let mut steps = 0;
    visited.insert((guard_pos, direction));
    loop {
        steps += 1;
        if steps > 40000 {
            println!("Too many steps");
            break;
        }
        //dbg!(&guard_pos);
        let next_loc = sum_vec(direction, guard_pos);
        if (next_loc.0 >= width) | (-next_loc.1 >= height) | (next_loc.0 < 0) | (next_loc.1 > 0) {
            //println!("Went outside after {steps} steps");
            break;
        }
        if obstacles.contains(&next_loc) {
            direction = rotate_vec(direction);
        } else {
            guard_pos = next_loc;
        }
        if visited.contains(&(guard_pos, direction)) {
            //println!("Found start position after {steps} steps");
            return true;
        }
        visited.insert((guard_pos, direction));
    }
    false
}

fn read_contents(cont: &str) -> (i32, i32) {
    let height = cont.lines().count() as i32;
    let width = cont.lines().next().expect("First line should exist").len() as i32;
    let mut obstacles = get_obstacles(cont);
    let mut direction: (i32,i32) = (0,1); // Going north, i.e positive y
    let start_position = obstacles.iter().find_map(|m| {
        if m.1 > 0 {
            Some((m.0, -(m.1 - 1)))
        } else {
            None
        }
    }).unwrap();
    let mut guard_pos = start_position;
    obstacles.remove(&guard_pos);


    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert(guard_pos);
    loop {
        let next_loc = sum_vec(direction, guard_pos);
        if (next_loc.0 >= width) | (-next_loc.1 >= height) | (next_loc.0 < 0) | (next_loc.1 > 0) {
            break;
        }
        if obstacles.contains(&next_loc) {
            direction = rotate_vec(direction);
        } else {
            guard_pos = next_loc;
            visited.insert(guard_pos);
        }
    }
    let part2 = add_obstacles(&obstacles, &visited, height, width, start_position);
    (visited.len() as i32, part2)
}

fn add_obstacles(obstacles: &HashSet<(i32, i32)>, visited: &HashSet<(i32, i32)>, height: i32, width: i32, start_position: (i32,i32)) -> i32 {
    let mut obst = obstacles.clone();
    visited.iter().map(|(x,y)| {
        if obstacles.contains(&(*x,*y)) {
            return 0
        }
        if (*x, *y) == start_position {
            return 0
        }
        obst.insert((*x, *y)) ;
        if check_loop(&obst, height, width, start_position) {
            obst.remove(&(*x, *y));
            return 1
        }
        obst.remove(&(*x, *y));
        0
    }).sum::<i32>()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let a = 
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(read_contents(&a).0, 41);
        assert_eq!(read_contents(&a).1, 6);
    }

    #[test]
    fn test_loop() {
        let a = 
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let mut obstacles = get_obstacles(&a);
        obstacles.insert((3, -6));
        assert!(check_loop(&obstacles, 10, 10, (4,-6)));

        obstacles.remove(&(3, -6));
        obstacles.insert((6, -7));
        assert!(check_loop(&obstacles, 10, 10, (4,-6)));
    }
}
