use clap::Parser;
use std::fs;
use std::collections::HashSet;


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


fn read_line(ln: &str) -> (i32, i32) {
    let (_a,b) = ln.split_once(':').unwrap();
    let (c, d)= b.split_once('|').unwrap();
    let wins: HashSet<&str> = HashSet::from_iter(
        c.split_whitespace()
    );

    let count = d.split_whitespace().filter(|m| {wins.contains(m)}).count();

    if count == 0 {
        (0, count as i32)
    } else {
        (i32::pow(2, (count - 1) as u32), count as i32)
    }
}

fn read_contents(cont: &str) -> (i32, i32) {
    let count = cont.lines().collect::<Vec<&str>>().len();
    let contents = cont
        .lines() // Split the string into an iterator
        .map(String::from); // Make each slice into a string
    let mut total = 0;
    let mut counts: Vec::<i32> = (0..count).map(|_| 1).collect();
    let mut wins: Vec::<i32> = vec![];

    for ln in contents {
        let (value, n) = read_line(&ln);
        total += value;
        wins.push(n);
    }
    for i in 0..count {
        if wins[i] > 0 {
            let cards = counts[i];
            let n = wins[i];
            for j in 1..=n {
                counts[i+j as usize] += cards; 
            }
        }
    }

    (total, counts.iter().sum())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conts() {
        let a: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(read_contents(&a).0, 13);
        assert_eq!(read_contents(&a).1, 30 );
    }
    #[test]
    fn lines() {
        assert_eq!(read_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), (8,4));
        assert_eq!(read_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), (2,2));
        assert_eq!(read_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), (2,2));
        assert_eq!(read_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), (1,1));
        assert_eq!(read_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), (0,0));
        assert_eq!(read_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), (0,0));
    }
}
