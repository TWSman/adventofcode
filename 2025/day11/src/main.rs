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
    name: String,
    targets: BTreeSet<String>,
    sources: BTreeSet<String>,
}

impl Node {
    fn new(name: String) -> Self{
        Self {name, targets: BTreeSet::new(), sources: BTreeSet::new()}
    }


    fn parse_line(ln: &str) -> (String, Self) {
        let (name, conns) = ln.split_once(':').unwrap();
        (
            name.to_string(),
            Self {
                name: name.to_string(),
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

    // 208404 is too low
    // 300 000 is too low
}

fn read_nodes(cont: &str) -> BTreeMap<String, Node> {
    let mut nodes: BTreeMap<String, Node> = cont.lines().map(|ln| {
        Node::parse_line(ln)
    }).collect::<BTreeMap<String, Node>>();

    nodes.insert("out".to_string(), Node::new("out".to_string()));
    
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

    dbg!(&nodes);

    println!("{} Nodes", nodes.len());
    let part1 = get_part1(&nodes);
    dbg!(&part1);
    //let part1 = 0;
    let part2 = get_part2(&nodes);
    dbg!(&part2);
    
    (part1, part2)
}

fn get_part1(nodes: &BTreeMap<String, Node>) -> i64 {
    get_route(nodes, "you", "out")
}

fn get_part2(nodes: &BTreeMap<String, Node>) -> i64 {
    let dac_to_fft = get_route(nodes, "dac", "fft");
    dbg!(&dac_to_fft);
    let fft_to_dac = get_route(nodes, "fft", "dac");
    dbg!(&fft_to_dac);

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
        panic!("No inter routes found");
    }

    let start_to_inter = get_route(nodes, "svr", inter1);
    let inter_to_end = get_route(nodes, inter2, "out");
    
    start_to_inter * inter_count * inter_to_end
}

fn get_route_inv(nodes: &BTreeMap<String, Node>, start: &str , end: &str) -> i64 {
    dbg!(&end);
    let mut heads: Vec<&str> = Vec::new();
    heads.push(end);
    let mut routes_to_target: BTreeMap<String, usize> = BTreeMap::new();
    routes_to_target.insert(end.to_string(), 1);
    loop {
        if heads.len() == 0 {
            break;
        }
        dbg!(&heads);
        let head = heads.pop().unwrap();
        // Get the number of routes from this node to end
        let r_to_target = routes_to_target.get(head).unwrap().clone();
        dbg!(&r_to_target);

        for source in nodes.get(head).unwrap().sources.iter() {
            let mut old_val: usize = 0;
            if routes_to_target.contains_key(source) {
                old_val = *routes_to_target.get(source).unwrap();
            } else {

            }
            routes_to_target.insert(source.to_string(), old_val + r_to_target);
            heads.push(source);
        }
    }
    0
}

//fn get_route_rec(nodes: &BTreeMap<String, Node>, start: &str , end: &str) -> i64 {
//    let mut out = 0;
//    for target in nodes.get(start).unwrap().targets.iter() {
//        if target == end {
//            out += 1;
//        } else if target == "out" {
//            //return get_route_rec(nodes, target, end);
//        } else {
//            out += get_route_rec(nodes, target, end);
//        }
//    }
//}

fn get_route(nodes: &BTreeMap<String, Node>, start: &str , end: &str) -> i64 {
    let mut heads: Vec<(&str, usize, Vec<String>)> = Vec::new();
    heads.push((start, 0, vec![start.to_string()]));
    let mut routes = 0;
    //let route_ln
    let mut loop_count: usize = 0;
    loop {
        //dbg!(&heads);
        loop_count += 1;
        if heads.len() == 0 {
            break;
        }
        if loop_count % 1_000_000== 0 {
            println!("Loop count {}", loop_count);
            dbg!(&heads.len());
            dbg!(routes);
        }
        let (head,n,history) = heads.pop().unwrap();
        if !nodes.contains_key(head) {
            continue;
        }
        let node = nodes.get(head).unwrap();
        for target in node.targets.iter() {
            if target == end {
                routes += 1;
                //println!("Found route to out via");
                //dbg!(&history);
            } else if target != "out" {
                let mut new_history = history.clone();
                new_history.push(target.to_string());
                heads.push((target, n+1, new_history));
            }
        }
    }
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
