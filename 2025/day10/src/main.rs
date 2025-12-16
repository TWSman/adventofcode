use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Display;
use regex::Regex;
use priority_queue::PriorityQueue;
extern crate rayon;

use ndarray::prelude::*;
use ndarray_linalg::Solve;

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
    fn vector(&self, n:usize) -> Vec<i32> {
        // Get the activations as a vector
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

    fn vector_f64(&self, n:usize) -> Vec<f64> {
        // Get the activations as a vector
        let mut vec = Vec::new();

        let mut div = self.changes;
        let mut m;
        for _i in 0..n {
            (div, m) = (div / 2, div % 2);
            if m == 1 {
                vec.push(1 as f64);
            } else {
                vec.push(0 as f64);
            }
        }
        vec

    }

    fn get_expected_count(&self, current: &Vec<i32>, target: &Vec<i32>) -> i32 {
        self.get_changes().iter().map(|i|{
            target[*i] - current[*i]
        }).min().unwrap_or(0)
    }

    fn get_changes(&self) -> Vec<usize> {
        // Return the list of indices of lights/joltages this button affects
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
    memory: BTreeMap<Vec<i32>, i32>,
}

#[derive(Debug, PartialEq)]
enum Comparison {
    Exact,
    Smaller,
    Larger,
}

fn add_to_vector(vec: &Vec<i32>, ind: usize, val: i32) -> Vec<i32> {
    // Add scalar to vector at given index
    vec.iter().enumerate().map(|(i,v)| {
        if i == ind {
            v + val
        } else {
            *v
        }
    }).collect()
}

fn add_vectors(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
    // Add two vectors
    vec1.iter().enumerate().map(|(i,v)| {
        v + vec2[i]
    }).collect()
}

fn subtract_vectors(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
    // Subtract vector 2 from vector 1
    vec1.iter().enumerate().map(|(i,v)| {
        v - vec2[i]
    }).collect()
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
        for (_, [start, group]) in re1.captures_iter(ln).map(|c| c.extract()) {
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
        Self {n_lights: n_lights,
            target_lights: target_lights.unwrap(),
            buttons: buttons,
            joltages: joltages.unwrap(),
            memory: BTreeMap::new(),
        }
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
        // Get the sum of activations if all buttons were pressed once
        self.buttons.iter().map(|b| b.change_sum as i32).sum()
    }

    fn get_part2_recur(&mut self) -> i32 {
        // Use a recursive solver
        self.solve(self.joltages.clone())
    }

    fn get_part2_linalg(&self) -> i64 {
        let mut min_val = 0;
        if self.buttons.len() < self.n_lights {
            //println!("Underdetermined system - infinite solutions");
        } else if self.buttons.len() == self.n_lights {
            //println!("Determined system - single solution");
            min_val = self.solve_linalg(&self.joltages);
        } else {
            //println!("Overdetermined system - no solution or multiple solutions");
        }

        if min_val == 0 {
            println!("Could not find solution via linear algebra");
        } else {
            println!("Found solution via linear algebra with {} presses", min_val);
        }
        min_val
    }

    fn solve_linalg(&self, target: &Vec<i32>) -> i64 {
        let matrix: Vec<Vec<f64>> = self.buttons.iter().map(|b| b.vector_f64(self.n_lights)).collect();

        //dbg!(&matrix);
        let M: Array2<f64> = Array2::from_shape_vec((self.n_lights, self.buttons.len()),
            matrix.iter().flat_map(|v| v.iter()).cloned().collect()
        ).unwrap().reversed_axes();
        //dbg!(&M);
        //let a: Array2<f64> = array![[3., 2., -1.], [2., -2., 4.], [-2., 1., -2.]];
        //let b: Array1<f64> = array![1., -2., 0.];
        let b = Array1::from_vec(target.iter().map(|v| *v as f64).collect());
        match M.solve_into(b) {
            Ok(x) => {
                return x.sum().round() as i64;
            },
            Err(_) => {
                return 0;
            }
        }
        0
    }

    fn solve(&mut self, target: Vec<i32>) -> i32 {
        //println!("Solving target:  ");
        //dbg!(&target);
        let mut min_val = 9999;
        if self.memory.contains_key(&target) {
            //dbg!(&target);
            //println!("Already checked");
            return *self.memory.get(&target).unwrap();
        }
        for i_button in 0..(self.buttons.len()) {
            let button = self.buttons[i_button];
            let change_map = button.vector(self.n_lights);
            if change_map  == target {
                println!("Found target");
                dbg!(&target);
                println!("With button");
                dbg!(&button);
                // 1 move is enough to reach target
                return 1;
            }
            let new_vec = subtract_vectors(&target, &change_map);
            if new_vec.iter().all(|v| *v >= 0) {
                let res = 1 + self.solve(new_vec);
                if res == 0 {
                    continue;
                }
                if res < min_val {
                    println!("Found new minimum {} with button {}", res, button);
                    min_val = res;
                }
            }
        }
        //println!("Solved target:  ");
        //dbg!(&target);
        //println!("got: {}", min_val);
        if min_val == 9999 {
            // No valid solutions were found
            // Use -1 to denote invalid
            min_val = -1;
        }
        self.memory.insert(target.clone(), min_val);
        min_val
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

    fn get_part2_fixed(&self) -> BTreeMap<usize, i32> {
        // Try to fix some values of the buttons

        //println!("Processing machine with {} buttons and {} lights", self.buttons.len(), self.n_lights);
        let mut fixed_indices: BTreeMap<usize, i32> = BTreeMap::new();
        // Check for each joltage value which buttons determine its state
        let mut joltage_buttons = self.get_affected();
        //dbg!(&joltage_buttons);
        let mut j_loop = 0;
        loop {
            let mut changes: bool = false;
            j_loop += 1;
            // First check if some joltage value is only affected by a single button
            for (i_light, v) in joltage_buttons.iter().enumerate() {
                if v.len() == 1 {
                    let i_button = v.iter().next().unwrap();
                    if fixed_indices.contains_key(&i_button) {
                        continue;
                    }
                    let target_value = self.joltages[i_light];
                    //dbg!(&target_value);
                    //println!("Index {i_light} is only affected by a single button {i_button}");
                    //println!("This button should be pressed {target_value} times");
                    changes = true;
                    fixed_indices.insert(*i_button, target_value);
                }
            }

            // Look for pairs of button activations, where there is only a difference of one
            // activation, i.e. joltage 'a' has all the same button sources as joltage 'b' plus one extra
            // Or vice versa
            // Difference in target joltages a&b then defines to value for this one different
            // button
            for (ia,a) in joltage_buttons.iter().enumerate() {
                for (ib,b) in joltage_buttons.iter().enumerate() {
                    // Skip cases where b is as long or longer than a
                    if b.len() >= a.len() {
                        continue;
                    }
                    // Elements in a but not in b
                    let diffs_a = a.iter().filter_map(|aa| {
                        if !b.contains(aa) {
                            Some(aa)
                        } else {
                            None
                        }
                    }).collect::<Vec<_>>();
                    if diffs_a.len() == 1 {
                        let mut target = self.joltages.clone();
                        // Remove the values of fixed indices which have been already found
                        for (i,v) in &fixed_indices {
                            target[*i] -= v;
                        }
                        let target_a = target[ia];
                        let target_b = target[ib];
                        //println!("Fix index {} to {}", *diffs_a[0], target_a - target_b);
                        //dbg!(&ia);
                        //dbg!(&ib);
                        //dbg!(&a);
                        //dbg!(&b);
                        fixed_indices.insert(*diffs_a[0], target_a - target_b);
                    }
                }
            }
            joltage_buttons = joltage_buttons.clone();
            for fi in fixed_indices.keys() {
                for v in joltage_buttons.iter_mut() {
                    let ind = v.iter().position(|x| x == fi);
                    match ind {
                        Some(indi) => {
                            v.remove(&fi);
                        },
                        _ => {},

                    }
                }
            }
            if !changes {
                break;
            }
        }
        if fixed_indices.len() > 0 {
            let mut new_target = self.joltages.clone();
            for (i,v) in &fixed_indices {
                for j in self.buttons[*i].get_changes() {
                    new_target[j] -= v;
                }
            }
        }
        fixed_indices
    }

    // Set priority to n_presses + abs(diff)
    fn get_part2_tree(&self) -> i32 {
        // Brute force solution to part2
        // Works but is too slow
        //
        // Get buttons whose activation count is fixed
        let fixed_indices = self.get_part2_fixed();
        let n_buttons = self.buttons.len();
        let mut heads = PriorityQueue::new();
        let starting_head: Vec<i32> = (0..n_buttons).map(|i| {
            match fixed_indices.get(&i) {
                Some(v) => *v,
                None => 0,
            }
        }).collect();
        //dbg!(&fixed_indices);
        let mut loop_count = 0;

        // Get number of presses. This is used as priority in the queue
        let press_count = starting_head.iter().sum::<i32>();
        let starting_sum = self.get_sum(&starting_head);
        let diff_sum = self.get_diff_sum(&starting_sum);
        //println!("Starting sum:");
        //dbg!(&starting_sum);
        //println!("Diff sum: {}", diff_sum);
        let prio = press_count + diff_sum;
        heads.push(starting_head, -prio);
        loop {
            if heads.len() == 0 {
                continue;
            }
            let (button_activation, _prio) = heads.pop().unwrap();

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
                        let new_head = add_to_vector(&button_activation, i_button, 1);
                        let new_sum = self.get_sum(&new_head);
                        let diff_sum = self.get_diff_sum(&new_sum);
                        let prio: i32 = diff_sum + new_head.iter().sum::<i32>() + 1;
                        //assert_eq!(new_head.iter().sum::<i32>(), old_len + 1);
                        heads.push(new_head, -prio);
                    }
                }
            }
        }
    }

    fn get_part2_tree_alt(&self) -> i32 {
        // Alternative version that keeps track of the joltage activation
        // instead of button activations
        //println!("Processing machine with {} buttons and {} lights", self.buttons.len(), self.n_lights);
        let fixed_indices = self.get_part2_fixed();
        let n_buttons = self.buttons.len();
        let n_lights = self.n_lights;
        let mut heads = PriorityQueue::new();

        let starting_head: Vec<i32> = (0..n_buttons).map(|i| {
            match fixed_indices.get(&i) {
                Some(v) => *v,
                None => 0,
            }
        }).collect();
        let press_count = starting_head.iter().sum::<i32>();

        println!("Processing machine with {} buttons and {} lights", self.buttons.len(), self.n_lights);
        let starting_lights = self.get_sum(&starting_head);
        dbg!(&fixed_indices);
        dbg!(&starting_lights);
        heads.push((starting_lights, press_count), -press_count);
        println!("Starting tree search with {} buttons", n_buttons);
        loop {
            if heads.len() == 0 {
                continue;
            }
            let ((head,button_count), prio) = heads.pop().unwrap();
            //dbg!(&prio);
            //println!("{}", heads.len());
            println!("Testing head with button activations {:?} (total presses {})", head, button_count);

            for (i,button) in self.buttons.iter().enumerate() {
                if fixed_indices.contains_key(&i) {
                    continue;
                }
                let new_head = add_vectors(&head, &button.vector(self.n_lights));
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
        // Get summed vector of button activations 
        let mut output: Vec<i32> = vec![0; self.n_lights];
        for (i, button) in self.buttons.iter().enumerate() {
            for v in button.get_changes() {
                output[v] += button_activations[i];
            }
        }
        output
    }

    fn get_diff_sum(&self, x: &Vec<i32>) -> i32 {
        // Get sum of differences between target joltages and given vector
        if &self.joltages == x {
            return 0;
        }
        (0..self.n_lights).map(|i| {
            &self.joltages[i] - x[i]
        }).sum()
    }

    fn compare_sum(&self, x: &Vec<i32>) -> Comparison {
        // Compare given vector to the target
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
        // Get the maximum allowed number of presses for each button
         self.buttons.iter().enumerate().map(|(_i,b)| {
            b.get_changes().iter().map(|v| {
                self.joltages[*v as usize]
            }).min().unwrap() as i64
        }).collect::<Vec<_>>()
    }

    fn get_affected(&self) -> Vec<BTreeSet<usize>> {
        // Check how many buttons affect each light
        (0..self.n_lights).map(|v| {
            self.buttons.iter().enumerate().filter_map(|(i,b)| {
                if  ((b.changes >> v) & 1) == 1 {
                    Some(i) 
                } else {
                    None
                }
            }).collect::<BTreeSet<usize>>()
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

    let a = machines.iter().map(|m| (m.n_lights, m.buttons.len())).collect::<Vec<_>>();
    println!("lights, buttons");
    for aa in a.iter() {
        println!("{}, {}", aa.0, aa.1);
    }
    let part2 = machines.iter().enumerate().collect::<Vec<_>>().par_iter().map(|(i,m)| {
        //println!("Processing machine {} / {}", i, machines.len() );
        //println!("{}", m);
        let res = m.get_part2_linalg() as i64;
        //println!("index {i}: part2: {res}");
        res
    }).sum();

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
        let res = b.get_changes();
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

        let mut m = Machine::from_str("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(m.get_part2_tree(), 12);
        //assert_eq!(m.get_part2_recur(), 12);
        assert_eq!(m.get_part2_tree_alt(), 12);

        let mut m = Machine::from_str("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(m.get_part2_tree(), 11);
        assert_eq!(m.get_part2_recur(), 11);
        assert_eq!(m.get_part2_tree_alt(), 11);

        //let m2 = Machine::from_str("l[#...##] (0,1,3,4,5) (0,4,5) (1,2,3,4) (0,1,2) {132,30,23,13,121,115}");
        //assert_eq!(m2.get_part2_tree(), 138);
        //assert_eq!(m2.get_part2_tree_alt(), 138);

        let a="[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(read_contents(&a).1, 33);
    }


    #[test]
    fn hard() {
        //let m = Machine::from_str("[#..###] (0,5) (1,3) (1,3,4) (0,1,2,3) (3) (0,1,3,4) (1,2,3) {22,51,19,63,22,1}");
        //assert_eq!(m.get_part2_iter(), 11);

        let mut m = Machine::from_str("[##.####.#.] (1,2,3,4,5,6,7,8,9) (0,1,2,3,4,5,6,8,9) (2,5,7,8) (1,3,4,5,7,9) (2,8) (2,3,5,6,7,8,9) (0,4,5,6,7,8,9) (5,9) (0,1,2,3,4,6,7,8,9) (2,4,5,6,8) {28,40,49,48,48,54,42,48,56,63}");
        assert_eq!(m.solve(vec![0,1,1,1,1,1,1,1,1,1]), 1);
        assert_eq!(m.solve(vec![0,2,2,2,2,2,2,2,2,2]), 2);
        assert_eq!(m.solve(vec![0,9,9,9,9,9,9,9,9,9]), 9);
        assert_eq!(m.get_part2_linalg(), 78);
        //assert_eq!(m.get_part2_tree(), 78);
    }
}

