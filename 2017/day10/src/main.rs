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
    let res = read_contents(&contents, 256);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str, l: usize) -> (i64, String) {
    let input1 = cont
        .split(',')
        .map(|s| s.trim().parse::<usize>().unwrap_or(0))
        .collect::<Vec<_>>();

    let part1 = get_part1(&input1, l);
    let part2 = get_part2(cont);
    (part1, part2)
}

fn get_part2(cont: &str) -> String {
    let mut input = cont.trim().chars().map(|c| c as usize).collect::<Vec<_>>();
    input.extend_from_slice(&[17, 31, 73, 47, 23]);
    let vec = knots(&input, 256, 64);
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

fn knots(lengths: &[usize], list_size: usize, loops: usize) -> Vec<usize> {
    // Run the knotting process for the given number of loops, and return the resulting vector
    let mut vec = (0..list_size).collect::<Vec<_>>();
    let mut current_pos = 0;
    let mut skip_size = 0;
    for _ in 0..loops {
        for length in lengths {
            let mut new_vec = vec.clone();
            for j in 0..*length {
                let ind = (j + current_pos) % list_size;
                let rev_ind = (current_pos + length - 1 - j) % list_size;
                new_vec[ind] = vec[rev_ind];
            }
            vec = new_vec;
            current_pos += length + skip_size;
            skip_size += 1;
        }
    }
    vec
}

fn get_part1(lengths: &[usize], list_size: usize) -> i64 {
    let vec = knots(lengths, list_size, 1);
    vec[0] as i64 * vec[1] as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(read_contents("3, 4, 1, 5", 5).0, 12);
    }

    #[test]
    fn part2() {
        assert_eq!(read_contents("", 5).1, "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(
            read_contents("AoC 2017", 5).1,
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(
            read_contents("1,2,3", 5).1,
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
        assert_eq!(
            read_contents("1,2,4", 5).1,
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
