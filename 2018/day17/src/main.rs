use clap::Parser;
use colored::Colorize;
use shared::Vec2D;
use std::collections::BTreeMap;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Stuff {
    Clay,
    WaterStill,
    WaterFlow,
    Spring,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let res = read_contents(&contents);
    // 30387 is too high
    // 30300 is too low
    // 30380 is wrong
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut grid: BTreeMap<Vec2D, Stuff> = BTreeMap::new();
    for line in cont.lines() {
        let (a, b) = line.split_once(',').unwrap();
        dbg!(&a);
        dbg!(&b);
        let (a1, a2) = a.split_once('=').unwrap();

        let (b1, b2) = b.trim().split_once('=').unwrap();
        let (c1, c2) = b2.split_once("..").unwrap();
        match a1 {
            "x" => {
                assert_eq!(b1, "y");
                let x = a2.parse::<i64>().unwrap();
                for y in c1.parse::<i64>().unwrap()..=c2.parse::<i64>().unwrap() {
                    grid.insert(Vec2D { x, y: -y }, Stuff::Clay);
                }
            }
            "y" => {
                assert_eq!(b1, "x");
                let y = a2.parse::<i64>().unwrap();
                for x in c1.parse::<i64>().unwrap()..=c2.parse::<i64>().unwrap() {
                    grid.insert(Vec2D { x, y: -y }, Stuff::Clay);
                }
            }
            _ => todo!(),
        }
    }
    let min_y = grid.keys().map(|v| v.y).min().unwrap();
    let max_y = grid.keys().map(|v| v.y).max().unwrap();

    grid.insert(Vec2D { x: 500, y: 0 }, Stuff::Spring);
    print_grid(&grid);
    fill_with_water(&mut grid, min_y);

    let part1 = grid
        .iter()
        .filter(|(i, obj)| {
            (*obj == &Stuff::WaterStill || *obj == &Stuff::WaterFlow) && i.y <= max_y
        })
        .count() as i64;
    let part2 = grid
        .iter()
        .filter(|(i, obj)| *obj == &Stuff::WaterStill && i.y <= max_y)
        .count() as i64;

    (part1, part2)
}

fn print_grid(grid: &BTreeMap<Vec2D, Stuff>) {
    let min_x = grid.keys().map(|v| v.x).min().unwrap() - 1;
    let max_x = grid.keys().map(|v| v.x).max().unwrap() + 1;
    let min_y = grid.keys().map(|v| v.y).min().unwrap() - 1;
    let max_y = grid.keys().map(|v| v.y).max().unwrap() + 1;

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            match grid.get(&Vec2D { x, y }) {
                Some(Stuff::Clay) => {
                    print!("{}", "#".red().on_red());
                }
                Some(Stuff::Spring) => {
                    print!("{}", "+".blue().on_black());
                }
                Some(Stuff::WaterStill) => {
                    print!("{}", "~".blue().on_blue());
                }
                Some(Stuff::WaterFlow) => {
                    print!("{}", "|".blue().on_black());
                }
                _ => {
                    print!("{}", ".".white().on_black());
                }
            }
        }
        println!();
    }
}

fn fill_with_water(grid: &mut BTreeMap<Vec2D, Stuff>, min_y: i64) {
    let mut heads: Vec<Vec2D> = Vec::new();
    let (spring, _) = grid.iter().find(|(_, obj)| *obj == &Stuff::Spring).unwrap();

    heads.push(*spring);
    const DOWN: Vec2D = Vec2D { x: 0, y: -1 };
    const LEFT: Vec2D = Vec2D { x: -1, y: 0 };
    const RIGHT: Vec2D = Vec2D { x: 1, y: 0 };

    loop {
        if heads.is_empty() {
            break;
        }
        let head = heads.pop().unwrap();
        let mut loc = head;
        loop {
            let stuff = grid.get(&(loc + DOWN));
            match stuff {
                None => {
                    if loc.y > min_y {
                        loc = loc + DOWN;
                        heads.push(loc);
                        grid.insert(loc, Stuff::WaterFlow);
                        continue;
                    } else {
                        break;
                    }
                }
                Some(Stuff::Clay) | Some(Stuff::WaterStill) => {
                    println!("Found {:?}", stuff);
                    let mut max_x = loc.x;
                    let mut min_x = loc.x;
                    let mut closed = true;
                    let mut loc_right = loc;
                    let y = loc.y;
                    loop {
                        loc_right = loc_right + RIGHT;
                        if grid.get(&(loc_right)) == Some(&Stuff::Clay) {
                            break;
                        }
                        max_x += 1;
                        match grid.get(&(loc_right + DOWN)) {
                            None | Some(Stuff::WaterFlow) => {
                                println!("Found edge at {}", loc_right);
                                heads.push(loc_right);
                                closed = false;
                                break;
                            }
                            _ => {}
                        }
                    }
                    let mut loc_left = loc;
                    loop {
                        loc_left = loc_left + LEFT;
                        if grid.get(&(loc_left)) == Some(&Stuff::Clay) {
                            break;
                        }
                        min_x -= 1;
                        match grid.get(&(loc_left + DOWN)) {
                            None | Some(Stuff::WaterFlow) => {
                                println!("Found edge at {}", loc_left);
                                heads.push(loc_left);
                                closed = false;
                                break;
                            }
                            _ => {}
                        }
                    }
                    if closed {
                        for x in min_x..=max_x {
                            grid.insert(Vec2D { x, y }, Stuff::WaterStill);
                        }
                        heads.push(loc - DOWN);
                    } else {
                        for x in min_x..=max_x {
                            grid.insert(Vec2D { x, y }, Stuff::WaterFlow);
                        }
                    }
                    break;
                }
                _ => {
                    break;
                }
            }
        }
    }
    println!();
    print_grid(grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

        assert_eq!(read_contents(&a).0, 57);
    }
}
