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
    println!("Original string: {}", cont);
    let mut chars = cont.chars().collect::<Vec<char>>();
    // Find garbage
    let mut remove: Vec<(usize, usize, isize)> = vec![];
    let mut i = 0;
    let mut is_garbage: isize = -1;
    let mut garbage_count = 0;

    while i < chars.len() {
        let c = chars[i];
        if is_garbage > -1 {
            if c == '!' {
                i += 2;
                continue;
            }
            if c == '>' {
                remove.push((is_garbage as usize, i + 1, i as isize + 1 - is_garbage));
                is_garbage = -1;
            } else {
                garbage_count += 1;
            }
        } else if c == '<' {
            is_garbage = i as isize;
        }
        i += 1;
    }
    for r in remove.iter().rev() {
        println!("{:?}", &cont[(r.0)..(r.1)]);
        for _ in 0..r.2 {
            chars.remove(r.0);
        }
    }
    println!(
        "After removing garbage: {}",
        chars.iter().collect::<String>()
    );
    let mut score = 0;
    let mut depth = 0;
    for c in chars {
        if c == '{' {
            depth += 1;
            score += depth;
        }
        if c == '}' {
            depth -= 1;
        }
    }
    (score, garbage_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(read_contents("{{<ab>},{<bc>},{<cd>},{<de>}}").0, 9);
        assert_eq!(read_contents("{{<!>},{<!>},{<!>},{<a>}}").0, 3);

        assert_eq!(read_contents("{}").0, 1);
        assert_eq!(read_contents("{{{}}}").0, 6);
        assert_eq!(read_contents("{{},{}}").0, 5);
        assert_eq!(read_contents("{{{},{},{{}}}}").0, 16);
        assert_eq!(read_contents("{<a>,<a>,<a>,<a>}").0, 1);

        assert_eq!(read_contents("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(read_contents("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
    }

    #[test]
    fn part2() {
        assert_eq!(read_contents("{}").1, 0);
        assert_eq!(read_contents("<random characters>").1, 17);
        assert_eq!(read_contents("<<<<>").1, 3);
        assert_eq!(read_contents("<{!>}>").1, 2);
        assert_eq!(read_contents("<!!>").1, 0);
        assert_eq!(read_contents("<!!!>>").1, 0);
        assert_eq!(read_contents("<{o\"i!a,<{i<a>").1, 10);
    }
}
