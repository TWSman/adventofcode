use clap::Parser;
use convolve2d::{DynamicMatrix, convolve2d};
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
    println!("Part 1 answer is {}, {}", res.0.0, res.0.1);
    println!("Part 2 answer is {:?}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> ((i64, i64), (i64, i64, i64)) {
    let input = cont.trim().parse::<i64>().unwrap();
    let matrix = get_matrix(input);
    let (part1, _) = get_conv(&matrix, 3);
    let part2 = get_part2(&matrix);
    (part1, part2)
}

fn get_matrix(input: i64) -> DynamicMatrix<i64> {
    let mut vec = Vec::new();
    for y in 1..=300 {
        for x in 1..=300 {
            vec.push(get_power(x, y, input));
        }
    }
    DynamicMatrix::new(300, 300, vec).unwrap()
}

fn get_conv(matrix: &DynamicMatrix<i64>, size: usize) -> ((i64, i64), i64) {
    let kernel = DynamicMatrix::new(size, size, vec![1; size * size]).unwrap();
    let output = convolve2d(matrix, &kernel);
    let tmp = output.into_parts();
    let t = tmp
        .2
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();
    (((t.0 % 300) as i64, (t.0 / 300) as i64), *t.1)
}

fn get_part2(matrix: &DynamicMatrix<i64>) -> (i64, i64, i64) {
    let mut max_coord = (0, 0, 0);
    let mut max_power: i64 = 0;
    for s in (1..=20).rev() {
        if max_power > s * s * 4 {
            return max_coord;
        }
        let (coord, power) = get_conv(matrix, s as usize);
        if power > max_power {
            max_coord = (coord.0 - s / 2 + 2, coord.1 - s / 2 + 2, s);
            max_power = power;
        }
    }
    max_coord
}

fn get_power(x: i64, y: i64, input: i64) -> i64 {
    let rack = x + 10;
    let mut power = rack * y + input;
    power *= rack;
    let power = power % 1000; // Keep only hundreds and below
    let power = power / 100; // Keep hundred
    power - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_power(3, 5, 8), 4);
        assert_eq!(get_power(122, 79, 57), -5);
        assert_eq!(get_power(217, 196, 39), 0);
        assert_eq!(get_power(101, 153, 71), 4);

        let matrix = get_matrix(18);
        assert_eq!(get_conv(&matrix, 3), ((33, 45), 29));

        let matrix = get_matrix(42);
        assert_eq!(get_conv(&matrix, 3), ((21, 61), 30));
    }

    #[test]
    fn part2() {
        let matrix = get_matrix(18);
        assert_eq!(get_part2(&matrix), (90, 269, 16));

        let matrix = get_matrix(42);
        assert_eq!(get_part2(&matrix), (232, 251, 12));
    }
}
