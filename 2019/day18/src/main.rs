use clap::Parser;
use colored::Colorize;
use priority_queue::PriorityQueue;
use shared::Dir;
use shared::Vec2D;
use std::cmp::Reverse;
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
    let grid = read_grid(cont);
    let part1 = get_part1(&grid);
    let part2 = get_part2(&grid);
    (part1, part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Object {
    Entrance,
    Empty,
    Wall,
    Key(char),
    Door(char),
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec2D, Object>,
    entrance: Vec2D,
    key_n: usize,
}

//type Grid = BTreeMap<Vec2D, Object>;

impl Grid {
    fn print_grid(&self) {
        let min_x = self.grid.keys().map(|v| v.x).min().unwrap() - 2;
        let max_x = self.grid.keys().map(|v| v.x).max().unwrap() + 2;
        let min_y = self.grid.keys().map(|v| v.y).min().unwrap() - 2;
        let max_y = self.grid.keys().map(|v| v.y).max().unwrap() + 2;

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                match self.grid.get(&Vec2D { x, y }) {
                    Some(Object::Wall) => {
                        print!("{}", "#".blue().on_black());
                    }
                    Some(&Object::Empty) => {
                        print!("{}", ".".black().on_black());
                    }
                    Some(&Object::Entrance) => {
                        print!("{}", "@".red().on_black());
                    }
                    Some(&Object::Key(c)) => {
                        print!("{}", c.to_string().yellow().on_black());
                    }
                    Some(&Object::Door(c)) => {
                        print!("{}", c.to_string().yellow().on_black());
                    }
                    None => {
                        print!("{}", ".".white().on_white());
                    }
                }
            }
            println!();
        }
    }
}

