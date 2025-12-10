use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::fmt::Display;
use core::fmt;
use regex::Regex;
use priority_queue::PriorityQueue;
extern crate rayon;

use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

// Found
/*(0,  138),
(1,  38),
(2,  52),
(3,  64),
(4,  62),
(11,  81),
(40,  57),
(41,  193),
(42,  37),
(46,  26),
(69,  33),
(93,  80),
(98,  142),
(116,  11),
(128,  147),
*/



fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}


#[derive(Debug, Clone, Copy)]
struct Button {
    changes: i32, // 
    change_sum: usize
}

impl Button {
    fn from_str(ln: &str) -> Self {
        let mut a = 0;
        let mut c = 0;
        for b in ln.split(",") {
            c += 1;
            a += 2_i32.pow(b.parse::<u32>().unwrap())
        }
        Self {changes: a, change_sum: c}
    }
    fn get_change_map(&self, n:usize) -> Vec<i32> {
        let mut vec = Vec::new();

        let mut div = self.changes;
        let mut m;
        for _i in 0..n {
            (div, m) = (div / 2, div % 2);
            if m == 1 {
                vec.push(1);
            } else {
                vec.push(0);
            }
        }
        vec

    }

    fn get_expected_count(&self, current: &Vec<i32>, target: &Vec<i32>) -> i32 {
        self.get_change_vector().iter().map(|i|{
            target[*i] - current[*i]
        }).min().unwrap_or(0)
    }

    fn get_change_vector(&self) -> Vec<usize> {
        let mut vec = Vec::new();

        let mut i = 0;

        let mut div = self.changes;
        let mut m;
        while div > 0 {
            (div, m) = (div / 2, div % 2);
            if m == 1 {
                vec.push(i);
            }
            i += 1;
        }
        vec
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}", self.changes)
    }
}


#[derive(Debug)]
struct Machine {
    n_lights: usize,
    target_lights: i32, // Target as binary mask
    buttons: Vec<Button>,
    joltages: Vec<i32>,
}

#[derive(Debug, PartialEq)]
enum Comparison {
    Exact,
    Smaller,
    Larger,
}

fn add_to_vector(vec: &Vec<i32>, ind: usize, val: i32) -> Vec<i32> {
    vec.iter().enumerate().map(|(i,v)| {
        if i == ind {
            v + val
        } else {
            *v
        }
    }).collect()
}

fn add_vectors(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
    vec1.iter().enumerate().map(|(i,v)| {
        v + vec2[i]
    }).collect()
}

fn get_hash(vec: &Vec<i32>) -> u64 {
    let mut hash: u64 = 0;
    let primes: Vec<u64> = vec![2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97];
    for (i,v) in vec.iter().enumerate() {
        hash += primes[i].pow(*v as u32);
    }
    hash
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = format!("Machine with {} lights, target {:08b}, joltages {:?}, and {} buttons:", self.n_lights, self.target_lights, self.joltages, self.buttons.len());
        for b in self.buttons.iter() {
            out += format!("\n {}", b).as_str();
        }
        write!(f, "{}", out)
    }
}

impl Machine {
    fn from_str(ln: &str) -> Self {
        let re1: regex::Regex = Regex::new("([\\(\\[\\{])([0-9.#,]*)[\\)\\},\\]]").unwrap();
        let mut target_lights: Option<i32> = None;
        let mut buttons: Vec<Button> = Vec::new();
        let mut joltages: Option<Vec<i32>> = None;
        let mut n_lights = 0; 
        dbg!(ln);
        for (_, [start, group]) in re1.captures_iter(ln).map(|c| c.extract()) {
            //dbg!(&start);
            //dbg!(&group);
            match start {
                "(" => {
                    buttons.push(Button::from_str(group));
                },
                "[" => {
                    n_lights = group.len();
                    target_lights = Some(group.chars().enumerate().map(|(i,v)| {
                        if v == '#' {
                            2_i32.pow(i as u32)
                        } else {
                            0
                        }
                    }).sum())
                }
                "{" => {
                    joltages = Some(group.split(",").map(|b| {
                        b.parse::<i32>().unwrap()
                        }).collect()
                    );
                }
                _ => {
                    continue;
                }
            }
        }
        //dbg!(&target_lights);
        //dbg!(&buttons);
        //dbg!(&joltages);
        Self {n_lights: n_lights, target_lights: target_lights.unwrap(), buttons: buttons, joltages: joltages.unwrap()}
    }

