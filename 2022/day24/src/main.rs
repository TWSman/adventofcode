use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use shared::Dir;
use std::io::{self, Write};
use colored::Colorize;
use priority_queue::PriorityQueue;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents, false);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn read_contents(cont: &str, wait: bool) -> (i64, i64) {
    let map = read_map(cont);
    dbg!(&map);

    let part1 = get_part1(&map, wait).0;
    let mut map = map.clone();
    let part2 = get_part2(&mut map, wait);

    (part1, part2)
}

#[derive(Debug, Clone)]
struct Blizzard {
    pos: (i64,i64),
    direction: Dir,
}

#[derive(Debug, Clone)]
struct Map {
    blizzards: Vec<Blizzard>,
    width: usize,
    height: usize,
    start: (i64,i64),
    end: (i64,i64),
}

impl Map {
    fn print_map(&self, blizzards: Option<&Vec<Blizzard>>, state: &BTreeMap<(i64,i64), usize>, loc: (i64, i64)) {
        dbg!(&loc);
        for row in (0..self.height as i64).rev() {
            for col in 0..self.width as i64 {
                if row == loc.1 && col == loc.0 {
                    print!("{}", "X".red());
                } else if (col, row) == self.start || (col, row) == self.end {
                    print!(".");
                } else if row == 0 || row == (self.height as i64 - 1) || col == 0 || col == (self.width as i64 - 1) {
                    print!("#");
                } else {
                    let count = state.get(&(col,row)).unwrap_or(&0);
                    if *count == 0 {
                        print!(".");
                    } else if *count == 1 {
                        if blizzards.is_none() {
                            print!("{}", "1".blue());
                            continue;
                        }
                        let dir = blizzards.unwrap().iter().find(|b| b.pos == (col,row)).unwrap_or_else(|| panic!("Should find blizzard at {col} {row}")).direction;
                        print!("{}", dir.get_char().to_string().blue());
                    } else {
                        print!("{}", count.to_string().blue());
                    }
                }
            }
            println!();
        }
    }
}

