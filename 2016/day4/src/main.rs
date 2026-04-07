use clap::Parser;
use std::collections::BTreeMap;
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
    let rooms = cont.lines().map(Room::new).collect::<Vec<_>>();
    let part1 = get_part1(&rooms);
    let part2 = get_part2(&rooms);
    (part1, part2)
}

#[derive(Debug)]
struct Room {
    name: String,
    id: i32,
    check: String,
}

impl Room {
    fn new(ln: &str) -> Self {
        let (name, rest) = ln.split_once('[').unwrap();
        let check = rest.strip_suffix(']').unwrap();
        let splits = name.split('-').collect::<Vec<_>>();
        let n = splits.len();
        let id = splits[n - 1].parse::<i32>().unwrap();
        let name = splits[..n - 1].join("-");
        Self {
            name: name.to_string(),
            id,
            check: check.to_string(),
        }
    }

    fn valid_id(&self) -> i32 {
        let mut counts = BTreeMap::new();
        for c in self.name.chars() {
            if !c.is_ascii_lowercase() {
                continue;
            }
            if !counts.contains_key(&c) {
                counts.insert(c, 1);
            } else {
                *counts.get_mut(&c).unwrap() += 1;
            }
        }
        let mut counts = counts
            .iter()
            .map(|(c, count)| (*c, *count))
            .collect::<Vec<_>>();
        counts.sort_by_key(|(c, count)| (-*count, *c));
        let check = counts.iter().take(5).map(|(c, _)| *c).collect::<String>();
        if check == self.check { self.id } else { 0 }
    }

    fn decrypt(&self) -> String {
        let mut decrypted = String::new();
        for c in self.name.chars() {
            if c == '-' {
                decrypted.push(' ');
            } else {
                let new_c = (c as u8 - b'a' + (self.id % 26) as u8) % 26 + b'a';
                decrypted.push(new_c as char);
            }
        }
        decrypted
    }
}

fn get_part1(rooms: &[Room]) -> i32 {
    rooms.iter().map(|ln| ln.valid_id()).sum()
}

fn get_part2(rooms: &[Room]) -> i32 {
    // Print id and decrypted name for all rooms
    for room in rooms {
        //println!("{} -> {}", room.id, room.decrypt());
        if room.decrypt() == "northpole object storage" {
            println!("Found northpole object storage with id {}", room.id);
            return room.id;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";
        assert_eq!(read_contents(&a).0, 1514);
    }

    #[test]
    fn part2() {
        let room = Room::new("qzmt-zixmtkozy-ivhz-343[abcde]");
        assert_eq!(room.decrypt(), "very encrypted name");
    }
}
