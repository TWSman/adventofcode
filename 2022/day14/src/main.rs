use clap::Parser;
use std::fs;
use std::collections::BTreeMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Stuff {
    Rock,
    Sand,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut grid: BTreeMap<(usize, usize), Stuff> = BTreeMap::new();
    let mut prev_x: Option<usize>;
    let mut prev_y: Option<usize>;
    let mut max_y = 0;
    for line in cont.lines() {
        prev_x = None;
        prev_y = None;
        for coord in line.split("->") {
            let (a,b) = coord.trim().split_once(',').unwrap();
            let new_x = a.parse::<usize>().unwrap();
            let new_y = b.parse::<usize>().unwrap();
            max_y = max_y.max(new_y);
            match (prev_x, prev_y) {
                (Some(x), Some(y)) => {
                    if new_x == x {
                        let start_y = new_y.min(y);
                        let end_y = new_y.max(y);
                        for yy in start_y..=end_y {
                            grid.insert((x, yy), Stuff::Rock);
                        }
                    }
                    else if new_y == y {
                        let start_x = new_x.min(x);
                        let end_x = new_x.max(x);
                        for xx in start_x..=end_x {
                            grid.insert((xx, y), Stuff::Rock);
                        }
                    } else {
                        panic!("X and Y Both differ");
                    }
                },
                (None, None) => {
                },
                _ => { panic!("Should not happen");},
            }
            prev_x = Some(new_x);
            prev_y = Some(new_y);
        }
    }
    let part1 = get_part1(grid.clone(), max_y);
    let part2 = get_part2(grid.clone(), max_y);
    (part1, part2)
}

fn get_part1(mut grid: BTreeMap<(usize, usize), Stuff>, max_y: usize) -> i64 {
    let mut sand_count = 0;
    let mut path: Vec<(usize, usize)> = Vec::new();
    loop {
        let (mut x, mut y);
        if path.is_empty() {
            (x, y) = (500, 0);
        } else {
            // Start from the last empty spot
            (x,y) = path.pop().unwrap();
        }
        loop {
            if y > max_y {
                break;
            }
            if !grid.contains_key(&(x, y+1)) {
                path.push((x,y));
                y += 1;
                continue;
            }
            if !grid.contains_key(&(x-1, y+1)) {
                // Sand moves down and left
                path.push((x,y));
                x -= 1;
                y += 1;
                continue;
            }
            if !grid.contains_key(&(x+1, y+1)) {
                // Sand moves down and right
                path.push((x,y));
                x += 1;
                y += 1;
                continue;
            }
            grid.insert((x,y), Stuff::Sand);
            sand_count += 1;
            break;
        }
        if y > max_y {
            break;
        }
    }
    sand_count
}


fn get_part2(mut grid: BTreeMap<(usize, usize), Stuff>, max_y: usize) -> i64 {
    let mut sand_count = 0;
    let mut path: Vec<(usize, usize)> = Vec::new();
    loop {
        let (mut x, mut y);
        if path.is_empty() {
            (x, y) = (500, 0);
        } else {
            // Start from the last empty spot
            (x,y) = path.pop().unwrap();
        }
        loop {
            if y == max_y + 1 {
                // We are at floor - 1 (floor is max_y+2)
                grid.insert((x,y), Stuff::Sand);
                sand_count += 1;
                break;
            }
            if !grid.contains_key(&(x, y+1)) {
                path.push((x,y));
                y += 1;
                continue;
            }
            if !grid.contains_key(&(x-1, y+1)) {
                // Sand moves down and left
                path.push((x,y));
                x -= 1;
                y += 1;
                continue;
            }
            if !grid.contains_key(&(x+1, y+1)) {
                // Sand moves down and right
                path.push((x,y));
                x += 1;
                y += 1;
                continue;
            }
            if y == 0 {
                break;
            }
            grid.insert((x,y), Stuff::Sand);
            sand_count += 1;
            break;
        }
        if y == 0 {
            sand_count += 1;
            break;
        }
        assert!(y <= max_y +1, "Something happened");
    }
    sand_count
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a ="498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(read_contents(&a).0, 24);
    }

    #[test]
    fn part2() {
        let a ="498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(read_contents(&a).1, 93);
    }
}
