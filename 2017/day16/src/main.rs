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

fn read_contents(cont: &str) -> (String, String) {
    let moves = cont.split(',').map(DanceMove::new).collect::<Vec<_>>();
    let part1 = get_part1(&moves);
    let part2 = get_part2(&moves);
    (part1, part2)
}

#[derive(Debug, PartialEq, Eq)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl DanceMove {
    fn new(ln: &str) -> Self {
        let ln = ln.trim();
        let c = ln.chars().next().unwrap();
        match c {
            's' => Self::Spin(ln[1..].parse().unwrap()),
            'x' => {
                let mut parts = ln[1..].split('/');
                Self::Exchange(
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            }
            'p' => {
                let mut parts = ln[1..].split('/');
                Self::Partner(
                    parts.next().unwrap().chars().next().unwrap(),
                    parts.next().unwrap().chars().next().unwrap(),
                )
            }
            _ => panic!("Invalid dance move"),
        }
    }
}

fn get_part1(moves: &[DanceMove]) -> String {
    let programs: Vec<char> = (b'a'..=b'p').map(|c| c as char).collect();
    reorder(&programs, moves)
}

fn get_part2(moves: &[DanceMove]) -> String {
    let programs: Vec<char> = (b'a'..=b'p').map(|c| c as char).collect();
    let mut vec = programs.to_vec();
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    let mut loop_count = 0;
    loop {
        let tmp = &vec.iter().collect::<String>();
        if seen.contains_key(tmp) {
            println!("Loop detected after {} iterations", loop_count);
            let mut v = seen.iter().collect::<Vec<_>>();
            v.sort_by_key(|&(_, &v)| v);
            let rem = 1_000_000 % loop_count;
            return v.get(rem).unwrap().0.clone();
        }
        seen.insert(vec.iter().collect::<String>(), loop_count);
        loop_count += 1;
        if loop_count == 1_000_000 {
            break;
        }
        for dance_move in moves {
            match dance_move {
                DanceMove::Spin(n) => {
                    for _ in 0..*n {
                        let val = vec.pop().unwrap();
                        vec.insert(0, val);
                    }
                }
                DanceMove::Exchange(a, b) => {
                    (vec[*a], vec[*b]) = (vec[*b], vec[*a]);
                }
                DanceMove::Partner(a, b) => {
                    let pos_a = vec.iter().position(|&c| c == *a).unwrap();
                    let pos_b = vec.iter().position(|&c| c == *b).unwrap();
                    (vec[pos_a], vec[pos_b]) = (vec[pos_b], vec[pos_a]);
                }
            }
        }
    }
    "".to_string()
}

fn reorder(programs: &[char], moves: &[DanceMove]) -> String {
    let mut vec = programs.to_vec();
    for dance_move in moves {
        match dance_move {
            DanceMove::Spin(n) => {
                for _ in 0..*n {
                    let val = vec.pop().unwrap();
                    vec.insert(0, val);
                }
            }
            DanceMove::Exchange(a, b) => {
                (vec[*a], vec[*b]) = (vec[*b], vec[*a]);
            }
            DanceMove::Partner(a, b) => {
                let pos_a = vec.iter().position(|&c| c == *a).unwrap();
                let pos_b = vec.iter().position(|&c| c == *b).unwrap();
                (vec[pos_a], vec[pos_b]) = (vec[pos_b], vec[pos_a]);
            }
        }
    }
    vec.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let programs = vec!['a', 'b', 'c', 'd', 'e'];
        assert_eq!(reorder(&programs, &[DanceMove::Spin(1)]), "eabcd");
        assert_eq!(reorder(&programs, &[DanceMove::Exchange(3, 4)]), "abced");
        assert_eq!(reorder(&programs, &[DanceMove::Partner('e', 'b')]), "aecdb");
        let a = "s1,x3/4,pe/b";
        assert_eq!(
            reorder(
                &programs,
                &a.split(',').map(DanceMove::new).collect::<Vec<_>>()
            ),
            "baedc"
        );
    }

    #[test]
    fn dancemove() {
        assert_eq!(DanceMove::new("s1"), DanceMove::Spin(1));
        assert_eq!(DanceMove::new("x1/2"), DanceMove::Exchange(1, 2));
        assert_eq!(DanceMove::new("pe/b"), DanceMove::Partner('e', 'b'));
    }
}
