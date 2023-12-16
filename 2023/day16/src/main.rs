use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::cmp::max;
use itertools::Itertools;
use itertools::iproduct;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}


#[derive(Debug, Clone, Copy)]
enum MarkerType {
    MirrorPlus, // /
    MirrorMinus, // \
    SplitterVertical, // |
    SplitterHorizontal, // -
    Empty,
}

#[derive(Debug, Clone)]
struct Location {
    directions: Vec<Direction>,
    marker: MarkerType,
}

impl Location {
    fn new(c:char) -> Location {
        let marker = match c {
            '/' => MarkerType::MirrorPlus,
            '\\' => MarkerType::MirrorMinus,
            '-' => MarkerType::SplitterHorizontal,
            '|' => MarkerType::SplitterVertical,
            '.' => MarkerType::Empty,
            _ => {
                panic!("Unknown character");
            },
        };
        Location {marker: marker, directions: Vec::new()}
    }

    fn push(&mut self, dir: Direction) {
        self.directions.push(dir);
    }
}

#[derive(Debug, Clone)]
struct Game {
    markers: BTreeMap<(i64,i64), Location>,
    n_cols: i64,
    n_rows: i64,
}

// This is the cycle order
#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn get_dx(&self, x: i64, y: i64) -> (i64,i64) {
        match self {
            Direction::North => (  x, y-1),
            Direction::South => (  x, y+1),
            Direction::East =>  (x+1,   y),
            Direction::West =>  (x-1,   y),
        }
    }

    // / 
    fn get_plus(&self, x: i64, y: i64) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East =>  Direction::North,
            Direction::West =>  Direction::South,
        }
    }
    fn get_minus(&self, x: i64, y: i64) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East =>  Direction::South,
            Direction::West =>  Direction::North,
        }
    }
}


impl Game {
    fn new(cont: &str) -> Game {
        let line_width = cont.lines().next().expect("Should be at least 1 line").len() as i64 + 1;

        let mut locations: BTreeMap<(i64,i64), Location> = BTreeMap::new();

        let mut max_y = 0;
        for (i,c) in cont.chars().enumerate() {
            let y = (i as i64) / line_width;
            max_y = max(y, max_y);
            match c {
                '\n' | ' ' => { continue; },
                c => {
                    let x = (i as i64) % line_width;
                    let t = Location::new(c);
                    locations.insert((x,y), t);
                },
            }
        }
        Game {markers: locations, n_cols: line_width - 1, n_rows: max_y + 1}
    }

    fn print(&self) -> String {
        print(&self.markers, self.n_rows, self.n_cols)
    }

    fn solve(&mut self, start_x: i64, start_y: i64, direction: Direction) {
        propagate(&mut self.markers, direction, start_x, start_y, self.n_cols, self.n_rows);
    }

    fn get_energized(&self) -> i64 {
        self.markers.iter().filter(|(_,v)| {
            v.directions.len() > 0
        }).count() as i64
    }
}

fn print(markers: &BTreeMap<(i64,i64), Location>, n_rows: i64, n_cols: i64) -> String {
    (0..n_rows).map(|i_row| {
        (0..n_cols).map(|i_col| {
            match markers.get(&(i_col, i_row)) {
                None => panic!("Could not find marker at {} {}", i_col, i_row),
                Some(l) => {
                    match l.marker {
                        MarkerType::Empty => {
                            match l.directions.len() {
                                0 => ".",
                                1 => match l.directions[0] {
                                    Direction::North => "^",
                                    Direction::South => "v",
                                    Direction::East => ">",
                                    Direction::West => "<",
                                }
                                _v => "*",
                            }
                        }
                        MarkerType::MirrorPlus => "/",
                        MarkerType::MirrorMinus => "\\",
                        MarkerType::SplitterHorizontal => "-",
                        MarkerType::SplitterVertical => "|",
                    }
                }
            }
        }).join("")
    }).join("\n")
}


