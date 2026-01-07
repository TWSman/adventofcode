use clap::Parser;
use std::fs;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rock {
    vals: Vec<(i64, i64)>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Rock {
    fn shift(&self, x: i64, y: i64) -> Self {
        Self { vals: self.vals.iter().map(|(vx, vy)| (vx + x, vy + y)).collect() }
    }

    fn max_y(&self) -> i64 {
        self.vals.iter().map(|(_, y)| *y).max().unwrap_or(0)
    }

    fn check_overlap(&self, grid: &BTreeSet<(i64, i64)>) -> bool {
        // Check if there is overlap between the rock and the grid
        // Or between the rock and the walls/floor
        // Floor is at y = -1
        // Walls are at x = -1 and x = 7
        self.vals.iter().any(|(x, y)| { 
            grid.contains(&(*x, *y)) || *x < 0 || *x > 6 || *y < 0
        })
    }
}


fn get_rocks() -> Vec<Rock> {
    // y=0 is the lowest row for each rock
    // x=0 is the leftmost column
    vec![ 
        // Rock1:
        // ####
     Rock { vals: vec![(0,0), (1,0), (2,0), (3,0)] },

        // Rock2:
        // .#.
        // ###
        // .#.
     Rock { vals: vec![(1,0), (0,1), (1,1), (2,1), (1,2)] },

        // Rock3:
        // ..#
        // ..#
        // ###
     Rock { vals: vec![(0,0), (1,0), (2,0), (2,1), (2,2)] },

        // Rock4:
        // #
        // #
        // #
        // #
     Rock { vals: vec![(0,0), (0,1), (0,2), (0,3)] },

        // Rock5:
        // ##
        // ##
     Rock { vals: vec![(0,0), (0,1), (1,0), (1,1)] },
    ]
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}

fn print_grid(grid: &BTreeSet<(i64, i64)>, min_y: i64, max_y: i64, rock: Option<&Rock>) {
    for y in (min_y..=max_y).rev() {
        print!("|");
        for x in 0..7 {
            if grid.contains(&(x,y)) {
                print!("#");
            } else if let Some(r) = rock {
                if r.vals.contains(&(x,y)) {
                    print!("@");
                } else {
                    print!(".");
                }
            } else {
                print!(".");
            }
        }
        print!("|");
        println!();
    }
    println!("+++++++++")
}


fn get_jet_pattern(cont: &str) -> Vec<Direction> {
    cont.trim().chars().map(|c| {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Unknown direction character {:?}", c),
        }
    }).collect::<Vec<Direction>>()
}

fn read_contents(cont: &str) -> (i64, i64) {
    let jet_pattern = get_jet_pattern(cont);
    dbg!(&jet_pattern.len());
    let part1 = get_height(&jet_pattern, 2022, false);
    //let part2 = get_part2(&jet_pattern, 10usize.pow(4), false);
    let part2 = get_height(&jet_pattern, 1_000_000_000_000, false);
    (part1, part2)
}