    fn get_part1(&self) -> i32 {
        let n = 2_i32.pow(self.buttons.len() as u32);
        // outer loop tests different options
        // println!("Testing machine with target {:b} and {} buttons", self.target_lights, self.buttons.len());
        (0..n).map(|i_opt| {
            //println!("Testing option {} {:b}", i_opt,i_opt);
            let mut s = 0;
            let mut c = 0;
            for i_button in 0..(self.buttons.len()){
                // Check if bit at index 'i_button' is set
                if ((i_opt >> i_button) & 1) == 1 {
                    //println!("  Pressing button {}", i_button);
                    c += 1;
                    s ^= self.buttons[i_button].changes;
                }
                else {
                    //println!("  Do not p button {}", i_button);
                }
            }
            if s == self.target_lights {
                //println!("Found match with option {:b}", i);
                c
            } else {
                999
            }
        }).min().unwrap()
    }

    fn button_change_sum(&self) -> i32 {
        self.buttons.iter().map(|b| b.change_sum as i32).sum()
    }

    fn get_part2_tree_smart(&self) -> i32 {
        // Make a first guess of the number of button presses needed
        // Then add or subtract presses depending on situation
        //println!("Smart Processing machine with {} buttons and {} lights", self.buttons.len(), self.n_lights);
        let target_sum = self.joltages.iter().sum::<i32>();
        let change_sum = self.button_change_sum();
        let estimated_count = target_sum / change_sum;
        let n_buttons = self.buttons.len();
        //dbg!(&target_sum);
        //dbg!(&change_sum);
        //dbg!(&estimated_count);
        let mut heads = PriorityQueue::new();

        let mut checked: BTreeSet<Vec<i32>> = BTreeSet::new();

        let mut prev_button_count = 999;

        heads.push(vec![estimated_count; n_buttons as usize], -estimated_count * (n_buttons as i32));
        heads.push(vec![1+estimated_count; n_buttons as usize], -(1+estimated_count) * (n_buttons as i32));
        loop {
            if heads.len() == 0 {
                continue;
            }
            let (button_activation, _prio) = heads.pop().unwrap();
            if checked.contains(&button_activation) {
                continue;
            }
            checked.insert(button_activation.clone());
            let old_len = button_activation.iter().sum::<i32>();

            if old_len < prev_button_count {
                println!("  Testing heads with total presses {}", old_len);
                prev_button_count = old_len;
            }

            //println!("Testing head with button activations {:?} (total presses {})", button_activation, old_len);
            let sum = self.get_sum(&button_activation);
            //dbg!(&sum);
            match self.compare_sum(&sum) {
                Comparison::Exact => {
                    let a = button_activation.iter().sum::<i32>();
                    //println!("Found solution with {a} presses");
                    return a;
                },
                Comparison::Larger => {
                    //println!("  Sum too large, reducing presses");
                    for i_button in 0..n_buttons {
                        if button_activation[i_button] == 0 {
                            continue;
                        }
                        let new_head = add_to_vector(&button_activation, i_button, -1);
                        assert_eq!(new_head.iter().sum::<i32>(), old_len - 1);
                        heads.push(new_head, -(old_len - 1));
                    }
                }
                Comparison::Smaller => {
                    //println!("  Sum too small, increasing presses");
                    for button in 0..n_buttons {
                        let count = self.buttons[button].get_expected_count(&sum, &self.joltages);
                        if count <= 0 {
                            continue;
                        }
                        //let count = self.buttons[button].get_expected_count(&button_activation, &self.joltages);
                        //dbg!(&count);
                        let new_head = add_to_vector(&button_activation, button, count);
                        assert_eq!(new_head.iter().sum::<i32>(), old_len + count);
                        heads.push(new_head, -(old_len + count));
                    }
                }
            }
        }
    }

