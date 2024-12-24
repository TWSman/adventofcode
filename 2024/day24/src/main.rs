use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

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
    switches: BTreeSet<String>,
    
}

impl Computer  {
    fn new (wires: BTreeMap<String, WireState>) -> Self {
        let input_size = wires.keys().filter(|k| k.starts_with('x')).map(|k| {
            k[1..].parse::<usize>().unwrap()
        }).max().unwrap() + 1;

        let wires_original = wires.clone();
        Self {wires, wires_original, input_size, switches: BTreeSet::new()}
    }

    fn reset(&mut self) {
        self.wires = self.wires_original.clone();
    }

    fn switch_inputs(&mut self, a: &str, b: &str) {
        let input_a = &self.wires.get(a).unwrap().clone();
        let input_b = &self.wires.get(b).unwrap().clone();
        self.wires.insert(a.to_string(), input_b.clone());
        self.wires.insert(b.to_string(), input_a.clone());
        self.switches.insert(a.to_string());
        self.switches.insert(b.to_string());
    }

    fn get_state(&mut self, key: &str) -> u8 {
        let w = self.wires.clone();
        let state = w.get(key);
        let (a,b) = match state {
            Some(WireState::And(a,b) | WireState::Xor(a,b) | WireState::Or(a,b)) => (a,b),
            Some(WireState::Zero) => return 0,
            Some(WireState::One) => return 1,
            _ => panic!(),
        };
        let res = match state {
            Some(WireState::And(..)) => {
                self.get_state(a) & self.get_state(b)
            },
            Some(WireState::Or(..)) => {
                self.get_state(a) | self.get_state(b)
            },
            Some(WireState::Xor(..)) => {
                // With XOR must do both
                self.get_state(a) ^ self.get_state(b)
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
            let _ = self.get_state(k);
        }
    }

    fn get_result(&self, which: char) -> u64 {
        self.wires.iter().filter(|(k,_)| k.starts_with(which)).map(|(k,v)| {
            let index = k[1..].parse::<u32>().unwrap();
            if v == &WireState::One {
                2_u64.pow(index)
            } else {
                0
            }
        }).sum::<u64>()
    }
}


// Part1: Calculate the state of Z__ gates and convert those to an integer
// Part2 Figure out which 4 pairs of wires need to be switched, so that the wiring works as a adder
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
        let mut a = spl[0].to_string();
        let mut b = spl[2].to_string();
        let wire = spl[4].to_string();
        if a > b {
            (b,a) = (a,b);
        }
        
        let state = match spl.get(1) {
            Some(&"AND") => WireState::And(a,b),
            Some(&"OR") => WireState::Or(a,b),
            Some(&"XOR") => WireState::Xor(a,b),
            _ => panic!("Unknown operator"),
        };
        return Some((wire, state))
    }
    None
}

fn get_computer(cont: &str) -> Computer {
    let wires = cont.lines().filter_map(parse_line).collect::<BTreeMap<String, WireState>>();
    Computer::new(wires)
}


