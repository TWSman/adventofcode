use clap::Parser;
use std::fs;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt;
use std::cmp::max;

use num_format::{Locale, ToFormattedString};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

// Only module that sends to rx is zr (conjunction)
// zr sends a low pulse when all memory inputs are high
// The zr inputs are cm, gc, sz, xf, all are conjunctions
// They must send a high input, which must they must received lows
// cm has input &ks, which has several flipflop inputs
// cm must send high, which means ks must send low,
// all ks inputs must be high
// gc has input &dn, which has several flipflop inputs
// All dn inputs must be high
// sz has input &ms, which has several flipflop inputs
// All ms inputs must be high
// fx has input &tc, which has several flipflop inputs
// All tc inputs must be high


fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");
    let res = part1(&contents, 1000);
    println!("Part 1 answer is {}", res);
    let res = part2(&contents);
    //println!("Part 2 answer is {}", res);
    println!("Part 2 answer is {}", res.to_formatted_string(&Locale::fr));
}

fn lcm(vals: Vec<i64>) -> i64 {
    let primes: Vec<i64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
        67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157,
        163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257,
        263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367,
        373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467,
        479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
        601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709,
        719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829,
        839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967,
        971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063,
        1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181,
        1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289,
        1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423,
        1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499,
        1511, 1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609,
        1613, 1619, 1621, 1627, 1637, 1657, 1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723,
        1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861,
        1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979,
        1987, 1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 2053, 2063, 2069, 2081, 2083,
        2087, 2089, 2099, 2111, 2113, 2129, 2131, 2137, 2141, 2143, 2153, 2161, 2179, 2203, 2207,
        2213, 2221, 2237, 2239, 2243, 2251, 2267, 2269, 2273, 2281, 2287, 2293, 2297, 2309, 2311,
        2333, 2339, 2341, 2347, 2351, 2357, 2371, 2377, 2381, 2383, 2389, 2393, 2399, 2411, 2417,
        2423, 2437, 2441, 2447, 2459, 2467, 2473, 2477, 2503, 2521, 2531, 2539, 2543, 2549, 2551,
        2557, 2579, 2591, 2593, 2609, 2617, 2621, 2633, 2647, 2657, 2659, 2663, 2671, 2677, 2683,
        2687, 2689, 2693, 2699, 2707, 2711, 2713, 2719, 2729, 2731, 2741, 2749, 2753, 2767, 2777,
        2789, 2791, 2797, 2801, 2803, 2819, 2833, 2837, 2843, 2851, 2857, 2861, 2879, 2887, 2897,
        2903, 2909, 2917, 2927, 2939, 2953, 2957, 2963, 2969, 2971, 2999, 3001, 3011, 3019, 3023,
        3037, 3041, 3049, 3061, 3067, 3079, 3083, 3089, 3109, 3119, 3121, 3137, 3163, 3167, 3169,
        3181, 3187, 3191, 3203, 3209, 3217, 3221, 3229, 3251, 3253, 3257, 3259, 3271, 3299, 3301,
        3307, 3313, 3319, 3323, 3329, 3331, 3343, 3347, 3359, 3361, 3371, 3373, 3389, 3391, 3407,
        3413, 3433, 3449, 3457, 3461, 3463, 3467, 3469, 3491, 3499, 3511, 3517, 3527, 3529, 3533,
        3539, 3541, 3547, 3557, 3559, 3571, 3581, 3583, 3593, 3607, 3613, 3617, 3623, 3631, 3637,
        3643, 3659, 3671, 3673, 3677, 3691, 3697, 3701, 3709, 3719, 3727, 3733, 3739, 3761, 3767,
        3769, 3779, 3793, 3797, 3803, 3821, 3823, 3833, 3847, 3851, 3853, 4073, 4091, 4093 ];
    let mut counts: BTreeMap<i64, u32> = BTreeMap::new();
    for v in vals {
        dbg!(&v);
        let i_sum: u32 = primes.iter().map(|p| {
            if p > &v {
                return 0;
            }
            let mut res = v;
            let mut i = 0;
            let mut remainder;
            loop {
                (res, remainder) = (res / p, res %p);
                if remainder > 0 {
                    break;
                }
                i += 1;
            }
            if i > 0 {
                if counts.contains_key(&p) {
                    let t = counts.get(&p).unwrap();
                    counts.insert(*p, max(i, *t));
                } else {
                    counts.insert(*p, i);
                }
            }
            i
        }).sum();
        if i_sum == 0 {
            panic!("No factors for {}", v);
        }
    }
    counts.iter().map(|(prime,pow)| {
        prime.pow(*pow)
    }).product()
}

