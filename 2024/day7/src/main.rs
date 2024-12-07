use clap::Parser;
use std::fs;
use std::iter::successors;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Debug, Copy, Clone)]
enum OP {
    Sum,
    Product,
    Concat,
}

impl OP {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            OP::Product => a * b,
            OP::Sum => a + b,
            OP::Concat => {
                let siz = successors(Some(b), |&n| (n >= 10).then_some(n / 10)).count() as u32;
                a * 10_i64.pow(siz) + b
            },
                
        }
    }
}

fn get_part2(ls: &Vec<i64>, target: i64) -> i64 {
    let n = (ls.len() - 1) as u32;
    // Part2 has 3 options for operations
    let options = [OP::Product, OP::Sum, OP::Concat];
    for i in 0..(3_i32.pow(n)) {
        let mut num = i;
        let mut combination = Vec::new();
        for _ in 0..n {
            let option = options[(num % 3) as usize];
            combination.push(option);
            num /= 3;
        }
        let mut res = ls.first().expect("Should exist").to_owned();
        for (i, c) in combination.iter().enumerate() {
            let b = ls[i + 1];
            res = c.apply(res, b);
            if res > target {
                // All operations make the number bigger
                // If the result is already too big, there is no reason to continue
                continue;
            }
        }
        // As soon as a combo is found, return target
        if res == target {
            return target
        }
    }
    0
}

fn get_part1(ls: &Vec<i64>, target: i64) -> i64 {
    let n = ls.len() - 1;
    // 1 << n gives 2 ^ n
    for i in 0..(1 << n) {
        let combination: Vec<_> = (0..n)
            .map(|j| {
                // Check if jth bit is set
                if (i & (1 << j)) != 0 {
                    OP::Product
                } else {
                    OP::Sum
                }
            })
        .collect();
        let mut res = ls.first().expect("Should exist").to_owned();
        for (i, c) in combination.iter().enumerate() {
            let b = ls[i + 1];
            res = c.apply(res, b);
            if res > target {
                continue;
            }
        }
        if res == target {
            return target
        }
    }
    // If no combo matched, we return 0
    0
}



fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}


fn read_contents(cont: &str) -> (i64, i64) {
    let part1 = cont.lines().map(|ln| {
        let target = ln.split(':').next().expect("Should exist").parse::<i64>().expect("This should be a number");
        let numbers = ln.split_whitespace()
                .filter_map(|m| m.parse::<i64>().ok())
                .collect::<Vec<i64>>();
        get_part1(&numbers, target)
    }).sum();

    let part2 = cont.lines().map(|ln| {
        let target = ln.split(':').next().expect("Should exist").parse::<i64>().expect("This should be a number");
        let numbers = ln.split_whitespace()
                .filter_map(|m| m.parse::<i64>().ok())
                .collect::<Vec<i64>>();
        get_part2(&numbers, target)
    }).sum();

    (part1, part2)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let a = 
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        assert_eq!(read_contents(&a).0, 3749);
        assert_eq!(read_contents(&a).1, 11387);
    }

    #[test]
    fn concat() {
        let op = OP::Concat;
        assert_eq!(op.apply(1,2), 12);
        assert_eq!(op.apply(12,22), 1222);
        assert_eq!(op.apply(15,6), 156);
        //assert_eq!(read_contents(&a).1, 6);
    }
}
