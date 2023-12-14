use clap::Parser;
use std::fs;
use std::collections::HashMap;
use std::cmp::max;
use itertools::Itertools;

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

struct Game {
    rocks: HashMap<(i64,i64), RockType>,
    already_found: HashMap<String, i64>,
    n_cols: i64,
    n_rows: i64,
}

// This is the cycle order
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}


impl Game {
    fn new(cont: &str) -> Game {
        let line_width = cont.lines().next().expect("Should be at least 1 line").len() as i64 + 1;

        let mut rocks: HashMap<(i64,i64), RockType> = HashMap::new();

        let mut max_y = 0;
        for (i,c) in cont.chars().enumerate() {
            let y = (i as i64) / line_width;
            max_y = max(y, max_y);
            match c {
                '.' | '\n' | ' ' => { continue; },
                'O' | '#' => {
                    let x = (i as i64) % line_width;
                    let t = RockType::new(c);
                    rocks.insert((x,y), t);
                },
                _ => { // Insert the Node but don't return
                    panic!("Unknown character");
                }
            }
        }
        Game {rocks: rocks, already_found: HashMap::new(), n_cols: line_width - 1, n_rows: max_y + 1}
    }

    fn save(&mut self, i: i64) -> Option<i64> {
        let tmp = self.print();
        match self.already_found.get(&tmp) {
            Some(j) => {
                return Some(*j)
            },
            None => (),
        }
        self.already_found.insert(tmp, i);
        None
    }

    fn print(&self) -> String {
        (0..self.n_rows).map(|i_row| {
            (0..self.n_cols).map(|i_col| {
                match self.rocks.get(&(i_col, i_row)) {
                    None => ".",
                    Some(RockType::Round) => "O", 
                    Some(RockType::Square) => "#", 
                }
            }).join("")
        }).join("\n")
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North | Direction::South => {
                for i_col in 0..(self.n_cols) {
                    let mut prev_i: i64 = if direction == Direction::North {
                        -1
                    } else {
                        self.n_rows 
                    };
                    let it: Box<dyn Iterator<Item = i64>> = if direction == Direction::North {
                        Box::new(0..(self.n_rows))
                    } else {
                        Box::new((0..(self.n_rows)).rev())
                    };
                    let add = if direction == Direction::North {
                        1
                    } else {
                        -1
                    };
                    for i_row in it {
                        match self.rocks.get(&(i_col, i_row)) {
                            None => continue,
                            Some(RockType::Square) => {
                                prev_i = i_row;
                            },
                            Some(RockType::Round) => {
                                prev_i += add;
                                if prev_i != i_row {
                                    self.rocks.insert((i_col, prev_i), RockType::Round);
                                    self.rocks.remove(&(i_col, i_row));
                                }
                            }
                        }
                    }
                }
            },
            Direction::East | Direction::West => {
                for i_row in 0..(self.n_rows) {
                    let mut prev_i: i64 = if direction == Direction::West {
                        -1
                    } else {
                        self.n_cols 
                    };
                    let it: Box<dyn Iterator<Item = i64>> = if direction == Direction::West {
                        Box::new(0..(self.n_cols))
                    } else {
                        Box::new((0..(self.n_cols)).rev())
                    };
                    let add = if direction == Direction::West {
                        1
                    } else {
                        -1
                    };
                    for i_col in it {
                        match self.rocks.get(&(i_col, i_row)) {
                            None => continue,
                            Some(RockType::Square) => {
                                prev_i = i_col;
                            },
                            Some(RockType::Round) => {
                                prev_i += add;
                                if prev_i != i_col {
                                    self.rocks.insert((prev_i, i_row), RockType::Round);
                                    self.rocks.remove(&(i_col, i_row));
                                }
                            }
                        }
                    }
                }
            },

        }
    }

    fn score(&self) -> i64{
        self.rocks.iter().map(|((_,y), v)| {
            match v {
                RockType::Round => self.n_rows - y,
                _ => 0
            }
        }).sum()
    }
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");

    // 0 cycles means just one tilt to north (part1)
    let res1 = read_contents(&contents, 0);
    println!("Part 1 answer is {}", res1);

    // In part2 we do 1e9 cycles
    let res2 = read_contents(&contents, 1_000_000_000);
    println!("Part 2 answer is {}", res2);
}

fn read_contents(cont: &str, cycles: i64) -> i64 {
    let mut game = Game::new(cont);
    if cycles == 0 {
        game.tilt(Direction::North);
    } else {
        game.save(0);
        for i in 0..cycles {
            game.tilt(Direction::North);    
            game.tilt(Direction::West);    
            game.tilt(Direction::South);    
            game.tilt(Direction::East);    
            match game.save(i) {
                None => (),
                Some(j) => {
                    println!("Already found at index {}, now at {}", j, i);
                    println!("{}",game.print());
                    if i > 2 {
                        let cycle_length = i - j;
                        let need_to_go = cycles - i;
                        let left_to_go = need_to_go % cycle_length - 1;
                        println!("{} steps left", left_to_go);
                        for _ in 0..left_to_go {
                            game.tilt(Direction::North);
                            game.tilt(Direction::West);   
                            game.tilt(Direction::South);
                            game.tilt(Direction::East);
                        }
                        return game.score();
                    }
                }
            }
        }
    }
    game.score()
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
        assert_eq!(read_contents(&a, 0), 136);
    }

    #[test]
    fn tilts() {
        let mut game = Game::new("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....");

        assert_eq!(game.n_cols, 10);
        assert_eq!(game.n_rows, 10);
        //println!("{}\n", game.print());
        game.tilt(Direction::North);
        //println!("{}\n", game.print());
        assert_eq!(game.print(), "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
);

        game.tilt(Direction::South);
        assert_eq!(game.print(), ".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O"
);
        println!("{}\n", game.print());
        game.tilt(Direction::East);
        println!("{}\n", game.print());

        assert_eq!(game.print(), ".....#....
....#....#
....O##...
...#......
.....OOO#O
.O#...O#.#
....O#....
......OOOO
#..OO###..
#.OOO#...O"
);
        println!("{}\n", game.print());
        game.tilt(Direction::West);
        println!("{}\n", game.print());

        assert_eq!(game.print(), ".....#....
....#....#
O....##...
...#......
OOO.....#O
O.#O...#.#
O....#....
OOOO......
#OO..###..
#OOO.#O..."
);

    }
    #[test]
    fn part2() {
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
        assert_eq!(read_contents(&a, 1_000_000_000), 64);
    }
}
