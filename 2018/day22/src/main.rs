use clap::Parser;
use colored::Colorize;
use priority_queue::PriorityQueue;
use shared::Dir;
use shared::Vec2D;
use std::cmp::Reverse;
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
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut depth = 0;
    let mut target = Vec2D { x: 0, y: 0 };
    for line in cont.lines() {
        match line.split_once(':') {
            None => continue,
            Some(("depth", b)) => {
                depth = b.trim().parse::<i64>().unwrap();
            }
            Some(("target", b)) => {
                let (x, y) = b.split_once(',').unwrap();
                target = Vec2D {
                    x: x.trim().parse::<i64>().unwrap(),
                    y: y.trim().parse::<i64>().unwrap(),
                }
            }
            _ => panic!(),
        }
    }
    let mut cave = get_cave(depth, target);
    print_grid(&cave.grid, 10, 10);
    let part1 = get_part1(&cave.grid);
    let part2 = get_part2(&mut cave, target);
    (part1, part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Object {
    Rocky,
    Narrow,
    Wet,
}

impl Object {
    fn risk_level(&self) -> i64 {
        match self {
            Object::Rocky => 0,
            Object::Wet => 1,
            Object::Narrow => 2,
        }
    }
}

#[derive(Debug, Clone)]
struct Cave {
    grid: BTreeMap<Vec2D, Object>,
    erosion: BTreeMap<Vec2D, i64>,
    cave_depth: i64,
}

impl Cave {
    fn get(&mut self, loc: &Vec2D) -> Object {
        if let Some(x) = self.grid.get(loc) {
            return *x;
        }
        let erosion = self.get_erosion(loc);
        let obj = match erosion % 3 {
            0 => Object::Rocky,
            1 => Object::Wet,
            2 => Object::Narrow,
            _ => panic!(),
        };
        self.grid.insert(*loc, obj);
        obj
    }

    fn get_erosion(&mut self, loc: &Vec2D) -> i64 {
        // println!("Get erosion {:?}", loc);
        if let Some(x) = self.erosion.get(loc) {
            return *x;
        }
        let x = loc.x;
        let y = loc.y;
        let erosion = if x == 0 {
            get_erosion(y * 48271, self.cave_depth)
        } else if y == 0 {
            get_erosion(x * 16807, self.cave_depth)
        } else {
            let a = self.get_erosion(&Vec2D { x: x - 1, y });
            let b = self.get_erosion(&Vec2D { x, y: y - 1 });
            get_erosion(a * b, self.cave_depth)
        };
        self.erosion.insert(Vec2D { x, y }, erosion);
        erosion
    }
}

fn print_grid(grid: &BTreeMap<Vec2D, Object>, max_x: i64, max_y: i64) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            match grid.get(&Vec2D { x, y }) {
                Some(Object::Rocky) => {
                    print!("{}", ".".red().on_black());
                }
                Some(Object::Narrow) => {
                    print!("{}", "|".white().on_black());
                }
                Some(Object::Wet) => {
                    print!("{}", "=".blue().on_black());
                }
                None => {
                    print!("{}", " ".white().on_white());
                }
            }
        }
        println!();
    }
}

fn get_cave(cave_depth: i64, target: Vec2D) -> Cave {
    let mut erosion = BTreeMap::new();
    let max_x = target.x;
    let max_y = target.y;
    erosion.insert(Vec2D { x: 0, y: 0 }, get_erosion(0, cave_depth));
    erosion.insert(target, get_erosion(0, cave_depth));
    for x in 1..=max_x {
        erosion.insert(Vec2D { x, y: 0 }, get_erosion(x * 16807, cave_depth));
    }
    for y in 1..=max_y {
        erosion.insert(Vec2D { x: 0, y }, get_erosion(y * 48271, cave_depth));
    }
    for x in 1..=max_x {
        for y in 1..=max_y {
            if x == max_x && y == max_y {
                continue;
            }
            let a = erosion.get(&Vec2D { x: x - 1, y }).unwrap();
            let b = erosion.get(&Vec2D { x, y: y - 1 }).unwrap();
            erosion.insert(Vec2D { x, y }, get_erosion(a * b, cave_depth));
        }
    }
    let grid = erosion
        .iter()
        .map(|(i, v)| match v % 3 {
            0 => (*i, Object::Rocky),
            1 => (*i, Object::Wet),
            2 => (*i, Object::Narrow),
            _ => panic!(),
        })
        .collect::<BTreeMap<_, _>>();
    Cave {
        grid,
        erosion,
        cave_depth,
    }
}

