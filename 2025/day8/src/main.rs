use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::fmt::Display;
use core::fmt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents, 1000);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}

#[derive(Debug, Clone, Copy)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
    id: usize,
}

impl Display for JunctionBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ({}, {}, {})", self.id, self.x, self.y, self.z)
    }
}

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }

    fn from_str(input: &str, id: usize) -> Self {
        let res: Vec<&str> = input.split(',').collect();
        assert_eq!(res.len(), 3);
        let x = res[0].parse::<i64>().unwrap();
        let y = res[1].parse::<i64>().unwrap();
        let z = res[2].parse::<i64>().unwrap();
        Self {
            x,
            y,
            z,
            id,
        }
    }
}

impl Ord for JunctionBox {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for JunctionBox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for JunctionBox {
    fn eq(&self, other: &Self) -> bool {
        (self.x, self.y, self.z) == (other.x, other.y, other.z)
    }
}

impl Eq for JunctionBox { }

fn read_contents(cont: &str, target_connections: usize) -> (i64, i64) {
    let boxes: BTreeMap<usize,JunctionBox> = cont.lines().enumerate().map(|(i,ln)| {
        (i, JunctionBox::from_str(ln, i))
    }).collect();
    dbg!(&boxes);


    let n = boxes.len();
    let mut distances: Vec<(JunctionBox, JunctionBox, i64)> = Vec::new();
    for i in 0..n {
        for j in (i+1)..n {
            let a = boxes.get(&i).unwrap();
            let b = boxes.get(&j).unwrap();
            distances.push((*a, *b, a.distance(b)));
        }
    }

    let node_count = boxes.len();

    // Sort by distance ascending
    distances.sort_by(|a, b| a.2.cmp(&b.2));

    let mut circuits: Vec<BTreeSet<usize>> = Vec::new();
    let mut part1: Option<i64> = None;
    let mut part2: Option<i64> = None;
    for (n, (box_a, box_b, _distance)) in distances.iter().enumerate() {
        if n >= target_connections && part1.is_none() {
            part1 = Some(get_part1(&circuits));
            //break;
        }
        // Get index in circuit vector
        let a_circuit_ind = circuits.iter().enumerate().filter_map(|(i,g)| {
            if g.contains(&box_a.id){
                Some(i)
            } else {
                None
            }
        }).next();
        let b_circuit_ind = circuits.iter().enumerate().filter_map(|(i,g)| {
            if g.contains(&box_b.id){
                Some(i)
            } else {
                None
            }
        }).next();

        let a_found = a_circuit_ind.is_some();
        let b_found = b_circuit_ind.is_some();

        // Options:
        // a) Neither is found in any circuit
        if !a_found && !b_found {
            circuits.push(BTreeSet::from([box_a.id, box_b.id]));
        }

        // b) One is found
        else if a_found && !b_found {
            circuits[a_circuit_ind.unwrap()].insert(box_b.id);
        }
        else if b_found && !a_found {
            circuits[b_circuit_ind.unwrap()].insert(box_a.id);
        }

        // c) Both are found but already connected
        else if a_found && b_found && a_circuit_ind.unwrap() == b_circuit_ind.unwrap() {
            println!("Boxes {} and {} are already connected", box_a.id, box_b.id);
            continue;

        // d) Both are found but are in different circuits
        } else {
            // Create a new circuit that is the union of both
            let new_circuit: BTreeSet<usize> = circuits[a_circuit_ind.unwrap()].union(&circuits[b_circuit_ind.unwrap()]).cloned().collect();
            let ia = a_circuit_ind.unwrap();
            let ib = b_circuit_ind.unwrap();

            // Remove previous circuits
            // Should remove the larger index first to avoid shifting
            if ia > ib {
                circuits.remove(ia);
                circuits.remove(ib);
            } else {
                circuits.remove(ib);
                circuits.remove(ia);
            }
            circuits.push(new_circuit);
        }
        println!("Connected junction box {} and junction box {}", box_a, box_b);

        // Target of part2 is to find the final
        // connection that connects all boxes into a single circuit
        // This happens when there is only one circuit left with size equal to node_count
        if circuits.len() == 1 && circuits[0].len() == node_count {
            // The answer is then the product of x coordinates of the boxes
            part2 = Some(box_a.x * box_b.x);
            break;
        }
    }

    (part1.unwrap(), part2.unwrap())
}


fn get_part1(circuits: &[BTreeSet<usize>]) -> i64 {
    // Answer to part1 is the product of size of the three largest circuits
    // First get sizes of circuits
    let mut sizes: Vec<usize> = circuits.iter().map(|v| v.len()).collect();

    // Sort descending
    sizes.sort_by(|a,b| b.cmp(a));

    // Get product of 3 largest
    (sizes.first().unwrap()
    * sizes.get(1).unwrap()
    * sizes.get(2).unwrap()) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a="162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(read_contents(&a, 10).0, 40);
        assert_eq!(read_contents(&a, 10).1, 25272);
    }

}

