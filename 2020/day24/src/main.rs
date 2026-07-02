use clap::Parser;
use std::collections::BTreeMap;
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
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i32, i32) {
    let list = cont.lines().map(read_line).collect::<Vec<_>>();
    //run(&list)
    let part1 = get_part1(&list);
    let part2 = get_part2(&list);
    (part1, part2)
}

fn read_line(ln: &str) -> Vec<HexDir> {
    dbg!(&ln);
    ln.split_inclusive(['e', 'w'])
        .map(HexDir::new)
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum HexDir {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
struct CubeVec {
    r: i32,
    s: i32,
    q: i32,
}

impl CubeVec {
    fn new() -> Self {
        Self { r: 0, s: 0, q: 0 }
    }
}

impl HexDir {
    fn new(ln: &str) -> Self {
        match ln.trim() {
            "ne" => Self::NorthEast,
            "e" => Self::East,
            "nw" => Self::NorthWest,
            "se" => Self::SouthEast,
            "w" => Self::West,
            "sw" => Self::SouthWest,
            _ => panic!("Unknown direction {}", ln),
        }
    }

    fn step(&self, loc: &CubeVec) -> CubeVec {
        let mut new_loc = loc.clone();
        match self {
            HexDir::East => {
                new_loc.q += 1;
                new_loc.s -= 1;
            }
            HexDir::West => {
                new_loc.q -= 1;
                new_loc.s += 1;
            }
            HexDir::NorthEast => {
                new_loc.r -= 1;
                new_loc.q += 1;
            }
            HexDir::SouthWest => {
                new_loc.r += 1;
                new_loc.q -= 1;
            }
            HexDir::SouthEast => {
                new_loc.r += 1;
                new_loc.s -= 1;
            }
            HexDir::NorthWest => {
                new_loc.r -= 1;
                new_loc.s += 1;
            }
        }
        // Sum should always be 0
        assert_eq!(new_loc.r + new_loc.s + new_loc.q, 0);
        new_loc
    }
}

fn get_part1(vec: &[Vec<HexDir>]) -> i32 {
    let mut counts: BTreeMap<CubeVec, bool> = BTreeMap::new();
    for v in vec {
        let loc = run(v);
        *counts.entry(loc).or_insert(false) ^= true;
    }
    counts.iter().filter(|(_i, c)| **c).count() as i32
}

fn get_part2(vec: &[Vec<HexDir>]) -> i32 {
    let mut counts: BTreeMap<CubeVec, bool> = BTreeMap::new();
    for v in vec {
        let loc = run(v);
        *counts.entry(loc).or_insert(false) ^= true;
    }
    let mut tiles = counts
        .iter()
        .filter(|(_i, c)| **c)
        .map(|(i, _c)| i.clone())
        .collect::<BTreeSet<_>>();
    for i in 0..100 {
        println!("Round: {i}");
        let mut neighbors: BTreeMap<CubeVec, usize> = BTreeMap::new();
        for tile in tiles.iter() {
            if !neighbors.contains_key(tile) {
                neighbors.insert(tile.clone(), 0);
            }
            for dir in [
                HexDir::East,
                HexDir::West,
                HexDir::NorthEast,
                HexDir::NorthWest,
                HexDir::SouthEast,
                HexDir::SouthWest,
            ] {
                let loc = dir.step(tile);
                *neighbors.entry(loc).or_default() += 1;
            }
        }
        for (loc, count) in neighbors.iter() {
            // White tile with exactly 2 black neighbors gets flipped to black
            if !tiles.contains(loc) && *count == 2 {
                tiles.insert(loc.clone());
            }
            // Black tiles with 0 or more than 2 neighbors gets flipped to white
            if tiles.contains(loc) && (*count == 0 || *count > 2) {
                tiles.remove(loc);
            }
        }
        println!("    {} black tiles", tiles.len());
    }
    tiles.len() as i32
}

// Use cube coordinates for the hex grid,
// In cube coordinates r + s + q == 0
fn run(list: &[HexDir]) -> CubeVec {
    println!("Running with {} directions", list.len());
    let mut loc = CubeVec::new();
    for dir in list {
        loc = dir.step(&loc);
    }
    loc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        assert_eq!(read_contents(&a).0, 10);
    }

    #[test]
    fn part2() {
        let a = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        assert_eq!(read_contents(&a).1, 2208);
    }
}
