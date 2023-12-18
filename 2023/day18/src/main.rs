#[macro_use]
extern crate num_derive;
use clap::Parser;
use std::fs;
use std::fmt;
use std::collections::BTreeMap;
use std::cmp::{max,min};
use indexmap::IndexMap;
use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple};
use regex::Regex;
use num_traits::FromPrimitive;


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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy, FromPrimitive)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn new(c: &str) -> Direction {
        match c {
            "L" => Direction::West,
            "D" => Direction::South,
            "U" => Direction::North,
            "R" => Direction::East,
            v => panic!("Unknown character {}", v),
        }
    }

    // Positive if turn from self to other is ccw
    // Negative if turn from self to other is cw
    fn get_turn(self, other: Direction) -> i64 {
        match ((other as i64) - (self as i64) + 4) % 4 - 4 {
            3 | -1 => -1,
            -3 | 1 => 1,
            v => panic!("Got {}", v),
        }
    }

    fn opposite(self) -> Direction {
        FromPrimitive::from_u8((self as u8 + 2) % 4).unwrap()
    }

    fn cw(self) -> Direction {
        FromPrimitive::from_u8((self as u8 + 1) % 4).unwrap()
    }

    fn ccw(self) -> Direction {
        FromPrimitive::from_u8((self as u8 + 3) % 4).unwrap()
    }

    fn get_d(self) -> (i64, i64) {
        match self {
            Direction::North => (0,1),
            Direction::South => (0,-1),
            Direction::East => (1,0),
            Direction::West => (-1,0),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::South => write!(f, "v"),
            Direction::West => write!(f, "<"),
            Direction::East => write!(f, ">"),
        }
    }
}


#[derive(Debug,PartialEq)]
pub struct Color {
  pub red:     u8,
  pub green:   u8,
  pub blue:    u8,
}

#[derive(Debug)]
struct Dig {
    direction: Direction,
    length: i64,
}


impl Dig {
    //L 5 (#8ceee2)
    fn new(direction: Direction, length: i64) -> Dig {
        Dig {direction: direction, length: length}
    }
}

fn parse_line(input: &str) -> Option<(Dig, Dig)> {
    let re = Regex::new(r"([UDLR]) (\d+) \((#[a-z0-9]+)\)").unwrap();
    let Some(res) = re.captures(input) else {return None; };
    let direction = Direction::new(&res[1]);
    let length = res[2].parse::<i64>().unwrap();
    let alt = instructions(&res[3]).unwrap().1;
    Some((Dig::new(direction, length), alt))
}

fn from_hex64(input: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}


fn instructions(input: &str) -> IResult<&str, Dig> {
    let (input, _) = tag("#")(input)?;
    let (input, (length, direction)) = tuple((
        map_res(take_while_m_n(5,5, is_hex_digit), from_hex64),
        map_res(take_while_m_n(1,1, is_hex_digit), from_hex64),
        ))(input)?;

    let dir = match direction {
        0 => Direction::East,
        1 => Direction::South,
        2 => Direction::West,
        3 => Direction::North,
        _ => panic!("Unknown direction"),
    };
    Ok((input, Dig::new(dir, length as i64)))
}

fn read_contents(cont: &str) -> (i64, i64) {
    let dig_pairs: Vec<(Dig, Dig)> = cont.lines().filter_map(|i| {
        parse_line(i)
    }).collect();

    let digs: Vec<&Dig> = dig_pairs.iter().map(|(a,_)| {a}).collect();
    let digs_alt: Vec<&Dig> = dig_pairs.iter().map(|(_,b)| {b}).collect();
    let part1 = analyze_digs(digs, false);
    println!("Part 1 answer is {}", part1);
    let part2 = analyze_digs(digs_alt, false);
    (part1, part2)
}

