use clap::Parser;
use std::fs;
use std::time::Instant;
use std::collections::BTreeMap;
use itertools::Itertools;

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

fn read_contents(cont: &str) -> (i32, String) {
    let list = cont
        .lines()
        .collect::<Vec<_>>();
    let part1 = get_part1(&list);
    let part2 = get_part2(&list);
    (part1, part2)
}

fn get_part1(list: &[&str]) -> i32 {
    let (a,b) = list.iter().fold((0,0), |acc, line| {
        let (has2, has3) = check_str(line);
        (acc.0 + has2, acc.1 + has3)
    });
    a * b
} 

fn get_part2(list: &[&str]) -> String {
    for l in list.iter().combinations(2) {
        let l1 = l[0];
        let l2 = l[1];
        assert_eq!(l1.len(), l2.len());
        let mut diffs = 0;
        let mut diff_ind = 0;
        for i in 0..l1.len() {
            if l1.chars().nth(i) != l2.chars().nth(i) {
                diffs += 1;
                if diffs == 2 {
                    break;
                }
                diff_ind = i;
            }
        }
        if diffs == 1 {
            return l1.chars().enumerate().filter_map(|(i,c)| if i != diff_ind {Some(c)} else {None}).collect::<String>();
        }
    }
    "".to_string()
}

fn check_str(str: &str) -> (i32, i32) {
    let mut seen: BTreeMap<char, usize> = BTreeMap::new(); 

    for c in str.chars() {
        *seen.entry(c).or_default() += 1;
    }
    let mut output = (0, 0);
    for (_, v) in seen.iter() {
        if *v == 2 {
            output.0 = 1;
        }
        if *v == 3 {
            output.1 = 1;
        }
    }
    output
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

        assert_eq!(check_str("abcdef"), (0, 0));
        assert_eq!(check_str("bababc"), (1, 1));
        assert_eq!(read_contents(&a).0, 12);
    }

    #[test]
    fn part2() {
        let a = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
        assert_eq!(read_contents(&a).1, "fgij");
    }
}
