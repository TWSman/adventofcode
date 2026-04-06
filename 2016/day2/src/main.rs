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

fn read_contents(cont: &str) -> (String, String) {
    let lines = cont.lines().collect::<Vec<_>>();
    let part1 = get_part1(&lines);
    let part2 = get_part2(&lines);
    (part1, part2)
}

// 1 2 3
// 4 5 6
// 7 8 9
// Part 1 has this arrangement, 5 is the center (0,0)
fn get_button_part1(loc: (i16, i16)) -> char {
    match loc {
        (-1, 1) => '1',
        (0, 1) => '2',
        (1, 1) => '3',
        (-1, 0) => '4',
        (0, 0) => '5',
        (1, 0) => '6',
        (-1, -1) => '7',
        (0, -1) => '8',
        (1, -1) => '9',
        _ => panic!(),
    }
}

fn get_part1(vec: &[&str]) -> String {
    let mut code = String::new();
    let mut loc = (0, 0); // start from 5
    for ln in vec {
        for c in ln.chars() {
            match c {
                'L' => {
                    loc.0 -= 1;
                }
                'R' => {
                    loc.0 += 1;
                }
                'U' => {
                    loc.1 += 1;
                }
                'D' => {
                    loc.1 -= 1;
                }
                _ => panic!(),
            }
            loc.0 = loc.0.clamp(-1, 1);
            loc.1 = loc.1.clamp(-1, 1);
        }
        let button = get_button_part1(loc);
        code.push(button);
    }
    code
}

//     1
//   2 3 4
// 5 6 7 8 9
//   A B C
//     D
// Part 2 has this arrangement. 7 is the center (0,0)

fn get_button_part2(loc: (i16, i16)) -> char {
    match loc {
        (0, 2) => '1',
        (-1, 1) => '2',
        (0, 1) => '3',
        (1, 1) => '4',
        (-2, 0) => '5',
        (-1, 0) => '6',
        (0, 0) => '7',
        (1, 0) => '8',
        (2, 0) => '9',
        (-1, -1) => 'A',
        (0, -1) => 'B',
        (1, -1) => 'C',
        (0, -2) => 'D',
        _ => panic!(),
    }
}

fn get_part2(vec: &[&str]) -> String {
    let mut code = String::new();
    let mut loc: (i16, i16) = (-2, 0); // start from 5
    for ln in vec {
        for c in ln.chars() {
            // candidate location
            let mut cand = loc;
            match c {
                'L' => {
                    cand.0 -= 1;
                }
                'R' => {
                    cand.0 += 1;
                }
                'U' => {
                    cand.1 += 1;
                }
                'D' => {
                    cand.1 -= 1;
                }
                _ => panic!(),
            }
            // Taxicab distance from the center (0,0) must be at most 2
            if cand.0.abs() + cand.1.abs() <= 2 {
                loc = cand;
            }
        }
        let button = get_button_part2(loc);
        code.push(button);
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "ULL
RRDDD
LURDL
UUUUD";
        assert_eq!(read_contents(&a).0, "1985");
    }

    #[test]
    fn part2() {
        let a = "ULL
RRDDD
LURDL
UUUUD";
        assert_eq!(read_contents(&a).1, "5DB3");
    }
}
