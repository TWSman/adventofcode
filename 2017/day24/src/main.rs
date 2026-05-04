use clap::Parser;
use std::collections::BTreeSet;
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
    let ports = cont.lines().map(Port::new).collect::<Vec<_>>();
    get_answer(&ports)
}

#[derive(Debug)]
struct Port {
    a: usize,
    b: usize,
}

impl Port {
    fn new(ln: &str) -> Self {
        let (a, b) = ln.split_once('/').unwrap();
        Self {
            a: a.parse::<usize>().unwrap(),
            b: b.parse::<usize>().unwrap(),
        }
    }

    fn strength(&self) -> usize {
        self.a + self.b
    }
}

#[derive(Debug)]
struct State {
    open_pins: usize,
    used: BTreeSet<usize>,
    strength: usize,
}

fn get_answer(ports: &[Port]) -> (i64, i64) {
    let mut queue = Vec::new();
    let state = State {
        open_pins: 0,
        used: BTreeSet::new(),
        strength: 0,
    };
    queue.push(state);
    let mut max_strength = 0;
    let mut longest = 0;
    let mut longest_strength = 0;

    loop {
        if queue.is_empty() {
            break;
        }
        let state = queue.pop().unwrap();
        if state.strength > max_strength {
            max_strength = state.strength;
        }
        if state.used.len() > longest {
            longest = state.used.len();
            longest_strength = state.strength;
        }

        if state.used.len() == longest && state.strength > longest_strength {
            longest_strength = state.strength;
        }

        for (i, port) in ports.iter().enumerate() {
            if state.used.contains(&i) {
                continue;
            }
            if port.a == state.open_pins {
                let mut used = state.used.clone();
                used.insert(i);
                let new_state = State {
                    open_pins: port.b,
                    used,
                    strength: state.strength + port.strength(),
                };
                queue.push(new_state);
            }

            if port.b == state.open_pins {
                let mut used = state.used.clone();
                used.insert(i);
                let new_state = State {
                    open_pins: port.a,
                    used,
                    strength: state.strength + port.strength(),
                };
                queue.push(new_state);
            }
        }
    }
    (max_strength as i64, longest_strength as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!(read_contents(&a).0, 31);
    }

    #[test]
    fn part2() {
        let a = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!(read_contents(&a).1, 19);
    }
}
