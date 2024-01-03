#[macro_use]
extern crate num_derive;

use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::max;
use std::fmt::Display;
use core::fmt;
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
        PathHead {x, y}
    }
}


#[allow(dead_code)]
fn print(blocks: &BTreeMap<(i64,i64), Block>, _paths: &HashSet<PathHead>, n_rows: i64, n_cols: i64) -> String {
    let str = (0..n_rows).map(|i_row| {
        let x = (0..n_cols).map(|i_col| {
            match blocks.get(&(i_col, i_row)) {
                None => panic!("Could not find marker at {} {}", i_col, i_row),
                Some(b) if b.block_type == BlockType::Rock => "#".to_string(),
                Some(b) if b.block_type == BlockType::Start => "S".to_string(),
                Some(b) if !b.visited.is_empty() => format!("{}", b.visited.len()),
                Some(_b) => ".".to_string(),
            }
        }).collect::<Vec<_>>().join("");
        x
    }).collect::<Vec<_>>().join("\n");
    str
}


fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");

    let res = part1(&contents, 64);
    println!("Part 1 answer is {}", res);

    let res = part2(&contents, 26_501_365);
    println!("Part 2 answer is {}", res);

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
    block_type: BlockType,
    visited: HashMap<(i64,i64), i64>
}

impl Block {
    fn new(block_type: BlockType) -> Block {
        Block {block_type, visited: HashMap::new()}
    }
}

fn get_blocks(cont: &str) -> (i64,i64,i64,i64, BTreeMap<(i64,i64), Block>) {
    let line_width = cont.lines().next().expect("Should be at least 1 line").len() as i64 + 1;
    let mut blocks: BTreeMap<(i64,i64), Block> = BTreeMap::new();

    let mut start_x = 0;
    let mut start_y = 0;

    assert_eq!((start_x + start_y) % 2, 0);

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
                let block = Block::new(block_type.clone());
                blocks.insert((x,y), block);
            },
        }
    }
    (start_x, start_y, line_width - 1, max_y + 1, blocks)
}

