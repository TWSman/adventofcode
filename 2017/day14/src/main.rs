use clap::Parser;
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
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i64, i64) {
    let part1 = get_part1(cont);
    let part2 = get_part2(cont);
    (part1, part2)
}

fn get_part1(cont: &str) -> i64 {
    (0..128)
        .map(|row| {
            let vec = knots(&format!("{}-{}", cont.trim(), row));
            get_binary(&vec).chars().filter(|c| *c == '1').count()
        })
        .sum::<usize>() as i64
}

fn get_part2(cont: &str) -> i64 {
    let mut used: BTreeSet<(i32, i32)> = BTreeSet::new();
    for row in 0..128 {
        let input = format!("{}-{}", cont.trim(), row);
        let vec = knots(&input);
        for (col, c) in get_binary(&vec).chars().enumerate() {
            if c == '1' {
                used.insert((row, col as i32));
            }
        }
    }
    let mut visited = BTreeSet::new();
    let mut group_count = 0;
    for row in 0..128 {
        for col in 0..128 {
            if visited.contains(&(row, col)) {
                continue;
            }
            if !used.contains(&(row, col)) {
                continue;
            }
            group_count += 1;
            visited.insert((row, col));
            let mut queue = vec![(row, col)];
            loop {
                if queue.is_empty() {
                    break;
                }
                let (row, col) = queue.pop().unwrap();
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let new_row = row + dx;
                    let new_col = col + dy;
                    if !used.contains(&(new_row, new_col)) {
                        continue;
                    }
                    if visited.contains(&(new_row, new_col)) {
                        continue;
                    }
                    queue.push((new_row, new_col));
                    visited.insert((new_row, new_col));
                }
            }
        }
    }
    group_count
}

fn knots(input: &str) -> String {
    // Run the knotting process and return the resulting hash
    let mut lengths = input.trim().chars().map(|c| c as usize).collect::<Vec<_>>();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    let mut vec = (0..256).collect::<Vec<_>>();
    let mut current_pos = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for length in &lengths {
            let mut new_vec = vec.clone();
            for j in 0..*length {
                let ind = (j + current_pos) % 256;
                let rev_ind = (current_pos + length - 1 - j) % 256;
                new_vec[ind] = vec[rev_ind];
            }
            vec = new_vec;
            current_pos += length + skip_size;
            skip_size += 1;
        }
    }
    let mut output = vec![];
    for j in 0..16 {
        let mut new_val = 0;
        for i in 0..16 {
            let ind = j * 16 + i;
            let v = vec[ind];
            new_val ^= v;
        }
        output.push(new_val);
    }
    output
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect::<String>()
}

fn get_binary(cont: &str) -> String {
    let v = cont
        .trim()
        .chars()
        .map(|i| u8::from_str_radix(&i.to_string(), 16).unwrap())
        .collect::<Vec<_>>();
    v.iter().map(|x| format!("{x:04b}")).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(read_contents("flqrgnkx").0, 8108);
    }

    #[test]
    fn hash() {
        assert_eq!(knots(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knots("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(get_binary("a0c2017"), "1010000011000010000000010111");
    }

    #[test]
    fn part2() {
        assert_eq!(read_contents("flqrgnkx").1, 1242);
    }
}
