use clap::Parser;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use priority_queue::PriorityQueue;
use std::fmt::Display;
use core::fmt;
use shared::Dir;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
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

    fn print_path(&self, path: Vec<(i64,i64, Dir)>) {
        let mut grid = self.grid.clone();
        for (x,y,_) in path {
            grid[y as usize][x as usize] = Object::Start;
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


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct PathHead {
    x: i64,
    y: i64,
    price: i64,
    direction: Dir,
    history: Vec<(i64,i64, Dir)>,
}

impl PathHead {
    fn new(x: i64,
        y: i64,
        price: i64,
        direction: Dir,
        history: Vec<(i64,i64, Dir)>
    ) -> Self {
        Self {x,
            y,
            price,
            direction,
            history,
        }
    }
}

impl Display for PathHead {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Price: {}, at ({}, {}) Going {}", self.price, self.x, self.y, self.direction)
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
    let (part1, part2) = read_contents(&contents);
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


fn read_contents(cont: &str) -> (i64, i64) {
    let map = read_map(cont);
    map.print_map();
    let mut paths = PriorityQueue::new();
    let mut already_found: HashMap<(i64, i64, Dir), i64> = HashMap::new();

    // Reindeer start by facing East
    let start = PathHead::new(map.start.0, map.start.1, 0, Dir::E, Vec::new());
    already_found.insert((start.x, start.y, start.direction), 0);

    let _ = &paths.push(start, 0);

    let target = map.end;


    // Keep track of the best price
    let mut bestprice: Option<i64> = None;
    // Keep track of equally good paths
    let mut winning_paths: Vec<PathHead> = Vec::new();
    loop {
        let (path, _priority) = match paths.pop() {
            None => {panic!("No pathheads");},
            Some(p) => p,
        };
        if let Some(val) = bestprice {
            if path.price > val {
                // This path is worse than the best one,
                // Also any subsequent paths will be worse
                break;
            }
        }

        if (path.x == target.0) & (path.y == target.1) {
            // We found the end
            bestprice = Some(path.price);
            winning_paths.push(path.clone());
        }

        let d = path.direction;

        // Three possible moves, straight, left and right
        let straight = (
            path.x + d.get_dir().0,
            path.y + d.get_dir().1,
            // Going straight costs only 1 point
            path.price + 1,
            d,
            );

        let left = (
            path.x + d.ccw().get_dir().0,
            path.y + d.ccw().get_dir().1,
            // Turning costs 1001 (1000 for turning, 1 for moving)
            path.price + 1001,
            d.ccw(),
        );

        let right = (
            path.x + d.cw().get_dir().0,
            path.y + d.cw().get_dir().1,
            // Turning costs 1001 (1000 for turning, 1 for moving)
            path.price + 1001,
            d.cw(),
        );

        for x in [straight, left, right].iter_mut() {
            if map.is_empty(x.0, x.1) {
                let mut new_vec = path.history.clone();
                new_vec.push((x.0, x.1, x.3));
                let path = PathHead::new(x.0, x.1, x.2, x.3, new_vec);
                let key = (x.0, x.1, x.3);
                match already_found.get(&key) {
                    Some(v) if v < &x.2 => (),
                    _ => {
                        paths.push(path, -x.2);
                        already_found.insert(key, x.2);
                    },
                }
            }
        }
    }
    let mut visited_positions: HashSet<(i64,i64)> = HashSet::new();
    visited_positions.insert((map.start.0, map.start.1));
    for p in &winning_paths {
        for x in &p.history {
            visited_positions.insert((x.0, x.1));
        }
    }
    map.print_path(winning_paths[0].history.clone());
    println!("Found {} paths", winning_paths.len());
    (bestprice.unwrap(), visited_positions.len() as i64)
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        let b = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        assert_eq!(read_contents(&a).0, 7036);
        assert_eq!(read_contents(&b).0, 11048);
        //assert_eq!(read_contents(&a).1, 81);
    }

}