    fn get_part2_tree(&self) -> i32 {
        //println!("Processing machine with {} buttons and {} lights", self.buttons.len(), self.n_lights);
        let mut fixed_indices: BTreeMap<usize, i32> = BTreeMap::new();
        let single_values = self.get_affected();
        //dbg!(&single_values);
        for (i_light,v) in self.get_affected().iter().enumerate() {
            if v.len() == 1 {
                let i_button = v[0];
                let target_value = self.joltages[i_light];
                //dbg!(&target_value);
                //println!("Index {i_light} is only affected by a single button {i_button}");
                //println!("This button should be pressed {target_value} times");
                fixed_indices.insert(i_button, target_value);
            }
        }

        let n_buttons = self.buttons.len();
        let _n_lights = self.n_lights;
        let mut heads = PriorityQueue::new();
        let starting_head: Vec<i32> = (0..n_buttons).map(|i| {
            match fixed_indices.get(&i) {
                Some(v) => *v,
                None => 0,
            }
        }).collect();
        let press_count = starting_head.iter().sum::<i32>();
        //println!("Starting tree search with {} buttons", n_buttons);
        //dbg!(&starting_head);
        heads.push(starting_head, -press_count);
        let mut prev_button_count = press_count;
        let max_button_presses = self.joltages.iter().sum::<i32>();
        loop {
            if heads.len() == 0 {
                continue;
            }
            let (button_activation, _prio) = heads.pop().unwrap();
            let old_len = button_activation.iter().sum::<i32>();
            if old_len > prev_button_count {
                //println!("  Testing heads with total presses {}, expected max: {}", old_len, max_button_presses);
                prev_button_count = old_len;
            }
            //println!("{}", heads.len());
            //println!("Testing head with button activations {:?} (total presses {})", button_activation, old_len);

            let sum = self.get_sum(&button_activation);
            match self.compare_sum(&sum) {
                Comparison::Exact => {
                    let a = button_activation.iter().sum::<i32>();
                    println!("Found solution with {a} presses");
                    return a;
                },
                Comparison::Larger => {
                    continue
                }
                Comparison::Smaller => {
                    for i_button in 0..n_buttons {
                        if fixed_indices.contains_key(&i_button) {
                            continue;
                        }
                        //let count = self.buttons[i_button].get_expected_count(&sum, &self.joltages);
                        let new_head = add_to_vector(&button_activation, i_button, 1);
                        assert_eq!(new_head.iter().sum::<i32>(), old_len + 1);
                        heads.push(new_head, -(old_len + 1));
                    }
                }
            }
        }
    }

    fn get_part2_tree_alt(&self) -> i32 {
        // Alternative version that keeps track of the joltage activation
        // instead of button activations
        println!("Processing machine with {} buttons and {} lights", self.buttons.len(), self.n_lights);
        let n_buttons = self.buttons.len();
        let n_lights = self.n_lights;
        let mut heads = PriorityQueue::new();
        heads.push((vec![0; n_lights],0), 0);
        println!("Starting tree search with {} buttons", n_buttons);
        loop {
            if heads.len() == 0 {
                continue;
            }
            let ((head,button_count), prio) = heads.pop().unwrap();
            //dbg!(&prio);
            //println!("{}", heads.len());
            //println!("Testing head with button activations {:?} (total presses {})", button_activation, old_len);

            for button in self.buttons.iter() {
                let new_head = add_vectors(&head, &button.get_change_map(self.n_lights));
                for i in 0..self.n_lights {
                    if new_head[i] > self.joltages[i] {
                        //println!("  Skipping new head {:?} as it exceeds target at index {}", new_head, i);
                        continue;
                    }
                }
                if new_head == self.joltages {
                    println!("Found exact match with button activations {:?} (total presses {})", new_head, button_count + 1);
                    return button_count + 1;
                }
                //assert_eq!(new_head.iter().sum::<i32>(), old_len + 1);
                heads.push((new_head, button_count + 1), prio - 1);
            }
        }
    }

    fn get_sum(&self, button_activations: &Vec<i32>) -> Vec<i32> {
        let mut output: Vec<i32> = vec![0; self.n_lights];
        for (i, button) in self.buttons.iter().enumerate() {
            for v in button.get_change_vector() {
                output[v] += button_activations[i];
            }
        }
        output
    }

    fn get_diff_sum(&self, x: &Vec<i32>) -> i32 {
        if &self.joltages == x {
            return 0;
        }
        (0..self.n_lights).map(|i| {
            &self.joltages[i] - x[i]
        }).sum()
    }

