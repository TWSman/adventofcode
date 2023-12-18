#[macro_use]
extern crate num_derive;
use clap::Parser;
use std::fs;
use std::fmt;
use std::cmp::{max, min};
use std::collections::BTreeMap;
use indexmap::IndexMap;
use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple};
use regex::Regex;
use num_format::{Locale, ToFormattedString};


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
    // Should be 
    let target: i64 = 159485361249806;
    println!("Part 2 answer is {}", res.1.to_formatted_string(&Locale::fr));
    println!("Part 2 expected  {}", target.to_formatted_string(&Locale::fr));
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


#[derive(Debug, Clone)]
struct Dig {
    direction: Direction,
    length: i64,
}


impl Dig {
    fn new(direction: Direction, length: i64) -> Dig {
        Dig {direction: direction, length: length}
    }
}

#[derive(Debug, Clone)]
struct DigLineVertical {
    direction: Direction,
    max_y: i64,
    min_y: i64,
    x: i64,
}

#[derive(Debug, Clone)]
struct DigLineHorizontal {
    max_x: i64,
    min_x: i64,
    y: i64,
}


impl DigLineVertical {
    fn intersects(&self, y: i64) -> bool {
        (y <= self.max_y) & (y >= self.min_y)
    }

    fn new(dig: &Dig, start_x: i64, start_y: i64) -> DigLineVertical {
        let (min_y, max_y) = match dig.direction {
            Direction::North => (start_y, start_y + dig.length),
            Direction::South => (start_y - dig.length, start_y),
            _ => panic!("Should not happen"),
        };
        DigLineVertical{ direction: dig.direction, max_y: max_y, min_y: min_y, x: start_x}
    }
}

impl DigLineHorizontal {
    fn new(dig: &Dig, start_x: i64, start_y: i64) -> DigLineHorizontal {
        let (min_x, max_x) = match dig.direction {
            Direction::East => (start_x, start_x + dig.length),
            Direction::West => (start_x - dig.length, start_x),
            _ => panic!("Should not happen"),
        };
        DigLineHorizontal{ max_x: max_x, min_x: min_x, y: start_y}
    }

    fn area(&self) -> i64 {
        self.max_x - self.min_x +1
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
    let part2 = analyze_digs(digs_alt, true);
    //let part2 = 0;
    (part1, part2)
}

fn analyze_digs(digs: Vec<&Dig>, verbose: bool) -> i64 {
    let start_x: i64 = 0;
    let start_y: i64 = 0;

    let (mut x, mut y) = (start_x, start_y);
    let mut prev_direction: Direction = digs.last().unwrap().direction;

    let mut turn_count = 0; // CCW increases, CW decreases
    let mut diglines: Vec<DigLineVertical> = Vec::new();
    let mut diglines_horizontal: Vec<DigLineHorizontal> = Vec::new();

    for (_i,d) in digs.iter().enumerate() {
        let dir = d.direction;
        let (dx, dy) = dir.get_d();
        turn_count += prev_direction.get_turn(dir);
        if (d.direction == Direction::North) | (d.direction == Direction::South) {
            diglines.push(DigLineVertical::new(d,x,y) );
        } else {
            diglines_horizontal.push(DigLineHorizontal::new(d,x,y) );
        }
        x += d.length * dx;
        y += d.length * dy;
        prev_direction = dir.clone();

    }

    let max_y = diglines.iter().map(|d| {d.max_y}).max().unwrap();
    let min_y = diglines.iter().map(|d| {d.min_y}).min().unwrap();

    // We should have arrived at the start
    assert_eq!(x, start_x);
    assert_eq!(y, start_y);

    // Sort the array here, otherwise we would need to sort the intersection array every time
    // separately
    diglines.sort_by_key(|k| k.x);
    area3(diglines, diglines_horizontal, turn_count, min_y, max_y, verbose) as i64
}


fn area3(diglines: Vec<DigLineVertical>, diglines_horizontal: Vec<DigLineHorizontal>, turn_count: i64, min_y: i64, max_y: i64, verbose: bool) -> i128 {
    let multi = if turn_count < 0 { 1 } else { -1 };
    dbg!(&diglines_horizontal);
    let count = max_y - min_y;
    let mut row_id = min_y;
    let mut summed_area: i128 = 0;
    if verbose {
        println!("Row: {}, Area: {}", row_id, summed_area);
    }
    loop {
        if row_id > max_y {
            break
        }
        let i = row_id - min_y;
        if (i % 100000 == 0) & verbose{
            println!("i: {} / {}", i, count);
        }
        // Since the input vector should be sorted, intersects will also be sorted
        let intersects: Vec<(i64, Direction, i64, i64)> = diglines.iter().filter_map(|d| {
            if d.intersects(row_id) {
                Some((d.x, d.direction, d.min_y, d.max_y))
            } else {
                None
            }
        }).collect();

        let horizontals: Vec<&DigLineHorizontal> = diglines_horizontal.iter().filter_map(|d| {
            if d.y == row_id {
                Some(d)
            } else {
                None
            }
        }).collect();


        // As long as there are no horizontal sections we can jump forward to the next horizontal
        let common_length = if horizontals.len() == 0 {
            let mut common_max: i64 = max_y;
            match diglines_horizontal.iter().filter_map(|h| {
                if h.y > row_id {
                    Some(h.y)
                } else {
                    None
                }
            }).min() {
                None => (),
                Some(h) => {
                    common_max = h;
                }
                };
            max(1, common_max - row_id)
        } else {
            1
        };

        let mut prev_x = intersects[0].0 - 1;
        let mut inside = -1;
        let mut a: i64 = 0;
        for (x, dir, _min_y, _max_y) in intersects {
            a += 1;
            //dbg!(&dir);
            if inside > 0 {
                a += x - prev_x - 1;
            } else {
                let tmp = horizontals.iter().filter_map(|h| {
                    if (h.max_x == x) & (h.min_x == prev_x) {
                        //dbg!(&h);
                        Some(h.area() - 2)
                    } else {
                        None
                    }
                }).sum::<i64>();
                a += tmp;
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
        summed_area += (a * common_length) as i128;
        if common_length > 1 {
            row_id += common_length;
        } else {
            row_id += common_length;
        }
        if verbose {
            println!("Row: {}, horizontals: {},  Area: {}", row_id, horizontals.len(), summed_area);
        }
    };
    return summed_area;
}

fn area2(rows: BTreeMap<i64, Vec<(i64, Direction)>>,
    turn_count: i64,
    max_y: i64,
    min_y: i64) -> i64 {
    // The following assumes that loop is clockwise, i.e. turn count is negative
    // The logic could be modified to accept ccw loops, by switching north and south
    assert!(turn_count < 0);
    let multi = if turn_count < 0 { 1 } else { -1 };
    //println!("Need to check {} rows", max_y - min_y + 1);
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
        println!("Row: {}, area: {}", row_id, a);
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
