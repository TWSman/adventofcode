#![feature(linked_list_cursors)]
// Run with 'cargo +nightly'

use clap::Parser;
use std::collections::BTreeMap;
use std::collections::LinkedList;
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

fn read_contents(cont: &str) -> (i64, i64) {
    let parts = cont.split_whitespace().collect::<Vec<_>>();
    let players = parts[0].parse::<usize>().unwrap();
    let last_marble = parts[6].parse::<usize>().unwrap();
    let part1 = get_part1(players, last_marble);
    let part2 = get_linked(players, 100 * last_marble);
    (part1, part2)
}

fn get_part1(players: usize, last_marble: usize) -> i64 {
    // Very slow. Takes 1h30min for part2
    let mut vec = vec![0];
    let mut scores: BTreeMap<usize, usize> = BTreeMap::new();
    let mut current_i = 0;
    for marble in 1..=last_marble {
        if marble % 100_000 == 0 {
            println!("{} / {}", marble + 1, last_marble);
        }
        if marble == 1 {
            vec.push(marble);
            current_i = 1;
            continue;
        }

        if marble % 23 == 0 {
            let player_i = marble % players;
            let rem_i = (current_i + vec.len() - 7) % vec.len();
            let t = vec.remove(rem_i);
            *scores.entry(player_i).or_default() += marble + t;
            current_i = rem_i;
            continue;
        }
        let new_i = (current_i + 2) % vec.len();

        if new_i == 0 {
            vec.push(marble);
            current_i = vec.len() - 1;
        } else {
            vec.insert(new_i, marble);
            current_i = new_i;
        }
    }
    *scores.values().max().unwrap_or(&0) as i64
}

fn get_linked(players: usize, last_marble: usize) -> i64 {
    // Uses a linked list. Much faster
    let mut vec = LinkedList::from([0]);
    let mut cur = vec.cursor_front_mut();
    let mut scores: BTreeMap<usize, usize> = BTreeMap::new();
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            let player_i = marble % players;
            for _ in 0..6 {
                cur.move_prev();
                if cur.index().is_none() {
                    cur = vec.cursor_back_mut();
                }
            }
            let t = cur.remove_current().unwrap();
            cur.move_prev();
            *scores.entry(player_i).or_default() += marble + t;
            continue;
        }
        cur.move_next();
        cur.move_next();
        match cur.index() {
            Some(_) => cur.insert_after(marble),
            None => {
                cur = vec.cursor_front_mut();
                cur.insert_after(marble);
            }
        }
    }
    *scores.values().max().unwrap_or(&0) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "9 players; last marble is worth 25 points";
        assert_eq!(read_contents(&a).0, 32);

        let a = "10 players; last marble is worth 1618 points";
        assert_eq!(read_contents(&a).0, 8317);

        let a = "13 players; last marble is worth 7999 points";
        assert_eq!(read_contents(&a).0, 146373);

        let a = "17 players; last marble is worth 1104 points";
        assert_eq!(read_contents(&a).0, 2764);

        let a = "21 players; last marble is worth 6111 points";
        assert_eq!(read_contents(&a).0, 54718);

        let a = "30 players; last marble is worth 5807 points";
        assert_eq!(read_contents(&a).0, 37305);
    }

    #[test]
    fn part2() {
        assert_eq!(get_linked(10, 46), 63);
        assert_eq!(get_linked(10, 100), 107);
        assert_eq!(get_linked(10, 1618), 8317);
        assert_eq!(get_linked(30, 5807), 37305);
    }
}