fn analyze_digs(digs: Vec<&Dig>, test: bool) -> i64 {
    let start_x: i64 = 0;
    let start_y: i64 = 0;

    let (mut max_x, mut max_y) = (0,0);
    let (mut min_x, mut min_y) = (0,0);

    let (mut x, mut y) = (start_x, start_y);
    let mut prev_direction: Direction = digs.last().unwrap().direction;

    let mut turn_count = 0; // CCW increases, CW decreases
    let mut coords_with_direction: IndexMap<(i64, i64), Direction> = IndexMap::new();
    let mut rows: BTreeMap<i64, Vec<(i64, Direction)>> = BTreeMap::new();
    for (i,d) in digs.iter().enumerate() {
        println!("{} / {}", i+1, digs.len());
        let dir = d.direction;
        let (dx, dy) = dir.get_d();
        turn_count += prev_direction.get_turn(dir);
        for i in 1..=d.length {
            let dir2 = match (i, prev_direction) {
                (1, Direction::North | Direction::South) => prev_direction,
                (_,_) => dir,
            };
            coords_with_direction.insert((x,y), dir2);
            match rows.get_mut(&y) {
                None => {
                    rows.insert(y, vec![(x, dir2)]);
                }
                Some(v) => {
                    v.push((x, dir2))
                },
            }
            x += dx;
            y += dy;
        }
        prev_direction = dir.clone();

        max_x = max(x, max_x);
        min_x = min(x, min_x);

        max_y = max(y, max_y);
        min_y = min(y, min_y);
    }


    assert_eq!(x, start_x);
    assert_eq!(y, start_y);

    if test {
        return coords_with_direction.len() as i64;
    }
    println!("We have {} coordinates", coords_with_direction.len());
    //area(coords_with_direction, turn_count)
    area2(rows, turn_count, max_y, min_y) + coords_with_direction.len() as i64
}


fn area2(rows: BTreeMap<i64, Vec<(i64, Direction)>>,
    turn_count: i64,
    max_y: i64,
    min_y: i64) -> i64 {
    // The following assumes that loop is clockwise, i.e. turn count is negative
    // The logic could be modified to accept ccw loops, by switching north and south
    assert!(turn_count < 0);
    let multi = if turn_count < 0 { 1 } else { -1 };
    println!("Need to check {} rows", max_y - min_y + 1);
    (min_y..=max_y).map(|row_id| {
        let i = row_id - min_y;
        if i % 10000 == 0{
            println!("i: {}", i);
        }
        let mut inside = -1;
        let mut row = rows.get(&row_id).unwrap().clone(); 
        row.sort_by_key(|k| k.0);
        let mut a = 0;
        let mut prev_x = row[0].0 - 1;
        for (x, dir) in row {
            //dbg!(&dir);
            if inside > 0 {
                a += x - prev_x - 1;
            }
            match dir {
                Direction::North => {
                    inside = 1 * multi;
                }
                Direction::South => {
                    inside = -1 * multi;
                }
                _ => (),
            }
            prev_x = x;
        }
        a
    }).sum()
}

fn area(coords_with_direction: IndexMap<(i64, i64), Direction>, turn_count: i64) -> i64{
    // If turn count is positive, our loop was CCW, otherwise CW
    // For CCW loops need to move in -x direction, whenever direction is North
    // For CW loops do the opposite
    //
    let add = if turn_count > 0 { -1 } else { 1 };
    let mut prev_dir: Option<Direction> = None;
    let area: i64 = coords_with_direction.iter().map(|(&(mut x, y), dir)| { 
        //output = repl_ind(&output, x, y, line_width, &format!("{}", dir)[..]);
        let mut i = 0;
        i += match (dir, &prev_dir) {
            (Direction::North, _) => {
                let mut i = 0;
                loop {
                    x += add;
                    match coords_with_direction.get(&(x, y)) {
                        Some(_) => {
                            break;
                        },
                        None => {
                            //output = repl_ind(&output, x,y, line_width, "I");
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
    // println!("\n{}", output);

    area + coords_with_direction.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn(){
        use Direction::*;
        assert_eq!(West.get_turn(North), -1);
        assert_eq!(North.get_turn(East), -1);
        assert_eq!(East.get_turn(South), -1);
        assert_eq!(South.get_turn(West), -1);

        assert_eq!(West.get_turn(South), 1);
        assert_eq!(North.get_turn(West), 1);
        assert_eq!(East.get_turn(North), 1);
        assert_eq!(South.get_turn(East), 1);
    }

    #[test]
    fn conts() {
        let a1: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        // Expected route
        let _b: &str = "#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######";
        assert_eq!(read_contents(&a1).0, 62);
        assert_eq!(read_contents(&a1).1, 952408144115);
    }
}
