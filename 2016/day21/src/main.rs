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
    let res = read_contents(&contents, "abcdefgh".to_string(), "fbgdceah".to_string());
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    Reverse(usize, usize),
    RotateLeft(usize),
    RotateRight(usize),
    Move(usize, usize),
    RotateLetter(char),
}

impl Instruction {
    fn new(ln: &str) -> Self {
        let splits = ln.split_whitespace().collect::<Vec<_>>();
        match (splits.first(), splits.get(1)) {
            (Some(&"swap"), Some(&"position")) => Instruction::SwapPosition(
                splits[2].parse::<usize>().unwrap(),
                splits[5].parse::<usize>().unwrap(),
            ),

            (Some(&"swap"), Some(&"letter")) => Instruction::SwapLetter(
                splits[2].chars().next().unwrap(),
                splits[5].chars().next().unwrap(),
            ),
            (Some(&"rotate"), Some(&"left")) => {
                Instruction::RotateLeft(splits[2].parse::<usize>().unwrap())
            }
            (Some(&"rotate"), Some(&"right")) => {
                Instruction::RotateRight(splits[2].parse::<usize>().unwrap())
            }
            (Some(&"rotate"), Some(&"based")) => {
                Instruction::RotateLetter(splits[6].chars().next().unwrap())
            }
            (Some(&"move"), _) => Instruction::Move(
                splits[2].parse::<usize>().unwrap(),
                splits[5].parse::<usize>().unwrap(),
            ),
            (Some(&"reverse"), _) => Instruction::Reverse(
                splits[2].parse::<usize>().unwrap(),
                splits[4].parse::<usize>().unwrap(),
            ),
            _ => panic!("Invalid instruction: {}", ln),
        }
    }

    fn apply(&self, input: &mut Vec<char>) {
        dbg!(self);
        let n = input.len();
        match self {
            Self::SwapPosition(x, y) => {
                input.swap(*x, *y);
            }
            Self::SwapLetter(a, b) => {
                let x = input.iter().position(|c| c == a).unwrap();
                let y = input.iter().position(|c| c == b).unwrap();
                input.swap(x, y);
            }
            Self::Reverse(x, y) => {
                input[*x..=*y].reverse();
            }
            Self::Move(x, y) => {
                let xx = input.remove(*x);
                input.insert(*y, xx);
            }
            Self::RotateLeft(x) => {
                input.rotate_left(*x);
            }
            Self::RotateRight(x) => {
                input.rotate_right(*x);
            }
            Self::RotateLetter(a) => {
                let x = input.iter().position(|c| c == a).unwrap();
                if x >= 4 {
                    input.rotate_right((x + 2) % n);
                } else {
                    input.rotate_right((x + 1) % n);
                }
            }
        }
    }

    fn apply_inverse(&self, input: &mut Vec<char>) {
        let n = input.len();
        match self {
            // These are directly invertible
            Self::SwapPosition(..) | Self::SwapLetter(..) | Self::Reverse(..) => {
                self.apply(input);
            }
            Self::Move(x, y) => {
                let yy = input.remove(*y);
                input.insert(*x, yy);
            }
            // Rotations are inverted by rotating in the opposite direction
            Self::RotateLeft(x) => {
                input.rotate_right(*x);
            }
            Self::RotateRight(x) => {
                input.rotate_left(*x);
            }
            Self::RotateLetter(_) => {
                for i in 0..n {
                    let mut test = input.clone();
                    test.rotate_left(i);
                    self.apply(&mut test);
                    if test == *input {
                        input.rotate_left(i);
                        break;
                    }
                }
            }
        }
    }
}

fn run(instructions: &[Instruction], start: Vec<char>) -> String {
    let mut output = start.clone();
    for inst in instructions {
        inst.apply(&mut output);
    }
    output.iter().collect::<String>()
}

fn run_inverse(instructions: &[Instruction], start: Vec<char>) -> String {
    let mut output = start.clone();
    for inst in instructions.iter().rev() {
        inst.apply_inverse(&mut output);
    }
    output.iter().collect::<String>()
}

fn read_contents(cont: &str, start: String, start2: String) -> (String, String) {
    let instructions = cont.lines().map(Instruction::new).collect::<Vec<_>>();
    let part1 = run(&instructions, start.chars().collect());
    let part2 = run_inverse(&instructions, start2.chars().collect());
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";
        assert_eq!(
            read_contents(&a, "abcde".to_string(), "decab".to_string()).0,
            "decab"
        );
    }

    #[test]
    fn part2() {
        let a = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

        let instructions = a.lines().map(Instruction::new).collect::<Vec<_>>();
        for inst in &instructions {
            // Check that inversions work
            let mut input = "abcde".chars().collect::<Vec<_>>();
            inst.apply(&mut input);
            inst.apply_inverse(&mut input);
            assert_eq!(input.iter().collect::<String>(), "abcde");
        }
        assert_eq!(
            read_contents(&a, "abcde".to_string(), "decab".to_string()).1,
            "abcde"
        );
    }
}
