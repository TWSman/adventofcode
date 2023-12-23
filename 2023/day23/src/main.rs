#[macro_use]
extern crate num_derive;

use clap::Parser;
use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::cmp::max;
use std::fmt::Display;
use core::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use num_traits::FromPrimitive;

#[allow(clippy::cast_possible_truncation)]
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
            Self::North => write!(f, "^"),
            Self::South => write!(f, "v"),
            Self::West => write!(f, "<"),
            Self::East => write!(f, ">"),
        }
    }
}

impl Direction {
    const fn get_dx(self) -> (i64,i64) {
        match self {
            Self::East => ( 1,  0),
            Self::West => (-1,  0),
            Self::North => (0, -1),
            Self::South => (0, 1),
        }
    }
    fn opposite(self) -> Self {
        FromPrimitive::from_u8((self as u8 + 2) % 4).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Vec2D {
    x: i64,
    y: i64,
}

impl Vec2D {
    fn add(&mut self, x: i64, y:i64) {
        self.x += x;
        self.y += y;
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{}, {}", self.x, self.y)
    }
}

impl PartialOrd for Vec2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x < other.x {
            Some(Ordering::Less)
        } else if self.x > other.x {
            Some(Ordering::Greater)
        } else if self.y < other.y {
            Some(Ordering::Less)
        } else if self.y > other.y {
            return Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[derive(Debug, Clone)]
struct PathHead {
    position: Vec2D,
    intersection: Vec2D,
    distance: i64,
    direction: Direction,
}


impl PathHead {
    const fn new(position: Vec2D, direction: Direction) -> Self {
        Self {
            position,
            intersection: position,
            distance: 0,
            direction,
        }
    }

    fn mmove(&mut self, direction: Direction) {
        let (dx, dy) = direction.get_dx(); 
        self.position.add(dx,dy);
        self.direction = direction;
        self.distance += 1;
    }
}


fn print(blocks: &HashMap<Vec2D, Path>, intersections: &HashMap<Vec2D, Intersection>, n_rows: i64, n_cols: i64) -> String {
    let mut str = (0..n_rows).map(|x| {
        let x = (0..n_cols).map(|y| {
            let coord = Vec2D {x, y};
            blocks.get(&coord).map_or_else(|| panic!("Could not find marker at {x} {x}"), |b| match b.path_type {
                PathType::Forest => "#".to_string(),
                PathType::Path => ".".to_string(),
                PathType::Slope(Direction::North) => "^".to_string(),
                PathType::Slope(Direction::South) => "v".to_string(),
                PathType::Slope(Direction::East) => ">".to_string(),
                PathType::Slope(Direction::West) => "<".to_string(),
            })
            }).collect::<String>();
        x
    }).collect::<Vec<_>>().join("\n");
    for pos in intersections.keys() {
        assert!((pos.x != n_cols) & (pos.y != n_rows));
        let i = usize::try_from((1+n_cols) * pos.y + pos.x).unwrap();
        str.replace_range(i..=i, "I");
    }
    str
}


fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");

    let res = read_contents(&contents, false);
    println!("Part 1 answer is {res}");

    let res = read_contents(&contents, true);
    println!("Part 2 answer is {res}");

}

#[derive(Debug, Eq, PartialEq, Clone)]
enum PathType {
    Path,
    Slope(Direction),
    Forest,
}

impl PathType {
    fn new(c: char) ->Self{
        match c {
            '#' => Self::Forest,
            '.' => Self::Path,
            '>' => Self::Slope(Direction::East),
            '<' => Self::Slope(Direction::West),
            'v' => Self::Slope(Direction::South),
            '^' => Self::Slope(Direction::North),
            v => panic!("Unknown character {v}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Intersection {
    targets: Vec<(Vec2D, i64)>,
}

impl Intersection {
    const fn new() -> Self {
        Self {targets: Vec::new()}
    }
}

#[derive(Debug)]
struct Path {
    path_type: PathType,
    visited: bool,
}

impl Path {
    const fn new(path_type: PathType) -> Self {
        Self {path_type, visited: false}
    }
}


fn get_paths(cont: &str) -> (Vec2D, i64,i64, HashMap<Vec2D, Path>) {
    let line_width = cont.lines().next().expect("Should be at least 1 line").len() + 1;
    let mut paths: HashMap<Vec2D, Path> = HashMap::new();

    let mut max_y = 0;
    let mut start: Option<Vec2D> = None;
    for (i,c) in cont.chars().enumerate() {
        let y = i/ line_width;
        max_y = max(y, max_y);
        match c {
            '\n' | ' ' => { continue; },
            c => {
                let x = i % line_width;
                let path_type = PathType::new(c);
                let mut path = Path::new(path_type.clone());
                let coord = Vec2D {x: i64::try_from(x).unwrap(), y: i64::try_from(y).unwrap()};
                if (y == 0) & (path.path_type == PathType::Path) & start.is_none() {
                    start = Some(coord);
                    path.visited = true;
                }
                paths.insert(coord, path);
            },
        }
    }
    (start.unwrap(), i64::try_from(line_width - 1).unwrap(),
        i64::try_from(max_y + 1).unwrap(),
        paths)
}

fn read_contents(cont: &str, part2: bool) -> i64 {
    let (start, n_cols, n_rows, mut blocks) = get_paths(cont);
    let mut paths: Vec<PathHead> = Vec::new();
    let mut intersections: HashMap<Vec2D, Intersection> = HashMap::new();
    let first_intersection = Intersection::new();
    intersections.insert(start, first_intersection);
    paths.push(PathHead::new(
        start,
        Direction::South));
    let mut connections: Vec<(Vec2D, Vec2D, i64)> = Vec::new();
    let mut target: Option<Vec2D> = None;
    loop {
        let mut p = match paths.pop() {
            None => break,
            Some(v) => v,
        };
        loop {
            match blocks.get_mut(&p.position) {
                Some(b) => {
                    b.visited = true;
                }
                None => panic!(""),
            }
            let mut new_paths: Vec<(i64,i64, Direction)> = Vec::new();
            let mut new_connections = 0;
            for direction in Direction::iter() {
                if direction == p.direction.opposite() {
                    continue;
                }
                let (dx, dy) = direction.get_dx();
                let x = dx + p.position.x;
                let y = dy + p.position.y;
                let new_pos = Vec2D {x,y};
                match blocks.get_mut(&new_pos) {
                    None => {
                        continue;
                    }
                    Some(b) if b.visited => {
                        new_connections += 1;
                        connections.push((p.intersection, new_pos, p.distance + 1));
                        if intersections.get(&new_pos).is_none() {
                            intersections.insert(new_pos, Intersection::new());
                        }
                    },
                    Some(b) => {
                        match b.path_type {
                            PathType::Forest => {
                                continue;
                            }
                            PathType::Slope(dir) if dir == direction.opposite() => {
                                new_connections += 1;
                                continue;
                            }
                            _ => {
                                new_connections += 1;
                                if y == n_rows - 1 {
                                    target = Some(new_pos);
                                    intersections.insert(new_pos, Intersection::new());
                                    connections.push((p.intersection, new_pos, p.distance + 1));
                                } else {
                                    new_paths.push((x,y, direction));
                                }
                            }
                        }
                    }
                }
            }
            if new_connections > 1 {
                connections.push((p.intersection, p.position, p.distance));
                if intersections.get(&p.position).is_none() {
                        let tmp = Intersection::new();
                        intersections.insert(
                            p.position,
                            tmp);
                };

                for (_x, _y, dir) in &new_paths {
                    let pos = Vec2D {x:p.position.x, y:p.position.y};
                    let mut new_head = PathHead::new(pos, *dir);
                    new_head.mmove(*dir);
                    paths.push(
                        new_head,
                    );
                }
                break;
            }
            match new_paths.first()  {
                None => break,
                Some((_,_,dir)) => p.mmove(*dir),
            }
        }
    }
    for (start, end, distance) in connections {
        let intersection = intersections.get_mut(&start).unwrap();
        intersection.targets.push((end,distance));
        if part2 {
            let intersection = intersections.get_mut(&end).unwrap();
            intersection.targets.push((start,distance));
        }
    }

    println!("{} intersections", intersections.len());
    println!("\n{}", print(&blocks, &intersections, n_rows, n_cols));
    let first_path = vec![start];
    let paths: Vec<(Vec<Vec2D>, i64)> = get_iteration(&first_path,
        0,
        &target.unwrap(),
        &intersections);
    *paths.iter().map(|(_v,d)| d).max().unwrap()
}

fn get_iteration(current_path: &[Vec2D],
    current_distance: i64,
    target: &Vec2D,
    intersections: &HashMap<Vec2D, Intersection>) -> Vec<(Vec<Vec2D>, i64)> {

    let current_pos = current_path.last().unwrap();
    let current_int = intersections.get(current_pos).unwrap();
    let mut new_paths: Vec<(Vec<Vec2D>, i64)> = Vec::new();
    for (t, dist) in &current_int.targets {
        if current_path.contains(t) {
            continue;
        }
        let new_distance = current_distance + dist;
        let mut new_path = current_path.to_owned();
        new_path.push(*t);
        if t == target {
            new_paths.push((new_path, new_distance));
            return new_paths;
        }
        let complete_paths = get_iteration(&new_path, new_distance, target, intersections);
        for complete_path in complete_paths {
            new_paths.push(complete_path);
        }
    }
    new_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(read_contents(&a, false), 94);
        assert_eq!(read_contents(&a, true), 154);
    }
}
