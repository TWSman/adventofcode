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

fn read_contents(cont: &str) -> (i64, i64) {
    let operations = read_op(cont);
    let part1 = get_part1(&operations);
    let part2 = get_part2(&operations);
    (part1, part2)
}

#[derive(Debug)]
struct Op {
    mask: Mask,
    operations: Vec<(u64, u64)>,
}

#[derive(Debug, Clone)]
struct Mask {
    mask: u64,     // Which bits are affected
    map: u64,      // The actual mask
    floating: u64, // The inverse of mask
    floating_bits: u32,
}

impl Mask {
    fn new(ln: &str) -> Self {
        let mask = u64::from_str_radix(
            &ln.chars()
                .map(|c| match c {
                    'X' => '0',
                    _ => '1',
                })
                .collect::<String>(),
            2,
        )
        .unwrap();
        let map = u64::from_str_radix(
            &ln.chars()
                .map(|c| match c {
                    '1' => '1',
                    _ => '0',
                })
                .collect::<String>(),
            2,
        )
        .unwrap();
        let floating_bits = ln.chars().filter(|c| *c == 'X').count();
        Self {
            mask,
            map,
            floating: !mask % 2_u64.pow(36),
            floating_bits: floating_bits as u32,
        }
    }

    fn apply(&self, a: u64) -> u64 {
        let keep = !self.mask & a;
        keep + self.map
    }

    fn apply2(&self, a: u64) -> Vec<u64> {
        let tmp = a | self.map;
        let keep = self.mask & tmp;
        let opts = 2_u32.pow(self.floating_bits);
        let mut output = Vec::new();
        for opt in 0..opts {
            let mut o = opt;
            let mut c;
            let mut str = format!("{:b}", self.floating).replace("1", "X");
            for _ in 0..self.floating_bits {
                (c, o) = (o % 2, o / 2);
                if c == 1 {
                    str = str.replacen("X", "1", 1);
                } else {
                    str = str.replacen("X", "0", 1);
                }
            }
            let floats = u64::from_str_radix(&str, 2).unwrap();
            output.push(keep + floats);
        }
        output
    }
}

fn read_op(cont: &str) -> Vec<Op> {
    let mut op = Vec::new();
    let mut mask: Option<Mask> = None;
    let mut mems = Vec::new();
    for line in cont.lines() {
        if line.starts_with("mask =") {
            if let Some(m) = mask {
                op.push(Op {
                    mask: m,
                    operations: mems,
                });
                mems = Vec::new();
            }
            let (_, b) = line.split_once(" = ").unwrap();
            mask = Some(Mask::new(b));
            continue;
        }
        if line.starts_with("mem") {
            let (a, b) = line.split_once(" = ").unwrap();
            let a = a
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let b = b.parse::<u64>().unwrap();
            mems.push((a, b));
            continue;
        }
        panic!()
    }
    if let Some(m) = mask {
        op.push(Op {
            mask: m,
            operations: mems,
        });
    }
    op
}

fn get_part1(operations: &[Op]) -> i64 {
    let mut memory: BTreeMap<u64, u64> = BTreeMap::new();
    for op in operations {
        let mask = &op.mask;
        for (mem, val) in &op.operations {
            let a = mask.apply(*val);
            memory.insert(*mem, a);
        }
    }
    memory.values().map(|c| *c as i64).sum()
}

fn get_part2(operations: &[Op]) -> i64 {
    let mut memory: BTreeMap<u64, u64> = BTreeMap::new();
    for op in operations {
        let mask = &op.mask;
        for (mem, val) in &op.operations {
            for m in mask.apply2(*mem) {
                memory.insert(m, *val);
            }
        }
    }
    memory.values().map(|c| *c as i64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask() {
        let mask = Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
        assert_eq!(mask.apply(0), 64);

        let mask = Mask::new("000000000000000000000000000000X1001X");
        assert_eq!(mask.apply2(42), vec![26, 58, 27, 59]);
    }

    #[test]
    fn part1() {
        let a = "mask = 00000000000000000000XXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(read_contents(&a).0, 165);
    }

    #[test]
    fn part2() {
        let a = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(read_contents(&a).1, 208);
    }
}
