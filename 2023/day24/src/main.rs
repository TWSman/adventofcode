#[macro_use]
extern crate assert_float_eq;

use clap::Parser;
use std::fs;
use regex::Regex;
use itertools::Itertools;
use std::fmt::Display;
use core::fmt;

use ndarray::array;
use ndarray_linalg::Solve;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[allow(clippy::cast_possible_truncation)]

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");

    let min_val = 2e14;
    let max_val = 4e14;
    //let min_val = 200_000_000_000_000.0;
    //let max_val = 400_000_000_000_000.0;
    let res = read_contents(&contents, min_val, max_val);
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);

}

#[derive(Debug, Clone)]
struct Vec3D {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone)]
struct SnowFlake {
    position: Vec3D,
    velocity: Vec3D,
}

#[derive(Debug, Clone)]
struct Linear {
    slope: f64,
    intersect: f64,
}

impl Linear {
    fn eval(&self, x: f64) -> f64 {
        self.intersect + self.slope * x
    }
}


impl Display for SnowFlake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) >> ({})", self.position, self.velocity)
    }
}

impl Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

impl SnowFlake {
    fn new(line: &str) -> Self {
        let re = Regex::new(r"([-0-9]*),\s*([-0-9]*),\s*([0-9]*)\s*@\s*(-?[0-9]*),\s*(-?[0-9]*),\s*([-?0-9]*)").unwrap();
        let Some(res) = re.captures(line) else { panic!("Could not parse input");};
        let position = Vec3D {
            x: res[1].parse::<i64>().unwrap(),
            y: res[2].parse::<i64>().unwrap(),
            z: res[3].parse::<i64>().unwrap(),
        };

        let velocity = Vec3D {
            x: res[4].parse::<i64>().unwrap(),
            y: res[5].parse::<i64>().unwrap(),
            z: res[6].parse::<i64>().unwrap(),
        };
        Self {position, velocity} 
    }

    fn get_t(&self, x: f64, y: f64)-> f64 {
        let tx = (x - self.position.x as f64) / (self.velocity.x as f64);
        let ty = (y - self.position.y as f64) / (self.velocity.y as f64);
        assert_float_relative_eq!(tx,ty);
        tx
    }

    fn equation(&self) -> Linear {
        // This only works if velocity.x is nonnegative,
        // which seems to be the case in the
        // input data
        assert_ne!(self.velocity.x, 0);
        let slope = (self.velocity.y as f64) / (self.velocity.x as f64);
        let intersect = (self.position.y as f64) - (self.position.x as f64) * slope;
        Linear { slope, intersect }
    }

    fn get_intersection(&self, other: &SnowFlake) -> Option<(f64, f64)> {
        let a = self.equation();
        let b = other.equation();
        if a.slope == b.slope {
            return None
        }
        let x = (b.intersect - a.intersect) / (a.slope - b.slope);
        let y = a.eval(x);
        Some((x,y))
    }
}

// 558415252330828 is the right answer to part2
fn part2(flakes: &Vec<SnowFlake>) -> i64 {
    let p1 = &flakes[0].position;
    let v1 = &flakes[0].velocity;

    let p2 = &flakes[1].position;
    let v2 = &flakes[1].velocity;

    let p3 = &flakes[3].position;
    let v3 = &flakes[3].velocity;

    let p4 = &flakes[4].position;
    let v4 = &flakes[4].velocity;

    let p5 = &flakes[2].position;
    let v5 = &flakes[2].velocity;

    let b1 = p2.x * v2.y - p2.y * v2.x - v1.y * p1.x + v1.x * p1.y;
    let b2 = p3.x * v3.y - p3.y * v3.x - v1.y * p1.x + v1.x * p1.y;
    let b3 = p4.x * v4.y - p4.y * v4.x - v1.y * p1.x + v1.x * p1.y;
    let b4 = p5.x * v5.y - p5.y * v5.x - v1.y * p1.x + v1.x * p1.y;

    let b = array![b1 as f64, b2 as f64, b3 as f64, b4 as f64];

    let m = array![
        [(v2.y - v1.y) as f64, (v1.x - v2.x) as f64, (p1.y - p2.y) as f64, (p2.x - p1.x) as f64],
        [(v3.y - v1.y) as f64, (v1.x - v3.x) as f64, (p1.y - p3.y) as f64, (p3.x - p1.x) as f64],
        [(v4.y - v1.y) as f64, (v1.x - v4.x) as f64, (p1.y - p4.y) as f64, (p4.x - p1.x) as f64],
        [(v5.y - v1.y) as f64, (v1.x - v5.x) as f64, (p1.y - p5.y) as f64, (p5.x - p1.x) as f64],
    ];
    
    let x = m.solve_into(b).unwrap();
    dbg!(m.dot(&x));
    let x0 = x[0].round() as i64;
    let y0 = x[1].round() as i64;
    let vx = x[2].round() as i64;
    let vy = x[3].round() as i64;
    dbg!(&x0);
    dbg!(&y0);
    dbg!(&vx);
    dbg!(&vy);
    let t1 = (x0 - p1.x) / (v1.x - vx);
    let t2 = (x0 - p2.x) / (v2.x - vx);
    let vz = (p1.z - p2.z + v1.z * t1 - v2.z * t2) / (t1 - t2);
    let z0 = p1.z + v1.z * t1 - vz * t1;
    dbg!(&z0);
    dbg!(&vz);
    dbg!(&t1);
    dbg!(&t2);
    x0 + y0 + z0
}

fn read_contents(cont: &str, min_val: f64, max_val: f64) -> (i64, i64) {
    let snowflakes = cont.lines().map(|ln| {SnowFlake::new(ln)}).collect::<Vec<SnowFlake>>();
    let part2 = part2(&snowflakes);
    let part1 = snowflakes.iter().combinations(2).filter(|v| {
        let a = v[0];
        let b = v[1];
        match a.get_intersection(b) {
            None => {
                //println!("No intersection");
                false
            },
            Some((x,y)) => {
                if (a.get_t(x,y) < 0.0) | (b.get_t(x,y) < 0.0) | (x < min_val) | (x > max_val) | (y < min_val) | (y > max_val) {
                    //Intersection outside the area
                    false
                }
                else {
                    //Intersection Inside the area
                    true
                }
            },
        }
    }).count() as i64;
    (part1, part2)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn equation() {
        let a = SnowFlake::new("0, 0, 0 @ 1, 1, 0");
        let eqa = a.equation();
        assert_eq!(eqa.slope, 1.0);
        assert_eq!(eqa.intersect, 0.0);

        let b = SnowFlake::new("0, 1, 0 @ 1, -1, 0");
        dbg!(&b);
        let eqb = b.equation();
        dbg!(&eqb);
        assert_eq!(eqb.eval(0.0), 1.0);
        assert_eq!(eqb.slope, -1.0);
        assert_eq!(eqb.intersect, 1.0);
        
        assert_eq!(a.get_intersection(&b), Some((0.5, 0.5)));

    }

    #[test]
    fn conts() {
        let a = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(read_contents(&a, 7.0, 27.0).0, 2);
        assert_eq!(read_contents(&a, 7.0, 27.0).1, 47);
    }
}