// Wiring seems to be following the pattern of full adders such that
//
// D = A XOR B
// E = A AND B
// F = C AND D
// SUM = C XOR D
// CARRY = F OR E
//
// expect for the first bit which is a half adder
// SUM = A XOR B
// CARRY = A AND B
//
// Start from X00 and Y00 and look for deviations in the above pattern
// TODO! Automize the switches
fn analyze_wires(comp: &mut Computer) -> BTreeSet<String> {
    comp.reset();
    // With these switches the program runs
    comp.switch_inputs("fkb", "z16");
    comp.switch_inputs("rqf", "nnr");
    comp.switch_inputs("rdn", "z31");
    comp.switch_inputs("z37", "rrn");
    let mut a_and_b: BTreeMap<usize, String> = BTreeMap::new();
    let mut a_xor_b: BTreeMap<usize, String> = BTreeMap::new();
    let mut carry: BTreeMap<usize, String> = BTreeMap::new();
    let mut c_and: BTreeMap<usize, String> = BTreeMap::new();
    let mut prev_carry = String::new();
    for i in 0..comp.input_size {
        println!("i: {i}");
        let xi = format!("{}{:02}", 'x', i);
        let yi = format!("{}{:02}", 'y', i);
        let zi = format!("{}{:02}", 'z', i);
        let zstate = comp.wires.get(&zi).unwrap();
        if i == 0 {
            for (k,v) in &comp.wires {
                if v == &WireState::And(xi.clone(), yi.clone()) {
                    carry.insert(i, k.to_string());
                    prev_carry = k.to_string();
                    println!("Found carry: {k}");
                }
                if v == &WireState::Xor(xi.clone(), yi.clone()) {
                    println!("Found sum: {k}");
                    assert!(zstate == v);
                }
            }
            continue;
        }
        let mut found = false;
        let xi = format!("{}{:02}", 'x', i);
        let yi = format!("{}{:02}", 'y', i);
        for (k,v) in &comp.wires {
            if v == &WireState::And(xi.clone(), yi.clone()) {
                a_and_b.insert(i, k.to_string());
                println!("Found A & B: {k}");
            }
            if v == &WireState::Xor(xi.clone(), yi.clone()) {
                a_xor_b.insert(i, k.to_string());
                println!("Found A ^ B: {k}");
            }
        }
        let a_xor_b_here = a_xor_b.get(&i).unwrap();
        let a_and_b_here = a_and_b.get(&i).unwrap();
        for (k,v) in &comp.wires {
            // Get Sum
            if v == &WireState::Xor(a_xor_b_here.clone(), prev_carry.clone()) {
                println!("Found sum: {k}");
                assert_eq!(k, &zi);
            }
            if v == &WireState::Xor(prev_carry.clone(), a_xor_b_here.clone()) {
                println!("Found sum: {k}");
                assert_eq!(k, &zi);
            }

            // Get C_and
            if v == &WireState::And(prev_carry.clone(), a_xor_b_here.clone()) {
                c_and.insert(i, k.to_string());
                println!("Found C_and: {k}");
                found = true;
            }

            if v == &WireState::And(a_xor_b_here.clone(), prev_carry.clone()) {
                println!("Found C_and: {k}");
                c_and.insert(i, k.to_string());
                found = true;
            }
        }
        if c_and.get(&i).is_none() {
            // Couldn't find a match for this combinator
            for v in comp.wires.values() {
                match v {
                   WireState::And(a, b) if a == &prev_carry => {
                        println!("{b}");
                        dbg!(&v);
                        panic!();
                    },
                   WireState::And(a, b) if b == &prev_carry => {
                        println!("{a}");
                        dbg!(&v);
                        panic!();
                    },
                    _ => {},
                }
            }
        }
        let c_and_here = c_and.get(&i).unwrap();

        for (k, v) in &comp.wires {
            if v == &WireState::Or(c_and_here.clone(), a_and_b_here.clone()) {
                carry.insert(i, k.to_string());
                println!("Found carry: {k}");
                prev_carry = k.to_string();
            }
            if v == &WireState::Or(a_and_b_here.clone(), c_and_here.clone()) {
                carry.insert(i, k.to_string());
                println!("Found carry: {k}");
                prev_carry = k.to_string();
            }
        }
        if !found {
            dbg!(&carry);
            println!("Prev carry is {prev_carry}");
            println!("Tmp is {a_xor_b_here}");
            panic!("Could not find sum for {i}");
        }
    }
    comp.switches.clone()
}


fn read_contents(cont: &str) -> (u64, Vec<String>) {
    let mut comp = get_computer(cont);
    dbg!(&comp.wires.len());
    comp.calculate_states();
    let part1 = comp.get_result('z');
    let part2 = analyze_wires(&mut comp);
    (part1, part2.iter().map(|c| c.to_string()).collect::<Vec<String>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
        let a = "
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
        assert_eq!(read_contents(&a).1, ["z00","z01","z02","z05"]);

    }
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
}
