use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
use std::cmp::max;
use std::fmt::Display;
use core::fmt;
use std::cmp::Ordering;
use std::iter::zip;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");

    // 0 cycles means just one tilt to north (part1)
    let res = part1(&contents);
    println!("Part 1 answer is {}", res);
}



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec3D {
    x: i64,
    y: i64,
    z: i64,
}

impl PartialOrd for Vec3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x < other.x {
            return Some(Ordering::Less);
        }
        if self.x > other.x {
            return Some(Ordering::Greater);
        }
        if self.y < other.y {
            return Some(Ordering::Less);
        }
        if self.y > other.y {
            return Some(Ordering::Greater);
        }
        if self.z < other.z {
            return Some(Ordering::Less);
        }
        if self.z > other.z {
            return Some(Ordering::Greater);
        }
        return Some(Ordering::Equal);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Block {
    start: Vec3D,
    end: Vec3D,
    name: String,
}


impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}) - ({})", self.name, self.start, self.end)
    }
}

impl Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

//0,2,3~2,2,3
impl Block {
    fn new(input: &str) -> Block {
        let re = Regex::new("([0-9]),([0-9]),([0-9])~([0-9]),([0-9]),([0-9])").unwrap();
        let Some(res) = re.captures(input) else {panic!("Could not parse {}", input)};
        let start = Vec3D {
            x: res[1].parse::<i64>().unwrap(),
            y: res[2].parse::<i64>().unwrap(),
            z: res[3].parse::<i64>().unwrap(),
        };
        let end = Vec3D {
            x: res[4].parse::<i64>().unwrap(),
            y: res[5].parse::<i64>().unwrap(),
            z: res[6].parse::<i64>().unwrap(),
        };
        dbg!(&input);
        let block = if start > end {
            Block {start: end, end:start, name: "block".to_string()}
        } else {
            Block {start: start, end:end, name: "block".to_string()}
        };
        block
    }

    fn overlaps(&self, other: &Block) -> bool {
        println!("Self: {}", self);
        println!("Other: {}", other);
        if (self.start.x <= other.end.x) & (self.end.x >= other.start.x) {
            if (self.start.y <= other.end.y) & (self.end.y >= other.start.y) {
                return true
            }
        }
        if other.end.x < self.start.x {
            println!("first");
            return false
        }
        if other.end.y < self.start.y {
            println!("second");
            return false
        }
        if other.start.x > self.end.x {
            println!("third");
            return false
        }
        if other.start.y > self.end.y {
            println!("fourth");
            return false
        }
        return true
    }

    fn above(&self, other: &Block) -> bool {
        self.start.z > other.end.z
    }

    fn below(&self, other: &Block) -> bool {
        !self.above(other)
    }
}

fn part1(cont: &str) -> i64 {
    let mut blocks: Vec<Block> = cont.lines().map(|ln| {
        Block::new(ln)
    }).collect();

    let names = ["A", "B", "C", "D", "E", "F", "G"];
    for i in (0..names.len()) {
        blocks[i].name = names[i].to_string();
    }

    let mut removable: HashSet<Block> = HashSet::new();
    blocks.iter().for_each(|b| {
        // Want list of blocks that are above b
        let tmp = blocks.iter().filter(|b_comp| {
            if &b == b_comp {
                return false
            }
            let is_below = b.overlaps(&b_comp) & b.below(&b_comp);
            if is_below {
                println!("Block {} is below block {}", b, b_comp);
            }
            is_below
        }).collect::<Vec<_>>();
        if tmp.len() > 1 {
            for t in tmp {
                removable.insert(t.clone());
            }
        }});
    for t in &removable {
        println!("Block {} can be removed", &t);
    }
    removable.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlaps() {
        let a = Block::new("0,0,1~2,0,1"); // 3 long horizontal y = 0
        let b = Block::new("0,1,2~2,1,2"); // 2 long horizontal y = 1
        let c = Block::new("3,0,3~4,0,3"); // 2 long horizontal y = 0, not overlapping
        let d = Block::new("2,0,4~3,0,4"); // 2 long horizontal y = 0, overlapping
        //
        let e = Block::new("1,0,4~1,1,5"); // 2 long vertical x = 1, overlapping
        assert!(!a.overlaps(&b));
        assert!(!a.overlaps(&c));
        assert!(a.overlaps(&d));
        assert!(a.overlaps(&e));

        assert!(a.overlaps(&Block::new("1,0,1~1,0,1")));
        assert!(a.overlaps(&Block::new("0,0,1~0,2,1")));

        let A = Block::new("1,0,1~1,2,1");
        let B = Block::new("0,0,2~2,0,2");
        let C = Block::new("0,2,3~2,2,3");
        let D = Block::new("0,0,4~0,2,4");
        let E = Block::new("0,1,6~2,1,6");
        let F = Block::new("2,0,5~2,2,5");
        let G = Block::new("1,1,8~1,1,9");
        assert!(A.overlaps(&B));
        assert!(A.overlaps(&C));

        assert!(B.overlaps(&D));
        println!("{}", B);
        println!("{}", E);
        assert!(E.overlaps(&B));
        assert!(B.overlaps(&E));

        assert!(C.overlaps(&D));
        assert!(C.overlaps(&E));

        assert!(D.overlaps(&F));
        assert!(E.overlaps(&F));

        assert!(F.overlaps(&G));
    }

    #[test]
    fn conts() {
        let a = "
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(part1(&a), 5);
    }

}