fn part1(input: &str, max_presses: i64) -> i64 {
    let mut modules: BTreeMap<String, Module> = BTreeMap::new();
    let module_names: Vec<String> =  input.lines().map(|ln| {
        let module = Module::new(&ln);
        let tmp = module.name.clone();
        modules.insert(module.name.clone(), module);
        tmp
    }).collect();

    let modules_clone = modules.clone();

    for module_name in &module_names {
        if let Some(module) = &modules_clone.get(module_name) {
            for t in &module.targets {
                if t == module_name {
                    panic!("Self as target");
                }
                if let Some(target_module) = modules.get_mut(t) {
                    target_module.add_input(module_name.clone());
                } else {
                    ()
                }
            }
        }
    }

    let mut pulses: VecDeque<PulseReturn> = VecDeque::new();
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    let mut button_presses = 0;
    loop {
        let pulse = match pulses.pop_front() {
            None => { 
                if button_presses < max_presses {
                    button_presses += 1;
                    pulses.push_back(PulseReturn {from:"button".to_string(), to: "broadcaster".to_string(), ptype: PulseType::Low});
                    continue;
                } else {
                    break;
                }
            },
            Some(p) => p,
        };
        match pulse.ptype {
            PulseType::High => {
                high_pulse_count += 1;
            },
            PulseType::Low => {
                low_pulse_count += 1;
            },
        }
        if pulse.to == "output" {
            continue;
        }
        if pulse.to == "rx" {
            continue;
        }
        for r in modules.get_mut(&pulse.to).expect("Target not found").receive(pulse.from, pulse.ptype) {
            pulses.push_back(r);
        }
    }
    low_pulse_count * high_pulse_count
}

fn part2(input: &str) -> i64 {
    let mut modules: BTreeMap<String, Module> = BTreeMap::new();
    let module_names: Vec<String> =  input.lines().map(|ln| {
        let module = Module::new(&ln);
        let tmp = module.name.clone();
        modules.insert(module.name.clone(), module);
        tmp
    }).collect();

    let modules_clone = modules.clone();

    for module_name in &module_names {
        if let Some(module) = &modules_clone.get(module_name) {
            for t in &module.targets {
                if t == module_name {
                    panic!("Self as target");
                }
                if let Some(target_module) = modules.get_mut(t) {
                    target_module.add_input(module_name.clone());
                } else {
                    ()
                }
            }
        }
    }
    let mut inputs_to_follow: BTreeMap<(String,String), (PulseType, Vec<i64>, i64)> = BTreeMap::new();
    for m in ["ks", "dn", "ms", "tc"].iter() {
        match modules.get(&m.to_string()) {
            Some(module) => {
                for t in &module.inputs {
                    inputs_to_follow.insert(
                        (module.name.clone(), t.to_string()),
                        (PulseType::Low, Vec::new(), -1)
                    );
                }
            }
            None => (),
        }
    }
    let mut pulses: VecDeque<PulseReturn> = VecDeque::new();
    let mut button_presses = 0;
    loop {
        let pulse = match pulses.pop_front() {
            None => { 
                button_presses += 1;
                pulses.push_back(PulseReturn {from:"button".to_string(), to: "broadcaster".to_string(), ptype: PulseType::Low});
                for ((mname, inputname),(old_type, v, c)) in inputs_to_follow.iter_mut() {
                    let module = modules.get(mname).unwrap();
                    let mem = module.memory.get(inputname).unwrap();
                    if (old_type == &PulseType::Low) & (mem == &PulseType::High) {
                        v.push(button_presses);
                    }
                    *old_type = mem.clone();
                    if (v.len() > 2048) & (c == &-1) {
                        for w in 1..=2048 {
                            let tmp_arr = &v.windows(w+1).map(|x| {x[w] - x[0]}).collect::<Vec<_>>();
                            if tmp_arr.iter().max() == tmp_arr.iter().min() {
                                *c = v[w] - v[0];
                                println!("Loop size: {}, in {} parts", c, w);
                                break
                            }
                        }
                        if c == &-1 {
                            dbg!(&v.windows(2).map(|x| {x[1] - x[0]}).collect::<Vec<_>>());
                            panic!("No loop found!");
                        }
                    }
                }

                if inputs_to_follow.values().all(|(_, _, c)| 
                    {c > &0}) {
                    let counts: Vec<i64> = inputs_to_follow.values().map(|(_,_,c)| c.to_owned()).collect::<Vec<_>>();
                    return lcm(counts);
                }
                continue;
            },
            Some(p) => p,
        };

        if pulse.to == "output" {
            continue;
        }
        if pulse.to == "rx" {
            continue;
        }
        for r in modules.get_mut(&pulse.to).expect("Target not found").receive(pulse.from, pulse.ptype) {
            pulses.push_back(r);
        }
    }
}

