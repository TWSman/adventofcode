use clap::Parser;
use std::collections::BTreeMap;
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
    let processes = read_processes(cont);
    let part1 = get_part1(&processes);
    let part2 = get_part2(&processes);
    (part1, part2)
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    stuff: BTreeMap<String, i64>,
    ore_used: i64,
}


impl State {
    fn get_positive_sum(&self) -> i64 {
        self.stuff.values().filter(|v| **v > 0).sum()
    }

    fn new() -> Self {
        Self {
            stuff: BTreeMap::new(),
            ore_used: 0,
        }
    }

    fn insert(&mut self, other: String, count: i64) {
        if other == "ORE" {
            self.ore_used += count;
        } else {
            self.stuff.entry(other).and_modify(|c| *c += count).or_insert(count);
        }
    }
}

fn get_part1(processes: &BTreeMap<String, Process>) -> i64 {
    get_ore(processes, 1)
}

fn get_ore(processes: &BTreeMap<String, Process>, fuel_count: i64) -> i64 {
    // Get amount of ore needed to produce given fuel count
    // Fuel count will be 1 for part1 and variable for part2
    let mut state = State::new();
    state.insert("FUEL".to_string(), fuel_count);
    loop {
        for (key, proc) in processes {
            if state.stuff.get(key).unwrap_or(&0) > &0 {
                let target_count = state.stuff.get(key).unwrap();
                let n_procs = (target_count + proc.output_count - 1) / proc.output_count;
                for (input, count) in &proc.inputs {
                    state.insert(input.clone(), *count * n_procs);
                }
                state.insert(proc.output.clone(), -(proc.output_count * n_procs));
            }
        }
        if state.get_positive_sum() == 0 {
            //println!("Found solution with ore used {}", state.ore_used);
            return state.ore_used;
        }
    }
}


fn get_part2(processes: &BTreeMap<String, Process>) -> i64 {
    // Binary search to find the maximum amount of fuel we can produce with 1 trillion ore.
    // Utilize part1 solution
    
    let ore_count = 1_000_000_000_000; // 1 trillion
    
    // get_ore gives the amount needed for 1 fuel
    // With 'ore_count' ore, we can make at least 'ore_count / get_ore(processes, 1)' fuel.
    let lower_limit = ore_count / get_ore(processes, 1);
    let (mut a, mut b) = (lower_limit, lower_limit * 2);
    assert!(get_ore(processes, a) <= ore_count);
    assert!(get_ore(processes, b) > ore_count);
    let mut i_loop = 0;
    loop {
        i_loop += 1;
        if i_loop > 40 {
            panic!("Too many loops, something is wrong");
        }
        if a == b {
            println!("Found solution with fuel count {} after {i_loop} loops", a);
            return a;
        }
        if b - a == 1 {
            let ore_used = get_ore(processes, b);
            if ore_used < ore_count {
                println!("Found solution with fuel count {} after {i_loop} loops", b);
                return b;
            }
            else {
                println!("Found solution with fuel count {} after {i_loop} loops", a);
                return a;
            }
        }
        let c = (a + b) / 2;
        let ore_used = get_ore(processes, c);

        if ore_used > ore_count {
            b = c;
        }
        else if ore_used < ore_count {
            a = c;
        }
        else {
            return c;
        }
    }
}



#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Process {
    inputs: Vec<(String, i64)>,
    output: String,
    output_count: i64,
}

impl Process {
    fn new(input: &str) -> Self {
        let (inputs, output) = input.split_once(" => ").unwrap();
        let (output_count, output) = output.split_once(" ").unwrap();
        let inputs = inputs.split(", ").map(|i| {
            let (count, name) = i.split_once(" ").unwrap();
            (name.to_string(), count.parse().unwrap())
        }).collect();
        Self {
            inputs,
            output: output.to_string(),
            output_count: output_count.parse().unwrap(),
        }
    }
}

fn read_processes(cont: &str) -> BTreeMap<String,Process> {
    // Check that no output is produced by multiple processes
    let mut outputs: BTreeSet<String> = BTreeSet::new();
    for ln in cont.lines() {
        let output = ln.split_once(" => ").unwrap().1.split_once(" ").unwrap().1;
        if outputs.contains(output) {
            panic!("Output {} is produced by multiple processes", output);
        }
        else {
            outputs.insert(output.to_string());
        }
    }
    
    cont.lines().map(|ln| {
        let tmp = Process::new(ln);
        (tmp.output.clone(), tmp)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1a() {
        let a = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let processes = read_processes(a);
        assert_eq!(get_part1(&processes), 31);
    }

    #[test]
    fn part1b() {
        let b = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

        assert_eq!(read_contents(b).0, 165);
    }

    #[test]
    fn part1c() {
        let c = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        assert_eq!(read_contents(c).0, 13312);
    }

    #[test]
    fn part1d() {
        let d = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        assert_eq!(read_contents(d).0, 180697);
    }

    #[test]
    fn part1e() {
        let e = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        assert_eq!(read_contents(e).0, 2210736);
    }

    #[test]
    fn part2c() {
        let c = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        assert_eq!(read_contents(c).1, 82892753);
    }

    #[test]
    fn part2d() {
        let d = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        assert_eq!(read_contents(d).1, 5586022);
    }

    #[test]
    fn part2e() {
        let e = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        assert_eq!(read_contents(e).1, 460664);
    }
}