    fn compare_sum(&self, x: &Vec<i32>) -> Comparison {
        if &self.joltages == x {
            return Comparison::Exact
        }
        for i in 0..self.n_lights {
            if self.joltages[i] < x[i] {
                //println!("At index {}: target {} < sum {}", i, self.joltages[i], x[i]);
                return Comparison::Larger
            }
        }
        Comparison::Smaller
    }

    fn get_maximum_presses(&self) -> Vec<i64> {
         self.buttons.iter().enumerate().map(|(_i,b)| {
            b.get_change_vector().iter().map(|v| {
                self.joltages[*v as usize]
            }).min().unwrap() as i64
        }).collect::<Vec<_>>()
    }

    fn get_affected(&self) -> Vec<Vec<usize>> {
        // Check how many buttons affect each light
        (0..self.n_lights).map(|v| {
            self.buttons.iter().enumerate().filter_map(|(i,b)| {
                if  ((b.changes >> v) & 1) == 1 {
                    Some(i) 
                } else {
                    None
                }
            }).collect::<Vec<usize>>()
        }).collect::<Vec<_>>()
    }
}


fn read_contents(cont: &str) -> (i64, i64) {
    let machines: Vec<Machine> = cont.lines().map(|ln| {
        Machine::from_str(ln)
    }).collect();
    dbg!(&machines);
    let part1 = machines.iter().map(|m| m.get_part1() as i64).sum();
    //let part2 = machines.iter().enumerate().map(|(i,m)| {
        //println!("Processing machine {} / {}", i, machines.len() );
        //println!("{}", m);
        //m.get_part2_tree() as i64
    //}).sum();

    let part2 = machines.iter().enumerate().collect::<Vec<_>>().par_iter().map(|(i,m)| {
        //println!("Processing machine {} / {}", i, machines.len() );
        //println!("{}", m);
        let res = m.get_part2_tree() as i64;
        println!("index {i}: part2: {res}");
        res
    }).sum();

    //cont.lines().enumerate().collect::<Vec<_>>().par_iter().map(|(_i, l)| {
        //read_line(l, repeat)
    //}).sum()
    (part1, part2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {

        let m = Machine::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(m.get_part1(), 2);

        let a="[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(read_contents(&a).0, 7);
    }
    #[test]
    fn button() {
        let b = Button::from_str("1,3,4");
        let res = b.get_change_vector();
        assert_eq!(res, vec![1,3,4]);

        assert_eq!(b.get_expected_count(&vec![0,0,0,0,0], &vec![2,1,0,3,4]), 1);

        let b = Button::from_str("0,1,2,3");
        assert_eq!(b.get_expected_count(&vec![0,0,0,0,0], &vec![5,5,5,3,4]), 3);
    }


    #[test]
    fn machine() {
        let m = Machine::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        //dbg!(&m);
        assert_eq!(m.n_lights, 4);
        assert_eq!(m.target_lights, 6);
        assert_eq!(m.buttons.len(), 6);
        assert_eq!(m.joltages, vec![3,5,4,7]);
        assert_eq!(m.get_sum(&vec![2,0,0,0,0,0]), vec![0,0,0,2]);
        assert_eq!(m.compare_sum(&vec![3,5,4,7]), Comparison::Exact);
        assert_eq!(m.compare_sum(&vec![3,4,5,7]), Comparison::Larger);
        assert_eq!(m.compare_sum(&vec![3,4,3,7]), Comparison::Smaller);

        assert_eq!(m.get_part2_tree(), 10);
        //assert_eq!(m.get_part2_tree_smart(), 10);
        assert_eq!(m.get_part2_tree_alt(), 10);
    }

    #[test]
    fn part2() {

        let m = Machine::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");

        dbg!(&m);
        assert_eq!(m.get_part2_tree(), 10);
        assert_eq!(m.get_part2_tree_alt(), 10);

        let m = Machine::from_str("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(m.get_part2_tree(), 12);
        assert_eq!(m.get_part2_tree_alt(), 12);

        let m = Machine::from_str("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(m.get_part2_tree(), 11);
        assert_eq!(m.get_part2_tree_alt(), 11);

        let m2 = Machine::from_str("l[#...##] (0,1,3,4,5) (0,4,5) (1,2,3,4) (0,1,2) {132,30,23,13,121,115}");
        assert_eq!(m2.get_part2_tree(), 138);

        let a="[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(read_contents(&a).1, 33);
    }

}