fn propagate(map: &mut BTreeMap<(i64,i64), Location>, mut direction: Direction, current_x: i64, current_y: i64, max_x: i64, max_y: i64) {
    let (mut x, mut y) = (current_x, current_y);
    loop {
        //println!("\n{}\n", print(&map, max_y, max_x));
        if (x < 0) | (x >= max_x) | (y < 0) | (y >= max_y) {
            break;
        }

        match map.get_mut(&(x, y)) {
            None => panic!("Not found {} {}", x, y),
            Some(v) => {
                if v.directions.contains(&direction) {
                    break;
                }
                v.push(direction.clone());
                match (v.marker, &direction) {
                    (MarkerType::Empty, _) =>  {
                        (x,y) = direction.get_dx(x,y);
                    },

                    (MarkerType::SplitterHorizontal, Direction::West | Direction::East) =>  {
                        (x,y) = direction.get_dx(x,y);
                    },
                    (MarkerType::SplitterVertical, Direction::North | Direction::South) =>  {
                        (x,y) = direction.get_dx(x,y);
                    },
                    (MarkerType::SplitterVertical,_) => {
                        propagate(map, Direction::North, x, y-1, max_x, max_y);
                        direction = Direction::South;
                        (x,y) = direction.get_dx(x,y);
                    },
                    (MarkerType::SplitterHorizontal,_) => {
                        propagate(map, Direction::West, x-1, y, max_x, max_y);
                        direction = Direction::East;
                        (x,y) = direction.get_dx(x,y);
                    },
                    (MarkerType::MirrorPlus, _) => {
                        direction = direction.get_plus(x,y);
                        (x,y) = direction.get_dx(x,y);
                    }
                    (MarkerType::MirrorMinus, _) => {
                        direction = direction.get_minus(x,y);
                        (x,y) = direction.get_dx(x,y);
                    }
                    _ => panic!("Stuff"),
                }
            },
        }
    }
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");

    // 0 cycles means just one tilt to north (part1)
    let res1 = read_contents(&contents);
    println!("Part 1 answer is {}", res1);

    let res1 = part2(&contents);
    println!("Part 2 answer is {}", res1);
}

fn read_contents(cont: &str) -> i64 {
    let mut game = Game::new(cont);
    //dbg!(&game);
    game.solve(0,0,Direction::East);
    println!("{}", game.print());
    game.get_energized()
}

fn part2(cont: &str) -> i64 {
    let game_orig = Game::new(cont);
    //dbg!(&game);
    let max_x = game_orig.n_cols - 1;
    let max_y = game_orig.n_rows - 1;
    let mut m: i64 = 0;
    iproduct!(0..max_x+1, 0..max_y+1).filter_map(|(x,y)| {
        if (x > 0) & (y > 0) & (x < max_x) & (y < max_y) {
            return None;
        }
        if x == 0 {
            let mut game = game_orig.clone();
            game.solve(x, y, Direction::East);
            let tmp = game.get_energized();
            if tmp > m {
                println!("{}", game.print());
                println!("{}\n", tmp);
            }
            m = max(tmp, m);
        }
        if x == max_x {
            let mut game = game_orig.clone();
            game.solve(x, y, Direction::West);
            let tmp = game.get_energized();
            if tmp > m {
                println!("{}", game.print());
                println!("{}\n", tmp);
            }
            m = max(tmp, m);
        }
        if y == 0 {
            let mut game = game_orig.clone();
            game.solve(x, y, Direction::South);
            let tmp = game.get_energized();
            if tmp > m {
                println!("{}", game.print());
                println!("{}\n", tmp);
            }
            m = max(tmp, m);
        }
        if y == max_y {
            let mut game = game_orig.clone();
            game.solve(x, y, Direction::North);
            let tmp = game.get_energized();
            if tmp > m {
                println!("{}", game.print());
                println!("{}\n", tmp);
            }
            m = max(tmp, m);
        }
        Some(m)
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        println!("{}", a);
        assert_eq!(read_contents(&a), 46);
        assert_eq!(part2(&a), 51);
    }

}
