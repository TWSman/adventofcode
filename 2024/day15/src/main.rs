use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Dir {
    N,
    E,
    S,
    W,
}


impl Dir{
    const fn get_dir(self) -> (i64, i64) {
        match self {
            Self::S => (0, 1),
            Self::E => (1, 0),
            Self::N => (0, -1),
            Self::W => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Object {
    Rock,
    Robot,
    Wall,
    Empty,
}

impl Object {
    fn new(c: char) -> Self {
        match c {
            'O' => Object::Rock,
            '.' => Object::Empty,
            '@' => Object::Robot,
            '#' => Object::Wall,
            _ => panic!("Unknown character"),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    robot_loc: (usize, usize),
    grid: Vec<Vec<Object>>,
}

fn sum_vec(a: (usize, usize), b: (i64, i64)) -> (usize, usize) {
    ((a.0 as i64 + b.0) as usize
     , (a.1 as i64 + b.1) as usize)
}

impl Map {
    fn print_field(&self) {
        for ln in self.grid.iter() {
            println!("{}", ln.iter().map(|m| match m {
                Object::Rock => 'O',
                Object::Robot => '@',
                Object::Wall => '#',
                Object::Empty => '.',
            }).collect::<String>());
        }
    }

    fn get_coord_sum(&self) -> i64 {
        // y coordinates times 100
        // + x coordinate
        self.grid.iter().enumerate().map(|(y,v)|
            {
                v.iter().enumerate().map(|(x,t)| {
                    if t == &Object::Rock {
                        (100 * y + x) as i64
                    } else {
                        0
                    }
                }
                ).sum::<i64>()
            }
        ).sum()
    }
    
    fn apply(&mut self, moves: &Vec<Dir>) {
        for m in moves {
            self.apply_single(*m);
        }
    }

    fn apply_single(&mut self, mov: Dir) {
        let robot_loc = self.robot_loc;
        let (mut x, mut y) = robot_loc;
        let mut can_move: bool = true;
        let v_dir = mov.get_dir();
        loop {
            (x,y) = sum_vec((x,y), v_dir);
            match self.grid[y][x] {
                Object::Rock => {println!("Found rock at {x}, {y}"); continue;}
                Object::Empty => {println!("Found empty at {x}, {y}"); break;}
                Object::Wall => {println!("Found wall at {x}, {y}"); can_move = false; break;}
                Object::Robot => {panic!("Should not happen");}
            }
        }
        dbg!((x,y));
        if can_move {
            println!("Can move");
            // Old location is now empty
            self.grid[robot_loc.1][robot_loc.0] = Object::Empty;
            let new_robot_loc = sum_vec(self.robot_loc, v_dir);
            let (xx, yy) = new_robot_loc;

            let c = self.grid[yy][xx];
            match c {
                Object::Empty => {()
                },
                Object::Wall | Object::Robot => {
                    panic!("Should not happen");
                },
                Object::Rock => {
                    self.grid[y][x] = Object::Rock;
                },
            }
            self.grid[yy][xx] = Object::Robot;
            self.robot_loc = new_robot_loc;
        }
    }

}


fn read_map(cont: &str) -> (Map, Vec<Dir>) {
    let grid: Vec<Vec<Object>> = cont.lines().filter(|ln| ln.starts_with('#')).map(|ln| {
            ln.chars().map(move |c| {
                Object::new(c)
            }).collect::<Vec<Object>>()
        }).collect::<Vec<Vec<Object>>>();
    let instructions = cont.chars().filter_map(|c|  {
        match c {
            '>' => Some(Dir::E),
            '<' => Some(Dir::W),
            '^' => Some(Dir::N),
            'v' => Some(Dir::S),
            _ => None,
        }
        
    }
        ).collect::<Vec<Dir>>();
    let mut robot_loc: Option<(usize, usize)> = None;
    for (y,v) in grid.iter().enumerate() {
        for (x,t) in v.iter().enumerate() {
            if t == &Object::Robot {
                robot_loc = Some((x,y))
            }
        }
    }
    let map = Map {robot_loc: robot_loc.unwrap(), grid};
    (map, instructions)
}

fn read_contents(cont: &str) -> (i64, i64) {
    let (map, moves) = read_map(cont);
    map.print_field();
    let part1 = get_part1(&mut map.clone(), &moves);
    map.print_field();
    (part1, get_part2(&map))
}

fn get_part1(map: &mut Map, moves: &Vec<Dir>) -> i64 {
    map.apply(moves);

    map.get_coord_sum()
}

fn get_part2(map: &Map) -> i64 {
    0
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########
<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

        let b = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(read_contents(&b).0, 2028);
        assert_eq!(read_contents(&a).0, 10092);
    }
}
