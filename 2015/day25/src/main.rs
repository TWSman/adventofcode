use clap::Parser;
use regex::Regex;
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
    println!("Part 1 answer is {res}");
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn get_code_at(steps: usize) -> i64 {
    let mut val: i64 = 20_151_125; // start from here
    for i in 0..steps {
        if i % 1_000_000 == 0 {
            println!("i: {i} / {steps}");
        }
        val = (val * 252_533) % 33_554_393;
    }
    val
}

fn get_index(row: usize, column: usize) -> usize {
    let diag = row + column - 2;
    let diag_first = diag * (diag + 1) / 2;
    let diag_ind = column - 1;
    diag_first + diag_ind
}

fn read_contents(cont: &str) -> i64 {
    let re = Regex::new(r"row (\d*), column (\d*)").unwrap();
    let res = re.captures(cont).unwrap();
    let row = res[1].parse::<usize>().unwrap();
    let col = res[2].parse::<usize>().unwrap();
    let ind = get_index(row, col);
    println!("Get index {ind}");
    get_code_at(ind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_code_at(0), 20151125);
        assert_eq!(get_code_at(1), 31916031);
        assert_eq!(get_code_at(2), 18749137);
        assert_eq!(get_code_at(3), 16080970);
        assert_eq!(get_code_at(4), 21629792);
        assert_eq!(get_code_at(5), 17289845);
        assert_eq!(get_code_at(20), 33511524);
    }

    #[test]
    fn index() {
        // Add +1 to match numbering in the example
        assert_eq!(get_index(1, 1) + 1, 1);
        assert_eq!(get_index(2, 1) + 1, 2);
        assert_eq!(get_index(1, 2) + 1, 3);
        assert_eq!(get_index(3, 1) + 1, 4);
        assert_eq!(get_index(2, 2) + 1, 5);
        assert_eq!(get_index(1, 3) + 1, 6);
        assert_eq!(get_index(1, 6) + 1, 21);
    }
}
