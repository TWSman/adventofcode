use clap::Parser;

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
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (u16, u16) {
    let wires = cont
        .lines()
        .map(|ln| {
            let wire = Wire::new(ln);
            (wire.target.clone(), wire)
        })
        .collect::<BTreeMap<String, Wire>>();
    let part1 = get_part1(&wires);
    let part2 = get_part2(&wires, part1);
    (part1, part2)
}

#[derive(Debug, Clone)]
struct Wire {
    target: String,
    operation: WireType,
}

impl Wire {
    fn new(ln: &str) -> Self {
        let (op, target) = ln.split_once(" -> ").unwrap();
        Self {
            target: target.to_string(),
            operation: WireType::new(op),
        }
    }
}

#[derive(Debug, Clone)]
enum WireType {
    Lshift(String, u16),
    Rshift(String, u16),
    And(String, String),
    Or(String, String),
    Not(String),
    Input(u16),
    Copy(String),
}

impl WireType {
    fn new(str: &str) -> Self {
        let split = str.split_whitespace().collect::<Vec<&str>>();
        if split.len() == 1 && split[0].chars().all(|c| c.is_ascii_digit()) {
            Self::Input(
                split[0]
                    .parse::<u16>()
                    .unwrap_or_else(|_| panic!("{} is not a number", split[0])),
            )
        } else if split.len() == 1 {
            Self::Copy(split[0].to_string())
        } else if split[0] == "NOT" {
            Self::Not(split[1].to_string())
        } else if split[1] == "AND" {
            Self::And(split[0].to_string(), split[2].to_string())
        } else if split[1] == "OR" {
            Self::Or(split[0].to_string(), split[2].to_string())
        } else if split[1] == "LSHIFT" {
            Self::Lshift(split[0].to_string(), split[2].parse::<u16>().unwrap())
        } else if split[1] == "RSHIFT" {
            Self::Rshift(split[0].to_string(), split[2].parse::<u16>().unwrap())
        } else {
            println!("Unknown wiretype: {}", str);
            panic!("Unknown wiretype");
        }
    }
}

struct Circuit {
    values: BTreeMap<String, u16>,
    wires: BTreeMap<String, Wire>,
}

impl Circuit {
    fn get_value(&mut self, wire: &str) -> u16 {
        if self.values.contains_key(wire) {
            return *self.values.get(wire).unwrap();
        }
        //println!("Getting value for {}", wire);
        if wire.chars().all(|c| c.is_ascii_digit()) {
            return wire
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("{} is not a number", wire));
        }
        let wr = self.wires.get_mut(wire).unwrap().to_owned();
        let res = match &wr.operation {
            WireType::Input(val) => *val,
            WireType::Not(input) => !self.get_value(input),
            WireType::Copy(input) => self.get_value(input),
            WireType::Or(in1, in2) => self.get_value(in1) | self.get_value(in2),
            WireType::And(in1, in2) => self.get_value(in1) & self.get_value(in2),
            WireType::Lshift(input, shift) => self.get_value(input) << shift,
            WireType::Rshift(input, shift) => self.get_value(input) >> shift,
        };
        //println!("Got {}", res);
        self.values.insert(wire.to_string(), res);
        res.to_owned()
    }
}

fn get_part1(wires: &BTreeMap<String, Wire>) -> u16 {
    let mut circuit = Circuit {
        values: BTreeMap::new(),
        wires: wires.to_owned(),
    };
    circuit.get_value("a")
}

fn get_part2(wires: &BTreeMap<String, Wire>, part1: u16) -> u16 {
    let mut circuit = Circuit {
        values: BTreeMap::new(),
        wires: wires.to_owned(),
    };
    circuit.values.insert("b".to_string(), part1);
    circuit.get_value("a")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> a";
        assert_eq!(read_contents(a).0, 65079);
    }
}
