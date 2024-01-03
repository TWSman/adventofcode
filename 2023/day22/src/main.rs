use clap::Parser;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use regex::Regex;
use std::fmt::Display;
use core::fmt;
use std::cmp::Ordering;

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



#[derive(Debug, Clone, PartialEq, Eq)]
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
        Some(Ordering::Equal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block {
    start: Vec3D,
    end: Vec3D,
    name: String,
    held_by: HashSet<String>,
    holds: HashSet<String>,
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

static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 
    'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 
    'Z',
];

//0,2,3~2,2,3
impl Block {
    fn new(input: &str) -> Block {
        let re = Regex::new("([0-9]*),([0-9]*),([0-9]*)~([0-9]*),([0-9]*),([0-9]*)").unwrap();
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
        if start > end {
            Block {
                start: end,
                end: start,
                name: "block".to_string(),
                held_by: HashSet::new(),
                holds: HashSet::new(),
            }
        } else {
            Block {
                start,
                end,
                name: "block".to_string(),
                held_by: HashSet::new(),
                holds: HashSet::new(),
            }
        }
    }

    fn overlaps(&self, other: &Block) -> bool {
        if (self.start.x <= other.end.x) & (self.end.x >= other.start.x)  &(self.start.y <= other.end.y) & (self.end.y >= other.start.y) {
            return true
        }
        if other.end.x < self.start.x {
            return false
        }
        if other.end.y < self.start.y {
            return false
        }
        if other.start.x > self.end.x {
            return false
        }
        if other.start.y > self.end.y {
            return false
        }
        true
    }

    fn above(&self, other: &Block) -> bool {
        self.start.z > other.end.z
    }

    fn drop(&mut self, steps: i64) {
        self.start.z -= steps;
        self.end.z -= steps;
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut blocks: Vec<Block> = cont.lines().map(|ln| {
        Block::new(ln)
    }).collect();

    // Give names to blocks
    // Example used A-F. Thus gives names AAA-ZZZ
    //for i in 0..blocks.len() {
    for (i, block) in blocks.iter_mut().enumerate() {
        let (div, a) = (i / 26, i % 26);
        let (div, b) = (div / 26, div % 26);
        let (_div, c) = (div / 26, div % 26);
        let mtp = format!("{}{}{}", ASCII_UPPER[c], ASCII_UPPER[b], ASCII_UPPER[a]);
        block.name = mtp;
    }
    let n = blocks.len();

    // When moving the blocks downwards, we assume that the list is in increasin z order
    blocks.sort_by_key(|b| b.start.z);

    // Below name, and above name
    let mut hold_relations: HashSet<(String, String)> = HashSet::new();

    // Move blocks downwards and take not of which blocks hold which ones
    for i in 0..n {
        let mut b = blocks[i].clone();
        let hold_list = (0..n).filter_map(|j| {
            if i == j {
                return None
            }
            let b_comp = &blocks[j];
            let is_above = b.overlaps(b_comp) & b.above(b_comp);
            if is_above {
                //println!("Block {} is above block {}", &b, &b_comp);
                Some((b_comp.name.clone(), b_comp.end.z))
            } else {
                None
            }
        }).collect::<Vec<_>>();
        let max_z = hold_list.iter().map(|(_, z)| *z).max().unwrap_or(0);
        for (name, z) in &hold_list {
            if z == &max_z {
                hold_relations.insert((name.to_string(), b.name.to_string()));
            }
        }
        if max_z == 0 {
            hold_relations.insert(("GROUND".to_string(), b.name.to_string()));
        }
        let new_z = max_z + 1;
        b.drop(b.start.z - new_z);
        blocks[i] = b;
    }

    // Convert vector to hashmap for easier retrieval
    let mut block_map: HashMap<String, Block> = blocks.into_iter().map(|b| (b.name.clone(), b)).collect();
    for (name_below, name_above) in hold_relations {
        if name_below != "GROUND" {
            let below = block_map.get_mut(&name_below).unwrap();
            below.holds.insert(name_above.to_string());
        }
        let above = block_map.get_mut(&name_above).unwrap();
        above.held_by.insert(name_below.to_string());
    }

    let mut part1 = 0;
    for block in block_map.values() {
        if block.holds.iter().filter(|b_name|{
            let b = block_map.get(&b_name.to_string()).unwrap();
            b.held_by.len() == 1
        }).count() == 0 {
            part1 += 1;
        }
    }

    let mut drops = 0;
    let n = block_map.len();
    for (i, name) in block_map.keys().enumerate() {
        println!("{:04} / {:04}, Dropping {}", i+1,n,&name);
        let mut bmap = block_map.clone();
        let mut dropped_blocks: VecDeque<String> = VecDeque::new();
        dropped_blocks.push_back(name.to_string());
        while let Some(dropped_name) = dropped_blocks.pop_front() {
            let dropped_block = block_map.get(&dropped_name).unwrap();
            for above_name in &dropped_block.holds {
                let above_block = bmap.get_mut(above_name).unwrap();
                if above_block.held_by.contains(&dropped_name) {
                    above_block.held_by.remove(&dropped_name);
                    if above_block.held_by.is_empty() {
                        dropped_blocks.push_back(above_name.clone());
                        drops += 1;
                    }
                }
            }
        }
    }
    (part1, drops)
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

        let a = Block::new("1,0,1~1,2,1");
        let b = Block::new("0,0,2~2,0,2");
        let c = Block::new("0,2,3~2,2,3");
        let d = Block::new("0,0,4~0,2,4");
        let e = Block::new("0,1,6~2,1,6");
        let f = Block::new("2,0,5~2,2,5");
        let g = Block::new("1,1,8~1,1,9");
        assert!(a.overlaps(&b));
        assert!(a.overlaps(&c));

        assert!(b.overlaps(&d));
        assert!(!e.overlaps(&b));
        assert!(!b.overlaps(&e));

        assert!(c.overlaps(&d));
        assert!(!c.overlaps(&e));

        assert!(!d.overlaps(&f));
        assert!(e.overlaps(&f));

        assert!(!f.overlaps(&g));
    }

    #[test]
    fn conts() {
        let a = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(read_contents(&a).0, 5);
        assert_eq!(read_contents(&a).1, 7);
    }
}
