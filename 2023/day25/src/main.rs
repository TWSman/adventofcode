
use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use itertools::Itertools;

#[allow(clippy::cast_possible_truncation)]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[derive(Debug, Clone)]
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

    fn drop(&mut self, other: &String) {
        if self.connections.contains(other) {
            self.connections.remove(other);
        }
    }
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");

    let res = read_contents(&contents);
    println!("Part 1 answer is {res}");

    //let res = read_contents(&contents);
    //println!("Part 2 answer is {res}");

}

fn parse_line(line: &str) -> Vec<(String, String)> {
    let mut a = line.split(":");
    let from = a.next().unwrap();
    let tmp = a.next().unwrap().split(" ").filter_map(|x| {
        if x == "" {
            None
        } else {
            Some((from.to_string(), x.to_string()))
        }
    }).collect::<Vec<_>>();
    tmp
}


fn get_size(start_node: String, nodes: &BTreeMap<String, Node>) -> i64 {
    let start = &nodes.get(&start_node).unwrap();
    let mut visited: BTreeSet<&String> = BTreeSet::new();
    let mut nodes_to_visit: Vec<&String> = vec![&start.name];
    loop {
        let node = match nodes_to_visit.pop() {
            Some(v) => nodes.get(v).unwrap(),
            None => break,
        };
        visited.insert(&node.name);
        for t in &node.connections {
            if !visited.contains(&t) {
                nodes_to_visit.push(&t);
            }
        }
    }
    visited.len() as i64
}

fn is_connected(nodes: &BTreeMap<String, Node>) -> bool {
    let start = &nodes.values().next().unwrap();
    let mut visited: BTreeSet<&String> = BTreeSet::new();
    let mut nodes_to_visit: Vec<&String> = vec![&start.name];
    loop {
        let node = match nodes_to_visit.pop() {
            Some(v) => nodes.get(v).unwrap(),
            None => break,
        };
        visited.insert(&node.name);
        for t in &node.connections {
            if !visited.contains(&t) {
                nodes_to_visit.push(&t);
            }
        }
    }
    if nodes.keys().all(|k| visited.contains(&k)) {
        //println!("Graph is connected");
        return true;
    } else {
        //println!("Graph is not connected");
        return false;
    }
}

fn read_contents(cont: &str) -> i64 {
    let mut connections = cont.lines().map(|c| {
        parse_line(c)
    }).flatten().collect::<Vec<_>>();
    println!("{} Connections", connections.len());
    let mut nodes: BTreeMap<String, Node> = BTreeMap::new();
    let n = connections.len();
    for (start, end) in &connections {
        let start_node = match nodes.get_mut(start) {
            Some(v) => v,
            None => {
                let node = Node::new(start.clone());
                nodes.insert(start.clone(), node);
                nodes.get_mut(start).unwrap()
            }
        };
        start_node.push(end.clone());
        let end_node = match nodes.get_mut(end) {
            Some(v) => v,
            None => {
                let node = Node::new(end.clone());
                nodes.insert(end.clone(), node);
                nodes.get_mut(end).unwrap()
            }
        };
        end_node.push(start.clone());
    }
    for i in 0..n {
        if i % 10 == 0 {
            println!("{} / {n}", i + 1);
        }
        let (start_i, end_i) = connections.get(i).unwrap();
        {   let node_start_i = nodes.get_mut(start_i).unwrap();
            node_start_i.drop(end_i);
        }
        {  
            let node_end_i = nodes.get_mut(end_i).unwrap();
            node_end_i.drop(start_i);
        }
        for j in 0..i {
            let (start_j, end_j) = connections.get(j).unwrap();

            if (start_j == start_i) | (start_j == end_i) | (end_j == start_i) | (end_j == end_i) {
                continue
            }
            {   let node_start_j = nodes.get_mut(start_j).unwrap();
                node_start_j.drop(end_j);
            }
            {  
                let node_end_j = nodes.get_mut(end_j).unwrap();
                node_end_j.drop(start_j);
            }
            for k in 0..j {
                let (start_k, end_k) = connections.get(k).unwrap();
                if (start_k == start_j) | (start_k == start_i) | (start_k == end_j) | (start_k == end_i) |
                    (end_k == start_j) | (end_k == start_i) | (end_k == end_j) | (end_k == end_i)  {
                    continue
                }
                {   let node_start_k = nodes.get_mut(start_k).unwrap();
                    node_start_k.drop(end_k);
                }
                {  
                    let node_end_k = nodes.get_mut(end_k).unwrap();
                    node_end_k.drop(start_k);
                }
                if !is_connected(&nodes) {
                    println!("Found set {i} {j} {k}");
                    dbg!(&connections[i]);
                    dbg!(&connections[j]);
                    dbg!(&connections[k]);
                    let (start, end) = &connections[i];
                    let a = get_size(start.to_string(), &nodes);
                    let b = get_size(end.to_string(), &nodes);
                    return a * b;
                };
                {   let node_start_k = nodes.get_mut(start_k).unwrap();
                    node_start_k.push(end_k.to_string());
                }
                {  
                    let node_end_k = nodes.get_mut(end_k).unwrap();
                    node_end_k.push(start_k.to_string());
                }
            }
            {   let node_start_j = nodes.get_mut(start_j).unwrap();
                node_start_j.push(end_j.to_string());
            }
            {  
                let node_end_j = nodes.get_mut(end_j).unwrap();
                node_end_j.push(start_j.to_string());
            }
        }
        {   let node_start_i = nodes.get_mut(start_i).unwrap();
            node_start_i.push(end_i.to_string());
        }
        {  
            let node_end_i = nodes.get_mut(end_i).unwrap();
            node_end_i.push(start_i.to_string());
        }
    }
    //dbg!(&nodes.len());
    //dbg!(&nodes);
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!(read_contents(&a), 54);
    }
}