fn is_all_same3(arr: &[usize]) -> bool {
    arr.windows(2).all(|w| w[0] == w[1])
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    mtype: ModuleType,
    state: ModuleState,
    targets: Vec<String>,
    inputs: Vec<String>,
    memory: BTreeMap<String, PulseType>
}

impl fmt::Display for ModuleState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModuleState::On => write!(f, "On"),
            ModuleState::Off => write!(f, "OFf"),
            ModuleState::NA => write!(f, "NA"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PulseType {
    Low,
    High
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{} High: {}/{}",self.mtype, self.name, self.memory.iter().filter(|(_k,v)| {
            v == &&PulseType::High
        }).count(), self.memory.len())
    }
}

impl fmt::Display for PulseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PulseType::Low => write!(f, "Low"),
            PulseType::High => write!(f, "High"),
        }
    }
}

impl fmt::Display for ModuleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModuleType::FlipFlop => write!(f, "%"),
            ModuleType::Conjunction => write!(f, "&"),
            ModuleType::Broadcaster => write!(f, "broad"),
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcaster, // Only One
    FlipFlop, // %
    Conjunction, // &
}

#[derive(Debug)]
struct PulseReturn {
    from: String,
    to: String,
    ptype: PulseType,
}

impl Module{
    fn new(input: &str) -> Module {
        let re = Regex::new(r"(&|%|broadcaster)([a-z]*) -> ([a-z, ]*)").unwrap();
        let Some(res) = re.captures(input) else { panic!("Could not parse input");};
        let mtype = match &res[1] {
            "broadcaster" => ModuleType::Broadcaster,
            "&" => ModuleType::Conjunction,
            "%" => ModuleType::FlipFlop,
            _ => panic!("Something happened"),
        };
        let mstate = match mtype {
            ModuleType::FlipFlop => ModuleState::Off,
            _ => ModuleState::NA,
        };
        let name = match mtype {
            ModuleType::Broadcaster => "broadcaster",
            _ => &res[2],
        };
        let targets: Vec<String> = res[3].split(",").map(|c| {
            c.trim().to_string()
        }).collect();

        Module {
            name: name.to_string(),
            mtype: mtype,
            state: mstate,
            targets: targets,
            memory: BTreeMap::new(),
            inputs: Vec::new()
        }
    }

    fn add_input(&mut self, input: String) {
        self.inputs.push(input.clone());
        match &self.mtype {
            ModuleType::Conjunction => {
                self.memory.insert(input, PulseType::Low);
            },
            _ =>(),
        }
    }

    fn receive(&mut self, from: String, pulse_type: PulseType) -> Vec<PulseReturn> {
        match (&self.mtype, &self.state, pulse_type) {
            (ModuleType::Broadcaster, _, _) => {
                return self.targets.iter().map(|m| {
                    PulseReturn { from: self.name.clone(), to: m.to_string(), ptype: pulse_type}
                }).collect();
            }
            // Flip-flop modules (prefix %) are either on or off; they are initially off.
            // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
            (ModuleType::FlipFlop, _, PulseType::High) => {
                return Vec::new();
            }
            // However, if a flip-flop module receives a low pulse, it flips between on and off.
            // If it was off, it turns on and sends a high pulse.
            (ModuleType::FlipFlop, ModuleState::Off, PulseType::Low) => {
                self.state = ModuleState::On;
                return self.targets.iter().map(|m| {
                    PulseReturn { from: self.name.clone(), to: m.clone(), ptype: PulseType::High}
                }).collect();
            }
            // If it was on, it turns off and sends a low pulse.
            (ModuleType::FlipFlop, ModuleState::On, PulseType::Low) => {
                self.state = ModuleState::Off;
                return self.targets.iter().map(|m| {
                    PulseReturn { from: self.name.clone(), to: m.clone(), ptype: PulseType::Low}
                }).collect();
            }

            // Conjunction modules remember the type of the
            // most recent pulse received from each of their connected input modules;
            (ModuleType::Conjunction, _, pulse_type) => {
                // When a pulse is received, the conjunction module first updates its memory for that input. Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                    self.memory.insert(from, pulse_type);
                let return_type = if self.memory.values().all(|p| {
                    p == &PulseType::High
                }) {
                    PulseType::Low 
                } else {
                    PulseType::High 
                };
                return self.targets.iter().map(|m| {
                    PulseReturn { from: self.name.clone(), to: m.clone(), ptype: return_type}
                }).collect();
            }
            _ => panic!("Not possible"),
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleState {
    On,
    Off,
    NA,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contents(){
        let a = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let b = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(part1(&a, 1), 32);
        assert_eq!(part1(&a, 1000), 32_000_000);
        assert_eq!(part1(&b, 1000), 11_687_500);
    }
}
