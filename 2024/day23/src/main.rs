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

#[derive(Debug)]
struct Node {
    name: String,
    connections: BTreeSet<String>,
}

impl Node {
    fn new(name: String) -> Self{
        Self {name, connections: BTreeSet::new() }
    }

    fn push(&mut self, other: String) {
        if !self.connections.contains(&other) {
            self.connections.insert(other);
        }
    }
}

// Part1: Find all cliques of size 3 that contain an element starting with 't'
// Part2: Find the largest clique
//        This solution assumes that there are at most 2 non-shared connection in the largest clique
//        i.e. If each node has 13 edges, largest clique will have at least 12 nodes 
//        (each node will have at least 11 edges inside the clique)
//
fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");

    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {}", part2.join(","));

}

fn parse_line(line: &str) -> (String, String) {
    let mut a = line.split('-');
    let from = a.next().unwrap();
    let tmp = a.next().unwrap();
    (from.to_string(), tmp.to_string())
}

fn read_contents(cont: &str) -> (i64, Vec<String>) {
    let connections = cont.lines().map(|c| {
        parse_line(c)
    }).collect::<Vec<_>>();
    println!("{} Connections", connections.len());

    let mut nodes: BTreeMap<String, Node> = BTreeMap::new();
    for (start, end) in &connections {
        let start_node = if let Some(v) = nodes.get_mut(start) { v } else {
            let node = Node::new(start.clone());
            nodes.insert(start.clone(), node);
            nodes.get_mut(start).unwrap()
        };
        start_node.push(end.clone());
        let end_node = if let Some(v) = nodes.get_mut(end) { v } else {
            let node = Node::new(end.clone());
            nodes.insert(end.clone(), node);
            nodes.get_mut(end).unwrap()
        };
        end_node.push(start.clone());
    }
    println!("{} Nodes", nodes.len());
    let mut cliques: BTreeSet<(String, String, String)> = BTreeSet::new();

    // Check part1
    for (a, node) in &nodes {
        if !a.starts_with('t') {
            continue;
        }
        // Loop over pairs of connections to see if they connect
        // Both of 
        for (x, b) in node.connections.iter().enumerate() {
            for c in node.connections.iter().skip(x) {
                let node_b = nodes.get(b).unwrap();
                if node_b.connections.contains(c) {
                    cliques.insert(sorted_clique(a, b, c));
                }
            }
        }
    }

    let mut checked_nodes: BTreeSet<String> = BTreeSet::new();
    let mut biggest_clique: BTreeSet<String> = BTreeSet::new();
    for (name,node) in &nodes {
        if checked_nodes.contains(name) {
            continue;
        }
        for x in &node.connections {
            let mut clique_candidate = node.connections.clone();
            // Remove 1 element for each set of connections
            clique_candidate.remove(x);
            let mut good: bool = true;
            for (i,a) in clique_candidate.iter().enumerate() {
                if !good {
                    break;
                }
                for b in clique_candidate.iter().skip(i+1) {
                    if !nodes.get(a).unwrap().connections.contains(b) {
                        good = false;
                        break;
                    }
                }
            }

            if !good & (clique_candidate.len() > biggest_clique.len()) {
                // Check if removing a second element works
                // TODO check removing N elements
                println!("Going for seconds");
                for y in &node.connections {
                    if y == x {
                        continue;
                    }
                    clique_candidate.remove(y);
                    good = true;
                    for (i,a) in clique_candidate.iter().enumerate() {
                        if !good {
                            break;
                        }
                        for b in clique_candidate.iter().skip(i+1) {
                            if !nodes.get(a).unwrap().connections.contains(b) {
                                good = false;
                                break;
                            }
                        }
                    }
                }
            }
            if good {
                for c in &clique_candidate {
                    checked_nodes.insert(c.to_string());
                }
                clique_candidate.insert(name.to_string());
                biggest_clique = clique_candidate;
            }
        }
    }
    dbg!(&biggest_clique);
    (cliques.len() as i64, biggest_clique.iter().map(|c| c.to_string()).collect::<Vec<String>>())
}

fn sorted_clique(a: &str, b: &str, c: &str) -> (String, String, String) {
    let mut out = [a, b,c];
    out.sort();
    (out[0].to_string(), out[1].to_string(), out[2].to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!(read_contents(&a).0, 7);
        assert_eq!(read_contents(&a).1, ["co", "de", "ka", "ta"]);
    }
}
