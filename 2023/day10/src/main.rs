// 7: S-W
// F: S-E
// J: N-W
// L: N-E
// -: E-W
// |: N-S
// . nothing
// S start
//
use clap::Parser;
use std::fs;
use std::fmt;
use std::collections::HashMap;
use indexmap::IndexMap;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}


fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
}

#[derive(Debug)]
enum Node {
    NorthEast, //L
    SouthWest, //7
    SouthEast, //F
    NorthWest, //J
    Vertical, //|
    Horizontal, //-
    Start, // S
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Turn {
    CW,
    CCW,
    Straight,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
            Direction::East => write!(f, "East"),
        }
    }
}

impl Node {
    fn new(c: char) -> Node {
        match c {
            '|' => Node::Vertical,
            '-' => Node::Horizontal,
            'L' => Node::NorthEast,
            '7' => Node::SouthWest,
            'F' => Node::SouthEast,
            'J' => Node::NorthWest,
            'S' => Node::Start,
            _ => panic!("Invalid character, '{}'", c),
        }
    }

    fn get_direction(&self, dir: Direction) -> Option<(Direction, Turn)> {
        match (self, dir) {
            (Node::Vertical | Node::Horizontal, dir) => Some((dir, Turn::Straight)),
            (Node::NorthEast, Direction::South) => Some((Direction::East,  Turn::CCW)),
            (Node::NorthEast, Direction::West) =>  Some((Direction::North, Turn::CW )),
            (Node::SouthEast, Direction::North) => Some((Direction::East,  Turn::CW )),
            (Node::SouthEast, Direction::West) =>  Some((Direction::South, Turn::CCW)),

            (Node::NorthWest, Direction::South) => Some((Direction::West,  Turn::CW )),
            (Node::NorthWest, Direction::East) =>  Some((Direction::North, Turn::CCW)),
            (Node::SouthWest, Direction::North) => Some((Direction::West,  Turn::CCW)),
            (Node::SouthWest, Direction::East) =>  Some((Direction::South, Turn::CW )),
            (_, _) => None,
        }
    }
}


fn repl_ind(input: &str, x: i64, y: i64, w: i64, c: &str) -> String {
    let ind: usize = (-1 * y * w + x).try_into().unwrap();
    let mut output = input.to_owned();
    output.replace_range(ind..(ind+1), c);
    output
}

fn read_contents(cont: &str) -> (i64, i64) {
    let line_width = cont.lines().next().expect("Should be at least 1 line").len() as i64 + 1;
    let mut output = cont.to_owned();
    let mut turns: HashMap<(i64, i64), Node> = HashMap::new();

    let (start_x, start_y) = cont.chars().enumerate().filter_map(|(i, c)| {
        match c {
            '.' | ' ' | '*' | 'O' => None,
            '\n' => None,
            'S' => { // Insert the start and return its coordinates
                turns.insert(((i as i64) % line_width, -1 * (i as i64) / line_width), Node::new(c));
                Some(((i as i64) %line_width, -1 * (i as i64) / line_width))
            },
            _ => { // Insert the Node but don't return
                turns.insert(((i as i64) % line_width, -1 * (i as i64) / line_width), Node::new(c));
                None
            }
        }
    }
    ).last().unwrap();
    
    let mut j = 0;
    let mut dir = Direction::North;
    let mut coords_with_direction: IndexMap<(i64, i64), Direction> = IndexMap::new();
    let mut step_count = 0;
    let (mut x, mut y) = (start_x, start_y);
    let mut turn_count = 0; // CCW increases, CW decreases
    loop {
        (x,y) = match dir {
            Direction::North => (x, y+1),
            Direction::South => (x, y-1),
            Direction::East => (x+1, y),
            Direction::West => (x-1, y),
        };
        match turns.get(&(x, y)) {
            None => { 
                // Found a empty square, thus we are no longer on the loop
                // Return to start and try another direction
                step_count = 0;
                turn_count = 0;
                (x,y) = (start_x, start_y);
                j += 1;
                dir = match j {
                    0 => Direction::East,
                    1 => Direction::South,
                    2 => Direction::West,
                    _ => panic!("Back to north"),
                };
                coords_with_direction = IndexMap::new();
                continue;
            }
            Some(Node::Start) => {
                // Found the start, we can break the loop
                coords_with_direction.insert((x,y), match j {
                    0 => Direction::North,
                    1 => Direction::East,
                    2 => Direction::South,
                    3 => Direction::West,
                    _ => panic!("Must not happen"),
                });
                break;
            },
            Some(val) => {
                step_count += 1;
                dir = match val.get_direction(dir) {
                    Some((val, Turn::Straight)) => val,
                    Some((val, Turn::CW)) => { turn_count -= 1; val},
                    Some((val, Turn::CCW)) => { turn_count += 1; val},
                    None => {
                        // Trying to move to a square from an invalid direction
                        step_count = 0;
                        turn_count = 0;
                        (x,y) = (start_x, start_y);
                        j += 1;
                        dir = match j {
                            0 => Direction::East,
                            1 => Direction::South,
                            2 => Direction::West,
                            _ => panic!("Back to north"),
                        };
                        coords_with_direction = IndexMap::new();
                        continue;
                    }
                };
                coords_with_direction.insert((x,y), dir.clone());
            },
        }
    }
    // If turn count is positive, our loop was CCW, otherwise CW
    // For CCW loops need to move in -x direction, whenever direction is North
    // For CW loops do the opposite
    let add = if turn_count > 0 { -1 } else { 1 };
    let mut prev_dir: Option<Direction> = None;
    let area: i64 = coords_with_direction.iter().map(|(&(mut x, y), dir)| { 
        let i = match (dir, &prev_dir) {
            (Direction::North, _) => {
                let mut i = 0;
                loop {
                    x += add;
                    if (x < 0) | (x > line_width) {
                        panic!("Must not happen");
                    }
                    match coords_with_direction.get(&(x, y)) {
                        Some(_) => {
                            break;
                        },
                        None => {
                            output = repl_ind(&output, x,y,line_width, "I");
                            i += 1;
                            continue;
                        },
                    };
                }
                i
            },
            (Direction::West, Some(Direction::North)) => {
                let mut i = 0;
                loop {
                    x += add;
                    if (x < 0) | (x > line_width) {
                        panic!("Must not happen");
                    }
                    match coords_with_direction.get(&(x, y)) {
                        Some(_) => {
                            break;
                        },
                        None => {
                            output = repl_ind(&output, x,y,line_width, "I");
                            i += 1;
                            continue;
                        },
                    };
                }
                i
            },
            _ => 0,
        };
        prev_dir = Some(dir.clone());
        i
    }).sum::<i64>();

    (step_count / 2 + 1, area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
        let a2: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let b1: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let b2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(read_contents(&a1).0, 4);
        assert_eq!(read_contents(&a2).0, 4);
        assert_eq!(read_contents(&b1).0, 8);
        assert_eq!(read_contents(&b2).0, 8);


        assert_eq!(read_contents(&a1).1, 1);
        assert_eq!(read_contents(&a2).1, 1);
        assert_eq!(read_contents(&b1).1, 1);
        assert_eq!(read_contents(&b2).1, 1);


let c1: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(read_contents(&c1).1, 4);

let d: &str = "OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJ*L-7OO
L--JOL7***LJS7F-7L7O
OOOOF-J**F7FJ|L7L7L7
OOOOL7*F7||L7|*L7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO";
        assert_eq!(read_contents(&d).1, 8);

let e = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(read_contents(&e).1, 10);
    }
}
