use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents, 25);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn read_contents(cont: &str, n: usize) -> (u64, u64) {
    let numbers: Vec<u64> = cont.lines().map(|x| x.parse().unwrap()).collect();

    // First number after preamble is n+1
    for i in (n+1)..numbers.len() {
        let num = numbers[i];
        let prev = numbers[i-n..i].to_vec();
        let mut found = false;
        for j in 0..n { 
            for k in (j+1)..n {
                if prev[j] + prev[k] == num {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return (num, part2(&numbers[0..i], num));
        }
    }
    (0, 0)
}

fn part2(numbers: &[u64], num: u64) -> u64 {
    let n = numbers.len();
    for j in 0..n {
        for k in (j+1)..n {
            let sum: u64 = numbers[j..k].iter().sum();
            if sum == num {
                return numbers[j..k].iter().min().unwrap() + numbers[j..k].iter().max().unwrap();
            }
        }
    }
    0
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(read_contents(&a, 5).0, 127);
        assert_eq!(read_contents(&a, 5).1, 62);
    }

}