fn read_grid(cont: &str) -> Grid {
    let grid = cont
        .lines()
        .enumerate()
        .fold(BTreeMap::new(), |mut grid, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let obj = match c {
                    '#' => Object::Wall,
                    '.' => Object::Empty,
                    '@' => Object::Entrance,
                    c if c.is_ascii_lowercase() => Object::Key(c),
                    c if c.is_ascii_uppercase() => Object::Door(c),
                    c => panic!("Unknown character: {c} in grid"),
                };
                grid.insert(
                    Vec2D {
                        x: x as i64,
                        y: -(y as i64),
                    },
                    obj,
                );
            });
            grid
        });
    let key_list = grid
        .values()
        .filter_map(|obj| match obj {
            Object::Key(c) => Some(*c),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let entrance = &grid
        .iter()
        .find(|(_, obj)| **obj == Object::Entrance)
        .unwrap()
        .0
        .clone();
    let key_n = key_list.len();
    Grid {
        grid,
        entrance: *entrance,
        key_n,
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct State {
    loc: Vec2D,
    keys: BTreeSet<char>,
    steps: usize,
}

impl State {
    fn keystring(&self) -> String {
        self.keys.iter().collect()
    }

    fn prio(&self, n: usize) -> i64 {
        self.steps as i64 + (n - self.keys.len()) as i64
    }
}

fn get_part1(grid: &Grid) -> i64 {
    grid.print_grid();
    let start_state = State {
        loc: grid.entrance,
        keys: BTreeSet::new(),
        steps: 0,
    };
    let mut visited: BTreeSet<(Vec2D, String)> = BTreeSet::new();
    let mut queue: PriorityQueue<State, _> = PriorityQueue::new();
    let prio = start_state.prio(grid.key_n);
    queue.push(start_state, Reverse(prio));
    visited.insert((grid.entrance, "".to_string()));
    let mut loop_count = 0;
    let mut max_keys = 0;
    loop {
        if queue.is_empty() {
            break;
        }
        loop_count += 1;
        let (state, _prio) = queue.pop().unwrap();
        if state.keys.len() > max_keys {
            max_keys = state.keys.len();
            println!("Loop: {loop_count}");
            println!(
                "State: steps: {}, Keys: {} ({} / {})",
                state.steps,
                state.keystring(),
                state.keys.len(),
                grid.key_n
            );
        }
        let loc = state.loc;
        for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
            let dx = dir.get_dir_true_vec();
            let new_loc = dx + loc;
            let mut found_key: Option<char> = None;
            match grid.grid.get(&new_loc).unwrap_or(&Object::Wall) {
                Object::Empty => {}
                Object::Wall => {
                    continue;
                }
                Object::Entrance => {}
                Object::Key(c) => {
                    found_key = Some(*c);
                }
                Object::Door(c) if state.keys.contains(&c.to_ascii_lowercase()) => {}
                Object::Door(_) => {
                    continue;
                }
            }
            //println!("Moving from {:?} to {:?} with keys {:?} and prio {:?}", loc, new_loc, state.keys, prio);
            let new_state = State {
                loc: new_loc,
                keys: match found_key {
                    Some(k) => state
                        .keys
                        .iter()
                        .cloned()
                        .chain(std::iter::once(k))
                        .collect(),
                    None => state.keys.clone(),
                },
                steps: state.steps + 1,
            };
            if visited.contains(&(new_loc, new_state.keystring())) {
                continue;
            }
            if new_state.keys.len() == grid.key_n {
                println!(
                    "Found solution with {} steps, keys: {}",
                    state.steps,
                    state.keystring()
                );
                return new_state.steps as i64;
            }
            visited.insert((new_state.loc, new_state.keystring()));
            let prio = new_state.prio(grid.key_n);
            queue.push(new_state, Reverse(prio));
        }
    }
    0
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct State2 {
    locs: Vec<Vec2D>,
    keys: BTreeSet<char>,
    steps: usize,
    prev_move: Option<usize>,
}

impl State2 {
    fn keystring(&self) -> String {
        self.keys.iter().collect()
    }

    fn prio(&self, n: usize) -> i64 {
        self.steps as i64 + (n - self.keys.len()) as i64
    }
}

fn get_part2(grid: &Grid) -> i64 {
    let mut grid = grid.clone();
    let entrance = grid.entrance;
    grid.grid.insert(entrance + (0, 0), Object::Wall);
    grid.grid.insert(entrance + (1, 0), Object::Wall);
    grid.grid.insert(entrance + (-1, 0), Object::Wall);
    grid.grid.insert(entrance + (0, 1), Object::Wall);
    grid.grid.insert(entrance + (0, -1), Object::Wall);

    grid.grid.insert(entrance + (1, 1), Object::Entrance);
    grid.grid.insert(entrance + (1, -1), Object::Entrance);
    grid.grid.insert(entrance + (-1, 1), Object::Entrance);
    grid.grid.insert(entrance + (-1, -1), Object::Entrance);
    grid.print_grid();

    let start_state = State2 {
        locs: vec![
            entrance + (1, 1),
            entrance + (1, -1),
            entrance + (-1, 1),
            entrance + (-1, -1),
        ],
        keys: BTreeSet::new(),
        steps: 0,
        prev_move: None,
    };
    let mut visited: BTreeSet<(Vec<Vec2D>, String)> = BTreeSet::new();
    let mut queue: PriorityQueue<State2, _> = PriorityQueue::new();
    let prio = start_state.prio(grid.key_n);
    visited.insert((start_state.locs.clone(), "".to_string()));
    queue.push(start_state, Reverse(prio));
    let mut loop_count = 0;
    let mut max_keys = 0;
    loop {
        if queue.is_empty() {
            break;
        }
        loop_count += 1;
        let (state, _prio) = queue.pop().unwrap();
        if state.keys.len() > max_keys {
            max_keys = state.keys.len();
            println!("Loop: {loop_count}");
            println!(
                "State: steps: {}, Keys: {} ({} / {})",
                state.steps,
                state.keystring(),
                state.keys.len(),
                grid.key_n
            );
        }
        for i in 0..4 {
            if state.prev_move.is_some() && state.prev_move != Some(i) {
                continue;
            }
            let loc = state.locs[i];
            for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
                let dx = dir.get_dir_true_vec();
                let new_loc = dx + loc;
                let mut found_key: Option<char> = None;
                match grid.grid.get(&new_loc).unwrap_or(&Object::Wall) {
                    Object::Empty => {}
                    Object::Wall => {
                        continue;
                    }
                    Object::Entrance => {}
                    Object::Key(c) => {
                        found_key = Some(*c);
                    }
                    Object::Door(c) if state.keys.contains(&c.to_ascii_lowercase()) => {}
                    Object::Door(_c) => {
                        continue;
                    }
                }
                //println!("Moving from {:?} to {:?} with keys {:?} and prio {:?}", loc, new_loc, state.keys, prio);
                let mut new_locs = state.locs.clone();
                new_locs[i] = new_loc;
                if visited.contains(&(new_locs.clone(), state.keystring())) {
                    continue;
                }
                let new_state = State2 {
                    locs: new_locs,
                    keys: match found_key {
                        Some(k) => state
                            .keys
                            .iter()
                            .cloned()
                            .chain(std::iter::once(k))
                            .collect(),
                        None => state.keys.clone(),
                    },
                    steps: state.steps + 1,
                    prev_move: if found_key.is_some() { None } else { Some(i) },
                };
                if new_state.keys.len() == grid.key_n {
                    println!(
                        "Found solution with {} steps, keys: {}",
                        state.steps,
                        state.keystring()
                    );
                    return new_state.steps as i64;
                }
                visited.insert((new_state.locs.clone(), new_state.keystring()));
                let prio = new_state.prio(grid.key_n);
                queue.push(new_state, Reverse(prio));
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1a() {
        let a = "
#########
#b.A.@.a#
#########";

        let grid = read_grid(a);
        grid.print_grid();
        assert_eq!(get_part1(&grid), 8);
    }

    #[test]
    fn part1b() {
        let b = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        let grid = read_grid(b);
        assert_eq!(get_part1(&grid), 86);
    }

    #[test]
    fn part1c() {
        let c = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        let grid = read_grid(c);
        assert_eq!(get_part1(&grid), 132);
    }

    #[test]
    fn part1d() {
        let d = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

        let grid = read_grid(d);
        assert_eq!(get_part1(&grid), 136);
    }

    #[test]
    fn part1e() {
        let e = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

        let grid = read_grid(e);
        assert_eq!(get_part1(&grid), 81);
    }

    #[test]
    fn part2a() {
        let a = "###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############";
        let grid = read_grid(a);
        assert_eq!(get_part2(&grid), 24);
    }
    #[test]
    fn part2b() {
        let b = "#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############";
        let grid = read_grid(b);
        assert_eq!(get_part2(&grid), 32);
    }

    #[test]
    fn part2c() {
        let c = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";
        let grid = read_grid(c);
        assert_eq!(get_part2(&grid), 72);
    }
}
