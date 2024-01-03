#[macro_use]
extern crate num_derive;

use clap::Parser;
use std::fs;
use std::collections::HashMap;
use std::cmp::max;
use priority_queue::PriorityQueue;
use std::fmt::Display;
use core::fmt;
use num_traits::FromPrimitive;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

// This is the cycle order
#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy, FromPrimitive)]
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

    fn swap_xy(&self, x: i64, y: i64, direction: Direction) -> (i64, i64, Direction) {
        // East is the default direction
        match self {
            Direction::East => (x, y, direction),
            Direction::West => (-x, -y, direction.opposite()),
            Direction::North => (-y, -x, direction.ccw()),
            Direction::South => (y, x, direction.cw()),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct PathHead {
    x: i64,
    y: i64,
    price: i64,
    direction: Direction,
    history: Vec<(i64,i64,Direction)>,
}


impl PathHead {
    fn new(x:i64, y:i64, price: i64, direction: Direction) -> PathHead {
        PathHead {x, y, price, direction, history: vec![(x,y,direction)]}
    }
}

impl Display for PathHead {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Price: {}, at ({}, {}) Going {}, has {} nodes", self.price, self.x, self.y, self.direction, self.history.len())
    }
}

#[derive(Debug, Clone)]
struct City {
    blocks: HashMap<(i64,i64), i64>,
    n_cols: i64,
    n_rows: i64,
    solved_history: Option<PathHead>,
}


#[allow(dead_code)]
impl City {
    fn new(cont: &str) -> City {
        let line_width = cont.lines().next().expect("Should be at least 1 line").len() as i64 + 1;

        let mut blocks: HashMap<(i64,i64), i64> = HashMap::new();

        let mut max_y = 0;
        for (i,c) in cont.chars().enumerate() {
            let y = (i as i64) / line_width;
            max_y = max(y, max_y);
            match c {
                '\n' | ' ' => { continue; },
                c => {
                    let x = (i as i64) % line_width;
                    let price = match c.to_digit(10) {
                        Some(v) => v,
                        None => panic!("Should not happen")
                    };

                    blocks.insert((x,y), price as i64);
                },
            }
        }
        City {blocks, n_cols: line_width - 1, n_rows: max_y + 1, solved_history: None}
    }

    fn print(&self) -> String {
        print(&self.blocks, self.n_rows, self.n_cols, &self.solved_history)
    }

    fn get(&self, x: i64, y: i64) -> i64 {
        match self.blocks.get(&(x, y)) {
            None => panic!("No such location"),
            Some(price) => *price,
        }
    }

    fn solve(&mut self, start_x: i64, start_y: i64, pos: Vec<(i64,i64, Direction)>) -> i64 {
        let target_x = self.n_cols - 1;
        let target_y = self.n_rows - 1;
        let mut paths = PriorityQueue::new();
        let mut already_found: HashMap<(i64, i64, Direction), i64> = HashMap::new();

        // We can start either by going west/
        let west = PathHead::new(start_x, start_y, 0, Direction::East);
        let south = PathHead::new(start_x, start_y, 0, Direction::South);
        let h  = taxicab(start_x, target_x, start_y, target_y);
        already_found.insert((west.x, west.y, west.direction), 0);
        already_found.insert((south.x, south.y, south.direction), 0);
        let _ = &paths.push(west, h);
        let _ = &paths.push(south, h);

        let mut count = 0;
        loop {
            let (path, _priority) = match paths.pop() {
                None => panic!("No pathheads"),
                Some(p) => p,
            };
            if (path.x == target_x) & (path.y == target_y) {
                self.solved_history = Some(path.clone());
                return path.price;
            }

            if count % 10_000 == 0 {
                println!("At step {}, {} squares to end", count,
                    taxicab(path.x, target_x, path.y, target_y)
                );
            }
            count += 1;
            for v in pos.iter() {
                let (mut dx, mut dy,dir) = path.direction.swap_xy(v.0, v.1, v.2);
                if (path.x + dx < 0) | (path.y + dy < 0) | (path.x + dx >= self.n_cols) | (path.y + dy >= self.n_rows) {
                    continue;
                }
                let mut multi: i64 = 1;
                if dx < 0 {
                    dx *= -1;
                    multi = -1;
                }
                if dy < 0 {
                    dy *= -1;
                    multi = -1;
                }
                let new_x = path.x + multi * dx;
                let new_y = path.y + multi * dy;
                let newprice: i64 = if dx > 0 {
                    (1..=dx).map(|x| {
                        self.get(path.x + multi * x, path.y)
                    }).sum::<i64>() + path.price
                } else {
                    (1..=dy).map(|y| {
                        self.get(path.x, path.y + multi * y)
                    }).sum::<i64>() + path.price
                };

                let h  = taxicab(new_x, target_x, new_y, target_y);
                let newhead = PathHead::new(new_x, new_y,newprice, dir);
                match already_found.get(&(new_x, new_y, dir)) {
                    Some(v) if v < &newprice => (),
                    _ => {
                        paths.push(newhead, -(newprice + h));
                        already_found.insert((new_x, new_y, dir), newprice);
                    },
                }
            }
        }
    }
}

fn taxicab(x1: i64, y1: i64, x2: i64, y2: i64) -> i64{
    i64::abs(x1-x2) + i64::abs(y1-y2)
}

fn print(blocks: &HashMap<(i64,i64), i64>, n_rows: i64, n_cols: i64, history: &Option<PathHead>) -> String {
    let mut str = (0..n_rows).map(|i_row| {
        let x = (0..n_cols).map(|i_col| {
            let tmp: String = 
            match blocks.get(&(i_col, i_row)) {
                None => panic!("Could not find marker at {} {}", i_col, i_row),
                Some(price) => format!("{}", price),
            };
            tmp
        }).collect::<Vec<_>>().join("");
        x
    }).collect::<Vec<_>>().join("\n");
    match history {
        None => (),
        Some(v) => {
            for (x,y,dir) in &v.history {
                let i = ((1+n_cols) * y + x) as usize;
                str.replace_range(i..(i+1), match dir {
                    Direction::North => "^",
                    Direction::South => "v",
                    Direction::West => "<",
                    Direction::East => ">",
                });
            }
        }
    }
    str
}


fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");

    // 0 cycles means just one tilt to north (part1)
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);

}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut city = City::new(cont);
    // In part1 we can move 1-3 steps and then turn
    let p1: Vec<(i64,i64, Direction)> = vec![
        (1,0, Direction::North),
        (2,0, Direction::North),
        (3,0, Direction::North),
        (1,0, Direction::South),
        (2,0, Direction::South),
        (3,0, Direction::South),
    ];

    // In part1 we can move 4-10 steps and then turn
    let p2: Vec<(i64,i64, Direction)> = vec![
        (4,0, Direction::North),
        (5,0, Direction::North),
        (6,0, Direction::North),
        (7,0, Direction::North),
        (8,0, Direction::North),
        (9,0, Direction::North),
        (10,0, Direction::North),
        (4,0, Direction::South),
        (5,0, Direction::South),
        (6,0, Direction::South),
        (7,0, Direction::South),
        (8,0, Direction::South),
        (9,0, Direction::South),
        (10,0, Direction::South),
    ];

    let part1 = city.solve(0, 0, p1);
    let part2 = city.solve(0, 0, p2);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction() {
        let w = Direction::West; // >
        let s = Direction::South; // v
        let n = Direction::North; // v
        let e = Direction::East; // v
        assert_eq!(e.swap_xy(2,0, Direction::North), (2,0, Direction::North));
        assert_eq!(s.swap_xy(2,0, Direction::North), (0,2, Direction::East));
        assert_eq!(w.swap_xy(2,0, Direction::North), (-2,0, Direction::South));
        assert_eq!(n.swap_xy(2,0, Direction::North), (0, -2, Direction::West));

        assert_eq!(e.swap_xy(0,2, Direction::North), (0,  2, Direction::North));
        assert_eq!(s.swap_xy(0,2, Direction::North), (2,  0, Direction::East));
        assert_eq!(w.swap_xy(0,2, Direction::North), (0, -2, Direction::South));
        assert_eq!(n.swap_xy(0,2, Direction::North), (-2, 0, Direction::West));
    }

    #[test]
    fn conts() {
        let a = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        //println!("{}", a);
        assert_eq!(read_contents(&a).0, 102);
        assert_eq!(read_contents(&a).1, 94);
        let b = "111111111111
999999999991
999999999991
999999999991
999999999991";

        assert_eq!(read_contents(&b).1, 71);
    }

}
