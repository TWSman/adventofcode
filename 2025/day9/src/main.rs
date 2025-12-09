use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use itertools::{Itertools, Position};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}



#[derive(Debug, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64
}

impl Coord {
    fn from_str(input: &str) -> Self {
        let res: Vec<&str> = input.split(',').collect();
        assert_eq!(res.len(), 2);
        let x = res[0].parse::<i64>().unwrap();
        let y = res[1].parse::<i64>().unwrap();
        Self {
            x,
            y,
        }
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let tiles: Vec<Coord> = cont.lines().map(Coord::from_str).collect();
    dbg!(&tiles.len());

    let part1 = get_part1(&tiles);
    let part2 = get_part2(&tiles);

    let x_max = tiles.iter().map(|x| x.x).max().unwrap();
    let x_min = tiles.iter().map(|x| x.x).min().unwrap();

    let y_max = tiles.iter().map(|x| x.y).max().unwrap();
    let y_min = tiles.iter().map(|x| x.y).min().unwrap();
    dbg!(x_max, x_min, y_max, y_min);


    (part1, part2)
}

fn get_part2(tiles: &Vec<Coord>) -> i64 {
    let mut grid: BTreeSet<(i64,i64)> = BTreeSet::new();
    let n_tiles = tiles.len();
    for i in 0..n_tiles {
        let (a, b);
        if i == n_tiles -1 {
            a = tiles[i];
            b = tiles[0];
        } else {
            a = tiles[i];
            b = tiles[i+1];
        }
        if a.x == b.x {
            let start = a.y.min(b.y);
            let end = a.y.max(b.y);
            for y in start..=end {
                grid.insert((a.x, y));
            }
        }
        else if a.y == b.y {
            let start = a.x.min(b.x);
            let end = a.x.max(b.x);
            for x in start..=end {
                grid.insert((x, a.y));
            }
        } else {
            panic!("Should not happen");
        }
    }
    dbg!(&grid.len());
    dbg!(&grid);

    // TODO MISSING The inside of the loop
    let mut current_max: i64 = 0;
    for (i,a) in tiles.iter().enumerate() {
        for (j, b) in tiles.iter().enumerate() {
            let candidate = ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1);
            if candidate < current_max {
                continue;
            }
            println!("Testing ({}, {}) and  ({}, {})", a.x, a.y, b.x, b.y);
            dbg!(&candidate);
            let start_x = a.x.min(b.x);
            let start_y = a.y.min(b.y);

            let end_x = a.x.max(b.x);
            let end_y = a.y.max(b.y);
            let mut found = false;
            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    if !grid.contains(&(x,y)) {
                        println!("{x}, {y} is not valid");
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if !found {
                dbg!(&candidate);
                current_max = candidate;
            }
        }
    }
    current_max
}

fn get_part1(tiles: &Vec<Coord>) -> i64 {
    tiles.iter().enumerate().map(|(i,a)| {
        tiles.iter().enumerate().map(|(j,b)| {
            ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
        }).max().unwrap_or(0)
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a="7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(read_contents(&a).0, 50);
        assert_eq!(read_contents(&a).1, 24);
    }

}

