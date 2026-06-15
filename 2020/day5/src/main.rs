use clap::Parser;
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
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i64, i64) {
    let vec = cont.lines().map(get_row_column).collect::<Vec<_>>();
    let part1 = get_part1(&vec);
    let part2 = get_part2(&vec);
    (part1, part2)
}

fn get_row_column(ln: &str) -> (u16, u16) {
    let row = &ln[..7];
    let col = &ln[7..];
    let row = u16::from_str_radix(&row.replace('F', "0").replace('B', "1"), 2).unwrap();
    let col = u16::from_str_radix(&col.replace('L', "0").replace('R', "1"), 2).unwrap();
    (row, col)
}

fn get_part1(list: &[(u16, u16)]) -> i64 {
    list.iter()
        .map(|(row, col)| *row as i64 * 8 + *col as i64)
        .max()
        .unwrap_or(0)
}

fn get_part2(list: &[(u16, u16)]) -> i64 {
    let mut ids = list
        .iter()
        .map(|(row, col)| *row as i64 * 8 + *col as i64)
        .collect::<Vec<_>>();
    ids.sort();
    let mut prev = None;
    for i in ids {
        if prev.is_none() {
            prev = Some(i);
            continue;
        }
        if prev.unwrap() + 1 != i {
            return i - 1;
        }
        prev = Some(i);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_row_column("FBFBBFFRLR"), (44, 5));
        assert_eq!(get_row_column("BFFFBBFRRR"), (70, 7));
        assert_eq!(get_row_column("FFFBBBFRRR"), (14, 7));
        assert_eq!(get_row_column("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn part2() {}
}
