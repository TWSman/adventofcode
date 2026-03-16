use clap::Parser;

use colored::Colorize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
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

fn read_contents(cont: &str) -> (i64, i64) {
    let state = State::new(cont);
    state.print_grid();
    let part1 = get_part1(&state);
    let part2 = get_part2(&state, 200);
    (part1, part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BugState {
    Empty,
    Bug,
}

#[derive(Debug, Clone)]
struct State {
    map: BTreeMap<(isize, isize), BugState>,
}

impl State {
    fn new(cont: &str) -> Self {
        let map = read_map(cont);
        let width = map.keys().map(|(x, _)| *x).max().unwrap_or(0) + 1
            - map.keys().map(|(x, _)| *x).min().unwrap_or(0);
        let height = map.keys().map(|(_, y)| *y).max().unwrap_or(0) + 1
            - map.keys().map(|(_, y)| *y).min().unwrap_or(0);
        assert_eq!(width, 5);
        assert_eq!(height, 5);
        Self { map }
    }

    fn print_grid(&self) {
        let max_x = self.map.keys().map(|(x, _)| *x).max().unwrap_or(0);
        let max_y = self.map.keys().map(|(_, y)| *y).max().unwrap_or(0);
        for y in 0..=max_y {
            for x in 0..=max_x {
                match self.map.get(&(x, y)) {
                    Some(BugState::Empty) => {
                        print!("{}", ".".red().on_white());
                    }
                    Some(BugState::Bug) => {
                        print!("{}", '#'.to_string().red().on_white());
                    }
                    None => {
                        print!("{}", ".".black().on_black());
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn stringify(&self) -> String {
        self.map
            .values()
            .map(|state| match state {
                BugState::Empty => '.',
                BugState::Bug => '#',
            })
            .collect::<String>()
    }

    fn evolve(&mut self) -> String {
        let mut new_map = self.map.clone();
        for x in -2..=2 {
            for y in -2..=2 {
                let mut neighbors = 0;
                // Check neighbors, left, right, up and down
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    if let Some(BugState::Bug) = self.map.get(&(x + dx, y + dy)) {
                        neighbors += 1;
                    }
                }
                if self.contains_bug(x, y) && neighbors != 1 {
                    // Bug dies if it has exactly one neighbor
                    new_map.insert((x, y), BugState::Empty);
                }
                if !self.contains_bug(x, y) && (neighbors == 1 || neighbors == 2) {
                    new_map.insert((x, y), BugState::Bug);
                }
            }
        }
        self.map = new_map;
        self.stringify()
    }

    fn contains_bug(&self, x: isize, y: isize) -> bool {
        self.map.get(&(x, y)).unwrap_or(&BugState::Empty) == &BugState::Bug
    }

    fn get_biodiversity(&self) -> i64 {
        self.map
            .iter()
            .map(|((x, y), state)| {
                let ind = (y + 2) * 5 + x + 2;
                2_i64.pow(ind as u32) * if state == &BugState::Bug { 1 } else { 0 }
            })
            .sum()
    }
}

#[derive(Debug, Clone)]
struct RecursiveState {
    // 0,0 will be the center of the grid, and z will be the depth level
    bugs: BTreeSet<(isize, isize, isize)>,
    max_level: isize,
    min_level: isize,
    neighbor_cache: BTreeMap<(isize, isize, isize), isize>,
}

impl RecursiveState {
    fn from_state(state: &State) -> Self {
        let bugs = state
            .map
            .iter()
            .filter_map(|((x, y), bug_state)| {
                if bug_state == &BugState::Bug {
                    Some((*x, *y, 0))
                } else {
                    None
                }
            })
            .collect();
        Self {
            bugs,
            max_level: 0,
            min_level: 0,
            neighbor_cache: BTreeMap::new(),
        }
    }

    fn evolve(&mut self) {
        let mut new_map = self.bugs.clone();
        let mut neighbor_cache = BTreeMap::new();
        for level in (self.min_level - 1)..=(self.max_level + 1) {
            let outer_level = level - 1;
            let inner_level = level + 1;
            for x in -2..=2 {
                for y in -2..=2 {
                    if (x, y) == (0, 0) {
                        continue; // Skip the center cell
                    }
                    let mut neighbors = 0;
                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let (xx, yy) = (x + dx, y + dy);
                        if (xx, yy) == (0, 0) {
                            // Check inwards
                            match (dx, dy) {
                                (-1, 0) | (1, 0) => {
                                    // Dx is negative/positive, check right/left side
                                    for y in -2..=2 {
                                        if self.bugs.contains(&(-2 * dx, y, inner_level)) {
                                            neighbors += 1;
                                        }
                                    }
                                }
                                (0, -1) | (0, 1) => {
                                    // Dy is negative/positive check bottom/upper side
                                    for x in -2..=2 {
                                        if self.bugs.contains(&(x, -2 * dy, inner_level)) {
                                            neighbors += 1;
                                        }
                                    }
                                }
                                _ => panic!("Invalid direction"),
                            }
                        }
                        // Check outwards
                        else if xx == -3 && self.bugs.contains(&(-1, 0, outer_level)) {
                            // Dx is negative, check innermost right side of outer level
                            neighbors += 1;
                        } else if xx == 3 && self.bugs.contains(&(1, 0, outer_level)) {
                            // Dx is positive, check innermost left side of outer level
                            neighbors += 1;
                        } else if yy == -3 && self.bugs.contains(&(0, -1, outer_level)) {
                            // Dy is negative, check innermost bottom side of outer level
                            neighbors += 1;
                        } else if yy == 3 && self.bugs.contains(&(0, 1, outer_level)) {
                            // Dy is positive, check innermost upper side of outer level
                            neighbors += 1;
                        } else if self.bugs.contains(&(xx, yy, level)) {
                            // Normal situation
                            neighbors += 1;
                        }
                    }

                    if self.bugs.contains(&(x, y, level)) {
                        neighbor_cache.insert((x, y, level), neighbors);
                    } else if neighbors == 0 {
                        neighbor_cache.insert((x, y, level), 99);
                    } else {
                        neighbor_cache.insert((x, y, level), -neighbors);
                    }

                    // Bug dies if it has exactly one neighbor
                    if self.bugs.contains(&(x, y, level)) && neighbors != 1 {
                        new_map.remove(&(x, y, level));
                    }

                    // Empty space spawns a bug if it has 1 or 2 neighboring bugs
                    if !self.bugs.contains(&(x, y, level)) && (neighbors == 1 || neighbors == 2) {
                        new_map.insert((x, y, level));
                        if level > self.max_level {
                            self.max_level = level;
                        }
                        if level < self.min_level {
                            self.min_level = level;
                        }
                    }
                }
            }
        }
        self.neighbor_cache = neighbor_cache;
        self.bugs = new_map;
    }

    fn print_grid(&self, min_level: isize, max_level: isize) {
        println!("Negative are outside, positive are inside");
        for level in min_level..=max_level {
            print!("Lv {:>2}  ", level)
        }
        println!();
        for y in -2..=2 {
            for level in min_level..=max_level {
                for x in -2..=2 {
                    if y == 0 && x == 0 {
                        print!("{}", '?'.to_string().red().on_white());
                    } else if self.bugs.contains(&(x, y, level)) {
                        print!("{}", '#'.to_string().red().on_white());
                    } else {
                        print!("{}", ".".red().on_white());
                    }
                }
                print!("  ");
            }
            println!();
        }
        println!();
    }

    fn print_neighbor_cache(&self, min_level: isize, max_level: isize) {
        println!("Negative are outside, positive are inside");
        for level in min_level..=max_level {
            print!("Lv {:>2}  ", level)
        }
        println!();
        for y in -2..=2 {
            for level in min_level..=max_level {
                for x in -2..=2 {
                    if y == 0 && x == 0 {
                        print!("{}", '?'.to_string().red().on_white());
                    } else {
                        let neighbors = self.neighbor_cache.get(&(x, y, level)).unwrap_or(&99);
                        match neighbors {
                            99 => print!("{}", "-".to_string().red().on_white()),
                            c if *c < 0 => print!("{}", (-c).to_string().blue().on_white()),
                            // There is a bug
                            c if *c >= 0 => print!("{}", c.to_string().red().on_white()),
                            c => print!("{}", c.to_string().blue().on_white()),
                        }
                    }
                }
                print!("  ");
            }
            println!();
        }
        println!();
    }
}

fn get_part1(state: &State) -> i64 {
    let mut state = state.clone();

    let mut seen = std::collections::HashSet::new();
    let mut loop_count = 0;
    loop {
        loop_count += 1;
        if loop_count > 1_000_000 {
            break;
        }
        let res = state.evolve();
        //state.print_grid();
        if seen.contains(&res) {
            println!("Found duplicate after {} iterations", loop_count);
            state.print_grid();
            return state.get_biodiversity();
        } else {
            seen.insert(res);
        }
    }
    0
}

fn get_part2(state: &State, time: usize) -> i64 {
    let mut state = RecursiveState::from_state(state);
    state.print_grid(0, 0);
    for i in 0..time {
        state.evolve();
        if i <= 7 {
            println!("After {} minutes:", i + 1);
            state.print_neighbor_cache(-1, 1);
            state.print_grid(-1, 1);
        }
    }
    state.print_grid(-6, 6);
    println!("Bugs on levels {} - {}", state.min_level, state.max_level);
    state.bugs.len() as i64
}

fn read_map(cont: &str) -> BTreeMap<(isize, isize), BugState> {
    cont.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let state = match c {
                    '.' => BugState::Empty,
                    '#' => BugState::Bug,
                    _ => panic!("Invalid character in map"),
                };
                ((x as isize - 2, y as isize - 2), state) // Coordinates are transformed such that center of the 5x5 grid is (0,0)
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "....#
#..#.
#..##
..#..
#....";

        assert_eq!(read_contents(&a).0, 2129920);
    }

    #[test]
    fn part2() {
        let a = "....#
#..#.
#..##
..#..
#....";

        let state = State::new(&a);
        assert_eq!(get_part2(&state, 10), 99);
    }
}
