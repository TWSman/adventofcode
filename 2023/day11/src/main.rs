use clap::Parser;
use std::fs;
use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[derive(Debug, Clone)]
struct Galaxy {
    x: i64,
    y: i64,
}

impl Galaxy {
    fn mmove(&mut self, dx: &[i64], dy: &[i64]) {
        self.x += dx.get(self.x as usize).unwrap();
        self.y += dy.get(self.y as usize).unwrap();
    }

    fn get_dist(&self, other: &Galaxy) -> i64 {
        i64::abs(self.x - other.x) + i64::abs(self.y - other.y)
    }
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    // In part 1 we add 1 one row/column for each empty one.
    // In other words multiply amount of empty space by 2
    let res1 = read_contents(&contents, 2);
    println!("Part 1 answer is {}", res1);
    // In part 2 we multiply the amount of empty space by 1000000
    let res2 = read_contents(&contents, 1e6 as i64);
    println!("Part 2 answer is {}", res2);
}

fn read_contents(cont: &str, expansion: i64) -> i64 {
    // Expansion gives the multipliciation of empty space
    // Adding 1 row, means multiplying the amount of empty space by 2
    // Relatedly when expansion is N, we need to add N -1 rows/columns
    let line_width = cont.lines().next().expect("Should be at least 1 line").len() as i64 + 1;

    let mut cols: HashSet<i64> = HashSet::new();
    let mut rows: HashSet<i64> = HashSet::new();

    let mut galaxies: Vec<Galaxy> = cont.chars().enumerate().filter_map(|(i, c)| {
        match c {
            '.' | '\n' | ' ' => None,
            '#' => {
                let x = (i as i64) % line_width;
                let y = (i as i64) / line_width;
                cols.insert(x);
                rows.insert(y);
                Some(Galaxy {x, y})
            },
            _ => { // Insert the Node but don't return
                panic!("Unknown character");
            }
        }
    }
    ).collect();
    let max_row = *rows.iter().max().unwrap();
    let max_col = *cols.iter().max().unwrap();
    let mut col_add: Vec<i64> = vec![0]; // No need to move first column
    let mut row_add: Vec<i64> = vec![0]; // No need to move first column
    for i_col in 0..max_col {
        if cols.contains(&i_col) {
            col_add.push(*col_add.last().unwrap());
        } else {
            // We add 'e-1' columns, i.e. multiply amount of columns by 'expansion'
            col_add.push(*col_add.last().unwrap() + expansion - 1);
        }
    }

    for i_row in 0..max_row {
        if rows.contains(&i_row) {
            row_add.push(*row_add.last().unwrap());
        } else {
            // We add 'e - 1' rows, i.e. multiply amount of rows by 'expansion'
            row_add.push(*row_add.last().unwrap() + expansion - 1);
        }
    }
    for g in galaxies.iter_mut() {
        g.mmove(&col_add, &row_add);
    }
    let distance_sum: i64 = galaxies.iter().map(|g| {
        galaxies.iter().map(|o| {g.get_dist(o)}).sum::<i64>()
    }).sum::<i64>();

    distance_sum / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        assert_eq!(read_contents(&a,2), 374);
        assert_eq!(read_contents(&a,10), 1030);
        assert_eq!(read_contents(&a,100), 8410);
    }
}
