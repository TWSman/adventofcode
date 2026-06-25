use clap::Parser;
use regex::Regex;
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
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i64, i64) {
    let part1 = cont.lines().map(get_part1).sum();
    let part2 = cont.lines().map(get_part2).sum();
    (part1, part2)
}

fn get_part1(ln: &str) -> i64 {
    let mut ln = ln.to_string();
    let re = Regex::new(r"\(([\d +\*]*)\)").unwrap();
    let mut new_ln = ln.clone();
    loop {
        for mat in re.captures_iter(&ln) {
            let res = get_part1(&mat[1]);
            new_ln = new_ln.replace(&mat[0], &format!("{}", res));
        }
        if ln == new_ln {
            break;
        }
        ln = new_ln.clone();
    }
    // All parentheses have been removed
    let mut res = 0;
    let mut op = "";
    for (i, c) in ln.split_whitespace().enumerate() {
        if i == 0 {
            res = c.parse::<i64>().unwrap();
            continue;
        }
        if i % 2 == 1 {
            op = c;
        } else {
            match op {
                "*" => res *= c.parse::<i64>().unwrap(),
                "+" => res += c.parse::<i64>().unwrap(),
                _ => panic!(),
            }
        }
    }
    res
}

fn get_part2(ln: &str) -> i64 {
    let mut ln = ln.to_string();
    //println!("\nEvaluating {}", ln);
    let re = Regex::new(r"\(([\d +\*]*)\)").unwrap();
    let mut new_ln = ln.clone();
    loop {
        for mat in re.captures_iter(&ln) {
            let res = get_part2(&mat[1]);
            new_ln = new_ln.replace(&mat[0], &format!("{}", res));
        }
        if ln == new_ln {
            break;
        }
        ln = new_ln.clone();
    }
    //println!("Parenthesis removed: {}", ln);

    // All parentheses have been removed
    // Next perform all sums
    let re = Regex::new(r"(\d+) \+ (\d+)").unwrap();
    let mut new_ln = ln.clone();
    loop {
        if let Some(mat) = re.captures(&ln) {
            let a = mat[1].parse::<i64>().unwrap();
            let b = mat[2].parse::<i64>().unwrap();
            let res = format!("{}", (a + b));
            new_ln = new_ln.replacen(&mat[0], &res, 1);
        }
        if ln == new_ln {
            break;
        }
        ln = new_ln.clone();
    }

    //println!("Sums removed: {}", ln);
    let mut res = 0;
    let mut op = "";
    for (i, c) in ln.split_whitespace().enumerate() {
        if i == 0 {
            res = c.parse::<i64>().unwrap();
            continue;
        }
        if i % 2 == 1 {
            op = c;
        } else {
            match op {
                "*" => res *= c.parse::<i64>().unwrap(),
                "+" => res += c.parse::<i64>().unwrap(),
                _ => panic!(),
            }
        }
    }
    //println!("Final Result: {}\n", res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(read_contents(&a).0, 71);

        let b = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(read_contents(&b).0, 51);

        let c = "2 * 3 + (4 * 5)";
        assert_eq!(read_contents(&c).0, 26);

        let d = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(read_contents(&d).0, 437);

        let e = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(read_contents(&e).0, 12240);

        let f = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(read_contents(&f).0, 13632);
    }

    #[test]
    fn part2() {
        let a = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(read_contents(&a).1, 231);

        let b = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(read_contents(&b).1, 51);

        let c = "2 * 3 + (4 * 5)";
        assert_eq!(read_contents(&c).1, 46);

        let d = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(read_contents(&d).1, 1445);

        let e = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(read_contents(&e).1, 669060);

        let f = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(read_contents(&f).1, 23340);

        let g1 = "5 + 3 + (7 * 9 + 8 + 8) + 9";
        assert_eq!(read_contents(&g1).1, 192);

        let g2 = "4 + 2 * ((2 + 8 * 7 * 9) * 7 * (2 + 4) + (7 + 9 + 5 + 6 * 2 * 6))";
        assert_eq!(read_contents(&g2).1, 8731800);

        let g3 = "5 + 8 * 5 * (6 + 8 * 7) * 5";
        assert_eq!(read_contents(&g3).1, 31850);

        let g11 = "((5 * 6 + 7 + 4 + 3) * 9) * (4 + 8 * 9 + (5 + 2 + 2 + 4 * 7) + (9 * 7 + 4 * 5 + 3) * 2) + 6 * 6 * (7 * 2 + 5) + 7";
        assert_eq!(read_contents(&g11).1, 6475593600);

        let g256 = "((8 + 3 + 7 + 5 + 8) * (9 + 7 + 4 * 4 + 7) + (3 + 7 + 8 * 7 + 2) + 6) + 3 * 8 * (9 + 3 * 8 + 7 * (5 * 7 * 3))";
        assert_eq!(read_contents(&g256).1, 1819087200);

        let g286 = "((5 + 3 * 3 + 4) + 6 * 6 + (5 + 8 * 2) + 3 * (9 * 3 * 6 * 9 * 9 + 6)) * 9 + (5 + 9 + 9 * 5 + (9 + 2 + 3 + 7 * 8)) + 5 + (6 + 3 + (5 * 5 + 3 + 9 + 3) + 8 + 2 + 8)";
        assert_eq!(read_contents(&g286).1, 195526548000);

        let g372 = "(9 + (8 * 9 * 4 * 5 + 4) * 9 * 4 * 9 * 4) * ((6 * 9) + 3) * (6 * (3 * 5 + 6 + 9 + 2) * 6 + (4 + 5 + 3 + 2 + 2 * 4)) + 8 * 4 + 9";
        assert_eq!(read_contents(&g372).1, 69259939377408);

        let g162 = "((4 * 5) + (3 * 4 + 9) * 3 + 4 + 9) + (4 * (5 + 6) * 2 * 6) * 7 + 6 * (6 + 8 * 9 * 9 * (8 * 2 + 4) * 8) + ((9 + 9 + 4 * 4 + 7) + 6 + 8 + 8 * 8)";
        assert_eq!(read_contents(&g162).1, 8373301248);
    }
}
