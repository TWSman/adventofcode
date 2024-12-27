use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use strum::IntoEnumIterator; // 0.17.1
use shared::Dir;
use priority_queue::PriorityQueue;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    height: u32,
    directions: Vec<Dir>,
    t: PointType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PointType {
    Start,
    End,
    Normal,
}

impl Point {
    const fn new(x: i64, y: i64, height: u32, t: PointType) -> Self {
        Self {x, y, height, directions: Vec::new(), t}
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct PathHead {
    x: i64,
    y: i64,
    price: i64,
}

impl PathHead {
    fn new(x: i64,
        y: i64,
        price: i64,
    ) -> Self {
        Self {x,
            y,
            price,
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

fn get_height(c: char) -> u32 {
    c as u32 - 'a' as u32
}

fn read_map(cont: &str) -> BTreeMap<(i64, i64), Point> {
    let mut points: BTreeMap<(i64, i64), Point> = cont.lines().enumerate().flat_map(|(i, ln)| {
            let y = i64::try_from(i).unwrap();
            ln.chars().enumerate().map(move |(j, c)| {
                let x = i64::try_from(j).unwrap();
                let (t, h) = match c {
                    'S' => (PointType::Start, get_height('a')),
                    'E' => (PointType::End, get_height('z')),
                    val => (PointType::Normal, get_height(val)),
                };
                ((x, y), Point::new(x, y, h, t))
            })
        }).collect::<BTreeMap<(i64, i64), Point>>();

    let points_snapshot = points.clone();
    for p in points.values_mut() {
        let x = p.x;
        let y = p.y;
        let h = p.height;
        for dir in Dir::iter() {
            let d = dir.get_dir();
            match points_snapshot.get(&(x + d.0, y + d.1)) {
                Some(val) if val.height <= h + 1 => {
                    p.directions.push(dir);
                }
                _ => continue,
            }
        }
    }
    points
}

fn read_contents(cont: &str) -> (i64, i64) {
    let map = read_map(cont);

    (get_part1(&map), get_part2(&map))
}

// Find number of steps from the defined start point to the defined target
fn get_part1(map: &BTreeMap<(i64, i64), Point>) -> i64 {
    let start = map.iter().find_map(|((_x,_y),p)|  {
        if p.t == PointType::Start {
            Some(p)
        } else {
            None
        }
    }).unwrap();
    let target = map.iter().find_map(|((_x,_y),p)| {
        if p.t == PointType::End {
            Some(p)
        } else {
            None}
    }).unwrap();
    get_route(map, start, target)
}

// Find number of steps from any point at height 0 ('a') to the target. Return the smallest
fn get_part2(map: &BTreeMap<(i64, i64), Point>) -> i64 {
    let starts = map.iter().filter_map(|((_x,_y),p)|  {
        if p.height == 0 {
            Some(p)
        } else {
            None
        }
    }).collect::<Vec<&Point>>();
    let target = map.iter().find_map(|((_x,_y),p)| {
        if p.t == PointType::End {
            Some(p)
        } else {
            None}
    }).unwrap();
    let mut bestscore = 9999;
    for s in starts {
        let r = get_route(map, s, target);
        if r < bestscore {
            bestscore = r;
        }
    }
    bestscore
}

// Finds number of steps between start and target
fn get_route(map: &BTreeMap<(i64, i64), Point>, start: &Point, target: &Point) -> i64 {
    let mut visited: BTreeSet<(i64,i64)> = BTreeSet::new();
    let mut paths = PriorityQueue::new();
    let mut already_found: BTreeMap<(i64, i64), i64> = BTreeMap::new();
    already_found.insert((start.x, start.y), 0);
    let _ = &paths.push(PathHead::new(start.x, start.y, 0), 0);
    loop {
        let (path, _priority) = match paths.pop() {
            None => {return 9999;},
            Some(p) => p,
        };
        if (path.x == target.x) & (path.y == target.y) {
            // We found the end
            return path.price;
        }
        let point = map.get(&(path.x, path.y)).unwrap();
        for dir in &point.directions {
            let d = dir.get_dir();
            let (x,y) = (path.x + d.0, path.y + d.1);
            if visited.contains(&(x,y)) {
                continue;
            }
            let new_path = PathHead::new(x, y, &path.price + 1);
            visited.insert((x,y));
            let p = -new_path.price;
            paths.push(new_path, p);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(read_contents(&a).0, 31);
        assert_eq!(read_contents(&a).1, 29);
    }

}
