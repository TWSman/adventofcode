use clap::Parser;
use std::fs;
use std::collections::HashSet;
use shared::Dir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Object {
    Box,
    Robot,
    Wall,
    Empty,
    Left,
    Right,
}

impl Object {
    fn new(c: char) -> Self {
        match c {
            'O' => Object::Box,
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
    (usize::try_from(a.0 as i64 + b.0).unwrap(),
      usize::try_from(a.1 as i64 + b.1).unwrap())
}

impl Map {
    fn print_field(&self) {
        for ln in &self.grid {
            println!("{}", ln.iter().map(|m| match m {
                Object::Box => 'O',
                Object::Robot => '@',
                Object::Wall => '#',
                Object::Empty => '.',
                Object::Left => '[',
                Object::Right => ']',
            }).collect::<String>());
        }
    }
    
    fn convert(&self) -> Map {
        let grid = self.grid.iter().map(|v| {
            v.iter().flat_map(|o| match o {
                Object::Box => [Object::Left, Object::Right],
                Object::Empty => [Object::Empty, Object::Empty],
                Object::Robot => [Object::Robot, Object::Empty],
                Object::Wall => [Object::Wall, Object::Wall],
                Object::Left | Object::Right => {panic!("Should not happen");},

            }).collect::<Vec<Object>>()
        }).collect::<Vec<Vec<Object>>>();

        let mut robot_loc: Option<(usize, usize)> = None;
        for (y,v) in grid.iter().enumerate() {
            for (x,t) in v.iter().enumerate() {
                if t == &Object::Robot {
                    robot_loc = Some((x,y));
                }
            }
        }
        Map {robot_loc: robot_loc.unwrap(), grid}
    }

    fn get_coord_sum(&self) -> u64 {
        // y coordinates times 100
        // + x coordinate
        self.grid.iter().enumerate().map(|(y,v)|
            {
                v.iter().enumerate().map(|(x,t)| {
                    if (t == &Object::Box) | (t == &Object::Left) {
                        (100 * y + x) as u64
                    } else {
                        0
                    }
                }
                ).sum::<u64>()
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
        let mut can_move: bool = true;
        let v_dir = mov.get_dir();
        let mut boxes_to_move: Vec<(usize, usize)> = Vec::new(); // List of boxes to move, left
        let mut head_list: Vec<(usize, usize)> = vec![(self.robot_loc)]; // List of boxes to move, left
        loop {
            if head_list.is_empty() {
                break;
            }
            let (mut x, mut y) = head_list.pop().unwrap(); // Get next candidate for moving
            (x,y) = sum_vec((x,y), v_dir); // Point for checking
            match self.grid[y].get(x).unwrap() {
                Object::Robot => {panic!("Should not happen");}
                Object::Empty => {continue;}
                Object::Wall => { can_move = false; break;}
                Object::Box => {
                    if !boxes_to_move.contains(&(x,y)) {
                        boxes_to_move.push((x,y));
                        head_list.push((x,y));
                    }
                }
                Object::Left => {
                    if !boxes_to_move.contains(&(x,y)) {
                        boxes_to_move.push((x,y));
                        head_list.push((x,y));
                    }
                    if !boxes_to_move.contains(&(x+1,y)) {
                        boxes_to_move.push((x+1,y));
                        head_list.push((x+1,y));
                    }
                },
                Object::Right => {
                    if !boxes_to_move.contains(&(x,y)) {
                        boxes_to_move.push((x,y));
                        head_list.push((x,y));
                    }
                    if !boxes_to_move.contains(&(x-1,y)) {
                        boxes_to_move.push((x-1,y));
                        head_list.push((x-1,y));
                    }
                },
            }
        }
        if can_move {
            // Old location is now empty
            self.grid[robot_loc.1][robot_loc.0] = Object::Empty;
            let new_robot_loc = sum_vec(self.robot_loc, v_dir);
            let (xx, yy) = new_robot_loc;

            let mut targets: HashSet<(usize, usize, Object)> = HashSet::new();
            let mut start_points: HashSet<(usize, usize)> = HashSet::new();
            for o in boxes_to_move {
                start_points.insert((o.0,o.1));
                let new_loc = sum_vec(o, v_dir);
                targets.insert((new_loc.0, new_loc.1, self.grid[o.1][o.0]));
            }
            for s in start_points {
                // Read what exists here currently
                let o = self.grid[s.1][s.0];
                // If no move targets this location, replace it with empty
                if !targets.contains(&(s.0,s.1,o)) {
                    self.grid[s.1][s.0] = Object::Empty;
                }
            }
            for t in targets {
                // Change target location to target object
                self.grid[t.1][t.0] = t.2;
            }
            // Set robot location
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
                robot_loc = Some((x,y));
            }
        }
    }
    let map = Map {robot_loc: robot_loc.unwrap(), grid};
    (map, instructions)
}

fn read_contents(cont: &str) -> (u64, u64) {
    let (map, moves) = read_map(cont);
    map.print_field();
    let part1 = get_part1(&mut map.clone(), &moves);
    let new_map = map.convert();
    new_map.print_field();
    (part1, get_part2(&mut new_map.clone(), &moves))
}

fn get_part1(map: &mut Map, moves: &Vec<Dir>) -> u64 {
    map.apply(moves);

    map.get_coord_sum()
}

fn get_part2(map: &mut Map, moves: &Vec<Dir>) -> u64 {
    map.apply(moves);

    map.get_coord_sum()
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
        assert_eq!(read_contents(&a).0, 10092);
        assert_eq!(read_contents(&a).1, 9021);
        assert_eq!(read_contents(&b).0, 2028);
    }
}
