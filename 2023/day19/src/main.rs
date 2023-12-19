use clap::Parser;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

use rand::prelude::*;

extern crate rayon;

use rayon::prelude::*;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}


fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(8).build_global().unwrap();
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
}


#[derive(Debug)]
struct Component {
    cool_factor: i64, //x
    musicality: i64, //m
    aero: i64, // a
    shininess: i64, // s
}

impl Component {
    fn new(input: &str) -> Component {
        let re = Regex::new(r"\{x=([0-9]*),m=([0-9]*),a=([0-9]*),s=([0-9]*)\}").unwrap();
        let Some(res) = re.captures(input) else { panic!("Could not parse input");};
        Component {
            cool_factor: res[1].parse::<i64>().unwrap(),
            musicality: res[2].parse::<i64>().unwrap(),
            aero: res[3].parse::<i64>().unwrap(),
            shininess: res[4].parse::<i64>().unwrap(),
        }

    }

    fn score(&self) -> i64 {
        self.cool_factor + self.musicality + self.aero + self.shininess
    }
}

#[derive(Clone,Debug, Copy, PartialEq, Eq, Hash)]
enum Factor {
    CoolFactor,
    Musicality,
    Aero,
    Shininess,
}

#[derive(Clone,Debug)]
struct Rule {
    min_val: Option<i64>,
    max_val: Option<i64>,
    factor: Option<Factor>,
    target: Target,
}

impl Rule {
    fn new(input: &str) -> Rule {
        let re = Regex::new("([xmas])([<>])([0-9]*):([a-zAR]*)").unwrap();
        match input {
            "R" => Rule {min_val: None, max_val: None, factor: None, target: Target::Reject},
            "A" => Rule {min_val: None, max_val: None, factor: None, target: Target::Accept},
            v if v.chars().all(char::is_alphanumeric) => {
                Rule {min_val: None, max_val: None, factor: None, target: Target::Goto(v.to_string())}
            },
            v => {
                let Some(res) = re.captures(v) else { panic!("Could not parse input");};
                let factor = match &res[1] {
                    "m" => Factor::Musicality,
                    "x" => Factor::CoolFactor,
                    "a" => Factor::Aero,
                    "s" => Factor::Shininess,
                    v => panic!("Unknown factor {}", v),
                };
                let val = res[3].parse::<i64>().unwrap();
                let (min_val, max_val) = match &res[2] {
                    ">" => (Some(val), None),
                    "<" => (None, Some(val)),
                    _ => panic!("Expected < or >"),
                };
                let target = match &res[4] {
                    "R" => Target::Reject,
                    "A" => Target::Accept,
                    v => Target::Goto(v.to_string()),
                };

                Rule {min_val: min_val, max_val: max_val, factor: Some(factor), target: target}
            }
        }
    }

    fn get_val(&self) -> Option<i64> {
        match (self.min_val, self.max_val) {
            (Some(v), None) => Some(v),
            (None, Some(v)) => Some(v),
            _ => None,
        }
    }

    fn check(&self, component: &Component) -> Option<&Target> {
        let val = match self.factor {
            None => {
                return Some(&self.target);
            },
            Some(Factor::CoolFactor) => component.cool_factor,
            Some(Factor::Musicality) => component.musicality,
            Some(Factor::Aero) => component.aero,
            Some(Factor::Shininess) => component.shininess,
        };

        match (self.min_val, self.max_val) {
            (Some(v), None) => if val > v {
                return Some(&self.target)
            } else {
                return None
            }
            (None, Some(v)) => if val < v {
                return Some(&self.target)
            } else {
                return None
            }
            _ => {panic!("Got something");},
        }
    }
}


#[derive(Clone,Debug)]
enum Target {
    Accept,
    Reject,
    Goto(String),
}

#[derive(Clone,Debug)]
struct WorkFlow {
    name: String,
    rules: Vec<Rule>,
}

enum CheckResult {
    Score(i64),
    Target(String),
}

impl WorkFlow {
    fn new(input: &str) -> WorkFlow {
        let re = Regex::new(r"(\w*)\{(.*)\}").unwrap();
        let Some(res) = re.captures(input) else { panic!("Could not parse input");};
        let name = res[1].to_string();
        let rules: Vec<Rule> = res[2].split(",").map(|s| {Rule::new(s)}).collect();
        WorkFlow {name: name, rules: rules}
    }

