use clap::Parser;
use colored::Colorize;
use priority_queue::PriorityQueue;
use shared::Dir;
use shared::Vec2D;
use std::cmp::Reverse;
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
    let grid = read_grid(cont);
    grid.print_grid(None);
    let part1 = get_part1(&grid);
    let part2 = get_part2(&grid);
    (part1, part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Object {
    Empty,
    Outside,
    PortalCandidate(char),
    Portal((char, char, PortalType)),
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PortalType {
    Outer,
    Inner,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec2D, Object>,
    entrance: Vec2D,
    nodes: BTreeMap<Vec2D, Node>,
    portals: BTreeMap<(char, char), Vec<(Vec2D, PortalType)>>,
}

//type Grid = BTreeMap<Vec2D, Object>;

impl Grid {
    fn print_grid(&self, loc: Option<Vec2D>) {
        let min_x = self.grid.keys().map(|v| v.x).min().unwrap();
        let max_x = self.grid.keys().map(|v| v.x).max().unwrap();
        let min_y = self.grid.keys().map(|v| v.y).min().unwrap();
        let max_y = self.grid.keys().map(|v| v.y).max().unwrap();

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if let Some(loc) = loc {
                    if (loc == Vec2D { x, y }) {
                        print!("{}", "X".red().on_white());
                        continue;
                    }
                }
                match self.grid.get(&Vec2D { x, y }) {
                    Some(Object::Wall) => {
                        print!("{}", ".".white().on_blue());
                    }
                    Some(Object::PortalCandidate(c)) => {
                        print!("{}", c.to_string().red().on_black());
                    }
                    Some(Object::Portal((_,__,PortalType::Outer))) => {
                        print!("{}", '#'.to_string().red().on_white());
                    }
                    Some(Object::Portal((_,__,PortalType::Inner))) => {
                        print!("{}", '#'.to_string().blue().on_white());
                    }
                    Some(&Object::Empty) => {
                        print!("{}", ".".white().on_white());
                    }
                    Some(&Object::Outside) => {
                        print!("{}", ".".white().on_black());
                    }
                    None => {
                        print!("{}", ".".white().on_white());
                    }
                }
            }
            println!();
        }
    }
}

fn analyze_grid(grid: &Grid) -> BTreeMap<Vec2D, Node> {
    let nodes: BTreeMap<Vec2D, Object> = grid
        .grid
        .iter()
        .filter_map(|(k, v)| match v {
            Object::Wall | Object::Empty | Object::Outside | Object::PortalCandidate(_) => None,
            Object::Portal(_)  => Some((*k, *v)),
        })
        .collect();

    let mut out = BTreeMap::new();
    for node in nodes {
        let routes = find_routes(grid, node.0);
        let nod = Node {obj: node.1,
            routes:routes };
        out.insert(node.0, nod);
    }
    out
}

#[derive(Debug, Clone)]
struct Node {
    obj: Object,
    routes: Vec<(Vec2D, Object, usize)>,
}

fn find_routes(grid: &Grid, start: Vec2D) -> Vec<(Vec2D, Object, usize)> {
    let start_state = State {
        loc: start,
        steps: 0,
    };
    let mut visited: BTreeSet<Vec2D> = BTreeSet::new();
    let mut found: Vec<_> = Vec::new();
    let mut queue = Vec::new();
    queue.push(start_state);
    visited.insert(start);
    loop {
        if queue.is_empty() {
            break;
        }
        let state = queue.pop().unwrap();
        let loc = state.loc;
        for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
            let dx = dir.get_dir_true_vec();
            let new_loc = dx + loc;
            if visited.contains(&(new_loc)) {
                continue;
            }
            match grid.grid.get(&new_loc).unwrap_or(&Object::Wall) {
                Object::Empty => {}
                Object::Wall | Object::Outside | Object::PortalCandidate(_) => {
                    continue;
                }
Object::Portal(p) => {
                    found.push((new_loc, Object::Portal(*p), state.steps + 1));
                    continue;
                }
            }
            let new_state = State {
                loc: new_loc,
                steps: state.steps + 1,
            };
            visited.insert(new_state.loc);
            queue.push(new_state);
        }
    }
    found
}

