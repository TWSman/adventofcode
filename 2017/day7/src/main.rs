use clap::Parser;
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
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (String, i32) {
    let nodes = parse_tree(cont);

    // Find the node with no parent. This will be the root
    let part1 = nodes
        .iter()
        .find(|(_, n)| n.parent.is_none())
        .expect("There should be node with no parent");

    let part2 = get_part2(&nodes);
    (part1.0.to_string(), part2)
}

fn get_part2(nodes: &BTreeMap<String, Node>) -> i32 {
    let (rootname, _) = nodes.iter().find(|(_, n)| n.parent.is_none()).unwrap();
    let mut candidate = rootname;
    loop {
        let node = nodes.get(candidate).unwrap();
        let mut weights = vec![];
        let mut mismatch = false;
        for child in node.children.iter() {
            let w = nodes.get(child).unwrap().total_weight;
            weights.push((child, w));
        }
        for (child, w) in &weights {
            if weights.iter().filter(|(_, ww)| ww != w).count() > 1 {
                mismatch = true;
                candidate = child;
            }
        }
        if !mismatch {
            // The children of this node are balanced.
            // Thus the problem must be the weight of this node
            println!("No mismatches found under '{candidate}'");
            break;
        }
    }
    let candidate_node = nodes.get(candidate).unwrap();
    let candidate_parent = candidate_node.parent.as_ref().unwrap();
    let candidate_total_weight = candidate_node.total_weight;

    // Check other children of this parent to calculate the correct weight
    for child in nodes.get(candidate_parent).unwrap().children.iter() {
        if child != candidate {
            let diff = candidate_total_weight - nodes.get(child).unwrap().total_weight;
            return candidate_node.weight - diff;
        }
    }
    0
}

#[derive(Debug, Clone)]
struct Node {
    _name: String,
    weight: i32,
    children: Vec<String>,
    parent: Option<String>,
    total_weight: i32,
}

fn get_total_weight(node: &Node, nodes: &BTreeMap<String, Node>) -> i32 {
    let mut total = node.weight;
    for child in &node.children {
        total += get_total_weight(nodes.get(child).unwrap(), nodes);
    }
    total
}

fn parse_tree(cont: &str) -> BTreeMap<String, Node> {
    let mut nodes = BTreeMap::new();
    for line in cont.lines() {
        let splits = line.split_whitespace().collect::<Vec<_>>();
        let name = splits[0];
        let weight = splits[1]
            .trim_matches(|c| c == ')' || c == '(')
            .parse::<i32>()
            .unwrap();
        let mut children = vec![];
        for spl in splits.iter().skip(3) {
            children.push(spl.replace(",", ""));
        }
        assert!(children.len() > 1 || children.is_empty());
        nodes.insert(
            name.to_string(),
            Node {
                _name: name.to_string(),
                weight,
                children,
                parent: None,
                total_weight: 0,
            },
        );
    }

    let keys = nodes.keys().map(|k| k.to_string()).collect::<Vec<_>>();

    // Rescan the tree to update parents
    for key in keys {
        let children = &nodes.get(&key).unwrap().children.clone();
        let total_weight = get_total_weight(nodes.get(&key).unwrap(), &nodes);
        nodes.get_mut(&key).unwrap().total_weight = total_weight;
        for c in children {
            nodes.get_mut(c).unwrap().parent = Some(key.to_string());
        }
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        assert_eq!(read_contents(&a).0, "tknk");
    }

    #[test]
    fn part2() {
        let a = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        assert_eq!(read_contents(&a).1, 60);
    }
}