fn get_height(jet_pattern: &[Direction], rock_count: usize, print: bool) -> i64 {
    // Get the height of rock pile after rock_count rocks have fallen
    let rocks = get_rocks();
    let mut rock_id = 0;
    let mut jet_id = 0;
    let mut max_height: i64 = 0;
    let mut grid: BTreeSet<(i64, i64)> = BTreeSet::new();

    let mut loop_storage: Option<(i64, i64)> = None;

    let mut jet_ids_seen: BTreeMap<usize, Vec<(usize, i64)>> = BTreeMap::new(); // Jet id and max height

    // Loop over each rock
    for i in 0..rock_count {
        let remaining_rocks = (rock_count - i) as i64;


        // Check if we have detected a loop
        if let Some(l) = loop_storage {
            let loop_size = l.0;
            let height_increase = l.1;
            if remaining_rocks % loop_size == 0 {
                println!("Loop detected, fast forwarding");
                let loops = remaining_rocks / loop_size;
                let res = max_height + loops * height_increase;
                dbg!(&res);
                return res;
            }
        }

        // Check for loop, when we are at the first rock
        if rock_id == 0 && loop_storage.is_none() {
            if jet_ids_seen.contains_key(&jet_id) {
                let v = jet_ids_seen.get_mut(&jet_id).unwrap();
                v.push((i, max_height));
                if v.len() > 1 {
                    // This is the second time we have detected this jet id
                    // Now we can calculate our loop size
                    // In theory there might be something weirder going on at the start
                    // (Because the first rock had fallen on a flat floor, instead of what the pile
                    // formation is right now. This could affect the first loop). 
                    // Which means that the very first loop candidate might not be the actual loop.
                    // But in practice this works for both the test data and the input given.
                    let last = v[v.len()-1];
                    let second_last = v[v.len()-2];
                    let loop_size = last.0 - second_last.0;
                    let height_increase = last.1 - second_last.1;
                    println!("Found loop size of {} rocks with height increase of {}", loop_size, height_increase);
                    loop_storage = Some((loop_size as i64, height_increase));
                    println!("Top of the stack:");
                    print_grid(&grid, max_height - 15, max_height + 1, None);
                }
            } else {
                let v = vec![(i, max_height)];
                jet_ids_seen.insert(jet_id, v);
            }
        }

        let mut rock = rocks[rock_id].clone();
        rock_id = (rock_id + 1) % rocks.len();

        // Rocks start 3 blocks above the highest rock (or floor)
        // Rocks start 2 units away from the left wall
        let start_height = max_height + 3;
        rock = rock.shift(2, start_height);

        if print {
            print_grid(&grid, 0, start_height + 3, Some(&rock));
        }
        let mut loop_count = 0;
        loop {
            loop_count += 1;
            if loop_count > 600 {
                panic!();
            }
            let dir = jet_pattern[jet_id];
            jet_id = (jet_id + 1) % jet_pattern.len();


            // Move rock tentatively left or right
            let shifted_rock = match dir {
                Direction::Left => rock.shift(-1, 0),
                Direction::Right => rock.shift(1, 0),
            };

            // Check for overlap. If no overlap, move rock
            if !shifted_rock.check_overlap(&grid) {
                rock = shifted_rock;
            }

            // Move rock tentatively downwards
            let fallen_rock = rock.shift(0, -1);

            // Check for overlap. If no overlap, move rock downwards
            // If there is overlap, the rock stops falling
            if !fallen_rock.check_overlap(&grid) {
                // No overlap
                //println!("Moved rock {:?} to the {:?}", rock, fallen_rock);
                rock = fallen_rock;
            } else {
                max_height = max_height.max(rock.max_y() + 1);
                for pos in &rock.vals {
                    grid.insert(*pos);
                }
                if print {
                    print_grid(&grid, 0, max_height + 1, None);
                }
                break;
            }
        }
    }
    max_height
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rocks() {
        let rocks = get_rocks();
        assert_eq!(rocks[0].vals.len(), 4);
        let rock1 = rocks[0].shift(2,3);
        assert_eq!(rock1.vals.len(), 4);
        assert_eq!(rock1.vals, vec![(2,3), (3,3), (4,3), (5,3)])

    }

    #[test]
    fn part1() {
        let a =">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        let jet_pattern = get_jet_pattern(a);
        assert_eq!(get_height(&jet_pattern, 1, false), 1);
        assert_eq!(get_height(&jet_pattern, 2, false), 4);
        assert_eq!(get_height(&jet_pattern, 3, false), 6);
        assert_eq!(get_height(&jet_pattern, 4, false), 7);
        assert_eq!(get_height(&jet_pattern, 5, false), 9);
        assert_eq!(get_height(&jet_pattern, 6, false), 10);
        assert_eq!(get_height(&jet_pattern, 7, false), 13);
        assert_eq!(get_height(&jet_pattern, 8, false), 15);
        assert_eq!(get_height(&jet_pattern, 9, false), 17);
        assert_eq!(get_height(&jet_pattern, 10, true), 17);
        assert_eq!(read_contents(&a).0, 3068);
    }
    
    #[test]
    fn part2() {
        let a =">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        assert_eq!(read_contents(&a).1, 1514285714288);

    }
}
