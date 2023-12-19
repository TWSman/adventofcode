use clap::Parser;
use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}


fn main() {
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

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Target::Accept => write!(f, "Accept"),
            Target::Reject => write!(f, "Reject"),
            Target::Goto(s) => write!(f, "{}", format!("Goto {s}")),
        }
    }
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

#[derive(Clone,Debug)]
struct ValRange {
    min: i64, // Inclusive
    max: i64, // Inclusive
}

impl ValRange {
    // Split val will be included in lower range
    fn split(&self, val: i64) -> (ValRange, ValRange) {
        (ValRange{min: self.min, max: val }, ValRange{ min: val + 1, max: self.max })
    }

    fn len(&self) -> i64 {
        self.max - self.min + 1
    }
}

#[derive(Clone,Debug)]
struct PossibilitySpace {
    cool_factor: ValRange,
    musicality: ValRange,
    aero: ValRange,
    shininess: ValRange,
}

impl PossibilitySpace {
    fn new(cool_factor: ValRange, musicality: ValRange, aero: ValRange, shininess: ValRange) -> PossibilitySpace {
        PossibilitySpace {
            cool_factor: cool_factor,
            musicality: musicality,
            aero: aero,
            shininess: shininess,
        }
    }

    fn size(&self) -> i64 {
        self.cool_factor.len() * self.musicality.len() * self.aero.len() * self.shininess.len()
    }

    // Split will be included in lower option
    fn split(&self, factor: Factor, split: i64) -> (Option<PossibilitySpace>, Option<PossibilitySpace>) {
        let mut first_split = self.clone();
        let mut second_split = self.clone();
        match factor {
            Factor::CoolFactor => {
                let (a,b) = self.cool_factor.split(split);
                first_split.cool_factor = a;
                if b.len() <= 0 {
                    return (Some(first_split), None)
                }
                second_split.cool_factor = b;
            },
            Factor::Musicality => {
                let (a,b) = self.musicality.split(split);
                first_split.musicality = a;
                if b.len() <= 0 {
                    return (Some(first_split), None)
                }
                second_split.musicality = b;
            },
            Factor::Aero => {
                let (a,b) = self.aero.split(split);
                first_split.aero = a;
                if b.len() <= 0 {
                    return (Some(first_split), None)
                }
                second_split.aero = b;
            },
            Factor::Shininess => {
                let (a,b) = self.shininess.split(split);
                first_split.shininess = a;
                if b.len() <= 0 {
                    return (Some(first_split), None)
                }
                second_split.shininess = b;
            },
        }
        (Some(first_split), Some(second_split))
    }
}

impl WorkFlow {
    fn new(input: &str) -> WorkFlow {
        let re = Regex::new(r"(\w*)\{(.*)\}").unwrap();
        let Some(res) = re.captures(input) else { panic!("Could not parse input");};
        let name = res[1].to_string();
        let rules: Vec<Rule> = res[2].split(",").map(|s| {Rule::new(s)}).collect();
        WorkFlow {name: name, rules: rules}
    }

    fn part2(&self, start_space: PossibilitySpace, workflows: &HashMap<String, WorkFlow>) -> i64 {
        let mut current_space = start_space.clone();
        let mut rules_iter = self.rules.iter();
        let mut rule = rules_iter.next().unwrap();
        let mut sum = 0;
        loop {
            match rule.factor {
                None => {
                    match &rule.target {
                        Target::Accept => { sum += current_space.size(); break; },
                        Target::Reject => { break; }
                        Target::Goto(target) => {
                            let wf = workflows.get(target).unwrap();
                            sum += wf.part2(current_space, workflows);
                            break;
                        }
                    }
                },
                Some(factor) => {
                    match rule.min_val {
                        None => (),
                        Some(val) =>  {
                            // We have a minimum value. This value should be in the lower
                            // split
                            // i.e. val should in first space
                            let (space1, space2) = current_space.split(factor, val);
                            // Passing space, i.e. second space should go to target
                            // First space continues to next rule
                            match space2 {
                                None => (),
                                Some(space) => {
                                    match &rule.target {
                                        Target::Goto(target) => {
                                            let wf = workflows.get(target).unwrap();
                                            sum += wf.part2(space, workflows);
                                        },
                                        Target::Accept => { sum += space.size(); }
                                        Target::Reject => (),
                                    };
                                }
                            }
                            current_space = space1.unwrap();
                            rule = rules_iter.next().unwrap();
                            continue;
                        }
                    };
                    match rule.max_val {
                        None => (),
                        Some(val) =>  {
                            // We have a maximum value. This value should be in the upper
                            // split
                            // i.e. val -1 should in first space
                            let (space1, space2) = current_space.split(factor, val - 1);
                            // Passing space, i.e. first space should go to target
                            // Second space continues to next rule
                            match space1 {
                                None => (),
                                Some(space) => {
                                    match &rule.target {
                                        Target::Goto(target) => {
                                            let wf = workflows.get(target).unwrap();
                                            sum += wf.part2(space, workflows);
                                        },
                                        Target::Accept => { sum += space.size(); },
                                        Target::Reject => (),
                                    };
                                }
                            }
                            current_space = space2.unwrap();
                            rule = rules_iter.next().unwrap();
                        }
                    };
                }
            }
        }
        sum
    }

    fn part1(&self, component: &Component) -> CheckResult {
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
        match workflow.part1(component) {
            CheckResult::Score(score) => {return score;},
            CheckResult::Target(target) => {
                workflow = workflows.get(&target).unwrap()},
        }
    }
}

fn part2(workflows: &HashMap<String, WorkFlow>) -> i64 {
    let default_range = ValRange {min: 1, max: 4000};
    let startspace = PossibilitySpace::new(default_range.clone(), default_range.clone(), default_range.clone(), default_range.clone());
    let workflow = workflows.get("in").unwrap();
    workflow.part2(startspace, workflows)
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
