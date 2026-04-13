use clap::Parser;

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
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (String, usize) {
    let check = cont.trim();
    let part1 = get_part1(check);
    let part2 = get_part2(check);

    (part1, part2)
}

fn get_part1(check: &str) -> String {
    let mut queue = VecDeque::new();
    let start = (0, 0);
    let target = (3, 3);
    let mut state = State {
        steps: Vec::new(),
        loc: start,
        hash: String::new(),
        check: check.to_string(),
    };
    state.update_hash();
    queue.push_back(state);

    loop {
        if queue.is_empty() {
            println!("Queue is empty");
            return "".to_string();
        }
        let state = queue.pop_front().unwrap();
        // println!("At {} {}", state.loc.0, state.loc.1);
        if state.loc == target {
            println!("Found path with length {}", state.steps.len());
            return state.steps.iter().map(|d| d.char()).collect();
        }
        for dir in state.get_allowed_directions() {
            let mut new_state = state.clone();
            new_state.add_step(dir);
            queue.push_back(new_state);
        }
    }
}

fn get_part2(check: &str) -> usize {
    let mut queue = VecDeque::new();
    let start = (0, 0);
    let target = (3, 3);
    let mut max_steps = 0;
    let mut state = State {
        steps: Vec::new(),
        loc: start,
        hash: String::new(),
        check: check.to_string(),
    };
    state.update_hash();
    queue.push_back(state);

    loop {
        if queue.is_empty() {
            println!("Queue is empty");
            return max_steps;
        }
        let state = queue.pop_back().unwrap();
        // println!("At {} {}", state.loc.0, state.loc.1);
        if state.loc == target {
            if state.steps.len() >= max_steps {
                println!("Found path with length {}", state.steps.len());
                max_steps = state.steps.len();
            }
            continue;
        }
        for dir in state.get_allowed_directions() {
            let mut new_state = state.clone();
            new_state.add_step(dir);
            queue.push_back(new_state);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn char(&self) -> char {
        match self {
            Dir::Up => 'U',
            Dir::Down => 'D',
            Dir::Right => 'R',
            Dir::Left => 'L',
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    steps: Vec<Dir>,
    loc: (i32, i32),
    hash: String,
    check: String,
}

impl State {
    fn add_step(&mut self, dir: Dir) {
        self.steps.push(dir);
        match dir {
            Dir::Up => self.loc.1 -= 1,
            Dir::Down => self.loc.1 += 1,
            Dir::Left => self.loc.0 -= 1,
            Dir::Right => self.loc.0 += 1,
        }
        assert!(self.loc.0 >= 0 && self.loc.0 <= 3 && self.loc.1 >= 0 && self.loc.1 <= 3);
        self.update_hash();
    }

    fn check_direction(&self, dir: Dir) -> bool {
        match dir {
            Dir::Up => self.hash.chars().nth(0).unwrap() > 'a',
            Dir::Down => self.hash.chars().nth(1).unwrap() > 'a',
            Dir::Left => self.hash.chars().nth(2).unwrap() > 'a',
            Dir::Right => self.hash.chars().nth(3).unwrap() > 'a',
        }
    }

    fn get_allowed_directions(&self) -> Vec<Dir> {
        let mut res = Vec::new();
        for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let new_xy = match dir {
                Dir::Up => self.loc.1 - 1,
                Dir::Down => self.loc.1 + 1,
                Dir::Left => self.loc.0 - 1,
                Dir::Right => self.loc.0 + 1,
            };
            if !(0..=3).contains(&new_xy) {
                continue;
            }
            if self.check_direction(dir) {
                res.push(dir);
            }
        }
        res
    }

    fn update_hash(&mut self) {
        let mut input = self.check.clone();
        for step in &self.steps {
            input.push(step.char());
        }
        self.hash = format!("{:x}", md5::compute(input));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut state = State {
            steps: Vec::new(),
            loc: (0, 0),
            hash: String::new(),
            check: "hijkl".to_string(),
        };
        state.update_hash();
        // Check direction only checks if door is open based on the hash
        // It does not check against outer walls
        assert_eq!(state.check_direction(Dir::Up), true);
        assert_eq!(state.check_direction(Dir::Down), true);
        assert_eq!(state.check_direction(Dir::Left), true);
        assert_eq!(state.check_direction(Dir::Right), false);

        state.add_step(Dir::Down);
        assert_eq!(state.check_direction(Dir::Up), true);
        assert_eq!(state.check_direction(Dir::Down), false);
        assert_eq!(state.check_direction(Dir::Left), true);
        assert_eq!(state.check_direction(Dir::Right), true);

        state.add_step(Dir::Up);
        assert_eq!(state.check_direction(Dir::Up), false);
        assert_eq!(state.check_direction(Dir::Down), false);
        assert_eq!(state.check_direction(Dir::Left), false);
        assert_eq!(state.check_direction(Dir::Right), true);

        state.add_step(Dir::Right);
        assert_eq!(state.check_direction(Dir::Up), false);
        assert_eq!(state.check_direction(Dir::Down), false);
        assert_eq!(state.check_direction(Dir::Left), false);
        assert_eq!(state.check_direction(Dir::Right), false);

        assert_eq!(get_part1(&"hijkl"), "");

        let mut state = State {
            steps: Vec::new(),
            loc: (0, 0),
            hash: String::new(),
            check: "ihgpwlah".to_string(),
        };
        state.update_hash();
        assert_eq!(state.check_direction(Dir::Down), true);
        state.add_step(Dir::Down);
        assert_eq!(state.check_direction(Dir::Down), true);
        state.add_step(Dir::Down);
        assert_eq!(state.check_direction(Dir::Right), true);
        state.add_step(Dir::Right);
        assert_eq!(state.check_direction(Dir::Right), true);
        state.add_step(Dir::Right);
        assert_eq!(state.check_direction(Dir::Down), true);
        state.add_step(Dir::Down);

        assert_eq!(get_part1(&"ihgpwlah"), "DDRRRD");
        assert_eq!(get_part1(&"ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
        assert_eq!(get_part1(&"kglvqrro"), "DDUDRLRRUDRD");
    }

    #[test]
    fn part2() {
        assert_eq!(get_part2(&"ihgpwlah"), 370);
        assert_eq!(get_part2(&"ulqzkmiv"), 830);
        assert_eq!(get_part2(&"kglvqrro"), 492);
    }
}
