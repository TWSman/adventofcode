use clap::Parser;
use priority_queue::PriorityQueue;
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
    println!("Execution lasted {elapsed:.2?}");
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    x: i32,
    y: i32,
    size: i32,
    used: i32,
    // avail: i32, not needed
    // useperc: i32, not needed
}

impl Node {
    fn new(ln: &str) -> Option<Self> {
        if !ln.starts_with("/dev/grid/node-") {
            return None;
        }
        let parts = ln.split_whitespace().collect::<Vec<_>>();
        let name = parts[0];
        let name_parts = name
            .trim_start_matches("/dev/grid/node-")
            .split('-')
            .collect::<Vec<_>>();
        let x = name_parts[0].trim_start_matches('x').parse().unwrap();
        let y = name_parts[1].trim_start_matches('y').parse().unwrap();
        let size = parts[1].trim_end_matches('T').parse().unwrap();
        let used = parts[2].trim_end_matches('T').parse().unwrap();
        //let avail = parts[3].trim_end_matches('T').parse().unwrap();
        //let useperc = parts[4].trim_end_matches('%').parse().unwrap();
        Self {
            x,
            y,
            size,
            used, /*avail, useperc*/
        }
        .into()
    }

    fn avail(&self) -> i32 {
        self.size - self.used
    }
}

fn check_viable(a: &Node, b: &Node) -> bool {
    if a == b {
        return false;
    }
    if a.used == 0 {
        return false;
    }
    a.used <= b.avail()
}

fn get_part1(nodes: &[Node]) -> i32 {
    let mut viable = 0;
    for a in nodes {
        for b in nodes {
            if check_viable(a, b) {
                //println!("{} is viable with {}", format!("{a:?}"), format!("{b:?}"));
                viable += 1;
            }
        }
    }
    viable
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    steps: usize,
    used: BTreeMap<(i32, i32), i32>,
    main_data: (i32, i32),
    empty: (i32, i32),
    history: Vec<((i32, i32), (i32, i32))>,
}

fn get_part2_spec(nodes: &BTreeMap<(i32, i32), &Node>) -> i32 {
    let (_, empty) = nodes.iter().find(|(_, n)| n.used == 0).unwrap();
    let x_max = nodes.values().map(|n| n.x).max().unwrap();

    // First find maximum amount of data in the first 2 rows
    let max_data = nodes
        .iter()
        .filter(|((_, y), _)| *y < 2)
        .map(|(_, n)| n.used)
        .max()
        .unwrap();
    let min_size = nodes
        .iter()
        .filter(|((_, y), _)| *y < 2)
        .map(|(_, n)| n.size)
        .min()
        .unwrap();
    // And check that data in the first 2 rows is completely interchangeable, i.e. we can move
    // between any two nodes
    assert!(min_size > max_data);

    let large_nodes = nodes
        .iter()
        .filter(|(_, n)| n.used > min_size)
        .collect::<Vec<_>>();

    // Check that all large nodes are on the same row
    let large_min_row = large_nodes.iter().map(|((_, y), _)| *y).min().unwrap();
    let large_max_row = large_nodes.iter().map(|((_, y), _)| *y).max().unwrap();
    assert_eq!(large_min_row, large_max_row);

    let large_min_col = large_nodes.iter().map(|((x, _), _)| *x).min().unwrap();
    let large_max_col = large_nodes.iter().map(|((x, _), _)| *x).max().unwrap();

    // Idealized distance from the empty node to the main data, which is at (x_max, 0)
    let distance_to_main = (empty.x - x_max).abs() + empty.y;

    // Check that the rows extends x_max, otherwise we wouldn't have to go around
    assert_eq!(large_max_col, x_max);
    // We need to go around the large nodes, first left and then right until we reach the same x
    let extra_steps = (1 + empty.x - large_min_col).max(0) * 2;

    // Number of steps to move the main data along the bottom row
    let max_steps = 5 * (x_max - 1);
    distance_to_main + max_steps + extra_steps
}

#[allow(dead_code)]
fn get_part2_route(nodes: &BTreeMap<(i32, i32), &Node>) -> i32 {
    // A* search to find the answer, relatively slow
    let used: BTreeMap<(i32, i32), i32> = nodes.values().map(|n| ((n.x, n.y), n.used)).collect();
    let x_max = nodes.values().map(|n| n.x).max().unwrap();
    let (_, empty) = nodes.iter().find(|(_, n)| n.used == 0).unwrap();
    let main_data = (x_max, 0);
    let mut queue = PriorityQueue::new();
    let empty = (empty.x, empty.y);
    queue.push(
        State {
            steps: 0,
            used,
            main_data,
            empty,
            history: Vec::new(),
        },
        0,
    );
    let mut min_dist = 99;
    let mut visited = BTreeSet::new();
    loop {
        if queue.is_empty() {
            println!("No solution found");
            return 0;
        }
        let (state, _) = queue.pop().unwrap();
        if state.main_data == (0, 0) {
            for (from, to) in &state.history {
                println!("Move from y{}-x{} to y{}-x{}", from.1, from.0, to.1, to.0);
            }
            println!("\nReached goal!");
            return state.steps as i32;
        }
        if state.steps > 300 {
            println!(
                "\nSteps: {}, main data at {:?}, empty at {:?}",
                state.steps, state.main_data, state.empty
            );
            return 0;
        }
        let main_dist = state.main_data.0 + state.main_data.1;
        if min_dist > main_dist {
            min_dist = main_dist;
            println!("State steps: {}, main distance: {}", state.steps, main_dist);
        }
        let empty = state.empty;
        let empty_size = nodes.get(&empty).unwrap().size;
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_empty = (empty.0 + dx, empty.1 + dy);
            if !(state.used.contains_key(&new_empty)) {
                continue;
            }
            if !(state.used.contains_key(&new_empty)) {
                continue;
            }
            let node_used = state.used.get(&new_empty).unwrap();
            let new_main = if new_empty == state.main_data {
                empty
            } else {
                state.main_data
            };
            if node_used > &empty_size {
                continue;
            }
            let mut new_used = state.used.clone();
            new_used.insert(empty, *node_used);
            new_used.insert(new_empty, 0);
            let steps = state.steps + 1;
            let prio = 2 * steps as i32
                + new_main.0 + new_main.1  // Distance of main data to goal
                + (new_empty.0 - new_main.0).abs() + (new_empty.1 - new_main.1).abs(); // Distance
            if visited.contains(&(new_main, new_empty)) {
                continue;
            }
            visited.insert((new_main, new_empty));
            let mut history = state.history.clone();
            history.push((new_empty, empty));
            queue.push(
                State {
                    steps,
                    used: new_used,
                    main_data: new_main,
                    empty: new_empty,
                    history,
                },
                -prio,
            );
        }
    }
}

fn read_contents(cont: &str) -> (i32, i32) {
    let nodes = cont.lines().filter_map(Node::new).collect::<Vec<_>>();
    let nodes2 = nodes
        .iter()
        .map(|n| ((n.x, n.y), n))
        .collect::<BTreeMap<(i32, i32), &Node>>();
    println!("{} nodes", nodes.len());
    let part1 = get_part1(&nodes);
    let part2 = get_part2_spec(&nodes2);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%";
        assert_eq!(read_contents(&a).0, 7);
    }

    #[test]
    fn part2() {
        let a = "Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%";
        assert_eq!(read_contents(&a).1, 7);
    }
}
