use clap::Parser;
use priority_queue::PriorityQueue;
use shared::Dir;
use shared::Vec2D;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;
use strum::IntoEnumIterator;

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
    let res = read_contents(&contents, Vec2D { x: 31, y: 39 });
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str, target: Vec2D) -> (i64, i64) {
    let input = cont.trim().parse::<i64>().unwrap();

    let state = State {
        steps: 0,
        loc: Vec2D::new(1, 1),
    };
    let part1 = find_route(&state, &target, input);
    let part2 = count_locs(&state, 50, input);
    (part1, part2)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    loc: Vec2D,
    steps: usize,
}

fn is_wall(loc: Vec2D, check: i64) -> bool {
    let x = loc.x;
    let y = loc.y;
    if x < 0 || y < 0 {
        return true;
    }
    let z = x * x + 3 * x + 2 * x * y + y + y * y + check;
    !z.count_ones().is_multiple_of(2)
}

fn find_route(start: &State, target: &Vec2D, check: i64) -> i64 {
    let mut queue = PriorityQueue::new();
    let manhattan = start.loc.manhattan(target);
    queue.push(start.clone(), -manhattan);
    loop {
        if queue.is_empty() {
            return 0;
        }
        let (state, _) = queue.pop().unwrap();

        if state.loc == *target {
            println!("Found target in {} steps", state.steps);
            return state.steps as i64;
        }
        for dir in Dir::iter() {
            let new_loc = state.loc + dir.get_dir_true_vec();

            if is_wall(new_loc, check) {
                continue;
            }
            let new_state = State {
                loc: new_loc,
                steps: state.steps + 1,
            };
            let manhattan = new_state.loc.manhattan(target);
            let priority = -(new_state.steps as i64 + manhattan);
            queue.push(new_state, priority);
        }
    }
}

fn count_locs(start: &State, steps: usize, check: i64) -> i64 {
    // No need for a priority queue here, just a regular fifo queue will do
    let mut queue = VecDeque::new();
    let mut visited: BTreeSet<Vec2D> = BTreeSet::new();
    queue.push_back(start.clone());
    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        if state.steps == steps {
            continue;
        }
        for dir in Dir::iter() {
            let new_loc = state.loc + dir.get_dir_true_vec();
            if is_wall(new_loc, check) {
                continue;
            }
            let new_state = State {
                loc: new_loc,
                steps: state.steps + 1,
            };
            if visited.contains(&new_state.loc) {
                continue;
            }
            visited.insert(new_state.loc);
            queue.push_back(new_state);
        }
    }
    visited.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "10";
        assert_eq!(read_contents(&a, Vec2D { x: 7, y: 4 }).0, 11);
    }

    #[test]
    fn valid() {
        assert_eq!(is_wall(Vec2D::new(0, 0), 10), false);
        assert_eq!(is_wall(Vec2D::new(0, 1), 10), false);
        assert_eq!(is_wall(Vec2D::new(2, 0), 10), false);
        assert_eq!(is_wall(Vec2D::new(9, 5), 10), false);
        assert_eq!(is_wall(Vec2D::new(1, 0), 10), true);
    }
}
