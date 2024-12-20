#[macro_use]
extern crate num_derive;

use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Display;
use core::fmt;
use num_traits::FromPrimitive;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, FromPrimitive)]
enum Dir {
    N,
    E,
    S,
    W,
}


impl Display for Dir{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dir::N => write!(f, "^"),
            Dir::S => write!(f, "v"),
            Dir::W => write!(f, "<"),
            Dir::E => write!(f, ">"),
        }
    }
}

impl Dir{
    const fn get_dir(self) -> (i64, i64) {
        match self {
            Self::N => (0, -1),
            Self::E => (1, 0),
            Self::S => (0, 1),
            Self::W => (-1, 0),
        }
    }
    
    fn cw(self) -> Self {
        FromPrimitive::from_u8((self as u8 + 1) % 4).unwrap()
    }

    fn opposite(self) -> Self {
        FromPrimitive::from_u8((self as u8 + 2) % 4).unwrap()
    }

    fn ccw(self) -> Self {
        FromPrimitive::from_u8((self as u8 + 3) % 4).unwrap()
    }
}

struct Map {
    grid: Vec<Vec<Object>>,
    start: (i64, i64),
    end: (i64, i64),
}

impl Map {
    fn print_map(&self) {
        for ln in &self.grid {
            println!("{}", ln.iter().map(|m| match m {
                Object::Wall => '#',
                Object::Empty => '.',
                Object::Start => 'S',
                Object::End => 'E',
            }).collect::<String>());
        }
    }

    fn print_path(&self, path: Vec<PathHead>) {
        let mut grid = self.grid.clone();
        for p in path {
            grid[p.y as usize][p.x as usize] = Object::Start;
        }
        for ln in &grid {
            println!("{}", ln.iter().map(|m| match m {
                Object::Wall => '#',
                Object::Empty => '.',
                Object::Start => 'S',
                Object::End => 'E',
            }).collect::<String>());
        }
    }
    
    fn is_empty(&self, x1: i64, y1: i64) -> bool {
        if x1 < 0 || y1 < 0 {
            return false;
        }
        if y1 as usize >= self.grid.len() {
            return false;
        }
        if x1 as usize >= self.grid[y1 as usize].len() {
            return false;
        }
        self.grid[y1 as usize][x1 as usize] != Object::Wall
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct PathHead {
    x: i64,
    y: i64,
    direction: Option<Dir>,
}

impl PathHead {
    fn new(x: i64,
        y: i64,
        direction: Option<Dir>,
    ) -> Self {
        Self {x,
            y,
            direction,
        }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Object {
    Wall,
    Empty,
    Start,
    End,
}

impl Object {
    fn new(c: char) -> Self {
        match c {
            '.' => Object::Empty,
            '#' => Object::Wall,
            'S' => Object::Start,
            'E' => Object::End,
            _ => panic!("Unknown character"),
        }
    }
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents, 100);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn read_map(cont: &str) -> Map {
    let grid: Vec<Vec<Object>> = cont.lines().filter(|ln| ln.starts_with('#')).map(|ln| {
            ln.chars().map(move |c| {
                Object::new(c)
            }).collect::<Vec<Object>>()
        }).collect::<Vec<Vec<Object>>>();
    let mut start: Option<(i64, i64)> = None;
    let mut end: Option<(i64, i64)> = None;
    for (y,v) in grid.iter().enumerate() {
        for (x,t) in v.iter().enumerate() {
            if t == &Object::Start {
                start = Some((x as i64,y as i64));
            }
            if t == &Object::End {
                end = Some((x as i64,y as i64));
            }
        }
    }

    Map {grid, start: start.unwrap(), end: end.unwrap()}
}


fn read_contents(cont: &str, min_cheat: i64) -> (i64, i64) {
    let map = read_map(cont);
    map.print_map();

    let mut path: Vec<PathHead> = Vec::new();

    let start = map.start;

    let mut pos = PathHead::new(start.0, start.1, None);
    path.push(pos);

    let target = map.end;

    loop {
        if (pos.x == target.0) & (pos.y == target.1) {
            // We found the end
            break;
        }

        for dir in Dir::iter() {
            let d = dir.get_dir();


            let x = (
                pos.x + d.0,
                pos.y + d.1,
            );
            dbg!(&pos);
            if let Some(dd) = &pos.direction  {
                if dd.opposite() == dir {
                    continue;
                }
            }
            if map.is_empty(x.0, x.1) {
                println!("At {}, {}, going {}", x.0, x.1, dir);
                pos = PathHead::new(x.0, x.1, Some(dir));
                path.push(pos);
            }
        }
    }
    map.print_path(path.clone());
    let cheats: usize = path.iter().enumerate().map(|(i, x)|
        {
            path.iter().enumerate().skip(i + 2 + min_cheat as usize).filter(|(j,y)| {
                taxicab(x.x, x.y, y.x, y.y) == 2
            }).count()
        }).sum();

    (cheats as i64, 0)
}

fn taxicab(x1: i64, y1: i64, x2: i64, y2: i64) -> i64{
    i64::abs(x1-x2) + i64::abs(y1-y2)
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        assert_eq!(read_contents(&a, 40).0, 2);
        //assert_eq!(read_contents(&a).1, 81);
    }

}
