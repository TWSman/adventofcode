use clap::Parser;
use shared::Vec4D;
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
    println!("Part 1 answer is {}", res);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> i64 {
    let points = cont
        .lines()
        .map(read_vec)
        .enumerate()
        .collect::<BTreeMap<_, _>>();
    get_part1(&points)
}

fn read_vec(ln: &str) -> Vec4D {
    let spl = ln.split(',').collect::<Vec<_>>();
    assert_eq!(spl.len(), 4);
    Vec4D {
        x: spl[0].trim().parse().unwrap(),
        y: spl[1].trim().parse().unwrap(),
        z: spl[2].trim().parse().unwrap(),
        t: spl[3].trim().parse().unwrap(),
    }
}

fn get_part1(points: &BTreeMap<usize, Vec4D>) -> i64 {
    let mut used: BTreeSet<usize> = BTreeSet::new();
    let n = points.len();
    let mut queue = Vec::new();
    queue.push(0); // Start from the first point
    used.insert(0);
    let mut constellations = 1; // There is at least 1
    loop {
        if queue.is_empty() {
            println!("Used: {}", used.len());
            // Check if there are unused points
            if let Some((c, _p)) = points.iter().find(|(i, _p)| !used.contains(i)) {
                queue.push(*c);
                used.insert(*c);
                constellations += 1;
                continue;
            }
            break;
        }
        let i = queue.pop().unwrap();
        let point = points.get(&i).unwrap();
        for j in 0..n {
            if used.contains(&j) {
                continue;
            }
            let point2 = points.get(&j).unwrap();
            if point.manhattan(point2) <= 3 {
                used.insert(j);
                queue.push(j);
            }
        }
    }
    constellations
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let a = "0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0";
        assert_eq!(read_contents(&a).0, 2);

        let b = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        assert_eq!(read_contents(&b).0, 4);

        let c = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
        assert_eq!(read_contents(&c).0, 3);

        let d = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
        assert_eq!(read_contents(&d).0, 8);
    }
}
