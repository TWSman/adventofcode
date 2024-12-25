use std::fs;
use std::collections::BTreeSet;
use clap::Parser;
use shared::{Dir, Diag};
use strum::IntoEnumIterator; // 0.17.1

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

struct Move {
    direction: Dir,
    count: u8,
}

fn read_contents(cont: &str) -> (u64, u64) {
    let moves = cont.lines().map(|ln| {
        let mut spl = ln.split_whitespace(); 
        let direction = match spl.next() {
            Some("U") => Dir::N,
            Some("D") => Dir::S,
            Some("R") => Dir::E,
            Some("L") => Dir::W,
            _ => panic!("Something wrong"),
        };
        let count = spl.next().unwrap().parse::<u8>().unwrap();
        Move {direction, count}
    }).collect::<Vec<Move>>();
    (get_results(&moves, 1), get_results(&moves, 9))
}

struct Rope {
    head: (i64, i64),
    node_count: usize,
    nodes: Vec<(i64, i64)>,
    visited: BTreeSet<(i64, i64)>,
}

const fn sum_vec(x: (i64, i64), y: (i64, i64)) -> (i64, i64) {
    (x.0 + y.0, x.1 + y.1)
}

fn get_col_distance(x: (i64, i64), y: (i64, i64)) -> i64 {
    let dx = i64::abs(x.0 - y.0);
    let dy = i64::abs(x.1 - y.1);
    i64::max(dx, dy)

}

impl Rope {
    fn new(node_count: usize) -> Self {
        // node count should not include the head
        let nodes = vec![(0,0); node_count];
        let mut s = Self {head: (0,0), node_count, nodes, visited: BTreeSet::new()};
        s.visited.insert((0,0));
        s
    }

    fn move_head(&mut self, direction: Dir) {
        let d = direction.get_dir();
        self.head = sum_vec(self.head, d);
        let mut prev_node = self.head;
        for n in 0..self.node_count {
            let node_loc = *self.nodes.get(n).unwrap();
            let dd = get_col_distance(prev_node, node_loc);
            if dd == 2 {
                let mut new_node_loc = None;
                for d2 in Dir::iter() {
                    let candidate = sum_vec(prev_node, d2.get_dir());
                    if get_col_distance(candidate, node_loc) == 1 {
                        new_node_loc = Some(candidate);
                    }
                }
                // None of the cardinal distances produced a results,
                // Check diagonals next
                if new_node_loc.is_none() {
                    for d2 in Diag::iter() {
                        let candidate = sum_vec(prev_node, d2.get_dir());
                        if get_col_distance(candidate, node_loc) == 1 {
                            new_node_loc = Some(candidate);
                        }
                    }
                }
                self.nodes[n] = new_node_loc.unwrap();
                prev_node = new_node_loc.unwrap();
            } else if dd > 2 {
                // Something has gone wrong
                panic!("Distance is too large");
            } else {
                prev_node = node_loc;
            }
        }
        self.visited.insert(self.nodes[self.node_count-1]);
    }
}

fn get_results(moves: &Vec<Move>, node_count: usize) -> u64 {
    let mut rope = Rope::new(node_count);
    for m in moves {
        let c = m.count;
        for _ in 0..c {
            rope.move_head(m.direction);
        }
    }
    rope.visited.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {

        let a = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(read_contents(&a).0, 13);
        assert_eq!(read_contents(&a).1, 1);

    }

    #[test]
    fn part2() {
        let a = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(read_contents(&a).1, 36);
    }

}
