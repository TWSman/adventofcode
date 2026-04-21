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

fn read_contents(cont: &str) -> (i32, i32) {
    let input = cont.trim().parse::<i32>().unwrap();
    let part1 = get_part1(input);
    let part2 = get_part2(input);
    (part1, part2)
}

fn get_part1(ind: i32) -> i32 {
    // Part1 is the manhattan distance to center
    let (x, y) = get_coords(ind);
    x.abs() + y.abs()
}

fn get_coords(ind: i32) -> (i32, i32) {
    let mut n: i32 = 0;
    while ind > (n * 2 + 1) * (n * 2 + 1) {
        n += 1
    }
    let max_val = (2 * n + 1) * (2 * n + 1); // Maximum value in this loop
    let mut l = 0;
    while max_val - l * n > ind {
        l += 2;
    }
    if ind == max_val {
        return (n, -n);
    }
    let closest_corner = max_val - l * n;
    match l {
        2 => (-n + (ind - closest_corner), -n),
        4 => (-n, n - (ind - closest_corner)),
        6 => (n - (ind - closest_corner), n),
        8 => (n, -n + (ind - closest_corner)),
        _ => (0, 0),
    }
}

fn get_ind(x: i32, y: i32) -> i32 {
    let n = x.abs().max(y.abs());
    let max_val = (2 * n + 1) * (2 * n + 1);
    if y == -n {
        // Bottom row
        max_val - (n - x)
    } else if x == -n {
        // Left column
        max_val - 2 * n + (-n - y)
    } else if y == n {
        // Top row
        max_val - 4 * n - (n + x)
    } else {
        // Right column
        max_val - 6 * n - (n - y)
    }
}

fn get_part2(ind: i32) -> i32 {
    let mut vec = vec![1];
    let mut i = 1;
    loop {
        i += 1;
        let coord = get_coords(i);
        let mut sum = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                let ind = get_ind(coord.0 + x, coord.1 + y);
                if ind < i {
                    sum += vec.get(ind as usize - 1).unwrap_or(&0);
                }
            }
        }
        if sum > ind {
            return sum;
        }
        vec.push(sum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_part1(1), 0);
        assert_eq!(get_part1(3), 2);
        assert_eq!(get_part1(12), 3);
        assert_eq!(get_part1(23), 2);
        assert_eq!(get_part1(1024), 31);
    }

    #[test]
    fn part2() {
        assert_eq!(get_part2(9), 10);
        assert_eq!(get_part2(55), 57);
        assert_eq!(get_part2(800), 806);
    }

    #[test]
    fn coords() {
        assert_eq!(get_coords(1), (0, 0));
        assert_eq!(get_coords(24), (1, -2));
        assert_eq!(get_coords(23), (0, -2));
        assert_eq!(get_coords(22), (-1, -2));
        assert_eq!(get_coords(21), (-2, -2));
        assert_eq!(get_coords(20), (-2, -1));
        assert_eq!(get_coords(16), (-1, 2));
        assert_eq!(get_coords(12), (2, 1));

        assert_eq!(get_coords(3), (1, 1));
        assert_eq!(get_coords(4), (0, 1));
        assert_eq!(get_coords(9), (1, -1));

        assert_eq!(get_ind(0, 0), 1);
        assert_eq!(get_ind(2, -2), 25);
        assert_eq!(get_ind(1, -2), 24);
        assert_eq!(get_ind(0, -2), 23);
        assert_eq!(get_ind(-1, -2), 22);
        assert_eq!(get_ind(-2, -2), 21);
        assert_eq!(get_ind(-2, -1), 20);
        assert_eq!(get_ind(-1, 2), 16);
        assert_eq!(get_ind(2, 1), 12);
        assert_eq!(get_ind(1, -1), 9);

        assert_eq!(get_ind(0, 0), 1);
        assert_eq!(get_ind(1, 1), 3);
        assert_eq!(get_ind(0, 1), 4);
        assert_eq!(get_ind(0, -1), 8);
    }
}
