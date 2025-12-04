// Target: For each list of numbers define if its truly decreasing or increasing
// And check that successive differences are 1 or 2

use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::cmp::max;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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

fn read_contents(cont: &str) -> (i64, i64) {
    let mut grid: BTreeMap<(i32,i32), bool> = BTreeMap::new();
    // Need to add +1, because of newline characters
    let line_width = cont.lines().next().expect("Should be at least 1 line").len() as i32 + 1;
    let max_x = line_width - 2;
    dbg!(line_width);
    let mut max_y = 0;

    for (i,c) in cont.chars().enumerate() {
        let y = (i as i32) / line_width;
        max_y = max(y, max_y);
        let x = (i as i32) % line_width;
        match c {
            '\n' | ' ' => { continue; },
            '@' => {
                grid.insert((x,y), true);
            },
            '.' => {
                grid.insert((x,y), false);
            },
            _ => {
                panic!("Unexpected character");
            }
        }
    }
    //dbg!(&grid);

    let p1 = get_part1(&grid);
    let p2 = get_part2(&grid, max_x, max_y);
    (p1,p2)
}

const OPTS: [(i32,i32); 8] = [
    (1,1),
    (1,0),
    (1,-1),
    (0,1),
    (0,-1),
    (-1,1),
    (-1,0),
    (-1,-1),
];

fn get_part1(grid: &BTreeMap<(i32,i32), bool>) -> i64 {
    grid.iter().map(|((x,y), v)| {
        if *v {
            let s: i32 = OPTS.iter().map(|(xx,yy)| {
                match grid.get(&(x+xx, y+yy)) {
                    Some(true) => 1,
                    _ => 0,
                }
            }).sum();
            if s < 4 {
                println!("Position ({x}, {y}) can be accessed, sum was {s}");
                1
            } else {
                0
            }
        } else {
            0
        }
    }).sum()
}

fn get_part2(grid: &BTreeMap<(i32,i32), bool>, max_x: i32, max_y: i32) -> i64 {
    let mut grid = grid.clone();
    let mut removed = 0;
    loop {
        let mut changed = false;
        for x in 0..=max_x {
            for y in 0..=max_y {
                if *grid.get(&(x,y)).expect("Grid point should exist") {
                    let s: i32 = OPTS.iter().map(|(xx,yy)| {
                        match grid.get(&(x+xx, y+yy)) {
                            Some(true) => 1,
                            _ => 0,
                        }
                    }).sum();

                    if s < 4 {
                        println!("Removed ({x}, {y})");
                        changed = true;
                        removed += 1;
                        grid.insert((x,y), false);
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    removed
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(read_contents(&a).0, 13);
        assert_eq!(read_contents(&a).1, 43);
    }
}
