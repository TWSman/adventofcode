#[macro_use]
extern crate num_derive;

use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::cmp::max;
use std::fmt::Display;
use core::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use num_traits::FromPrimitive;

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
    fn opposite(self) -> Direction {
        FromPrimitive::from_u8((self as u8 + 2) % 4).unwrap()
    }

    fn cw(self) -> Direction {
        FromPrimitive::from_u8((self as u8 + 1) % 4).unwrap()
    }

    fn ccw(self) -> Direction {
        FromPrimitive::from_u8((self as u8 + 3) % 4).unwrap()
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
            return Some(Ordering::Less);
        }
        if self.x > other.x {
            return Some(Ordering::Greater);
        }
        if self.y < other.y {
            return Some(Ordering::Less);
        }
        if self.y > other.y {
            return Some(Ordering::Greater);
        }
        return Some(Ordering::Equal);
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
    fn new(position: Vec2D, direction: Direction) -> PathHead {
        PathHead {
            position,
            intersection: position.clone(),
            distance: 0,
            direction,
        }
    }

    fn mmove(&mut self, direction: &Direction) {
        let (dx, dy) = direction.get_dx(); 
        self.position.add(dx,dy);
        self.direction = *direction;
        self.distance += 1;
    }
}


fn print(blocks: &BTreeMap<(i64,i64), Path>, intersections: &HashMap<Vec2D, Intersection>, n_rows: i64, n_cols: i64) -> String {
    let mut str = (0..n_rows).map(|i_row| {
        let x = (0..n_cols).map(|i_col| {
            match blocks.get(&(i_col, i_row)) {
                None => panic!("Could not find marker at {} {}", i_col, i_row),
                //Some(b) if b.visited => ",".to_string(),
                Some(b) => {
                    match b.path_type {
                        PathType::Forest => "#".to_string(),
                        PathType::Path => ".".to_string(),
                        PathType::Slope(Direction::North) => "^".to_string(),
                        PathType::Slope(Direction::South) => "v".to_string(),
                        PathType::Slope(Direction::East) => ">".to_string(),
                        PathType::Slope(Direction::West) => "<".to_string(),
                    }
                }
            }
        }).collect::<Vec<_>>().join("");
        x
    }).collect::<Vec<_>>().join("\n");
    for pos in intersections.keys() {
        if (pos.x == n_cols) | (pos.y == n_rows) {
            panic!();
        }
        let i = ((1+n_cols) * pos.y + pos.x) as usize;
        str.replace_range(i..(i+1), "I");
    }
    str
}


fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");

    // 0 cycles means just one tilt to north (part1)
    let res = part1(&contents);
    println!("Part 1 answer is {}", res);

    //let res = part2(&contents, 26_501_365);
    //println!("Part 2 answer is {}", res);

}

#[derive(Debug, Eq, PartialEq, Clone)]
enum PathType {
    Path,
    Slope(Direction),
    Forest,
}

impl PathType {
    fn new(c: char) ->PathType{
        match c {
            '#' => PathType::Forest,
            '.' => PathType::Path,
            '>' => PathType::Slope(Direction::East),
            '<' => PathType::Slope(Direction::West),
            'v' => PathType::Slope(Direction::South),
            '^' => PathType::Slope(Direction::North),
            v => panic!("Unknown character {}", v),
        }
    }
}

#[derive(Debug, Clone)]
struct Intersection {
    targets: Vec<(Vec2D, i64)>,
}

impl Intersection {
    fn new() -> Intersection {
        Intersection {targets: Vec::new()}
    }
}

#[derive(Debug)]
struct Path {
    path_type: PathType,
    visited: bool,
}

impl Path {
    fn new(path_type: PathType) -> Path {
        Path {path_type: path_type, visited: false}
    }
}


fn get_paths(cont: &str) -> (Vec2D, i64,i64, BTreeMap<(i64,i64), Path>) {
    let line_width = cont.lines().next().expect("Should be at least 1 line").len() as i64 + 1;
    let mut paths: BTreeMap<(i64,i64), Path> = BTreeMap::new();

    let mut max_y = 0;
    let mut start: Option<Vec2D> = None;
    for (i,c) in cont.chars().enumerate() {
        let y = (i as i64) / line_width;
        max_y = max(y, max_y);
        match c {
            '\n' | ' ' => { continue; },
            c => {
                let x = (i as i64) % line_width;
                let path_type = PathType::new(c);
                let mut path = Path::new(path_type.clone());
                if (y == 0) & (path.path_type == PathType::Path) & start.is_none() {
                    start = Some(Vec2D {x: x, y:y});
                    path.visited = true;
                }
                paths.insert((x,y), path);
            },
        }
    }
    (start.unwrap(), line_width - 1, max_y + 1, paths)
}

