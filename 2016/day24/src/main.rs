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
    let mut grid = read_grid(cont);
    let analyzed = analyze_grid(&grid);
    grid.nodes = analyzed;
    grid.print_grid();
    let part1 = get_answer(&grid, false);
    let part2 = get_answer(&grid, true);
    (part1, part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Object {
    Empty,
    Wall,
    Poi(char),
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec2D, Object>,
    entrance: Vec2D,
    key_n: usize,
    nodes: BTreeMap<Vec2D, Node>,
}

impl Grid {
    fn print_grid(&self) {
        let min_x = self.grid.keys().map(|v| v.x).min().unwrap();
        let max_x = self.grid.keys().map(|v| v.x).max().unwrap();
        let min_y = self.grid.keys().map(|v| v.y).min().unwrap();
        let max_y = self.grid.keys().map(|v| v.y).max().unwrap();

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                match self.grid.get(&Vec2D { x, y }) {
                    Some(Object::Wall) => {
                        print!("{}", "#".blue().on_blue());
                    }
                    Some(&Object::Empty) => {
                        print!("{}", ".".black().on_black());
                    }
                    Some(&Object::Poi('0')) => {
                        print!("{}", "0".red().on_black());
                    }
                    Some(&Object::Poi(c)) => {
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
                    c if c.is_numeric() => Object::Poi(c),
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
    let poi_list = grid
        .values()
        .filter_map(|obj| match obj {
            Object::Poi(c) => Some(c),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let entrance = &grid
        .iter()
        .find(|(_, obj)| **obj == Object::Poi('0'))
        .unwrap()
        .0
        .clone();
    let key_n = poi_list.len();
    let mut gg = Grid {
        grid,
        entrance: *entrance,
        key_n,
        nodes: BTreeMap::new(),
    };
    gg.nodes = analyze_grid(&gg);
    gg
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

#[derive(Debug, Clone)]
struct Node {
    _name: Object,
    routes: Vec<(Vec2D, Object, usize)>,
}

fn analyze_grid(grid: &Grid) -> BTreeMap<Vec2D, Node> {
    let nodes: BTreeMap<Vec2D, Object> = grid
        .grid
        .iter()
        .filter_map(|(k, v)| match v {
            Object::Wall | Object::Empty => None,
            Object::Poi(_) => Some((*k, *v)),
        })
        .collect();

    let mut out = BTreeMap::new();
    for node in nodes {
        let routes = find_routes(grid, node.0);
        let nod = Node {
            _name: node.1,
            routes,
        };
        out.insert(node.0, nod);
    }
    out
}

#[derive(Hash, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
struct State0 {
    loc: Vec2D,
    steps: usize,
}

fn find_routes(grid: &Grid, start: Vec2D) -> Vec<(Vec2D, Object, usize)> {
    let start_state = State0 {
        loc: start,
        steps: 0,
    };
    let mut visited: BTreeSet<Vec2D> = BTreeSet::new();
    let mut found: BTreeMap<_, usize> = BTreeMap::new();
    let mut queue = PriorityQueue::new();
    queue.push(start_state, Reverse(0));
    visited.insert(start);
    loop {
        if queue.is_empty() {
            break;
        }
        let (state, _prio) = queue.pop().unwrap();
        let loc = state.loc;
        for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
            let dx = dir.get_dir_true_vec();
            let new_loc = dx + loc;
            if visited.contains(&(new_loc)) {
                continue;
            }
            match grid.grid.get(&new_loc).unwrap_or(&Object::Wall) {
                Object::Empty => {}
                Object::Wall => {
                    continue;
                }
                c => {
                    if found.get(&(new_loc, *c)).unwrap_or(&99999) <= &state.steps {
                        //continue;
                    } else {
                        found.insert((new_loc, *c), state.steps + 1);
                    }
                    //continue;
                }
            }
            let new_state = State0 {
                loc: new_loc,
                steps: state.steps + 1,
            };
            visited.insert(new_state.loc);
            let prio = Reverse(new_state.steps);
            queue.push(new_state, prio);
        }
    }
    found
        .iter()
        .map(|((loc, obj), steps)| (*loc, *obj, *steps))
        .collect()
}

fn get_answer(grid: &Grid, part2: bool) -> i64 {
    let start_state = State {
        loc: grid.entrance,
        keys: BTreeSet::new(),
        steps: 0,
    };
    if grid.nodes.is_empty() {
        panic!("Grid not analyzed yet");
    }
    let mut visited: BTreeMap<(Vec2D, String), usize> = BTreeMap::new();
    let mut queue: PriorityQueue<State, _> = PriorityQueue::new();
    let prio = start_state.prio(grid.key_n);
    queue.push(start_state, Reverse(prio));
    visited.insert((grid.entrance, "0".to_string()), 0);
    loop {
        if queue.is_empty() {
            break;
        }
        let (state, _prio) = queue.pop().unwrap();
        if state.keys.len() == grid.key_n && (!part2 || state.loc == grid.entrance) {
            println!(
                "Found solution with {} steps, keys: {}",
                state.steps,
                state.keystring()
            );
            return state.steps as i64;
        }

        let node = grid.nodes.get(&state.loc).unwrap();
        //println!("At location ({:?}), keys: {}, steps: {}", node.name, state.keystring(), state.steps);
        for target in &node.routes {
            let new_loc = target.0;
            let new_state = State {
                loc: new_loc,
                keys: match target.1 {
                    Object::Poi(k) => state
                        .keys
                        .iter()
                        .cloned()
                        .chain(std::iter::once(k))
                        .collect(),
                    _ => state.keys.clone(),
                },
                steps: state.steps + target.2,
            };
            if visited
                .get(&(new_loc, new_state.keystring()))
                .unwrap_or(&99999)
                <= &new_state.steps
            {
                continue;
            }
            visited.insert((new_state.loc, new_state.keystring()), new_state.steps);
            let prio = new_state.prio(grid.key_n);
            //println!("    From '{:?}', looking at target '{:?}', steps: {}", node.name, target.1, target.2);
            queue.push(new_state, Reverse(prio));
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1a() {
        let a = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

        let grid = read_grid(a);
        assert_eq!(get_answer(&grid, false), 14);
        assert_eq!(get_answer(&grid, true), 20);
    }
}