fn part1(cont: &str, steps: i64) -> i64 {
    let (start_x, start_y, _n_cols, _n_rows, mut blocks) = get_blocks(cont);
    let mut paths: HashSet<PathHead> = HashSet::new();
    paths.insert(PathHead::new(start_x, start_y));
    for i in 0..steps {
        let mut new_paths: HashSet<PathHead> = HashSet::new();
        for p in paths.drain() {
            for direction in Direction::iter() {
                let (dx, dy) = direction.get_dx();
                let x = dx + p.x;
                let y = dy + p.y;
                match blocks.get_mut(&(x,y)) {
                    None => continue,
                    Some(b) if b.block_type == BlockType::Rock => continue,
                    Some(b) if b.visited.get(&(0,0)).is_some() => continue,
                    Some(b) => {
                        b.visited.insert((0,0), i);
                        new_paths.insert(PathHead::new(x,y));
                    }
                }
                
            }
        }
        paths = new_paths;
        //println!("\n{}", print(&blocks, &paths, max_y, line_width - 1));
    }
    blocks.iter().filter(|((x,y), b)| {
        let visited = b.visited.contains_key(&(0,0));
        visited & ((x + y) % 2 == 0)
    }).count() as i64
}
// 600336060511101 is the correct answer
fn part2(cont: &str, steps: i64) -> i64 {
    let (start_x, start_y, n_cols, n_rows, mut blocks) = get_blocks(cont);
    let mut paths: HashSet<PathHead> = HashSet::new();
    paths.insert(PathHead::new(start_x, start_y));
    let mut path_cache: HashMap<Vec<i64>, (i64, i64)> = HashMap::new();
    let mut visit_counts: Vec<i64> = Vec::new();
    let loop_size = 262;
    for i in 0..steps {
        if (steps - i)  % loop_size == 0 {
            println!("step: {}, {} visited", i, get_count(&blocks, steps % 2));
        }
        let mut new_paths: HashSet<PathHead> = HashSet::new();
        // Loop seems to be 131 steps
        for p in paths.drain() {
            for direction in Direction::iter() {
                let (dx, dy) = direction.get_dx();
                let (ix, x) = (
                    (dx + p.x).div_euclid(n_cols),
                    (dx + p.x).rem_euclid(n_cols));
                let (iy, y) = (
                    (dy + p.y).div_euclid(n_rows),
                    (dy + p.y).rem_euclid(n_rows));
                match blocks.get_mut(&(x,y)) {
                    Some(b) if b.block_type == BlockType::Rock => continue,
                    Some(b) if b.visited.contains_key(&(ix,iy)) => continue,
                    Some(b) => {
                        b.visited.insert((ix,iy), i);
                        new_paths.insert(PathHead::new(p.x + dx, p.y + dy));
                    }
                    _ => panic!("Got nothing with {}, {}", x,y),
                }
            }
        }
        //println!("\ni: {}\n{}", i, print(&blocks, &paths, n_rows, n_cols));
        let cached_set = new_paths.iter().map(|head| {
            let (x,y) = (head.x.rem_euclid(n_cols), head.y.rem_euclid(n_rows));
            (1+n_cols) * x + y
        }).collect::<HashSet<_>>();

        let mut cached_paths = cached_set.into_iter().collect::<Vec<_>>();
        cached_paths.sort();

        match path_cache.get(&cached_paths) {
            None => {
                path_cache.insert(cached_paths, (i, get_count(&blocks, steps % 2)));
            },
            Some((_step, _count)) => {
                if (steps - i)  % loop_size == 0 {
                    let new_count = get_count(&blocks, steps % 2);
                    visit_counts.push(new_count);
                    let loops_to_go: i64 = (steps -i) / loop_size;
                    // 91853 is the count at step 327
                    // Every 262 steps the count increases by n*117352 + 205504, where n is the
                    // amount of 262 step loops since step 327
                    let final_est: i64 = new_count + (0..loops_to_go).map(|n| n*117352 + 205504).sum::<i64>();
                    return final_est;
                }
            }
        }
        paths = new_paths;
    }
    get_count(&blocks, steps % 2)
}

fn get_count(blocks: &BTreeMap<(i64,i64), Block>, parity: i64) -> i64 {
    assert!(parity < 2);
    assert!(parity > -1);
    let t = blocks.iter().map(|((x,y), b)| {
        let tmp = b.visited.iter().filter(|((ix,iy), _i_step)| {
            (x + y + ix.abs() + iy.abs()) % 2 == parity
        }).count() as i64;
        tmp
    }).collect::<Vec<_>>();
    t.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div() {
        let a: i64 = 3;
        let b: i64 = -3;
        assert_eq!(a.div_euclid(2), 1);
        assert_eq!(b.div_euclid(2), -2);
        assert_eq!(b.rem_euclid(2), 1);
        assert_eq!(b.div_euclid(2) * 2 + b.rem_euclid(2), b);
        assert_eq!(b / 2, -1);

        let cols: i64 = 11;
        let x: i64 = -1;
        assert_eq!(x.div_euclid(cols), -1);
        assert_eq!(x.rem_euclid(cols), 10);

        let x: i64 = 11;
        assert_eq!(x.div_euclid(cols), 1);
        assert_eq!(x.rem_euclid(cols), 0);
    }

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
        assert_eq!(part1(&a, 6), 16);
        assert_eq!(part2(&a, 6), 16);
        //assert_eq!(part2(&a, 10), 50);
        assert_eq!(part2(&a, 50), 1594);
        assert_eq!(part2(&a, 100), 6536);  
        assert_eq!(part2(&a, 500), 167004);  
        assert_eq!(part2(&a, 1000), 668697);  
        assert_eq!(part2(&a, 5000), 16733044);  
    }

}
