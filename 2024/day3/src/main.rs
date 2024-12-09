use regex::Regex;
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
    let part1 = get_part1(&contents);
    println!("Part 1 answer is {}", part1);  
    let part2 = get_part2(&contents);
    println!("Part 2 answer is {}", part2); 
}

// Find any substrings of form mul(a,b) and get the sum of a*b
fn get_part1(ln: &str) -> i64 {
    let re: Regex = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();
    let res = re.captures_iter(ln).map(|m| {
        m[1].parse::<i64>().unwrap() * m[2].parse::<i64>().unwrap()
    }).sum();
    res
}

// Find any substrings of form mul(a,b) and get the sum of a*b
// After command don't() any multiplication commands
// should be ignored until there is a do() command
fn get_part2(ln: &str) -> i64 {
    let re: Regex = Regex::new(r"mul\(([0-9]*),([0-9]*)\)|do\(\)|don't\(\)").unwrap();
    let mut enable: bool = true;
    let res = re.captures_iter(ln).map(|m| {
        match &m[0] {
            "do()" => {enable = true; 0},
            "don't()" => {enable = false; 0},
            _ => {
                if enable {
                    m[1].parse::<i64>().unwrap() * m[2].parse::<i64>().unwrap()
                } else {
                    0
                }
            },
        }
    }).sum();
    res
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let a = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(get_part1(&a), 161);
    }

    #[test]
    fn part2() {
        let a = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(get_part2(&a), 48);
    }

}
