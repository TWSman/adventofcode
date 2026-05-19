use clap::Parser;
use colored::Colorize;
use shared::Vec2D;
use std::collections::BTreeSet;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("\n########################");
    println!("Part 2 answer is {}", res);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> i64 {
    let mut points = cont.lines().map(Point::new).collect::<Vec<_>>();
    let max_x = points.iter().map(|p| p.location.x).max().unwrap() + 2;
    let min_x = points.iter().map(|p| p.location.x).min().unwrap() - 2;
    let max_y = points.iter().map(|p| p.location.y).max().unwrap() + 2;
    let min_y = points.iter().map(|p| p.location.y).min().unwrap() - 2;
    let n = points.len() as i64;

    // Keep track of the area needed to cover all points
    let mut min_area = (max_x - min_x) * (max_y - min_y);
    let mut min_index = 0;
    println!("{} points", n);

    for i in 1..40000000 {
        for p in &mut points {
            p.location = p.location + p.velocity;
        }

        let max_x = points.iter().map(|p| p.location.x).max().unwrap() + 2;
        let min_x = points.iter().map(|p| p.location.x).min().unwrap() - 2;
        let max_y = points.iter().map(|p| p.location.y).max().unwrap() + 2;
        let min_y = points.iter().map(|p| p.location.y).min().unwrap() - 2;
        let area = (max_x - min_x) * (max_y - min_y);
        if area < min_area {
            if n > area / 4 {
                // At least 1 in 4 points is covered within the area
                print_points(&points);
            }
            min_area = area;
            min_index = i;
        }

        if area > min_area * 2 {
            return min_index;
        }
    }
    0
}

#[derive(Debug, Clone)]
struct Point {
    location: Vec2D,
    velocity: Vec2D,
}

impl Point {
    fn new(ln: &str) -> Self {
        let (a, b) = ln.split_once('>').unwrap();
        let pos = a.split_once('<').unwrap().1.trim();
        let pos = pos.split_once(',').unwrap();

        let vel = b
            .split_once('<')
            .unwrap()
            .1
            .strip_suffix('>')
            .unwrap()
            .split_once(',')
            .unwrap();

        Self {
            location: Vec2D {
                x: pos.0.trim().parse::<i64>().unwrap(),
                y: -pos.1.trim().parse::<i64>().unwrap(),
            },
            velocity: Vec2D {
                x: vel.0.trim().parse::<i64>().unwrap(),
                y: -vel.1.trim().parse::<i64>().unwrap(),
            },
        }
    }
}

fn print_points(points: &[Point]) {
    let mut grid = BTreeSet::new();
    for p in points {
        grid.insert(p.location);
    }

    let max_x = points.iter().map(|p| p.location.x).max().unwrap() + 2;
    let min_x = points.iter().map(|p| p.location.x).min().unwrap() - 2;
    let max_y = points.iter().map(|p| p.location.y).max().unwrap() + 2;
    let min_y = points.iter().map(|p| p.location.y).min().unwrap() - 2;

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if grid.contains(&Vec2D { x, y }) {
                print!("{}", "#".red().on_red());
            } else {
                print!("{}", ".".black().on_black());
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points() {
        let p = Point::new("position=< 9,  1> velocity=< 0,  2>");
        assert_eq!(p.location.x, 9);
        assert_eq!(p.velocity.y, -2); // Invert y direction such that positive y points up
    }

    #[test]
    fn part2() {
        let a = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        assert_eq!(read_contents(&a), 3);
    }
}
