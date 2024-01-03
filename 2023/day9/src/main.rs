use clap::Parser;
use std::fs;
use itertools::Itertools;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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


fn read_line(input: &str) -> (i64, i64) {
    let mut nums: Vec<i64> = input.split_whitespace().map(|m| {m.parse::<i64>().unwrap() }).collect();
    let mut lasts: Vec<i64> = Vec::new();
    let mut firsts: Vec<i64> = Vec::new();
    // Get the difference until difference in each element is 0
    // Only need to keep track of the first and last elements in each round
    while nums.iter().any(|m| m != &0) {
        lasts.push(*nums.last().unwrap());
        firsts.push(*nums.first().unwrap());
        dbg!(&nums);
        nums = nums.iter().tuple_windows().map(|(x,y)| { y-x}).collect();
    }

    // When looping over the firsts, the iterator must be reversed, and sum is replaced with
    // alternating sum
    (lasts.iter().sum(),
        firsts.iter().rev().fold(0, |sum, l| l - sum)
    )
}


fn read_contents(cont: &str) -> (i64, i64) {
    let mut res1: i64 = 0;
    let mut res2: i64 = 0;
    for ln in cont.lines() {
        let res = read_line(ln);
        res1 += res.0;
        res2 += res.1;
    }
    (res1, res2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(read_contents(&a).0, 114);
        assert_eq!(read_contents(&a).1, 2);
    }
    
    #[test]
    fn part1() {
        assert_eq!(read_line("-5 -11 -19 -29 -41 -55 -71 -89 -109 -131 -155 -181 -209 -239 -271 -305 -341 -379 -419 -461 -505").0, -551);
        assert_eq!(read_line("0 3 6 9 12 15").0, 18);
        assert_eq!(read_line("1 3 6 10 15 21").0, 28);
        assert_eq!(read_line("10 13 16 21 30 45").0, 68);
        assert_eq!(read_line("27 49 92 176 327 586 1039 1879 3511 6711 12850 24194 44291 78456 134365 222769 358339 560653 855336 1275364 1862543").0, 2669174);
        assert_eq!(read_line("2 -1 -1 5 35 140 429 1101 2482 5067 9586 17171 29829 51664 91688 169675 327410 648943 1295161 2560233 4960367").0, 9368956);
    }
    #[test]
    fn part2() {
        assert_eq!(read_line("-5 -11 -19 -29 -41 -55 -71 -89 -109 -131 -155 -181 -209 -239 -271 -305 -341 -379 -419 -461 -505").1, -1);
        assert_eq!(read_line("0 3 6 9 12 15").1, -3);
        assert_eq!(read_line("1 3 6 10 15 21").1, 0);
        assert_eq!(read_line("10 13 16 21 30 45").1, 5);
    }
}
