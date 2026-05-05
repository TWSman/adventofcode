use clap::Parser;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
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
    println!("Part 1 answer is {}", res);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> i64 {
    let mut machine = read_machine(cont);
    machine.run();
    machine.check_sum() as i64
}

fn read_machine(cont: &str) -> TuringMachine {
    let mut states: BTreeMap<char, State> = BTreeMap::new();

    let mut lines = cont.lines();
    let start_state = lines.next().unwrap().strip_suffix(".").unwrap().chars().last().unwrap();
    let steps = lines.next().unwrap().split_whitespace().nth(5).unwrap().parse::<usize>().unwrap();

    loop {
        if lines.next().is_none()  { break }
        let reading_state = lines.next().unwrap().strip_suffix(":").unwrap().chars().last().unwrap();
        let _ = lines.next();
        let zero_action = (
            lines.next().unwrap().chars().filter_map(|c| c.to_digit(10)).next().unwrap(),
            Dir::new(lines.next().unwrap().strip_suffix(".").unwrap().rsplit_once(" ").unwrap().1),
            lines.next().unwrap().strip_suffix(".").unwrap().chars().last().unwrap(),
        );
        let _ = lines.next();
        let one_action = (
            lines.next().unwrap().chars().filter_map(|c| c.to_digit(10)).next().unwrap(),
            Dir::new(lines.next().unwrap().strip_suffix(".").unwrap().rsplit_once(" ").unwrap().1),
            lines.next().unwrap().strip_suffix(".").unwrap().chars().last().unwrap(),
        );
        states.insert(reading_state, State {
            one_action,
            zero_action,
        });
    }

    TuringMachine {
        ones: BTreeSet::new(),
        state: start_state,
        states,
        steps,
        index: 0,
    }
}


#[derive(Debug)]
struct TuringMachine {
    index: i32,
    ones: BTreeSet<i32>,
    steps: usize,
    state: char,
    states: BTreeMap<char,State>,
}

impl TuringMachine {
    fn step(&mut self) {
        let current_state = self.states.get(&self.state).unwrap();
        let actions = if self.ones.contains(&self.index) {
            current_state.one_action
        } else {
            current_state.zero_action
        };

        if actions.0 == 1 {
            self.ones.insert(self.index);
        } else {
            self.ones.remove(&self.index);
        }

        if actions.1 == Dir::Left {
            self.index -= 1;
        } else {
            self.index += 1;
        }

        self.state = actions.2;
    }


    #[allow(dead_code)]
    fn get_str(&self) -> String {
        let mut output = format!("{}_{}_", self.state, self.index);
        let max_val = self.ones.iter().max().unwrap_or(&0).abs();
        let min_val = self.ones.iter().min().unwrap_or(&0).abs();
        let ind = max_val.max(min_val);
        for i in 0..=ind{
            if self.ones.contains(&i) {
                output.push('1');
            } else {
                output.push('0');
            }

            if self.ones.contains(&-i) {
                output.push('1');
            } else {
                output.push('0');
            }

        }
        output
    }

    #[allow(dead_code)]
    fn print_tape(&self, min_val: i32, max_val: i32) {
        for i in min_val..=max_val {
            if self.ones.contains(&i) {
                print!("1 ")
            }
            else {
                print!("0 ")
            }
        }
        println!();

    }

    fn run(&mut self) {
        for step in 0..self.steps {
            self.step();
        }
    }

    fn check_sum(&self) -> usize {
        self.ones.len()
    }
}

#[derive(Debug)]
struct State {
    zero_action: (u32, Dir, char), // Write, Move, New state
    one_action: (u32, Dir, char), // Write, Move, New state
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    Left,
    Right
}

impl Dir {
    fn new(ln: &str) -> Self {
        match ln {
            "left" => Self::Left,
            "right" => Self::Right,
            _ => {panic!("Unknown direction: {}", ln);},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";
        assert_eq!(read_contents(&a), 3);
    }

}
