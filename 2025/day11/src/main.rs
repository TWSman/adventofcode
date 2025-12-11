use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[derive(Debug, Clone)]
struct Node {
    targets: BTreeSet<String>,
    sources: BTreeSet<String>,
}

impl Node {
    fn new() -> Self{
        Self {targets: BTreeSet::new(), sources: BTreeSet::new()}
    }

    fn parse_line(ln: &str) -> (String, Self) {
        let (name, conns) = ln.split_once(':').unwrap();
        (
            name.to_string(),
            Self {
                sources: BTreeSet::new(),
                targets: conns.split(' ').filter_map(|x| {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.to_string())
                    }
                }).collect()
            }
        )
    }
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");

    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
}

fn read_nodes(cont: &str) -> BTreeMap<String, Node> {
    let mut nodes: BTreeMap<String, Node> = cont.lines().map(|ln| {
        Node::parse_line(ln)
    }).collect::<BTreeMap<String, Node>>();

    nodes.insert("out".to_string(), Node::new());
    
    let keys = nodes.keys().map(|v| v.to_owned()).collect::<Vec<_>>();

    for key in keys {
        let targets: BTreeSet<String> = nodes.get(&key).unwrap().targets.clone();
        for target in targets {
            let target_node = nodes.get_mut(&target).unwrap();
            target_node.sources.insert(key.to_string());
        }
    }
    nodes
}

fn read_contents(cont: &str) -> (i64, i64) {
    let nodes = read_nodes(cont);

    println!("{} Nodes", nodes.len());
    let part1 = get_part1(&nodes);
    let part2 = get_part2(&nodes);
    dbg!(&part2);
    
    (part1, part2)
}

fn get_part1(nodes: &BTreeMap<String, Node>) -> i64 {
    // Check results with two different methods
    // First old but slow method. Still fast enough for part 1
    let a = get_route(nodes, "you", "out");
    // New optimized method
    let b = get_route_solver(nodes, "you", "out");
    assert_eq!(a,b);
    a
}

fn get_part2(nodes: &BTreeMap<String, Node>) -> i64 {
    // Check which node would come first
    let dac_to_fft = get_route_solver(nodes, "dac", "fft");
    let fft_to_dac = get_route_solver(nodes, "fft", "dac");

    let (inter1, inter2);
    let inter_count;
    if dac_to_fft > 0 && fft_to_dac > 0 {
        panic!("Both routes exist");
    } 
    else if dac_to_fft > 0 {
        // First step is dac
        inter1 = "dac";
        inter2 = "fft";
        inter_count = dac_to_fft;
    } 
    else if fft_to_dac > 0 {
        // First step is fft
        inter1 = "fft";
        inter2 = "dac";
        inter_count = fft_to_dac;
    } else {
        panic!("No intermediate routes found");
    }

    // Get number of routes from start to first intermediate point
    let start_to_inter = get_route_solver(nodes, "svr", inter1);
    // And number of routes from second intermediate point to end
    let inter_to_end = get_route_solver(nodes, inter2, "out");
    
    // Total number of routes is product of three parts
    start_to_inter * inter_count * inter_to_end
}

struct Solver {
    nodes: BTreeMap<String, Node>,
    memory: BTreeMap<String, i64>, // Keep memory of how many routes from node to end
    end: String,
}

impl Solver {
    fn new(nodes: BTreeMap<String, Node>, end: &str) -> Self {
        Self {nodes, memory: BTreeMap::new(), end: end.to_string()}
    }

    fn solve(&mut self, start: &str) -> i64 {
        let mut routes: i64 = 0;
        let start_node = self.nodes.get(start).unwrap().clone();
        for target in start_node.targets.iter() {
            if target == &self.end {
                routes += 1;
            } else if target == "out" {
                // This is a dead end. No point in continuing
            } else {
                let sub_routes: i64 = if self.memory.contains_key(target) {
                    *self.memory.get(target).unwrap()
                } else {
                    self.solve(target)
                };
                routes += sub_routes;
            }
        }
        // Store result in memory
        self.memory.insert(start.to_string(), routes);
        routes
    }
}


fn get_route_solver(nodes: &BTreeMap<String, Node>, start: &str , end: &str) -> i64 {
    let mut solver = Solver::new(nodes.clone(),end);
    solver.solve(start)
}

fn get_route(nodes: &BTreeMap<String, Node>, start: &str , end: &str) -> i64 {
    let mut heads: Vec<(&str, usize, Vec<String>)> = Vec::new();
    heads.push((start, 0, vec![start.to_string()]));
    let mut routes = 0;
    let mut visited_nodes: BTreeSet<String> = BTreeSet::new();
    loop {
        if heads.is_empty() {
            break;
        }
        let (head,n,history) = heads.pop().unwrap();
        if !nodes.contains_key(head) {
            continue;
        }
        let node = nodes.get(head).unwrap();
        for target in node.targets.iter() {
            if target == end {
                routes += 1;
                for v in history.iter() {
                    visited_nodes.insert(v.to_string());
                }
                //println!("Found route: {:?}", history);
            } else if target != "out" {
                let mut new_history = history.clone();
                new_history.push(target.to_string());
                heads.push((target, n+1, new_history));
            }
        }
    }
    println!("Visited {} nodes between '{}' and '{}'l", visited_nodes.len(), start, end);
    dbg!(&visited_nodes);
    routes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let nodes = read_nodes(&a);
        assert_eq!(get_route(&nodes,"you", "out"), 5);
        assert_eq!(get_route_solver(&nodes,"you", "out"), 5);
        assert_eq!(get_part1(&nodes), 5);
    }

    #[test]
    fn part2() {
        let a = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let nodes = read_nodes(&a);
        assert_eq!(get_part2(&nodes), 2);
    }
}

// you
// you-bbb
// you-ccc
//
// you-bbb-ddd-ggg-out
// you-bbb-eee-out *
//
// you-ccc-ddd-ggg-out *
// you-ccc-eee-out *
// you-ccc-fff-out *
//
//