fn read_grid(cont: &str) -> Grid {
    let mut grid = cont
        .lines()
        .enumerate()
        .fold(BTreeMap::new(), |mut grid, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let obj = match c {
                    '#' => Object::Wall,
                    '.' => Object::Empty,
                    ' ' => Object::Outside,
                    c if c.is_ascii_uppercase() => Object::PortalCandidate(c),
                    c => panic!("Unknown character: {c} in grid"),
                };
                grid.insert(
                    Vec2D {
                        x: x as i64,
                        y: -(y as i64),
                    },
                    obj,
                );
            });
            grid
        });
    let portal_candidates = grid
        .iter()
        .filter(|(_, obj)| matches!(obj, Object::PortalCandidate(_)))
        .map(|(loc, obj)| (loc, obj))
        .collect::<Vec<_>>();


    let min_x = grid.keys().map(|v| v.x).min().unwrap();
    let max_x = grid.keys().map(|v| v.x).max().unwrap();
    let min_y = grid.keys().map(|v| v.y).min().unwrap();
    let max_y = grid.keys().map(|v| v.y).max().unwrap();

    let mut portals = BTreeMap::new();
    for (portal_loc, obj) in &portal_candidates {
        // There should be something either right or below the portal candidate
        let right = Vec2D {
            x: portal_loc.x + 1,
            y: portal_loc.y,
        };
        let below = Vec2D {
            x: portal_loc.x,
            y: portal_loc.y - 1,
        };
        let port_c = if let Object::PortalCandidate(c) = obj {
            c
        } else {
            panic!("Should only be iterating over portal candidates");
        };

        let portal_type = if portal_loc.x < min_x + 2 || portal_loc.x > max_x - 2 {
            PortalType::Outer
        } else if portal_loc.y < min_y + 2 || portal_loc.y > max_y - 2 {
            PortalType::Outer
        } else {
            PortalType::Inner
        };

        let right_obj = grid.get(&right).unwrap_or(&Object::Empty);
        let below_obj = grid.get(&below).unwrap_or(&Object::Empty);

        match (right_obj, below_obj) {
            (_, Object::PortalCandidate(c))=> {
                let (portal_loc, portal_name) = if grid.get(&(**portal_loc + Vec2D { x: 0, y: -2 })).unwrap_or(&Object::Outside) == &Object::Outside {
                    (**portal_loc + Vec2D { x: 0, y: 1 }, (*port_c, *c))
                } else {
                    (**portal_loc + Vec2D { x: 0, y: -2 }, (*port_c, *c))
                };
                portals
                    .entry(portal_name)
                    .or_insert_with(Vec::new)
                    .push((portal_loc, portal_type));
                println!("Found portal at {:?} with name {:?}", portal_loc, portal_name);
            }
            (Object::PortalCandidate(c), _) => {
                let (portal_loc, portal_name) = if grid.get(&(**portal_loc - Vec2D { x: 2, y: 0 })).unwrap_or(&Object::Outside) == &Object::Outside {
                    (**portal_loc + Vec2D { x: 2, y: 0 }, (*port_c, *c))
                } else {
                    (**portal_loc + Vec2D { x: -1, y: 0 }, (*port_c, *c))
                };
                portals
                    .entry(portal_name)
                    .or_insert_with(Vec::new)
                    .push((portal_loc, portal_type));
                println!("Found portal at {:?} with name {:?}", portal_loc, portal_name);
            }
            _ => continue,
        }
    }

    //dbg!(&portal_candidates);

    for (portal_name, locs) in &portals {
        for (loc, portal_type) in locs {
            grid.insert(*loc, Object::Portal((portal_name.0, portal_name.1, *portal_type)));
        }
    }

    let entrance = &grid
        .iter()
        .find(|(_, obj)| **obj == Object::Portal(('A','A', PortalType::Outer)))
        .unwrap()
        .0
        .clone();

    let gg = Grid {
        grid,
        portals: portals,
        nodes: BTreeMap::new(),
        entrance: *entrance,
    };

    dbg!(&gg.portals);
    gg
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct State {
    loc: Vec2D,
    steps: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct State2 {
    loc: Vec2D,
    level: usize,
    steps: usize,
}


fn get_part2(grid: &Grid) -> i64 {
    let mut grid = grid.clone();
    grid.nodes = analyze_grid(&grid);
    dbg!(&grid.nodes);

    let start_loc = grid.entrance;
    let start_state = State2 {
        loc: start_loc,
        level: 0,
        steps: 0,
    };

    let prio =  Reverse(start_state.steps);
    let mut queue = PriorityQueue::new();
    grid.print_grid(Some(start_state.loc));
    queue.push(start_state, prio);

    loop {
        if queue.is_empty() {
            println!("No solution found");
            break;
        }
        let (state, _prio) = queue.pop().unwrap();

        let node = grid.nodes.get(&state.loc).unwrap();
        println!("Now at: ");
        dbg!(&state.loc);
        dbg!(&node);

        for (new_loc, target, steps)  in &node.routes {
            if *target == Object::Portal(('Z','Z', PortalType::Outer)) {
                if state.level == 0 {
                    println!("Found solution with {} steps", state.steps + steps);
                    return (state.steps) as i64;
                } else {
                    continue;
                }
            }
            if *target == Object::Portal(('A','A', PortalType::Outer)) {
                continue;
            }
            if matches!(target, Object::Portal((_,_,PortalType::Outer))) && state.level == 0 {
                continue;
            }
            let portal_name = match target {
                Object::Portal((a,b, _)) => (*a,*b),
                _ => continue,
            };
            let portal_type = match target {
                Object::Portal((_,_,t)) => t,
                _ => continue,
            };
            let other_portal_locs = grid.portals.get(&portal_name).unwrap();
            let (other_portal_loc, _) = other_portal_locs
                .iter()
                .find(|(loc, _)| loc != new_loc)
                .unwrap();

            queue.push(
                State2 {
                    loc: *other_portal_loc,
                    level: match portal_type {
                        PortalType::Outer => state.level - 1,
                        PortalType::Inner => state.level + 1,
                    },
                    steps: state.steps + steps + 1,
                },
                Reverse(state.steps),
            );

        }
    }
    0
}

fn get_part1(grid: &Grid) -> i64 {
    let start_loc = grid.entrance;

    let start_state = State {
        loc: start_loc,
        steps: 0,
    };

    let prio =  Reverse(start_state.steps);
    let mut queue = PriorityQueue::new();
    grid.print_grid(Some(start_state.loc));
    queue.push(start_state, prio);

    loop {
        if queue.is_empty() {
            println!("No solution found");
            break;
        }
        let (state, _prio) = queue.pop().unwrap();

        for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
            let new_loc = state.loc + dir.get_dir_true_vec();
            match grid.grid.get(&new_loc) {
                Some(Object::Wall) | None | Some(Object::Outside) | Some(Object::PortalCandidate(_)) => continue,
                Some(Object::Empty) => {
                    queue.push(
                        State {
                            loc: new_loc,
                            steps: state.steps + 1,
                        },
                        Reverse(state.steps + 1),
                    );
                }
                Some(Object::Portal(portal_name)) => {
                    if *portal_name == ('A','A', PortalType::Outer) {
                        // No point in going back to the entrance
                        continue;
                    }
                    if *portal_name == ('Z','Z', PortalType::Outer) {
                        println!("Found solution with {} steps", state.steps + 1);
                        return (state.steps + 1) as i64;
                    }
                    let other_portal_locs = grid.portals.get(&(portal_name.0, portal_name.1)).unwrap();
                    let (other_portal_loc, _) = other_portal_locs
                        .iter()
                        .find(|(loc, _)| *loc != new_loc)
                        .unwrap();
                    queue.push(
                        State {
                            loc: *other_portal_loc,
                            steps: state.steps + 2,
                        },
                        Reverse(state.steps),
                    );
                }
            }
        }
    }
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1a() {
        let a = "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";  
        let grid = read_grid(a);
        assert_eq!(get_part1(&grid), 23);
    }

    #[test]
    fn part1b() {
        let a ="
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

        let grid = read_grid(a);
        assert_eq!(get_part1(&grid), 58);

    }


    #[test]
    fn part2() {
        let a = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

        let grid = read_grid(a);
        assert_eq!(get_part1(&grid), 77);
        assert_eq!(get_part2(&grid), 396);
    }

}
