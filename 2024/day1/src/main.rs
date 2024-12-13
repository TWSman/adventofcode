use clap::Parser;
use std::fs;

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

fn get_part1(left: &[i64], right: &[i64]) -> i64 {
    // In part1 need to sort the list and
    // calculate sum of pairwise differences of the sorted listsw
    let a1 = bubble_sort(left);
    let b1 = bubble_sort(right);
    a1.iter().zip(b1.iter()).map(|(a, b)| {
        i64::abs(a - b)
    }).sum()
}

fn get_part2(left: &[i64], right: &[i64]) -> i64 {
    // Part2 need to count how many times each element of left appears in right
    left.iter().map(|a| {
        a * right.iter().filter(|b| *b == a).count() as i64
    }).sum()
}

fn bubble_sort(vec: &[i64]) -> Vec<i64> {
    let mut outvec = vec.to_vec();
    let mut n = outvec.len();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..n {
            if outvec[i - 1] > outvec[i] {
                outvec.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }
    outvec
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    for ln in cont.lines() {
        let a: Vec<i64> = ln.split_whitespace().map(|m| {
            m.parse::<i64>().unwrap()
        }
        ).collect();
        left.push(a[0]);
        right.push(a[1]);
    }
    (get_part1(&left, &right),
        get_part2(&left, &right))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(read_contents(&a).0, 11);
        assert_eq!(read_contents(&a).1, 31);
    }
}
