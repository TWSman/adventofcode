use clap::Parser;
use colored::*;
use itertools::Itertools;
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
    let res = read_contents(&contents, 25, 6);
    println!("\n########################");
    println!("Part 1 answer is {}", res);

    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str, w: usize, h: usize) -> i64 {
    let n = cont.trim().len();
    let layer_size = w * h;
    assert_eq!(n % layer_size, 0);
    let layers = cont
        .trim()
        .chars()
        .chunks(layer_size)
        .into_iter()
        .map(|chunk| chunk.map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<_>>();

    let zero_counts = layers
        .iter()
        .map(|layer| layer.iter().filter(|&&d| d == 0).count())
        .collect::<Vec<_>>();
    let i_min_zero = zero_counts
        .iter()
        .enumerate()
        .min_by_key(|&(_, &count)| count)
        .map(|(i, _)| i)
        .unwrap();
    let one_counts = layers[i_min_zero].iter().filter(|&&d| d == 1).count();
    let two_counts = layers[i_min_zero].iter().filter(|&&d| d == 2).count();
    let part1 = one_counts as i64 * two_counts as i64;

    println!("Part 2 image:");
    for y in 0..h {
        for x in 0..w {
            let ind = y * w + x;
            let col = (0..layers.len())
                .filter_map(|i| {
                    let x = layers[i][ind];
                    // 2 is transparent
                    if x != 2 { Some(x) } else { None }
                })
                .next();

            match col {
                // 0 is black
                Some(0) => print!("{}", " ".on_black()),
                // 1 is white
                Some(1) => print!("{}", " ".on_white()),
                _ => print!("{}", " ".on_red()),
            }
        }
        println!();
    }
    part1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "123456789012";
        assert_eq!(read_contents(a, 3, 2), 1);

        let b = "0222112222120000";
        assert!(read_contents(b, 2, 2) >= 3); // There could 3 or 4
    }
}