fn wait_for_enter() {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn read_map(cont: &str) -> Map {
    let height = cont.lines().count();
    let width = cont.lines().next().unwrap().chars().count();
    let blizzards = cont.lines().enumerate().flat_map(|(row,ln)| {
        ln.chars().enumerate().filter_map(move |(col,ch)| {
            match ch {
                '#' | '.' => None,
                c =>  Some(Blizzard { pos: (col as i64, (height - row - 1) as i64), direction: Dir::new(c)})
            }
    })
    }).collect::<Vec<Blizzard>>();
    Map {
        blizzards,
        width,
        height,
        start: (1, height as i64 - 1),
        end: (width as i64 - 2, 0),
    }
}

fn get_part2(map: &mut Map, wait: bool) -> i64 {
    let (steps1, blizzards) = get_part1(map, false);
    println!("Reached end first time in {} steps", steps1);

    // Update blizzard state
    map.blizzards = blizzards;

    // Swith start and end
    std::mem::swap(&mut map.end, &mut map.start);
    //
    let (steps2, blizzards) = get_part1(map, wait);
    println!("Back to start after {} steps", steps2);
    map.blizzards = blizzards;

    // Swith start and end again
    std::mem::swap(&mut map.end, &mut map.start);

    let (steps3, _) = get_part1(map, false);
    println!("Reached end second time in {} steps", steps3);
    
    steps1 + steps2 + steps3
}

fn get_part1(map: &Map, wait: bool) -> (i64, Vec<Blizzard>) {
    let mut blizzards = map.blizzards.clone();
    // Store the amount of blizzards in each position for each time step
    let mut blizzard_states: Vec<BTreeMap<(i64,i64), usize>> = Vec::new();

    let max_x = map.width as i64 - 2;
    let max_y = map.height as i64 - 2;

    let mut current_state: BTreeMap<(i64,i64), usize> = BTreeMap::new();
    for blizzard in blizzards.iter() {
        *current_state.entry(blizzard.pos).or_insert(0) += 1;
    }

    blizzard_states.push(current_state.clone());

    let n = blizzards.len();

    assert_eq!(current_state.values().sum::<usize>(), n);

    map.print_map(Some(&blizzards), &current_state, map.start);
    // Keep track of position and number of steps
    let mut queue: PriorityQueue<((i64,i64), usize), i64> = PriorityQueue::new();
    let dist = manhattan(map.start, map.end);
    queue.push((map.start,0), -dist);
    loop {
        if queue.is_empty() {
            break;
        }
        let ((pos, steps), _prio) = queue.pop().unwrap();

        if wait {
            println!();
            println!();
            wait_for_enter();
        }
        let new_steps = steps + 1;

        if new_steps + 1 > blizzard_states.len() {
            //println!("Calculating new blizzard state for step {}", new_steps);
            // Need to calculate new blizzard state
            for b in blizzards.iter_mut() {
                // Remove old position
                if let Some(c) = current_state.get_mut(&b.pos) {
                    *c -= 1;
                }
                if current_state.get(&b.pos).unwrap_or_else(|| panic!("Should find content at {} {}", b.pos.0, b.pos.1)) == &0 {
                    current_state.remove(&b.pos);
                }
                match b.direction {
                    Dir::E => b.pos.0 += 1,
                    Dir::W => b.pos.0 -= 1,
                    Dir::N => b.pos.1 += 1,
                    Dir::S => b.pos.1 -= 1,
                }
                if b.pos.0 == 0 {
                    b.pos.0 = max_x;
                }
                if b.pos.0 == max_x + 1 {
                    b.pos.0 = 1;
                }
                if b.pos.1 == max_y + 1 {
                    b.pos.1 = 1;
                }
                if b.pos.1 == 0 {
                    b.pos.1 = max_y;
                }
                if b.pos.0 < 0 || b.pos.0 > max_x || b.pos.1 < 0 || b.pos.1 > max_y {
                    panic!("Blizzard out of bounds: {:?}", b);
                }
                //current_state.entry(b.pos).and_modify(|c| *c += 1).or_insert(1);
                *current_state.entry(b.pos).or_insert(0) += 1;
            }
            blizzard_states.push(current_state.clone());
            if wait {
                println!("Updated blizzard state");
                println!("{:?}", current_state);
                println!("{:?}", blizzards);
                map.print_map(Some(&blizzards), &current_state, pos);
            }
        }
        let new_stat = &blizzard_states[new_steps];

        // Possible moves: stay, N, S, E, W
        for dir in [Some(Dir::N), Some(Dir::S), Some(Dir::E), Some(Dir::W), None] {
            let new_pos = match dir {
                Some(Dir::N) => (pos.0, pos.1 + 1),
                Some(Dir::S) => (pos.0, pos.1 - 1),
                Some(Dir::E) => (pos.0 + 1, pos.1),
                Some(Dir::W) => (pos.0 - 1, pos.1),
                None => (pos.0, pos.1),
            };
            if new_pos == map.end {
                println!("Reached the end in {} steps", new_steps);
                assert_eq!(new_steps + 1, blizzard_states.len()); // States should include Starting state + 1 state for each step
                return (new_steps as i64, blizzards);
            }
            if new_pos.0 <= 0 || new_pos.0 >= (map.width as i64 - 1) || new_pos.1 <= 0 || new_pos.1 >= (map.height as i64 - 1) {
                // Wall
                if new_pos != map.start && new_pos != map.end {
                    continue;
                }
            }
            if new_stat.get(&new_pos).unwrap_or(&0) > &0 {
                // Blizzard in the way
                continue;
            }
            queue.push((new_pos, new_steps), - (new_steps as i64 + manhattan(new_pos, map.end)));
        }
    }
    (0, blizzards)
}

fn manhattan(a: (i64,i64), b: (i64, i64)) -> i64 {
    // Manhattan distance
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let b = "#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#";
        let mut map = read_map(b);

        assert_eq!(get_part1(&map, false).0, 10);
        assert_eq!(get_part2(&mut map, false), 30);

        let a = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

        let mut map = read_map(a);

        assert_eq!(get_part1(&map, false).0, 18);
        assert_eq!(get_part2(&mut map, false), 54);

        assert_eq!(read_contents(&a, false).0, 18);
        assert_eq!(read_contents(&a, false).1, 54);
    }
}
