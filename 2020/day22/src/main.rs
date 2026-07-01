use clap::Parser;
use std::collections::BTreeSet;
use std::collections::VecDeque;
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

fn read_contents(cont: &str) -> (usize, usize) {
    let mut player_a = VecDeque::new();
    let mut player_b = VecDeque::new();
    let mut vec = &mut player_a;
    for line in cont.lines() {
        if line.starts_with("Player 1") {
            continue;
        }
        if line.starts_with("Player 2") {
            vec = &mut player_b;
            continue;
        }
        if let Ok(res) = line.parse::<usize>() {
            vec.push_back(res);
        }
    }

    let part1 = get_part1(&player_a, &player_b);
    let part2 = get_part2(&player_a, &player_b, 1).1;
    (part1, part2)
}

fn get_hash(vec: &VecDeque<usize>) -> String {
    vec.iter().map(|v| format!("{},", v)).collect::<String>()
}

fn get_result(vec: &VecDeque<usize>) -> usize {
    let n = vec.len();
    vec.iter().enumerate().map(|(i, x)| (n - i) * x).sum()
}

fn get_part1(player_a: &VecDeque<usize>, player_b: &VecDeque<usize>) -> usize {
    let mut a = player_a.clone();
    let mut b = player_b.clone();
    let mut round = 0;

    loop {
        round += 1;
        println!("Round: {}", round);
        if a.is_empty() {
            return get_result(&b);
        }
        if b.is_empty() {
            return get_result(&a);
        }
        let card1 = a.pop_front().unwrap();
        let card2 = b.pop_front().unwrap();
        if card1 > card2 {
            println!("Player A wins with {} against {}", card1, card2);
            a.push_back(card1);
            a.push_back(card2);
        } else {
            println!("Player B wins with {} against {}", card2, card1);
            b.push_back(card2);
            b.push_back(card1);
        }
    }
}

fn get_part2(
    player_a: &VecDeque<usize>,
    player_b: &VecDeque<usize>,
    level: usize,
) -> (usize, usize) {
    let mut a = player_a.clone();
    let mut b = player_b.clone();
    let mut seen = BTreeSet::new();
    let mut round = 0;
    let debug = false;

    loop {
        round += 1;
        if level == 1 {
            println!("Level: {}, Round: {}", level, round);
        }
        if debug {
            println!("    Player 1: {:?}", a);
            println!("    Player 2: {:?}", b);
        }
        let hash = format!("{}-{}", get_hash(&a), get_hash(&b));
        if seen.contains(&hash) {
            return (1, if level == 1 { get_result(&a) } else { 0 });
        }
        seen.insert(hash);

        if a.is_empty() {
            if debug {
                println!("Player 1 out of cards");
            }
            return (2, if level == 1 { get_result(&b) } else { 0 });
        }
        if b.is_empty() {
            if debug {
                println!("Player 2 out of cards");
            }
            return (1, if level == 1 { get_result(&a) } else { 0 });
        }

        let card1 = a.pop_front().unwrap();
        let card2 = b.pop_front().unwrap();
        if debug {
            println!("Cards played: {} - {}", card1, card2);
        }
        if a.len() < card1 || b.len() < card2 {
            // Not enough cards for recursive play
            if card1 > card2 {
                if debug {
                    println!("Player 1 wins with {} against {}", card1, card2);
                }
                a.push_back(card1);
                a.push_back(card2);
            } else {
                if debug {
                    println!("Player 2 wins with {} against {}", card2, card1);
                }
                b.push_back(card2);
                b.push_back(card1);
            }
            continue;
        }
        let deck_a = a.iter().take(card1).copied().collect::<VecDeque<_>>();
        let deck_b = b.iter().take(card2).copied().collect::<VecDeque<_>>();
        if debug {
            println!("Enter recursive play");
            println!("{:?}", deck_a);
            println!("{:?}", deck_b);
        }
        if get_part2(&deck_a, &deck_b, level + 1).0 == 1 {
            if debug {
                println!(
                    "Level: {} Round {}, Player 1 won through recursion",
                    level, round
                );
            }
            a.push_back(card1);
            a.push_back(card2);
        } else {
            if debug {
                println!(
                    "Level {} Round {}: Player 2 won through recursion",
                    level, round
                );
            }
            b.push_back(card2);
            b.push_back(card1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        assert_eq!(read_contents(&a).0, 306);
    }

    #[test]
    fn part2() {
        let a = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        assert_eq!(read_contents(&a).1, 291);
    }
}