fn get_erosion(geo_index: i64, depth: i64) -> i64 {
    (geo_index + depth) % 20183
}

fn get_part1(grid: &BTreeMap<Vec2D, Object>) -> i64 {
    grid.values().map(|v| v.risk_level()).sum()
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
enum Gear {
    Torch,
    Climb,
    Nothing,
}

impl Gear {
    fn valid(&self, obj: &Object) -> bool {
        match (self, obj) {
            (Gear::Torch, Object::Rocky) => true,
            (Gear::Torch, Object::Wet) => false,
            (Gear::Torch, Object::Narrow) => true,
            (Gear::Nothing, Object::Rocky) => false,
            (Gear::Nothing, Object::Wet) => true,
            (Gear::Nothing, Object::Narrow) => true,
            (Gear::Climb, Object::Wet) => true,
            (Gear::Climb, Object::Rocky) => true,
            (Gear::Climb, Object::Narrow) => false,
        }
    }

    fn get_valid_gear(start: Object, end: Object) -> Self {
        match (start, end) {
            (Object::Rocky, Object::Wet) => Gear::Climb,
            (Object::Rocky, Object::Narrow) => Gear::Torch,
            (Object::Wet, Object::Narrow) => Gear::Nothing,
            (Object::Wet, Object::Rocky) => Gear::Climb,
            (Object::Narrow, Object::Wet) => Gear::Nothing,
            (Object::Narrow, Object::Rocky) => Gear::Torch,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    loc: Vec2D,
    time: i64,
    gear: Gear,
}

fn get_part2(cave: &mut Cave, target: Vec2D) -> i64 {
    let mut seen: BTreeMap<(Vec2D, Gear), i64> = BTreeMap::new();
    let mut queue: PriorityQueue<State, Reverse<i64>> = PriorityQueue::new();
    let state = State {
        loc: Vec2D { x: 0, y: 0 },
        time: 0,
        gear: Gear::Torch,
    };
    let manhattan = target.manhattan(&state.loc);
    let mut loop_count = 0;
    queue.push(state, Reverse(manhattan));
    let mut best_time = 9999;
    loop {
        if queue.is_empty() {
            break;
        }
        loop_count += 1;
        if loop_count > 2_000_000_000 {
            println!("Too many loops");
            break;
        }
        let (state, _) = queue.pop().unwrap();
        if let Some(t) = seen.get(&(state.loc, state.gear))
            && *t <= state.time
        {
            continue;
        }
        seen.insert((state.loc, state.gear), state.time);
        if state.time > best_time {
            break;
        }
        if state.loc == target && state.gear == Gear::Torch {
            best_time = state.time;
            continue;
        }
        if state.loc == target {
            best_time = state.time + 7;
            continue;
        }
        let obj = cave.get(&state.loc);
        for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
            let new_loc = state.loc + dir.get_dir_true_vec();
            if new_loc.x < 0 || new_loc.y < 0 {
                continue;
            }
            let new_obj = cave.get(&new_loc);
            let new_state = if state.gear.valid(&new_obj) {
                State {
                    loc: new_loc,
                    time: state.time + 1,
                    gear: state.gear,
                }
            } else {
                let new_gear = Gear::get_valid_gear(obj, new_obj);
                State {
                    loc: new_loc,
                    time: state.time + 8,
                    gear: new_gear,
                }
            };
            let manhattan = new_state.loc.manhattan(&target);
            let prio =
                manhattan + new_state.time + if new_state.gear == Gear::Torch { 0 } else { 7 };
            queue.push(new_state, Reverse(prio));
        }
    }
    best_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "depth: 510
target: 10,10";
        assert_eq!(read_contents(&a).0, 114);
    }

    #[test]
    fn part2() {
        let a = "depth: 510
target: 10,10";
        assert_eq!(read_contents(&a).1, 45);
    }
}
