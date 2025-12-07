use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}

fn read_contents(cont: &str) -> (i64, i64) {
    let max_row = cont.lines().count();
    dbg!(max_row);
    let (grid, start_column) = read_grid(cont);
    let part1 = get_part1(&grid, start_column, max_row);
    let part2 = get_part2(&grid, start_column, max_row);
    (part1, part2)
}

fn get_part1(splitters: &BTreeSet<(usize, usize)>, start_column: usize, max_row: usize) -> i64 {
    let mut beams: BTreeSet<usize> = BTreeSet::new();
    beams.insert(start_column);
    let mut splits: i64 = 0;
    for i_row in 1..=max_row {
        let mut beams_to_remove: Vec<usize> = Vec::new();
        //let mut beams_to_add: BTreeSet<usize> = BTreeSet::new();
        for beam_column in &beams {
            if splitters.contains(&(*beam_column, i_row)) {
                splits += 1;
                println!("Found splitter at {beam_column}, {i_row}");
                beams_to_remove.push(*beam_column);
            }
        }
        // Remove beams that hit splitters
        for beam_column in beams_to_remove {
            beams.remove(&beam_column);
            if !beams.contains(&(beam_column +1)) {
                beams.insert(beam_column + 1);
            }
            if !beams.contains(&(beam_column - 1)) {
                beams.insert(beam_column - 1);
            }
        }
    }
    splits
}


fn get_part2(splitters: &BTreeSet<(usize, usize)>, start_column: usize, max_row: usize) -> i64 {
    // column and count of how many beams there are
    let mut beams: BTreeMap<usize,usize> = BTreeMap::new();
    // 1 beam starts from start column
    beams.insert(start_column, 1);
    for i_row in 1..=max_row {
        println!("Row {i_row} of {max_row}");
        let mut beams_to_remove: Vec<usize> = Vec::new();
        let mut beams_to_add: Vec<(usize, usize)> = Vec::new();
        for (beam_column, beam_count) in &beams {
            if splitters.contains(&(*beam_column, i_row)) {
                // Remove this id
                beams_to_remove.push(*beam_column);
                // Need to add beam_count to left/right columns
                beams_to_add.push((beam_column + 1, *beam_count));
                beams_to_add.push((beam_column - 1, *beam_count));
            }
        }
        for beam_column in beams_to_remove {
            beams.remove(&beam_column);
        }
        for (beam_column, beam_count) in beams_to_add {
            match beams.get(&beam_column) {
                None => {
                    beams.insert(beam_column, beam_count);
                }
                Some(v) => {
                    beams.insert(beam_column, beam_count + v);
                }
            }
        }
    }
    beams.values().map(|v| *v as i64).sum()
}

fn read_grid(cont: &str) -> (BTreeSet<(usize,usize)>, usize) {
    let mut splitters: BTreeSet<(usize,usize)> = BTreeSet::new();
    let mut start_column: Option<usize> = None;
    for (i_ln, ln) in cont.lines().enumerate() {
        for (i_c, c) in ln.chars().enumerate() {
            match c {
                'S' => {
                    start_column = Some(i_c);
                }
                '.' => {
                    continue;
                }
                '^' => {
                    splitters.insert((i_c,i_ln));
                }
                _ => {
                    panic!("Unknown character");
                }
            }
        }
    }
    (splitters, start_column.unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a=".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(read_contents(&a).0, 21);
    }

    #[test]
    fn part2() {
        let a=".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(read_contents(&a).1, 40);
    }
}

