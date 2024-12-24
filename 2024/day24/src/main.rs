use clap::Parser;
use std::fs;
use std::collections::BTreeMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum WireState {
    One,
    Zero,
    Or(String, String),
    And(String, String),
    Xor(String, String),
}

#[derive(Debug)]
struct Computer {
    wires: BTreeMap<String, WireState>,
    wires_original: BTreeMap<String, WireState>,
    input_size: usize,
    
}

impl Computer  {
    fn new (wires: BTreeMap<String, WireState>) -> Self {
        let input_size = wires.keys().filter(|k| k.starts_with('x')).map(|k| {
            k[1..].parse::<usize>().unwrap()
        }).max().unwrap() + 1;

        let wires_original = wires.clone();
        Self {wires, wires_original: wires_original, input_size}
    }


    fn get_state(&mut self, key: &str) -> u8 {
        let w = self.wires.clone();
        let state = w.get(key);
        let (a,b) = match state {
            Some(WireState::And(a,b)) => (a,b),
            Some(WireState::Or(a,b)) => (a,b),
            Some(WireState::Xor(a,b)) => (a,b),
            Some(WireState::Zero) => return 0,
            Some(WireState::One) => return 1,
            _ => panic!(),
        };
        let res = match state {
            Some(WireState::And(..)) => {
                if self.get_state(&a) == 0 {
                    // If a is false, no need to check b
                    0
                } else {
                    self.get_state(&b)
                }
            },
            Some(WireState::Or(..)) => {
                if self.get_state(&a) == 1 {
                    // if a is true, no need to check b
                    1
                } else {
                    self.get_state(&b)
                }
            },
            Some(WireState::Xor(..)) => {
                // With XOR must do both
                self.get_state(&a) ^ self.get_state(&b)
            }

            _ => panic!(),
        };
        if res == 0 {
            self.wires.insert(key.to_string(), WireState::Zero); 
        } else {
            self.wires.insert(key.to_string(), WireState::One); 
        }
        res
    }

    fn calculate_states(&mut self) {
        let keys = &self.wires.keys().map(|m| m.to_string()).collect::<Vec<String>>();
        for k in keys {
            let _ = self.get_state(&k);
        }
    }

    fn get_result(&self, which: char) -> u64 {
        self.wires.iter().filter(|(k,v)| k.starts_with(which)).map(|(k,v)| {
            let index = k[1..].parse::<u32>().unwrap();
            if v == &WireState::One {
                2_u64.pow(index)
            } else {
                0
            }
        }).sum::<u64>()
    }
}


fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");

    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {}", part2.join(","));

}

fn parse_line(line: &str) -> Option<(String, WireState)> {
    if line.contains(':') {
        let mut spl = line.split(':');
        let wire = spl.next().unwrap();
        let state = if spl.next().unwrap().trim() == "0" {
            WireState::Zero
        } else {
            WireState::One
        };
        return Some((wire.to_string(), state))
    }
    if line.contains("->") {
        // y16 AND x16 -> bss
        let spl = line.split_whitespace().collect::<Vec<_>>();
        let a = spl[0].to_string();
        let b = spl[2].to_string();
        let wire = spl[4].to_string();
        
        let state = match spl[1] {
            "AND" => WireState::And(a,b),
            "OR" => WireState::Or(a,b),
            "XOR" => WireState::Xor(a,b),
            _ => panic!("Unknown operator"),
        };
        return Some((wire, state))
    }
    None
}

fn get_computer(cont: &str) -> Computer {
    let wires = cont.lines().filter_map(|ln| parse_line(ln)).collect::<BTreeMap<String, WireState>>();
    Computer::new(wires)
}


fn read_contents(cont: &str) -> (u64, Vec<String>) {
    let mut comp = get_computer(cont);
    dbg!(&comp.wires.len());
    comp.calculate_states();
    let part1 = comp.get_result('z');
    //let part2 = get_part2(&mut comp);
    let part2 = Vec::new();
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!(read_contents(&a).0, 4);
        
        let b = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

        assert_eq!(read_contents(&b).0, 2024);
    }

    #[test]
    fn set() {
        let a = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        let mut comp = get_computer(&a);
        comp.set_val('x', 1);
        dbg!(&comp);
        assert_eq!(comp.get_result('x'), 1);

        comp.set_val('x', 7);
        assert_eq!(comp.get_result('x'), 7);

        comp.set_val('y', 5);
        assert_eq!(comp.get_result('y'), 5);
    }
}
