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

fn read_contents(cont: &str) -> (String, usize) {
    let input = cont.trim().parse::<usize>().unwrap();
    let part1 = get_part1(input);
    let part2 = get_part2(input);
    (part1, part2)
}

fn get_part1(input: usize) -> String {
    let mut vec: Vec<usize> = vec![3, 7];
    let mut i1 = 0;
    let mut i2 = 1;
    loop {
        if vec.len() > input + 10 {
            return (0..10)
                .map(|i| char::from_digit(vec[input + i] as u32, 10).unwrap())
                .collect::<String>();
        }
        let s1 = vec[i1];
        let s2 = vec[i2];

        let sum = s1 + s2;
        if sum >= 10 {
            vec.push(sum / 10);
            vec.push(sum % 10);
        } else {
            vec.push(sum);
        }
        i1 = (i1 + s1 + 1) % vec.len();
        i2 = (i2 + s2 + 1) % vec.len();
    }
}

fn get_part2(input: usize) -> usize {
    let target = input
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let n = target.len();
    let mut vec: Vec<usize> = vec![3, 7];
    let mut i1 = 0;
    let mut i2 = 1;
    loop {
        if vec.len() > n + 1 {
            assert_eq!(vec[(vec.len() - n)..].len(), n);
            if target == vec[(vec.len() - n)..] {
                return vec.len() - n;
            }
            assert_eq!(vec[(vec.len() - n - 1)..(vec.len() - 1)].len(), n);
            if target == vec[(vec.len() - n - 1)..(vec.len() - 1)] {
                return vec.len() - n - 1;
            }
        }
        let s1 = vec[i1];
        let s2 = vec[i2];

        let sum = s1 + s2;
        if sum >= 10 {
            vec.push(sum / 10);
            vec.push(sum % 10);
        } else {
            vec.push(sum);
        }
        i1 = (i1 + s1 + 1) % vec.len();
        i2 = (i2 + s2 + 1) % vec.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_part1(9), "5158916779");
        assert_eq!(get_part1(5), "0124515891");
        assert_eq!(get_part1(18), "9251071085");
        assert_eq!(get_part1(2018), "5941429882");
    }

    #[test]
    fn part2() {
        assert_eq!(get_part2(51589), 9);
        assert_eq!(get_part2(92510), 18);
        assert_eq!(get_part2(59414), 2018);
    }
}
