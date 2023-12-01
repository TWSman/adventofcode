use clap::Parser;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
}

fn read_line(ln: &str) -> i32 {
    let mut first: char = 'a'; 
    let mut last: char = 'a';
    for c in ln.chars() {
        if !c.is_numeric() {
            continue;
        }
        if first == 'a' {
            first = c;
        }
        last = c;
    }

    let s: String = vec![first, last].iter().collect();
    match s.parse() {
        Ok(val) => val,
        Err(_) => 0 // This can happen if the line didn't include any numerical characters
    }
}

fn read_line2(ln: &str) -> i32 {
    let to_replace = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let re: Vec<Regex> = to_replace.keys().map(|k| {Regex::new(k).unwrap()}).collect();
    let mut first_ind: usize = 99;
    let mut last_ind: usize = 0;

    let mut first: char = 'a'; 
    let mut last: char = 'a';
    for (i, c) in ln.chars().enumerate() {
        if !c.is_numeric() {
            continue;
        }
        if first == 'a' {
            first = c;
            first_ind = i;
        }
        last = c;
        last_ind = i;
    }

    for r in re {
        for c in r.find_iter(&ln) {
            if c.start() < first_ind {
                first = to_replace[c.as_str()];
                first_ind = c.start();
            }
            if c.start() > last_ind {
                last = to_replace[c.as_str()];
                last_ind = c.start();
            }
        }
    }
    let s: String = vec![first, last].iter().collect();
    match s.parse() {
        Ok(val) => val,
        Err(_) => 0
    }
}

fn read_contents(cont: &str) -> (i32, i32) {
    // todo!()
    let contents = cont
        .lines() // Split the string into an iterator
        .map(String::from); // Make each slice into a string
    let mut total = 0;
    let mut total2 = 0;
    for ln in contents {
        total += read_line(&ln);
        total2 += read_line2(&ln);
    }
    (total, total2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn line() {
        assert_eq!(read_line("1abc2"), 12);
        assert_eq!(read_line("pqr3stu8vwx"), 38);
        assert_eq!(read_line("a1b2c3d4e5f"), 15);
        assert_eq!(read_line("treb7uchet"), 77);
    }
    #[test]
    fn conts() {
        let a: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(read_contents(&a).0, 142);

        let b: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(read_contents(b).0, 11 + 22 + 33 + 42 + 24 + 77);
        assert_eq!(read_contents(b).1, 281);
    }

    #[test]
    fn line2() {
        assert_eq!(read_line("two1nine"), 11);
        assert_eq!(read_line("eightwothree"), 0);
        assert_eq!(read_line("abcone2threexyz"), 22);
        assert_eq!(read_line("xtwone3four"), 33);
        assert_eq!(read_line("4nineeightseven2"), 42);
        assert_eq!(read_line("zoneight234"), 24);
        assert_eq!(read_line("7pqrstsixteen"), 77);

        assert_eq!(read_line2("eighthree"), 83);
        assert_eq!(read_line2("sevennine"), 79);
        assert_eq!(read_line2("two1nine"), 29);
        assert_eq!(read_line2("eightwothree"), 83);
        assert_eq!(read_line2("abcone2threexyz"), 13);
        assert_eq!(read_line2("xtwone3four"), 24);
        assert_eq!(read_line2("4nineeightseven2"), 42);
        assert_eq!(read_line2("zoneight234"), 14);
        assert_eq!(read_line2("7pqrstsixteen"), 76);
    }

}