fn part1(cont: &str) -> i64 {
    let (start, n_cols, n_rows, mut blocks) = get_paths(cont);
    let mut paths: Vec<PathHead> = Vec::new();
    let mut intersections: HashMap<Vec2D, Intersection> = HashMap::new();
    let first_intersection = Intersection::new();
    intersections.insert(start.clone(), first_intersection);
    //println!("\n{}", print(&blocks, &intersections, n_rows, n_cols));
    paths.push(PathHead::new(
        start.clone(),
        Direction::South));
    let mut connections: Vec<(Vec2D, Vec2D, i64)> = Vec::new();
    let mut target: Option<Vec2D> = None;
    loop {
        let mut p = match paths.pop() {
            None => break,
            Some(v) => v,
        };
        loop {
            //println!("\nHeading {}", p.direction);
            match blocks.get_mut(&(p.position.x,p.position.y)) {
                Some(b) => {
                    b.visited = true;
                }
                None => panic!(""),
            }
            let mut new_paths: Vec<(i64,i64, Direction)> = Vec::new();
            let mut cons = 0;
            for direction in Direction::iter() {
                //println!("Trying {}", direction);
                if direction == p.direction.opposite() {
                    //println!("Cant U turn to {}", direction);
                    continue;
                }
                let (dx, dy) = direction.get_dx();
                let x = dx + p.position.x;
                let y = dy + p.position.y;
                let new_pos = Vec2D {x,y};
                match blocks.get_mut(&(x,y)) {
                    None => {
                        //println!("Cant go {}, out of field", direction);
                        continue;
                    }
                    Some(b) if b.visited => {
                        cons += 1;
                        connections.push((p.intersection.clone(), new_pos.clone(), p.distance + 1));
                        match intersections.get(&new_pos) {
                            None => {
                                //println!("Already visited, add new intersection {}, {}", x,y);
                                intersections.insert(new_pos.clone(), Intersection::new());
                            }
                            Some(_) => {
                                //println!("Already visited, increment intersection");
                            },
                        }
                    },
                    Some(b) => {
                        match b.path_type {
                            PathType::Forest => {
                                //println!("Cant go {}, Forest", direction);
                                continue;
                            }
                            PathType::Slope(dir) if dir == direction.opposite() => {
                                //println!("Cant go {}, upslope", direction);
                                cons += 1;
                                continue;
                            }
                            _ => {
                                //println!("New path {}", direction);
                                cons += 1;
                                if y == n_rows - 1 {
                                    //println!("Found the end");
                                    target = Some(new_pos.clone());
                                    intersections.insert(new_pos.clone(), Intersection::new());
                                    connections.push((p.intersection.clone(), new_pos.clone(), p.distance + 1));
                                } else {
                                    new_paths.push((x,y, direction));
                                }
                            }
                        }
                    }
                }
            }
            if cons > 1 {
                //start_intersection.targets.push(((p.position.x, p.position.y), p.distance));
                connections.push((p.intersection.clone(), p.position.clone(), p.distance));
                //println!("More than 1 path");
                match intersections.get(&p.position) {
                    None => {
                        //println!("Intersection {},{} does not exist", p.position.x, p.position.y);
                        let tmp = Intersection::new();
                        intersections.insert(
                            p.position.clone(),
                            tmp);
                    },
                    Some(inter) => {
                        //println!("Intersection exists");
                        //start_intersection.targets.push( ((p.position.x, p.position.y), p.distance));
                    },
                };

                for (x,y,dir) in &new_paths {
                    let pos = Vec2D {x:p.position.x, y:p.position.y};
                    let mut tmp = PathHead::new(pos, *dir);
                    tmp.mmove(dir);
                    paths.push(
                        tmp,
                    );
                }
                //println!("\n{}", print(&blocks, &intersections, n_rows, n_cols));
                break;
            }
            //println!("\n{}", print(&blocks, &intersections, n_rows, n_cols));
            match new_paths.first()  {
                None => break,
                Some((_,_,dir)) => p.mmove(dir),
            }
        }
    }
    for (start, end, distance) in connections {
        let intersection = intersections.get_mut(&start).unwrap();
        intersection.targets.push((end,distance));
    }
    //dbg!(&intersections);
    println!("\n{}", print(&blocks, &intersections, n_rows, n_cols));
    dbg!(&intersections.len());
    for (loc, intersection) in &intersections {
        println!("From {}", loc);
        for (t,dist) in &intersection.targets {
            println!("\tTo {}, {} steps", t, dist);
        }
    }
    let first_path = vec![(start.clone(),0)];
    let paths: Vec<Vec<(Vec2D, i64)>> = get_iteration(first_path,
        &target.unwrap(),
        &intersections);
    //dbg!(&paths);
    let distances = paths.iter().map(|v| {
        v.iter().map(|(_,d)| d).sum::<i64>()
    }).collect::<Vec<_>>();

    for i in 0..paths.len() {
        let p = paths.get(i).unwrap();
        let d = distances[i];
        println!("\nPath {}, total distance {}", i, d);
        for (x,dist) in p {
            println!("\tTo {}, {} steps", x, dist);
        }
    }
    dbg!(&distances);
    *distances.iter().max().unwrap()
    //blocks.iter().filter(|((x,y), b)| {
    //    let visited = b.visited.contains_key(&(0,0));
    //    visited & ((x + y) % 2 == 0)
    //}).count() as i64
}

fn get_iteration(current_path: Vec<(Vec2D, i64)>,
    target: &Vec2D,
    intersections: &HashMap<Vec2D, Intersection>) -> Vec<Vec<(Vec2D, i64)>> {

    let current_pos = current_path.last().unwrap().0;
    let current_int = intersections.get(&current_pos).unwrap();
    let mut new_paths: Vec<Vec<(Vec2D, i64)>> = Vec::new();
    for (t, dist) in current_int.targets.iter() {
        let mut new_path = current_path.clone();
        new_path.push((t.clone(), dist.clone()));
        if t == target {
            new_paths.push(new_path);
            return new_paths;
        }
        let complete_paths = get_iteration(new_path, target, intersections);
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
        assert_eq!(part1(&a), 94);
    }

}
