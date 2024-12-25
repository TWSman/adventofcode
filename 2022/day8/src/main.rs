use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use clap::Parser;
use shared::Dir;
use strum::IntoEnumIterator; // 0.17.1

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[derive(Debug)]
struct Map {
    heights: BTreeMap<(i64, i64), i8>,
    width: i64,
    height: i64,

}

impl Map {
    fn new(heights: BTreeMap<(i64, i64), i8>) -> Self {
        let height = heights.keys().map(|(_x,y)| y).max().unwrap() + 1;
        let width = heights.keys().map(|(x,_y)| x).max().unwrap() + 1;
        assert_eq!(height, width);
        Self {heights, width, height}

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

fn read_contents(cont: &str) -> (u64, i64) {
    let map = read_map(cont);
    dbg!(map.height);
    dbg!(map.width);
    //dbg!(&map);
    (get_part1(&map).len() as u64, get_part2(&map))
}


fn get_part2(map: &Map) -> i64 {
    let mut best_score = 0;
    for ((start_x, start_y), tree_house) in &map.heights {
        let mut score = 1;
        for d in Dir::iter() {
            let dd = d.get_dir();
            let mut j = 0;
            loop {
                j += 1;
                let x = start_x + j * dd.0;
                let y = start_y + j * dd.1;
                let h = match map.heights.get(&(x,y)) {
                    None => {score *= j-1; break;},
                    Some(v) => v,
                };
                if h >= tree_house {
                    score *= j;
                    break;
                }
            }
        }
        if score > best_score {
            best_score = score;
        }
    }
    best_score
}
fn get_part1(map: &Map) -> BTreeSet<(i64, i64)> {
    let mut visible_ones: BTreeSet<(i64,i64)> = BTreeSet::new();
    let n = map.height - 1;
    for d in Dir::iter() {
        let dd = d.get_dir();
        for i in 0..=n {
            let mut j = 0;
            let mut highest: i8 = -1;
            loop {
                if j > n {
                    break;
                }
                let (x,y) = match d {
                    Dir::E => (j * dd.0,  i),
                    Dir::W => (n + j * dd.0, i),
                    Dir::S => (i, j * dd.1),
                    Dir::N => (i, n + j * dd.1),
                };
                j += 1;
                let Some(h) =  map.heights.get(&(x,y)) else {break};
                if *h  > highest {
                    visible_ones.insert((x,y));
                    highest = *h;
                }
                if highest == 9 {
                    break;
                }
            }
        }
    }
    visible_ones
}


fn read_map(cont: &str) -> Map {
    let points: BTreeMap<(i64, i64), i8> = cont.lines().enumerate().flat_map(|(i, ln)| {
            let y = i64::try_from(i).unwrap();
            ln.chars().enumerate().map(move |(j, c)| {
                let x = i64::try_from(j).unwrap();
                let h = c.to_digit(10).unwrap();
                ((x, y), i8::try_from(h).unwrap())
            })
        }).collect::<BTreeMap<(i64, i64), i8>>();
    Map::new(points)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
let a = "30373
25512
65332
33549
35390";
        assert_eq!(read_contents(&a).0, 21);
        assert_eq!(read_contents(&a).1, 8);
    }
     
    #[test]
    fn visible() {
        let a = "30373
25512
65332
33549
35390";

        // 30373
        // 25512
        // 65332
        // 33549
        // 35390
        let map = read_map(&a);
        let visible = get_part1(&map);
        let expected_visible = [
            (0,0), (0,1), (0,2), (0,3), (0,4), // Left column
            (4,0), (4,1), (4,2), (4,3), (4,4), // Right column
            (0,0), (1,0), (2,0), (3,0), (4,0), // Top row
            (0,4), (1,4), (2,4), (3,4), (4,4), // Bottom row

        ];
        for exp in expected_visible {
            //dbg!(&exp);
            if !visible.contains(&exp) {
                dbg!(&exp);
                assert!(visible.contains(&exp));
            }
        }
    }
}

