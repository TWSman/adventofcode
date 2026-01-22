use clap::Parser;
use std::fs;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use shared::AllDir as Dir;
use std::io::{self, Write};
use colored::Colorize;

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
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn read_contents(cont: &str) -> (i64, i64) {
    let dwarves = read_map(cont);

    let part1 = get_part1(&dwarves, Some(10)).0;
    let part2 = get_part1(&dwarves, None).1;

    (part1, part2)
}

#[derive(Debug, Clone)]
struct Dwarf {
    pos: (i64,i64),
    target: Option<(i64,i64)>,
}

fn wait_for_enter() {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn read_map(cont: &str) -> Vec<Dwarf> {
    let dwarves = cont.lines().enumerate().flat_map(|(row,ln)| {
        ln.chars().enumerate().filter_map(move |(col,ch)| {
            match ch {
                '#' => {
                    Some(Dwarf {
                        pos: (col as i64, -(row as i64)),
                        target: None,
                    }
                    )},
                _ => None,
            }
        })
    }).collect::<Vec<Dwarf>>();
    dwarves
}

fn dir_allowed(grid: &BTreeSet<(i64,i64)>, dwarf: &Dwarf, moves: &[Dir]) -> bool {
    for d in moves {
        let (dx, dy) = d.get_dir_true();
        if grid.contains(&(dwarf.pos.0 + dx, dwarf.pos.1 + dy)) {
            return false;
        }
    }
    true
}


fn print_grid(grid: &BTreeSet<(i64, i64)>, min_y: i64, max_y: i64, min_x: i64, max_x: i64) {
    print!("   ");
    for x in min_x..=max_x {
        if x < 0 {
            print!(" ");
        } else {
            print!("{}", (x/10).to_string().red());
        }
    }
    println!();
    print!("   ");
    for x in min_x..=max_x {
        if x < 0 {
            print!(" ");
        } else {
            print!("{}", (x%10).to_string().red());
        }
    }
    println!();
    for y in (min_y..=max_y).rev() {
        //if (0..10).contains(&y) {
        //    print!(" {}", y.to_string().red());
        //} else if (-9..0).contains(&y) {
        //    print!("{}", y.to_string().red());
        //} else {
        //    print!("  ");
        //}
        print!("{}", format!("{:>3}", y).red().to_string());
        for x in min_x..=max_x {
            if grid.contains(&(x,y)) {
                print!("{}", "#".blue());
            } else {
                print!("{}", ".".normal());
            }
        }
        println!();
    }
}

fn get_part1(dwarves: &Vec<Dwarf>, max_rounds: Option<usize>) -> (i64, i64) {
    let mut dwarves = dwarves.to_owned();

    // Preference order: N, S, W, E
    let pref_order: Vec<Vec<Dir>> = vec![
        vec![Dir::N, Dir::NE, Dir::NW],
        vec![Dir::S, Dir::SE, Dir::SW],
        vec![Dir::W, Dir::NW, Dir::SW],
        vec![Dir::E, Dir::NE, Dir::SE],
    ];

    let all_moves: Vec<Dir> = vec![Dir::N, Dir::NE, Dir::NW, Dir::S, Dir::SE, Dir::SW, Dir::E, Dir::W];

    let mut grid: BTreeSet<(i64,i64)> = dwarves.iter().map(|d| d.pos).collect();

    let mut round = 0;
    loop {
        // List of places where dwarves would move, and how many would move to that spot
        let mut proposed_moves: BTreeMap<(i64,i64), usize> = BTreeMap::new();

        //print_grid(&grid, -10, 10, -10, 10);
        if Some(round) == max_rounds {
            break;
        }
        round += 1;
        //wait_for_enter();
        for dwarf in dwarves.iter_mut() {
            if dir_allowed(&grid, dwarf, &all_moves) {
                // Dwarves won't move if they have no neighbors
                //println!("Dwarf at {:?} sees no neighbors, skipping", dwarf.pos);
                continue;
            }
            for i_pref in 0..4 {
                let pref_dir = &pref_order[(round - 1 + i_pref) % 4]; // At round = 1, i_pref == 0 gives index 0
                //println!("Dwarf at {:?} considering direction {:?}", dwarf.pos, pref_dir[0]);
                if dir_allowed(&grid, dwarf, pref_dir) {
                    let new_target = (
                        dwarf.pos.0 + pref_dir[0].get_dir_true().0,
                        dwarf.pos.1 + pref_dir[0].get_dir_true().1,
                    );
                    dwarf.target = Some(new_target);
                    proposed_moves.entry(new_target).and_modify(|e| *e += 1).or_insert(1);
                    //println!("Dwarf {:?} proposes move to {:?}", dwarf.pos, new_target);
                    break;
                } else {
                    dwarf.target = None; // Target will be None, if no direction is allowed
                }
            }
            if dwarf.target.is_none() {
                //println!("Dwarf {:?} cannot move", dwarf.pos);
            }
        }

        let keys = proposed_moves.keys().cloned().collect::<Vec<(i64,i64)>>();
        for mov in keys {
            if proposed_moves[&mov] > 1 {
                proposed_moves.remove(&mov);
                //println!("Conflict at {:?}", mov);
            }
        }
        for dwarf in dwarves.iter_mut() {
            if let Some(target) = dwarf.target {
                if proposed_moves.contains_key(&target) {
                    grid.remove(&dwarf.pos);
                    dwarf.pos = target;
                    dwarf.target = None;
                    grid.insert(dwarf.pos);
                } else {
                    dwarf.target = None;
                }
            } else {
                continue;
            }
        }

        if max_rounds.is_none() {
            println!("Round: {}, {} moves", round, proposed_moves.len());
        }
        // No proposed moves, end simulation
        if proposed_moves.is_empty() {
            break;
        }
    }
    let max_x = dwarves.iter().map(|d| d.pos.0).max().unwrap();
    let min_x = dwarves.iter().map(|d| d.pos.0).min().unwrap();

    let max_y = dwarves.iter().map(|d| d.pos.1).max().unwrap();
    let min_y = dwarves.iter().map(|d| d.pos.1).min().unwrap();
    println!("Final grid after {} rounds:", round);
    print_grid(&grid, min_y, max_y, min_x, max_x);

    (
        (max_x - min_x + 1) * (max_y - min_y + 1) - dwarves.len() as i64,
        round as i64
    )
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part1() {

        let b = ".....
..##.
..#..
.....
..##.
.....";

        let dwarves = read_map(b);
        assert_eq!(dwarves.len(), 5);
        assert_eq!(read_contents(&b).0, 25);


        let a = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

        let dwarves = read_map(a);
        assert_eq!(dwarves.len(), 22);
        assert_eq!(read_contents(&a).0, 110);
        assert_eq!(read_contents(&a).1, 20);
    }
}
