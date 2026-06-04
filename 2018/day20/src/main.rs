use clap::Parser;
use shared::Dir;
use shared::Vec2D;
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
    let nodes = parse_tree(cont);
    let mut heads = Vec::new();
    let loc = Vec2D {x: 0, y: 0};
    heads.push((loc, 1, 0));
    let mut distance: BTreeMap<Vec2D, usize> = BTreeMap::new();
    let mut visited: BTreeSet<(Vec2D, usize)>=  BTreeSet::new();

    // This assumes that the shortest path to any room is 
    loop {
        if heads.is_empty() {
            break;
        }
        let (mut loc, next, mut steps) = heads.pop().unwrap();
        if visited.contains(&(loc, next)) {
            continue
        }
        visited.insert((loc, next));
        let node = nodes.get(&next).unwrap();
        for dir in &node.contents {
            let new_loc = loc + dir.get_dir_true_vec();
            steps += 1;
            let old_dist = *distance.get(&new_loc).unwrap_or(&999999);
            distance.insert(new_loc, steps.min(old_dist));
            loc = new_loc;
        }
        for child in &node.children {
            heads.push((loc, *child, steps));
        }
    }
    let part1 = *distance.values().max().unwrap() as i64;
    let part2 = distance.iter().filter(|(_,v)| **v >= 1000).count() as i64;
    (part1, part2)
}


#[derive(Debug)]
struct Node {
    contents: Vec<Dir>,
    children: Vec<usize>,
    level: usize,
}


fn parse_tree(input: &str) -> BTreeMap<usize, Node> {
    let input = input.trim().strip_prefix('^').unwrap().strip_suffix('$').unwrap();
    let mut splits_ind = Vec::new();
    let mut splits: Vec<&str> = Vec::new();
    let mut nodes: BTreeMap<usize, Node> = BTreeMap::new();
    for (i,c) in input.chars().enumerate() {
        match c {
            ')' | '(' | '|' => { 
                splits_ind.push(i)
            }
            _ => {},
        }
    }
    let mut prev = 0;
    let mut id = 0;
    let mut level = 0;
    for i in splits_ind {
        splits.push(&input[prev..i]);
        prev = i;
    }
    splits.push(&input[prev..]);
    let mut prev_by_level: BTreeMap<usize, usize> = BTreeMap::new();
    for spl in splits {
        let (start, mut res)= spl.split_at(1);
        id += 1;
        let new_level = match start {
            ")" => {
                for (_j,node) in nodes.iter_mut() {
                    if node.level != level {
                        continue;
                    }
                    if node.children.is_empty() {
                        node.children.push(id);
                    }
                }
                assert!(level > 0); level - 1
            },
            "|" => {
                let prev = prev_by_level.get(&level).unwrap();
                nodes.get_mut(prev).unwrap().children.push(id);
                level
            },
            "(" => {
                for (j,node) in nodes.iter_mut().rev() {
                    if node.level != level {
                        continue;
                    }
                    if node.children.is_empty() {
                        node.children.push(id);
                        prev_by_level.insert(level + 1, *j);
                        break;
                    }
                }
                level + 1
            },
            _ => {
                res = spl;
                0
            },
        };
        let contents = res.chars().map(|c| match c {
            'N' => Dir::N,
            'W' => Dir::W,
            'E' => Dir::E,
            'S' => Dir::S,
            _ => panic!(),
        }).collect::<Vec<_>>();
        let node = Node {
            contents,
            level: new_level,
            children: Vec::new(),
        };

        level = new_level;

        nodes.insert(id, node);
    }
    nodes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let a = "^ENWWW(NEEE|SSE(EE|N))$";
        let b = parse_tree(&a);
        assert_eq!(b.len(), 7);
    }

    #[test]
    fn part1() {
        let a = "^WNE$";
        assert_eq!(read_contents(&a).0, 3);
        let a = "^ENWWW(NEEE|SSE(EE|N))$";
        assert_eq!(read_contents(&a).0, 10);
        let a = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        assert_eq!(read_contents(&a).0, 18);
        let a = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        assert_eq!(read_contents(&a).0, 23);
        let a = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        assert_eq!(read_contents(&a).0, 31);
    }


}
