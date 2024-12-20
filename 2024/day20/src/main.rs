#[macro_use]
extern crate num_derive;

use clap::Parser;
use std::fs;
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


#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, FromPrimitive, Hash)]
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
    fn new(c: char) -> Self {
        match c {
            '^' => Dir::N,
            'v' => Dir::S,
            '<' => Dir::W,
            '>' => Dir::E,
            _ => panic!("Unknown character"),
        }
    }
    const fn get_dir(self) -> (i64, i64) {
        match self {
            Self::N => (0, -1),
            Self::E => (1, 0),
            Self::S => (0, 1),
            Self::W => (-1, 0),
        }
    }

    const fn get_char(self) -> char {
        match self {
            Self::N => '^',
            Self::E => '>',
            Self::S => 'v',
            Self::W => '<',
        }
    }

    fn opposite(self) -> Self {
        FromPrimitive::from_u8((self as u8 + 2) % 4).unwrap()
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
                Object::Path(v) => v.get_char(),
            }).collect::<String>());
        }
    }

    fn print_path(&self, path: Vec<PathNode>) {
        let mut grid = self.grid.clone();
        for p in path {
            if grid[p.y as usize][p.x as usize] != Object::Empty {
                continue;
            }
            grid[p.y as usize][p.x as usize] = match p.direction {
                Some(val) => Object::Path(val),
                None => Object::Start,
            }
        }
        for ln in &grid {
            println!("{}", ln.iter().map(|m| match m {
                Object::Wall => '#',
                Object::Empty => '.',
                Object::Start => 'S',
                Object::End => 'E',
                Object::Path(v) => v.get_char(),
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
struct PathNode {
    x: i64,
    y: i64,
    direction: Option<Dir>,
}

impl PathNode {
    fn new(x: i64, y: i64, direction: Option<Dir>) -> Self {
        Self {x, y, direction}
    }
}


impl Display for PathNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Object {
    Wall,
    Empty,
    Start,
    End,
    Path(Dir),
}

impl Object {
    fn new(c: char) -> Self {
        match c {
            '.' => Object::Empty,
            '#' => Object::Wall,
            'S' => Object::Start,
            'E' => Object::End,
            v => Object::Path(Dir::new(v)),
        }
    }
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents, 100, 20);
    println!("Part 1 answer is {part1}");
    // 976645 is too low
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


fn read_contents(cont: &str, min_cheat: usize, cheat_time: usize) -> (usize, usize) {
    let map = read_map(cont);
    map.print_map();

    let mut path: Vec<PathNode> = Vec::new();

    let mut pos = PathNode::new(map.start.0, map.start.1, None);
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
            if let Some(dd) = &pos.direction  {
                if dd.opposite() == dir {
                    continue;
                }
            }
            if map.is_empty(x.0, x.1) {
                // println!("At {}, {}, going {}", x.0, x.1, dir);
                pos = PathNode::new(x.0, x.1, Some(dir));
                path.push(pos);
            }
        }
    }
    println!("Found path with {} steps ({} picoseconds)", path.len(), path.len() - 1);
    map.print_path(path.clone());
    let part1 = path.iter().enumerate().map(|(i, x)|
        {
            path.iter().skip(i + 2 + min_cheat).filter(|y| {
                taxicab(x,y) == 2
            }).count()
        }).sum();

    let part2 = path.iter().enumerate().map(|(i, x)|
        {
            path.iter().enumerate().skip(i).filter(|(j,y)| {
                let shortcut_distance = taxicab(x, y);
                let path_distance = j - i;
                if shortcut_distance > cheat_time as i64 {
                    // Shortcut is too long
                    return false;
                }
                let saved_distance = path_distance - shortcut_distance as usize;
                if saved_distance < min_cheat {
                    // Cheat is not good enough
                    return false
                }
                //println!("Found cheat with {} steps from {} to {}, saving {} steps", shortcut_distance, x, y, saved_distance);
                true
            }).count()
        }).sum();

    (part1, part2)
}

fn taxicab(x1: &PathNode, x2: &PathNode) -> i64{
    i64::abs(x1.x-x2.x) + i64::abs(x1.y-x2.y)
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

        assert_eq!(read_contents(&a, 40, 20).0, 2);

        //There are 3 cheats that save 76 picoseconds.
        assert_eq!(read_contents(&a, 76, 20).1, 3);

        //There are 4 cheats that save 74 picoseconds.
        assert_eq!(read_contents(&a, 74, 20).1, 7);
        
        //There are 22 cheats that save 72 picoseconds.
        assert_eq!(read_contents(&a, 72, 20).1, 7 + 22);

        //There are 12 cheats that save 70 picoseconds.
        assert_eq!(read_contents(&a, 70, 20).1, 7 + 22 + 12);
    }
}
