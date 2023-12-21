#[macro_use]
extern crate num_derive;

use clap::Parser;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::max;
use std::fmt::Display;
use core::fmt;
use num_traits::FromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

// This is the cycle order
#[derive(EnumIter, Debug, PartialEq, Eq, Clone, Hash, Copy, FromPrimitive)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::South => write!(f, "v"),
            Direction::West => write!(f, "<"),
            Direction::East => write!(f, ">"),
        }
    }
}

impl Direction {
    fn opposite(self) -> Direction {
        FromPrimitive::from_u8((self as u8 + 2) % 4).unwrap()
    }

    fn cw(self) -> Direction {
        FromPrimitive::from_u8((self as u8 + 1) % 4).unwrap()
    }

    fn ccw(self) -> Direction {
        FromPrimitive::from_u8((self as u8 + 3) % 4).unwrap()
    }

    fn get_dx(&self) -> (i64,i64) {
        match self {
            Direction::East => ( 1,  0),
            Direction::West => (-1,  0),
            Direction::North => (0, -1),
            Direction::South => (0, 1),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct PathHead {
    x: i64,
    y: i64,
}


impl PathHead {
    fn new(x:i64, y:i64) -> PathHead {
        PathHead {x: x, y:y}
    }
}


fn taxicab(x1: i64, y1: i64, x2: i64, y2: i64) -> i64{
    i64::abs(x1-x2) + i64::abs(y1-y2)
}

fn print(blocks: &HashMap<(i64,i64), Block>, n_rows: i64, n_cols: i64) -> String {
    let str = (0..n_rows).map(|i_row| {
        let x = (0..n_cols).map(|i_col| {
            let tmp: String = 
            match blocks.get(&(i_col, i_row)) {
                None => panic!("Could not find marker at {} {}", i_col, i_row),
                Some(b) if b.block_type == BlockType::Rock => "#",
                Some(b) if b.block_type == BlockType::Start => "S",
                Some(b) if b.visited => "O",
                Some(_b) => ".",
            }.to_string();
            tmp
        }).collect::<Vec<_>>().join("");
        x
    }).collect::<Vec<_>>().join("\n");
    str
}


fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");

    // 0 cycles means just one tilt to north (part1)
    let res = read_contents(&contents, 64);
    println!("Part 1 answer is {}", res);

}

#[derive(Debug, PartialEq, Eq, Clone)]
enum BlockType {
    Garden,
    Rock,
    Start,
}

impl BlockType {
    fn new(c: char) -> BlockType {
        match c {
            '#' => BlockType::Rock,
            '.' => BlockType::Garden,
            'S' => BlockType::Start,
            v => panic!("Unknown character {}", v),
        }
    }
}

#[derive(Debug)]
struct Block {
    x: i64,
    y: i64,
    block_type: BlockType,
    visited: bool,
}

impl Block {
    fn new(x:i64,y:i64, block_type: BlockType) -> Block {
        Block {x:x, y:y, block_type: block_type, visited: false}
    }
}

fn read_contents(cont: &str, steps: i64) -> i64 {
    let line_width = cont.lines().next().expect("Should be at least 1 line").len() as i64 + 1;

    let mut blocks: HashMap<(i64,i64), Block> = HashMap::new();

    let mut start_x = 0;
    let mut start_y = 0;

    let mut max_y = 0;
    for (i,c) in cont.chars().enumerate() {
        let y = (i as i64) / line_width;
        max_y = max(y, max_y);
        match c {
            '\n' | ' ' => { continue; },
            c => {
                let x = (i as i64) % line_width;
                let block_type = BlockType::new(c);
                if block_type == BlockType::Start {
                    start_x = x;
                    start_y = y;
                }
                let block = Block::new(x,y,block_type.clone());
                blocks.insert((x,y), block);
            },
        }
    }
    let mut paths: Vec<PathHead> = vec![PathHead::new(start_x, start_y)];
    for _ in 0..steps {
        let mut new_paths: HashSet<PathHead> = HashSet::new();
        for p in &paths {
            for direction in Direction::iter() {
                let (dx, dy) = direction.get_dx();
                let x = dx + p.x;
                let y = dy + p.y;
                match blocks.get_mut(&(x,y)) {
                    None => continue,
                    Some(b) if b.block_type == BlockType::Rock => continue,
                    Some(b) if b.visited => continue,
                    Some(b) => {
                        new_paths.insert(PathHead::new(x,y));
                        b.visited = true;
                    }
                }
                
            }
        }
        for p in new_paths {
            paths.push(p);
        }
        println!("\n{}", print(&blocks, max_y, line_width - 1));
    }
    blocks.values().filter(|v| {
        v.visited
    }).count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        //println!("{}", a);
        assert_eq!(read_contents(&a, 6), 16);
        //assert_eq!(read_contents(&a).1, 94);
    }

}
