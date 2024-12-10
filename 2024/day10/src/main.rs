use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Debug, Clone, Copy, EnumIter)]
enum Dir {
    N,
    E,
    S,
    W,
}


impl Dir{
    const fn get_dir(self) -> (i64, i64) {
        match self {
            Self::N => (0, 1),
            Self::E => (1, 0),
            Self::S => (0, -1),
            Self::W => (-1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    height: u8,
    directions: Vec<Dir>,
}

impl Point {
    const fn new(x: i64, y: i64, height: u8) -> Self {
        Self {x, y, height, directions: Vec::new()}
    }
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn read_map(cont: &str) -> BTreeMap<(i64, i64), Point> {
    let mut points: BTreeMap<(i64, i64), Point> = cont.lines().enumerate().flat_map(|(i, ln)| {
            let y = -i64::try_from(i).unwrap();
            ln.chars().enumerate().map(move |(j, c)| {
                let x = i64::try_from(j).unwrap();
                let h = c.to_digit(10).unwrap();
                ((x, y), Point::new(x, y, u8::try_from(h).unwrap()))
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
                Some(val) if val.height == h + 1 => {
                    p.directions.push(dir);
                }
                _ => continue,
            }
        }
    }
    points
}

fn read_contents(cont: &str) -> (u64, u64) {
    let map = read_map(cont);

    (get_part1(&map), get_part2(&map))
}

fn get_part1(map: &BTreeMap<(i64, i64), Point>) -> u64 {
    let trailheads = map.iter().filter(|((_x, _y), p)| {p.height == 0}).collect::<BTreeMap<&(i64, i64), &Point>>();
    trailheads.iter().map(|((_x, _y),p)| {
        let mut visited: BTreeSet<(i64,i64)> = BTreeSet::new();
        let mut heads: Vec<&Point> = vec![p];
        loop {
            match heads.pop() {
                None => {
                    // Stop if there are no more heads
                    break;
                },
                Some(head) => {
                    for dir in &head.directions  {
                        let (dx,dy) = dir.get_dir();
                        if visited.get(&(head.x  + dx, head.y + dy)).is_some() {
                            // If we have already visited this head, we can stop
                            continue;
                        }
                        // If this is a new place add it to the list of heads
                        let new_head = map.get(&(head.x  + dx, head.y + dy)).unwrap();
                        visited.insert((new_head.x, new_head.y));
                            heads.push(new_head);
                    }
                }
            }
        }
        
        visited.iter().filter(|i| map.get(i).unwrap().height == 9).count() as u64
    }).sum()
}

fn get_part2(map: &BTreeMap<(i64, i64), Point>) -> u64 {
    let trailheads = map.iter().filter(|((_x, _y), p)| {p.height == 0}).collect::<BTreeMap<&(i64, i64), &Point>>();
    trailheads.iter().map(|((_x, _y),p)| {
        let mut heads: Vec<&Point> = vec![p];
        let mut count = 0;
        loop {
            match heads.pop() {
                None => {
                    // Stop if there are no more heads
                    break;
                },
                Some(head) => {
                    for dir in &head.directions  {
                        let (dx,dy) = dir.get_dir();
                        let new_head = map.get(&(head.x  + dx, head.y + dy)).unwrap();
                        if new_head.height == 9 {
                            count += 1;
                        } else {
                            heads.push(new_head);
                        }
                    }
                }
            }
        }
        count
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        // Take the start of main puzzle input
        let a = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(read_contents(&a).0, 36);
        assert_eq!(read_contents(&a).1, 81);
    }

}
