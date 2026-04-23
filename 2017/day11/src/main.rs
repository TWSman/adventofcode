use clap::Parser;
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
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i32, i32) {
    let list = cont.split(',').map(HexDir::new).collect::<Vec<_>>();
    run(&list)
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum HexDir {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

#[derive(Debug, Clone)]
struct CubeVec {
    r: i32,
    s: i32,
    q: i32,
}

impl CubeVec {
    fn dist(&self) -> i32 {
        // Each hexagon corresponds to a cube in 3D space
        // But adjacent hexagons are 2 steps apart in the cube grid
        // (Each steps moves +1 unit in 1 dimensions, and -1 in another dimension)
        (self.r.abs() + self.s.abs() + self.q.abs()) / 2
    }
}

impl HexDir {
    fn new(ln: &str) -> Self {
        match ln.trim() {
            "ne" => Self::NorthEast,
            "n" => Self::North,
            "nw" => Self::NorthWest,
            "se" => Self::SouthEast,
            "s" => Self::South,
            "sw" => Self::SouthWest,
            _ => panic!("Unknown direction {}", ln),
        }
    }

    fn step(&self, loc: CubeVec) -> CubeVec {
        let mut new_loc = loc.clone();
        match self {
            HexDir::North => {
                new_loc.q += 1;
                new_loc.s -= 1;
            }
            HexDir::South => {
                new_loc.q -= 1;
                new_loc.s += 1;
            }
            HexDir::NorthEast => {
                new_loc.r += 1;
                new_loc.s -= 1;
            }
            HexDir::SouthEast => {
                new_loc.r += 1;
                new_loc.q -= 1;
            }
            HexDir::NorthWest => {
                new_loc.r -= 1;
                new_loc.q += 1;
            }
            HexDir::SouthWest => {
                new_loc.r -= 1;
                new_loc.s += 1;
            }
        }
        // Sum should always be 0
        assert_eq!(new_loc.r + new_loc.s + new_loc.q, 0);
        new_loc
    }
}

// Use cube coordinates for the hex grid,
// In cube coordinates r + s + q == 0
fn run(list: &[HexDir]) -> (i32, i32) {
    println!("Running with {} directions", list.len());
    let mut loc = CubeVec { r: 0, s: 0, q: 0 }; // X and Y
    let mut max_dist = 0;
    let mut max_loc = loc.clone();
    for dir in list {
        loc = dir.step(loc);
        let dist = loc.dist();
        if dist > max_dist {
            max_dist = dist;
            max_loc = loc.clone();
        }
    }
    println!("Location with max distance: {:?}", max_loc);
    println!("Final location: {:?}", loc);
    (loc.dist(), max_dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(read_contents("ne,ne,ne").0, 3);
        assert_eq!(read_contents("ne,ne,sw,sw").0, 0);
        assert_eq!(read_contents("ne,ne,s,s").0, 2);
        assert_eq!(read_contents("se,sw,se,sw,sw").0, 3);
    }
}
