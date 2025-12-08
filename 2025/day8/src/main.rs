use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

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
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}

#[derive(Debug)]
struct Box {
    x: i64,
    y: i64,
    z: i64,
}

impl Box {
    fn distance(&self, other: &Box) -> i64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn from_str(input: &str) -> Self {
        let res: Vec<&str> = input.split(',').collect();
        assert_eq!(res.len(), 3);
        let x = res[0].parse::<i64>().unwrap();
        let y = res[1].parse::<i64>().unwrap();
        let z = res[2].parse::<i64>().unwrap();
        Self {
            x,
            y,
            z,
        }
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let boxes: BTreeMap<usize,Box> = cont.lines().enumerate().map(|(i,ln)| {
        (i, Box::from_str(ln))
    }).collect();
    dbg!(&boxes);

    let part1 = get_part1(&boxes);


    (part1, 0)
}

fn get_part1(boxes: &BTreeMap<usize, Box>) -> i64 {
    let n = boxes.len();
    let mut distances: BTreeMap<(usize, usize), i64> = BTreeMap::new();
    for i in 0..n {
        for j in (i+1)..n {
            let a = boxes.get(&i).unwrap();
            let b = boxes.get(&j).unwrap();
            distances.insert((i,j), a.distance(&b));
        }
    }
    //dbg!(&distances);
    dbg!(&distances.len());
    0
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
        assert_eq!(read_contents(&a).0, 40);
    }

}

