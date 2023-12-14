use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::cmp::max;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}


#[derive(Debug, Clone)]
enum RockType {
    Round, //O
    Square, //#
}

impl RockType {
    fn new(c:char) -> RockType {
        match c {
            'O' => RockType::Round,
            '#' => RockType::Square,
            _ => {
                panic!("Unknown character");
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Rock {
    x: i64,
    y: i64,
    t: RockType
}

impl Rock {
    fn mmove(&mut self, dx: &Vec<i64>, dy: &Vec<i64>) {
        self.x += dx.get(self.x as usize).unwrap();
        self.y += dy.get(self.y as usize).unwrap();
    }
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");
    // In part 1 we add 1 one row/column for each empty one.
    // In other words multiply amount of empty space by 2
    let res1 = read_contents(&contents);
    println!("Part 1 answer is {}", res1);
    // In part 2 we multiply the amount of empty space by 1000000
    //let res2 = read_contents(&contents, 1e6 as i64);
    //println!("Part 2 answer is {}", res2);
}

fn read_contents(cont: &str) -> i64 {
    // Expansion gives the multipliciation of empty space
    // Adding 1 row, means multiplying the amount of empty space by 2
    // Relatedly when expansion is N, we need to add N -1 rows/columns
    let line_width = cont.lines().next().expect("Should be at least 1 line").len() as i64 + 1;

    let mut cols: BTreeMap<i64, Vec<Rock>> = BTreeMap::new();
    //let mut rows: Vec<> = HashSet::new();

    let mut max_y = 0;
    for (i,c) in cont.chars().enumerate() {
        let mut y = (i as i64) / line_width;
        max_y = max(y, max_y);
        match c {
            '.' | '\n' | ' ' => { continue; },
            'O' | '#' => {
                let x = (i as i64) % line_width;
                let col = match cols.get_mut(&x) {
                    None => {
                        let c = Vec::new();
                        cols.insert(x, c);
                        cols.get_mut(&x).unwrap()
                    },
                    Some(v) => {
                        v
                    }
                };
                let t = RockType::new(c);
                match (col.last(), &t) {
                    (_, RockType::Square) => {
                        y = (i as i64) / line_width;
                    },
                    (None, RockType::Round) => {
                        y = 0;
                    },
                    (Some(v), RockType::Round) => {
                        y = v.y +1;
                    }
                }
                //let y = (i as i64) / line_width;
                let r = Rock {x: x, y: y, t: t};
                col.push(r);
            },
            _ => { // Insert the Node but don't return
                panic!("Unknown character");
            }
        }
    }

    dbg!(&cols);
    cols.values().map(|col| {
        col.iter().map(|r| {
            match r.t {
                RockType::Round => 1 + max_y - r.y,
                _ => 0
            }
        }).sum::<i64>()
    }
    ).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(read_contents(&a), 136);
    }
}
