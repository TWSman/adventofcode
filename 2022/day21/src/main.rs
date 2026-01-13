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


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}


fn get_monkeys(cont: &str) -> BTreeMap<String, Monkey> {
    cont.lines().map(|ln| {
        let m = Monkey::new(ln);
        (m.id.clone(), m)
    }).collect()
}

fn read_contents(cont: &str) -> (i64, i64) {
    let monkeys = get_monkeys(cont);

    println!("{} Monkeys", monkeys.len());

    // Answer to part1 is the result of evaluating the "root" monkey
    let part1 = get_value("root", &monkeys);

    let part2 = get_part2(&monkeys);
    (part1, part2)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Calculation {
    Sum,
    Division,
    Subtract,
    Product,
    Value,
    Equality,
    Solve,
}

impl Calculation {
    fn new(op: &str) -> Self {
        match op {
            "+" => Calculation::Sum,
            "-" => Calculation::Subtract,
            "*" => Calculation::Product,
            "/" => Calculation::Division,
            "=" => Calculation::Equality, // Does not appear in input
            "?" => Calculation::Solve, // Does not appear in input
            _ => Calculation::Value,
        }
    }
}



#[derive(Debug, Clone)]
struct Monkey {
    id: String,
    value: Option<i64>,
    calculation: Calculation,
    source_a: String,
    source_b: String,
    target: Option<String>,
}

impl Monkey {
    fn new(ln: &str) -> Self {
        let (a, b) = ln.split_once(": ").unwrap();
        if let Ok(v) = b.parse::<i64>() {
            Monkey {
            id: a.to_string(),
            value: Some(v),
            calculation: Calculation::Value,
            source_a: "".to_string(),
            source_b: "".to_string(),
                target: None,
            }
        } else {
            let (src_a, rest) = b.split_once(" ").unwrap();
            let (op, src_b) = rest.split_once(" ").unwrap();
            Monkey {
            id: a.to_string(),
            value: None,
            calculation: Calculation::new(op),
            source_a: src_a.to_string(),
            source_b: src_b.to_string(),
                target: None,
            }
        }
    }
}


fn get_part2(monkeys: &BTreeMap<String,Monkey>) -> i64 {
    let mut mon: BTreeMap<String, Monkey> = monkeys.clone();
    let keys = mon.keys().cloned().collect::<Vec<String>>();

    // Set up target links
    for key in keys {
        let monkey = mon.get(&key).unwrap().clone();
        if monkey.calculation == Calculation::Value {
            continue;
        }
        let source_a = &monkey.source_a;
        let source_b = &monkey.source_b;
        assert!(mon.get(source_a).unwrap().target.is_none());
        assert!(mon.get(source_b).unwrap().target.is_none());
        mon.get_mut(source_a).unwrap().target = Some(key.clone());

        mon.get_mut(source_b).unwrap().target = Some(key.clone());
    }
     
    // Modify root and humn nodes for part 2
    // root is the equality node
    mon.get_mut("root").unwrap().calculation = Calculation::Equality;
    // humn is the node we want to solve for
    mon.get_mut("humn").unwrap().calculation = Calculation::Solve;
    mon.get_mut("humn").unwrap().value = None;
    get_inverse("humn", "", &mon)
}

fn get_value(monkey_id: &str, monkeys: &BTreeMap<String,Monkey>) -> i64 {
    let monkey = monkeys.get(monkey_id).unwrap(); // Get current monkey
    if let Some(v) = monkey.value {
        v
    } else {
        let val_a = get_value(&monkey.source_a, monkeys);
        let val_b = get_value(&monkey.source_b, monkeys);
        match monkey.calculation {
            Calculation::Sum => val_a + val_b,
            Calculation::Subtract => val_a - val_b,
            Calculation::Product => val_a * val_b,
            Calculation::Division => val_a / val_b,
            Calculation::Equality => if val_a == val_b { 1 } else { 0 },
            // If the calculation is Value, monkey.value should also exist
            Calculation::Value => unreachable!(),
            // Solve nodes should not be reached in the forward evaluation
            Calculation::Solve => panic!("Reached a Solve node in get_value. This should not happen"),
        }
    }
}

fn get_inverse(monkey_id: &str, source_id: &str, monkeys: &BTreeMap<String,Monkey>) -> i64 {
    let monkey = monkeys.get(monkey_id).unwrap(); // Get current monkey
    
    // We shouldn't reach monkeys with given values here
    assert!(monkey.value.is_none());

    if monkey.calculation == Calculation::Solve {
        return get_inverse(monkey.target.as_ref().unwrap(), monkey_id, monkeys)
    }

    let other_source = if source_id == monkey.source_a {&monkey.source_b}  else {&monkey.source_a};
    let other_value = get_value(other_source, monkeys);
    match monkey.calculation {
        Calculation::Equality => {
            // We found the equality node
            // Get value from the other source
            other_value
        }
        Calculation::Sum => {
            let target_value = get_inverse(monkey.target.as_ref().unwrap(), monkey_id, monkeys);
            // result + other_value = target_value => result = target - other
            target_value - other_value
        }
        Calculation::Product => {
            let target_value = get_inverse(monkey.target.as_ref().unwrap(), monkey_id, monkeys);
            // result * other_value = target_value => result = target / other
            target_value / other_value
        }
        Calculation::Subtract => {
            let target_value = get_inverse(monkey.target.as_ref().unwrap(), monkey_id, monkeys);
            if source_id == monkey.source_a {
                // result - other_value = target_value => result = target + other
                target_value + other_value
            } else {
                // other_value - result = target_value => result = other - target
                other_value - target_value
            }
        }
        Calculation::Division => {
            let target_value = get_inverse(monkey.target.as_ref().unwrap(), monkey_id, monkeys);
            if source_id == monkey.source_a {
                // result / other_value = target_value => result = target * other
                target_value * other_value
            } else {
                // other_value / result = target_value => result = other / target
                other_value - target_value
            }
        }
        _ => {
            dbg!(&monkey);
            panic!("Should not reach here");
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a ="root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

        assert_eq!(read_contents(&a).0, 152);
    }
    
    #[test]
    fn part2() {
        let a ="root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        assert_eq!(read_contents(&a).1, 301);
    }
}
