use clap::Parser;
use std::fs;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}
 
#[derive(Debug)]
struct Lock {
    heights: Vec<u8>,
}

#[derive(Debug)]
struct Key {
    heights: Vec<u8>,
}

impl Key {
    fn check_lock(&self, lock: &Lock) -> bool {
        self.heights.iter().zip(&lock.heights).all(|(a,b)| a + b <= 5)
    }
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");

    let part1 = read_contents(&contents);
    println!("Part 1 answer is {part1}");
}

fn read_contents(cont: &str) -> u64 {
    let mut keys: Vec<Key> = Vec::new();
    let mut locks: Vec<Lock> = Vec::new();
    for chunk in &cont.lines().chunks(8) {
        let data = chunk.map(|m| m.to_string()).collect::<Vec<String>>();

        // Collect height data to a vector
        // Just counting number of # works for both keys and locks
        let heights = (0..5).map(|i| {
            (0..7).filter(|j| {
                data[*j].chars().nth(i).unwrap() == '#'
            }).count() as u8 - 1
        }).collect::<Vec<u8>>();

        if data[0] == "#####" {
            // Locks have top row of #
            // And should have a bottom row of .
            assert_eq!(data[6], ".....");
            locks.push(Lock {heights});

        } else if data[0] == "....." {
            // Keys have top row of .
            // And should have a bottom row of #
            assert_eq!(data[6], "#####");
            keys.push(Key {heights});
        } else {
            panic!("Top row should either be ##### or .....");
        }
    }
    println!("Found {} locks", locks.len());
    println!("Found {} keys", keys.len());
    locks.iter().cartesian_product(keys.iter()).filter(|(lock, key)| {
        key.check_lock(lock)
    }).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
assert_eq!(read_contents(&a), 3);
    }

    #[test]
    fn check() {
        // Lock 0,5,3,4,3 and key 5,0,2,1,3: overlap in the last column.
        let lock = Lock { heights: vec![0,5,3,4,3] };
        let key_a = Key { heights:   vec![5,0,2,1,3] };
        assert!(!key_a.check_lock(&lock));

        // Lock 0,5,3,4,3 and key 3,0,2,0,1: all columns fit!
        let key_bb = Key { heights:   vec![3,0,2,0,1] };
        assert!(key_bb.check_lock(&lock));

    }
}