    fn run(&self, component: &Component) -> CheckResult {
        let mut rules_iter = self.rules.iter();
        let mut rule = rules_iter.next().unwrap();
        loop {
            match rule.check(&component) {
                None => { // Component did not pass the test
                    rule = rules_iter.next().unwrap() ;
                }, // Passed the test, need to figure out, what next
                Some(t) => {
                    match t {
                        // Component was accepted
                        Target::Accept => {
                            return CheckResult::Score(component.score());
                        },
                        // Component was rejected
                        Target::Reject => {
                            return CheckResult::Score(0);
                        }
                        // 
                        Target::Goto(r) => 
                            {
                                return CheckResult::Target(r.to_owned())
                            }
                    }
                },
            };
        }
    }

    fn get_limits(&self, limits: &mut HashMap<Factor, Vec<i64>>) {
        for rule in &self.rules {
            match rule.factor {
                None => continue,
                Some(factor) => {
                    limits.get_mut(&factor).unwrap().push(rule.get_val().unwrap());
                    //limits.get_mut(&factor).unwrap().push(rule.get_val().unwrap() + 1);
                    //limits.get_mut(&factor).unwrap().push(rule.get_val().unwrap() - 1);
                },
            }
        }
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut components: Vec<Component> = Vec::new();
    let mut workflows: HashMap<String,WorkFlow> = HashMap::new();
    for ln in cont.lines() {
        if ln.starts_with("{") {
            components.push(Component::new(ln));
        }
        else if ln.len() > 0 {
            let w = WorkFlow::new(ln);
            workflows.insert(w.name.clone(), w);
        }
    }
    
    let res1 = components.iter().map(|c| {
        get_score(&c, &workflows)
    }).sum();
    let res2 = part2(&workflows);
    (res1, res2)
}

fn get_score(component: &Component, workflows: &HashMap<String, WorkFlow>) -> i64{
    let mut workflow = workflows.get("in").unwrap();
    loop {
        match workflow.run(component) {
            CheckResult::Score(score) => {return score;},
            CheckResult::Target(target) => {
                workflow = workflows.get(&target).unwrap()},
        }
    }
}

// 131757030400000 is too low
// 131766159360000 is too low
// 131776159360000 is too low
// 131793696640000 is ?
// 131802378240000 is ?
fn part2(workflows: &HashMap<String, WorkFlow>) -> i64 {
    let mut limits: HashMap<Factor, Vec<i64>> = HashMap::new();
    limits.insert(Factor::CoolFactor, vec![1,4000]);
    limits.insert(Factor::Musicality, vec![1,4000]);
    limits.insert(Factor::Shininess, vec![1,4000]);
    limits.insert(Factor::Aero, vec![1,4000]);

    // Limits are the limit values, these are the failing ones
    for wf in workflows.values() {
        wf.get_limits(&mut limits);
    }
    //dbg!(&limits);
    let limit_count: i64 = limits.values().map(|v|{
        v.len() as i64
    }).product();
    dbg!(&limit_count);

    let _min_val = 1;
    let max_val: i64 = 4000;
    let n_combos = max_val.pow(4);
    //let tries = 500_000_000; 
    //let tries = 50_000_000;  // Takes 4 minutes on single thread
    //1e7 per minute per thread
    let tries = 8_000; // 10 minutes on 8 threads

    let successes: i64 = (0..tries).into_par_iter().map(|_| {
        let comp = Component {
            cool_factor: get_random(),
            musicality: get_random(),
            aero: get_random(),
            shininess: get_random(),
        };
        match get_score(&comp, &workflows) {
            i if i > 0 => 1,
            _ => 0
        }
    }).sum();
    let ratio = (successes as f64) / (tries as f64);
    (ratio * (n_combos as f64)).round() as i64
}

fn get_random() -> i64{
    (1 + rand::random::<u64>() % 4000) as i64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contents(){
        let a = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(read_contents(&a).0, 19114);
        assert_eq!(read_contents(&a).1, 167409079868000);
    }
}
