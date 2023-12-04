use clap::Parser;
use std::fs; use std::collections::HashMap;
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
    let mut counts = HashMap::from([
    ("red", 12),
    ("green", 13),
    ("blue", 14),
    ]);

    let re1 = Regex::new("Game ([0-9]*)").unwrap();
    let Some(res) = re1.captures(&ln) else { return 0; };
    let id = res[1].parse::<i32>().unwrap();

    let re = Regex::new(r"([0-9]*) (blue|red|green)").unwrap();
    for (_, [count, ball]) in re.captures_iter(ln).map(|c| c.extract()) {
        let tmp = count.parse::<i32>().unwrap();
        if tmp > counts[&ball] {
            return 0;
        } else {
            continue;
        }
    }
    id
}

fn read_line2(ln: &str) -> i32 {
    let mut counts = HashMap::from([
    ("red", 0),
    ("green", 0),
    ("blue", 0),
    ]);

    let re = Regex::new(r"([0-9]*) (blue|red|green)").unwrap();
    for (_, [count, ball]) in re.captures_iter(ln).map(|c| c.extract()) {
        // movies.push((title, year.parse::<i64>()?));
        let tmp = count.parse::<i32>().unwrap();
        if tmp > counts[&ball] {
            *counts.get_mut(&ball).unwrap() = tmp;
        } else {
            continue;
        }
    }
    counts["green"] * counts["red"] * counts["blue"]
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
        assert_eq!(read_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 1);
        assert_eq!(read_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 2);
        assert_eq!(read_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 0);
        assert_eq!(read_line("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 0);
        assert_eq!(read_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 5);
    }
    #[test]
    fn line2() {
        assert_eq!(read_line2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 48);
        assert_eq!(read_line2("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 12);
        assert_eq!(read_line2("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 1560);
        assert_eq!(read_line2("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 630);
        assert_eq!(read_line2("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 36);
    }
    #[test]
    fn conts() {
        let a: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(read_contents(&a).0, 8);
        assert_eq!(read_contents(&a).1, 2286);
    }
}
