use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::fmt::Display;
use regex::Regex;
use priority_queue::PriorityQueue;

use ndarray::prelude::*;
use ndarray_linalg::Solve;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] struct Args {
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

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}


#[derive(Debug, Clone, Copy)]
struct Button {
    changes: i32, // 
}

impl Button {
    fn from_str(ln: &str) -> Self {
        let mut a = 0;
        for b in ln.split(',') {
            a += 2_i32.pow(b.parse::<u32>().unwrap())
        }
        Self {changes: a}
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
        // Get the activations as a f64 vector
        // This is needed for ndarray solver
        let mut vec = Vec::new();

        let mut div = self.changes;
        let mut m;
        for _i in 0..n {
            (div, m) = (div / 2, div % 2);
            if m == 1 {
                vec.push(1_f64);
            } else {
                vec.push(0_f64);
            }
        }
        vec

    }

    fn get_changes(self) -> Vec<usize> {
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
}

#[derive(Debug, PartialEq)]
enum Comparison {
    Exact,
    Smaller,
    Larger,
}

fn add_to_vector(vec: &[i32], ind: usize, val: i32) -> Vec<i32> {
    // Add scalar to vector at given index
    vec.iter().enumerate().map(|(i,v)| {
        if i == ind {
            v + val
        } else {
            *v
        }
    }).collect()
}


impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = format!("Machine with {} lights, target {:08b}, joltages {:?}, and {} buttons:", self.n_lights, self.target_lights, self.joltages, self.buttons.len());
        for b in self.buttons.iter() {
            out += format!("\n {b}").as_str();
        }
        write!(f, "{out}")
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
                    }).sum());
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
        Self {n_lights,
            target_lights: target_lights.unwrap(),
            buttons,
            joltages: joltages.unwrap(),
        }
    }
    
    fn check_part2(&self, activation: &[i32]) -> bool {
        let sum = self.get_sum(activation);
        sum == self.joltages
    }

    fn get_part1(&self) -> i32 {
        let n = 2_i32.pow(self.buttons.len() as u32);
        // outer loop tests different options
        (0..n).map(|i_opt| {
            let mut s = 0;
            let mut c = 0;
            for i_button in 0..(self.buttons.len()){
                // Check if bit at index 'i_button' is set
                if ((i_opt >> i_button) & 1) == 1 {
                    c += 1;
                    s ^= self.buttons[i_button].changes;
                }
            }
            if s == self.target_lights {
                c
            } else {
                999
            }
        }).min().unwrap()
    }

    fn get_part2_linalg(&self) -> i64 {
        let mut min_val = 0;
        println!("Processing machine with {} buttons (unknowns) and {} lights (equations)", self.buttons.len(), self.n_lights);
        println!("{}", self);
        if self.buttons.len() < self.n_lights {
            // Too many equations
            assert!(self.buttons.len() >= self.n_lights - 2);
            match self.n_lights - self.buttons.len()  {
                2 => {
                    for i in 0..self.n_lights {
                        for j in 0..self.n_lights {
                            if j >= i {
                                continue;
                            }
                            println!("Removing equations {i} and {j}");
                            min_val = self.solve_linalg(&self.joltages, vec![i,j]);
                            if min_val > 0 {
                                println!("Found solution by removing equations {i} and {j}");
                                return min_val;
                            }
                        }
                    }
                },
                1 => {
                    println!("Removing 1 equations");
                    for i in 0..self.n_lights {
                        min_val = self.solve_linalg(&self.joltages, vec![i]);
                        if min_val > 0 {
                            return min_val;
                        }
                    }
                    //return -2;
                },
                _ => {
                    // There should always be either 1 or 2 extra equations
                    panic!("Unexpected equation count");
                }
            }
        } else if self.buttons.len() == self.n_lights {
            //println!("Determined system - single solution");
            min_val = self.solve_linalg(&self.joltages, vec![]);
            if min_val == 0 {
                println!("Could not find solution via linear algebra");
            } else {
                println!("Found solution via linear algebra with {} presses", min_val);
            }
        } else {
            println!("Underdetermined system - Possibly infinite solutions");
        }
        if min_val > 0 {
            min_val
        } else {
            self.get_part2_tree() as i64
        }
    }

    fn solve_linalg(&self, target: &[i32], drop_indices: Vec<usize>) -> i64 {
        let mut matrix: Vec<Vec<f64>> = transpose(self.buttons.iter().map(|b| b.vector_f64(self.n_lights)).collect());
        if !drop_indices.is_empty() {
            matrix = matrix.iter().enumerate().filter_map(|(i,v)| {
                if drop_indices.contains(&i) {
                    None
                } else {
                    Some(v.clone())
                }
            }).collect();
        }

        let b = if drop_indices.is_empty() {
            Array1::from_vec(target.iter().map(|v| *v as f64).collect())
        } else {
            Array1::from_vec(target.iter().enumerate().filter_map(|(i,v)|{
                if drop_indices.contains(&i) {
                    None
                } else {
                    Some(*v as f64)
                }
            }).collect())
        };

        let n = matrix.len();
        let m: Array2<f64> = Array2::from_shape_vec((n, n),
            matrix.iter().flat_map(|v| v.iter()).cloned().collect()
        ).unwrap();

        match m.solve_into(b) {
            Ok(x) => {
                x.sum().round() as i64
            },
            Err(_) => {
                0
            }
        }
    }


    fn algebraic_solver(&self, verbose: usize) -> i32 {
        if verbose > 0 {
            println!("Processing machine with {} buttons (unknowns) and {} lights (equations)", self.buttons.len(), self.n_lights);
            println!("{self}");
        }
        let matrix: Vec<Vec<i32>> = transpose(self.buttons.iter().map(|b| b.vector(self.n_lights)).collect());
        let eqsys = EquationSystem::new(self.joltages.clone(), matrix.clone());
        let (res, sol) = solve_recursive(eqsys, verbose);
        if res == 9999 {
            return -1;
        }
        let v = sol.unwrap();
        if !self.check_part2(&v) {
            println!("Solver gave a solution {res} but it is wrong");
            println!("Target: {:?}", self.joltages);
            println!("Sum: {:?}", self.get_sum(&v));
            return -1;
        }
        res
    }


    fn get_part2_fixed(&self) -> BTreeMap<usize, i32> {
        println!("Trying to solve some values");
        let matrix: Vec<Vec<i32>> = transpose(self.buttons.iter().map(|b| b.vector(self.n_lights)).collect());

        let mut eqsys = EquationSystem::new(self.joltages.clone(), matrix);
        eqsys.solve(0);

        let (matx, sol) = eqsys.get_unique();
        for i in 0..matx.len() {
            println!("{:?} = {}", matx[i], sol[i]);
        }

        // Collect possible pairs
        let mut pairs: Vec<(usize,usize,i32)> = Vec::new();
        for (j,v) in matx.iter().enumerate() {
            if 2 == v.iter().sum() {
                let ind = v.iter().enumerate().filter_map(|(i,x)| {
                    if *x != 0 { Some(i) } else { None }
                }).collect::<Vec<usize>>();
                let t = sol[j];
                println!("X{} and X{} sum to {}", ind[0], ind[1], t);
                pairs.push((ind[0], ind[1], t));
            } else {
                assert!(1 < v.iter().sum());
            }
        }
        pairs.sort_by(|a,b| a.2.cmp(&b.2));
        let tmp_solution: BTreeMap<usize, i32> = eqsys.solution.iter().enumerate().filter_map(|(i,v)| {
            v.as_ref().map(|x| (i, *x))
        }).collect();
        dbg!(&tmp_solution);

        let mut min_sum = 99999;
        let mut best_solution: Option<Vec<Option<i32>>> = None;
        if !pairs.is_empty() {
            let (i1,i2,target) = pairs[0];
            println!("  X{i1} + X{i2} = {target}");
            for v1 in 0..=target {
                let v2 = target - v1;
                println!("    Trying X{i1} = {v1}, X{i2} = {v2}");
                let mut new_sys = eqsys.clone();
                new_sys.add_solution((i1,v1));
                new_sys.add_solution((i2,v2));
                new_sys.solve(0);
                //dbg!(&new_sys.solution);
                let score = new_sys.solution.iter().map(|x| {
                    match x {
                        Some(v) if *v > 0 => *v,
                        _ => -999999, // Add a large negative number to guarantee that score
                        // will be negative when not all variables are solved
                    }
                }).sum::<i32>();
                if score > 0 && score < min_sum {
                    min_sum = score;
                    best_solution = Some(new_sys.solution.clone());
                }
            }
            //let solution = eqsys.solution.clone();
            //new_sys.solution;
        } else if tmp_solution.is_empty() {
            println!("Make guesses");
            for j in 0..10 {
                // Loop over indices
                let max_val = eqsys.get_max(j);
                println!("Try x{j} = 0-{max_val}"); 
                for i in 0..max_val {
                    let mut new_sys = eqsys.clone();
                    new_sys.add_solution((j,i));
                    new_sys.solve(0);
                    if new_sys.get_nsolved() > 2 {
                        if new_sys.solution.iter().all(|v| v.unwrap_or(0) >= 0) {
                            println!("{:?}", &new_sys.get_solution());
                            let (matx, sol) = new_sys.get_unique();
                            for i in 0..matx.len() {
                                println!("{:?} = {}", matx[i], sol[i]);
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        let source = best_solution.as_ref().unwrap_or(&eqsys.solution);


        let fixed_map: BTreeMap<usize, i32> = source
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.as_ref().map(|x| (i, *x)))
            .collect();

        println!("Found {} values", &fixed_map.len());
        fixed_map
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
        //dbg!(&starting_head);

        // Get number of presses. This is used as priority in the queue
        let press_count = starting_head.iter().sum::<i32>();
        let starting_sum = self.get_sum(&starting_head);
        if starting_sum == self.joltages {
            println!("Found solution with {press_count} presses");
            return press_count;
        }
        let diff_sum = self.get_diff_sum(&starting_sum);
        let prio = press_count + diff_sum;
        heads.push(starting_head, -prio);
        loop {
            if heads.is_empty() {
                break -1;
            }
            let (button_activation, _prio) = heads.pop().unwrap();

            loop_count += 1;
            if loop_count > 10000 {
                println!("Processed {loop_count} heads so far.... Quit");
                return -1;
            }

            for i_button in 0..n_buttons {
                if fixed_indices.contains_key(&i_button) {
                    continue;
                }
                let new_head = add_to_vector(&button_activation, i_button, 1);
                let new_sum = self.get_sum(&new_head);
                match self.compare_sum(&new_sum) {
                    Comparison::Larger => {
                    },
                    Comparison::Exact => {
                        let a = new_head.iter().sum::<i32>();
                        println!("Found solution with {a} presses");
                        return a;
                    },
                    Comparison::Smaller => {
                        let diff_sum = self.get_diff_sum(&new_sum);
                        //dbg!(&new_head);
                        //dbg!(&new_sum);
                        //dbg!(&diff_sum);
                        let prio: i32 = diff_sum + new_head.iter().sum::<i32>() + 1;
                        //assert_eq!(new_head.iter().sum::<i32>(), old_len + 1);
                        heads.push(new_head, -prio);
                    }
                }
            }
        }
    }


    fn get_sum(&self, button_activations: &[i32]) -> Vec<i32> {
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
            self.joltages[i] - x[i]
        }).sum()
    }

    fn compare_sum(&self, x: &Vec<i32>) -> Comparison {
        // Compare given vector to the target
        if &self.joltages == x {
            return Comparison::Exact
        }
        assert_eq!(self.joltages.len(), x.len());
        for (i,xx) in x.iter().enumerate() {
            if self.joltages[i] < *xx {
                //println!("At index {}: target {} < sum {}", i, self.joltages[i], x[i]);
                return Comparison::Larger
            }
        }
        Comparison::Smaller
    }
}

#[derive(Debug, Clone)]
struct EquationSystem {
    target: Vec<i32>,
    matrix: Vec<Vec<i32>>,
    solution: Vec<Option<i32>>,
    is_duplicate: Vec<bool>,
}


impl EquationSystem {
    fn new(target: Vec<i32>, matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        assert_eq!(target.len(), matrix.len());
        let buttons = matrix[0].len();
        for v in &matrix {
            assert_eq!(v.len(), buttons);
        }
        Self {
            target,
            matrix,
            solution: vec![None; buttons],
            is_duplicate: vec![false; n],
        }
    }

    fn remove_duplicates(&mut self) {
        let mut unique_vectors: Vec<Vec<i32>> = Vec::new();
        let mut unique_targets: Vec<i32> = Vec::new();
        for (i,v) in self.matrix.iter().enumerate() {
            if unique_vectors.contains(v) {
                self.is_duplicate[i] = true;
            } else {
                unique_vectors.push(v.clone());
                unique_targets.push(self.target[i]);
            }
        }
    }
    
    fn get_solution(&self) -> BTreeMap<usize, Option<i32>> {
        self.solution.iter().enumerate().map(|(i,v)| {
            (i, *v)
        }).collect()
    }

    fn get_solution_vec(&self) -> Vec<i32> {
        // Should only be called when the solution is full
        self.solution.iter().map(|v| {
           v.unwrap() 
        }).collect()
    }
     

    fn get_nsolved(&self) -> usize {
        self.solution.iter().filter(|v| v.is_some()).count()
    }

    fn is_solved(&self) -> bool {
        // All values are solved and target is 0
        self.solution.iter().all(|v| v.is_some()) &&
        self.target.iter().all(|v| *v == 0)
    }

    fn get_sum(&self) -> i32 {
        self.solution.iter().map(|v| v.unwrap_or(999)).sum()
    }

    fn get_unique(&self) -> (Vec<Vec<i32>>, Vec<i32>) {
        let targets: Vec<i32> = self.target.iter().enumerate().filter_map(|(i,v)| {
            if !self.is_duplicate[i] && self.target[i] != 0 {
                Some(*v)
            } else {
                None
            }
        }).collect();
        let matrix = self.matrix.iter().enumerate().filter_map(|(i,v)| {
            if !self.is_duplicate[i] && self.target[i] != 0 {
                Some(v.clone())
            } else {
                None
            }
        }).collect();
        (matrix, targets)
    }

    fn check_vec(&self, vec: &[i32], target: i32) -> Option<(usize, i32)> {
        let m = vec.iter().enumerate().filter_map(|(j,x)|{
            if *x != 0 { Some((j, x)) } else { None }
        }).collect::<Vec<_>>();

        if 1 == m.len() {
            let (xi, val) = m[0];
            //println!("Only a single variable (index {}) in vector at index {} {:?}", xi, vi, v);
            //println!("Target is {target}, coeff is {val}");
            //println!("Return {xi} {}", target/val);
            Some((xi, target / val))
        } else {
            None
        }
    }

    fn add_solution(&mut self, solution: (usize, i32)) {
        let (ix, x) = solution;
        self.solution[ix] = Some(x);
        for (vj,v) in self.matrix.iter_mut().enumerate() {
            if v[ix] != 0 {
                let coeff = v[ix];
                assert_eq!(coeff, 1);
                self.target[vj] -= coeff * x;
                v[ix] = 0;
            }
        }
    }

    fn get_max(&self, xi: usize) -> i32 {
        self.matrix.iter().enumerate().map(|(i,v)| if v[xi] > 0 {
            self.target[i] 
        } else {
                999
            }).min().unwrap()
    }

    fn solve(&mut self, verbose: usize) {
        loop {
            let mut new_solution: Option<(usize,i32)> = None;
            for (vi,v) in self.matrix.iter().enumerate() {
                new_solution = self.check_vec(v, self.target[vi]);
                if new_solution.is_some() {
                    break;
                }
            }

            if new_solution.is_none() {
                //println!("Try Next Step");
                for (ai, a) in self.matrix.iter().enumerate() {
                    if new_solution.is_some() {
                        //println!("Breaking outer loop");
                        break;
                    }
                    if self.is_duplicate[ai] {
                        continue;
                    }
                    for (bi, b) in self.matrix.iter().enumerate() {
                        if self.is_duplicate[bi] || ai == bi {
                            continue;
                        }
                        let target_a = self.target[ai];
                        let target_b = self.target[bi];
                        if target_a > target_b {
                            continue
                        }
                        let new_vec = a.iter().enumerate().map(|(i,av)| {
                            b[i] - av
                        }).collect::<Vec<_>>();

                        let target = target_b - target_a;
                        new_solution = self.check_vec(&new_vec, target);
                        if new_solution.is_some() {
                            break;
                        }
                    }
                }
            }

            if new_solution.is_none() {
                let (vec_remaining, target_remaining)  = self.get_unique();
                if target_remaining.is_empty() {
                    // No more unique values
                    break;
                }
                let combos = 3_i32;
                let i_max = combos.pow(vec_remaining.len() as u32);
                if verbose > 2 {
                    println!("Try combinations");
                    println!("{i_max} combinations");
                }
                for i in 0..i_max {
                    let mut test_vec: Vec<i32> = vec![0; vec_remaining[0].len()];
                    let mut target_test = 0;
                    for (j,v) in vec_remaining.iter().enumerate() {
                        let coeff = (i / combos.pow(j as u32)) % combos - (combos / 2);
                        target_test += coeff * target_remaining[j];
                        for (k, vk) in v.iter().enumerate() {
                            test_vec[k] += coeff * vk;
                        }
                    }
                    new_solution = self.check_vec(&test_vec, target_test);
                    if new_solution.is_some() {
                        break;
                    }
                }
            }

            if let Some((ix,x)) = new_solution {
                self.add_solution((ix, x));
            }  else {
                break;
            }

            self.remove_duplicates();
            let (matx, sol) = self.get_unique();
            if sol.is_empty() {
                // No unknowns left
                break;
            }
            if verbose > 2 {
                for i in 0..matx.len() {
                    println!("{:?} = {}", matx[i], sol[i]);
                }
            }
        }
    }

}

fn solve_recursive(mut eqsys: EquationSystem, verbose: usize) -> (i32, Option<Vec<i32>>) {
    let n_buttons = eqsys.matrix[0].len(); // Get number of unknowns

    let (matx, sol) = eqsys.get_unique();
    if verbose > 0 {
        println!("Starting solve for");
        for i in 0..matx.len() {
            println!("{:?} = {}", matx[i], sol[i]);
        }
    }
    eqsys.solve(verbose);
    let n_solved = eqsys.get_nsolved();
    if eqsys.is_solved() {
        return (eqsys.get_sum(), Some(eqsys.get_solution_vec()));
    }
    let mut min_val = 9999;
    let mut best_solution: Option<Vec<i32>> = None;
    let (matx, sol) = eqsys.get_unique();
    if verbose > 0 {
        println!("Solved {n_solved} out of {n_buttons} buttons");

        println!("Remaining");
        for i in 0..matx.len() {
            println!("{:?} = {}", matx[i], sol[i]);
        }
    }

    // Define vector to hold: indexA, indexB, target
    let mut pairs: Vec<(usize,usize,i32)> = Vec::new();

    // Look for targets that are determined only by two inputs
    for (j,v) in matx.iter().enumerate() {
        if 2 == v.iter().sum() {
            let ind = v.iter().enumerate().filter_map(|(i,x)| {
                if *x != 0 { Some(i) } else { None }
            }).collect::<Vec<usize>>();
            let t = sol[j];
            if verbose > 1 {
                println!("X{} and X{} sum to {}", ind[0], ind[1], t);
            }
            pairs.push((ind[0], ind[1], t));
        } else {
            assert!(1 < v.iter().sum());
        }
    }
    // Sort pairs to be ascending with target value
    pairs.sort_by(|a,b| a.2.cmp(&b.2));

    if pairs.is_empty() {
        // Start trying different values for the remaining unknowns
        let n_solved = eqsys.get_nsolved();
        let mut found_something = false;
        for j in 0..n_buttons {
            // Loop over indices
            if found_something {
                // Already found something, no need to try different buttons
                break;
            }
            if eqsys.solution[j].is_some() {
                // Skip already known values
                continue;
            }
            let max_val = eqsys.get_max(j);
            if verbose > 1 {
                println!("Try x{j} = 0-{max_val}"); 
            }
            for i in 0..max_val {
                let mut new_sys = eqsys.clone();
                new_sys.add_solution((j,i));
                new_sys.solve(verbose);
                if new_sys.target.iter().any(|v| *v < 0) {
                    // Some targets are now negative. This won't lead to a valid solution
                    continue;
                }
                if new_sys.solution.iter().any(|v| v.unwrap_or(0) < 0) {
                    // Some values are now negative. This won't lead to a valid solution
                    continue;
                }
                if new_sys.is_solved() {
                    if verbose > 0 {
                        println!("Found a solution with sum {}, old sum is {}", new_sys.get_sum(), min_val);
                    }
                    if new_sys.get_sum() < min_val {
                        min_val = new_sys.get_sum();
                        best_solution = Some(new_sys.get_solution_vec());
                    }
                    found_something = true;
                } else if new_sys.get_nsolved() == n_buttons {
                    // No remaining unknowns, but the solution is wrong
                } else if new_sys.get_nsolved() > n_solved + 1 {
                    // There is at least 1 other unknown that is now solved
                    if verbose > 0 {
                        println!("Found a partial solution with {} solved buttons, old count was {}", new_sys.get_nsolved(), n_solved);
                    }
                    let (res, sol) = solve_recursive(new_sys, verbose);
                    if res < min_val {
                        min_val = res;
                        best_solution = sol;
                    }
                    found_something = true;
                } else {
                    // This effort didn't solve any new values
                    break;
                }
            }
        }
    }
    else {
        // Get the first pair, this should be the pair with the lowest sum. Thus having the
        // least amount of different values to be tested
        let (i1,i2,target) = pairs[0];
        if verbose > 1 {
            println!("  X{i1} + X{i2} = {target}");
        }
        for v1 in 0..=target {
            let v2 = target - v1;
            if verbose > 1 {
                println!("    Trying X{i1} = {v1}, X{i2} = {v2}");
            }
            let mut new_sys = eqsys.clone();
            new_sys.add_solution((i1,v1));
            new_sys.add_solution((i2,v2));
            new_sys.solve(verbose);
            if new_sys.target.iter().any(|v| *v < 0) {
                // Some targets are now negative. This won't lead to a valid solution
                continue;
            }
            if new_sys.solution.iter().any(|v| v.unwrap_or(0) < 0) {
                // Some values are now negative. This won't lead to a valid solution
                continue;
            }
            if new_sys.is_solved() {
                // Found a solution
                if verbose > 0 {
                    println!("Found a solution with sum {}, old sum is {}", new_sys.get_sum(), min_val);
                }
                if new_sys.get_sum() < min_val {
                    min_val = new_sys.get_sum();
                    best_solution = Some(new_sys.get_solution_vec());
                }
            } else {
                // Otherwise try to find a solution for the new system
                let (res, sol) = solve_recursive(new_sys, verbose);
                if res < min_val {
                    min_val = res;
                    best_solution = sol;
                }
            }
        }
    }
    (min_val, best_solution)
}


fn read_contents(cont: &str) -> (i64, i64) {
    let machines: Vec<Machine> = cont.lines().map(|ln| {
        Machine::from_str(ln)
    }).collect();
    //dbg!(&machines);
    let part1 = machines.iter().map(|m| i64::from(m.get_part1())).sum();
    let part2 = machines.iter().enumerate().map(|(i,m)| {
        let res = i64::from(m.algebraic_solver(0));
        if res < 0 {
            println!("Invalid solution for machine {i}, try linalg solver");
            let r = m.get_part2_linalg();
            assert!(r >= 0, "Linalg solver also failed");
            println!("index {i:03}: part2: {r}");
            return r;
        }
        println!("index {i:03}: part2: {res}");
        res
    }).sum();
    (part1, part2)
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn equation() {
        let mut sys = EquationSystem::new(vec![5,7,3], vec![
            vec![1,1,1],
            vec![1,0,0],
            vec![1,1,0],
        ]);

        sys.solve(0);
        assert_eq!(sys.solution, vec![Some(7), Some(-4), Some(2)]);

        let mat = vec![
                      vec![0, 1, 0, 0, 0, 0, 1, 0, 1, 0],
                      vec![1, 1, 0, 1, 0, 0, 0, 0, 1, 0],
                      vec![1, 1, 1, 0, 1, 1, 0, 0, 1, 1],
                      vec![1, 1, 0, 1, 0, 1, 0, 0, 1, 0],
                      vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1],
                      vec![1, 1, 1, 1, 0, 1, 1, 1, 0, 1],
                      vec![1, 1, 0, 0, 0, 1, 1, 0, 1, 1],
                      vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 0],
                      vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 1],
                      vec![1, 1, 0, 1, 0, 1, 1, 1, 1, 0],
        ];
        let target = vec![28,40,49,48,48,54,42,48,56,63];

        let mut sys = EquationSystem::new(target, mat);
        sys.solve(0);
        let target_solution = vec![Some(5), Some(9), Some(2), Some(14), Some(12), Some(8), Some(7), Some(8), Some(12), Some(1)];
        assert_eq!(sys.solution, target_solution);
    }

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
    }

    #[test]
    fn part2() {

        let m = Machine::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");

        assert_eq!(m.get_part2_tree(), 10);

        let m = Machine::from_str("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(m.algebraic_solver(1), 12);

        let m = Machine::from_str("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(m.algebraic_solver(1), 11);


        let a="[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(read_contents(&a).1, 33);
    }


    #[test]
    fn index8() {
        let m = Machine::from_str("[##.####.#.] (1,2,3,4,5,6,7,8,9) (0,1,2,3,4,5,6,8,9) (2,5,7,8) (1,3,4,5,7,9) (2,8) (2,3,5,6,7,8,9) (0,4,5,6,7,8,9) (5,9) (0,1,2,3,4,6,7,8,9) (2,4,5,6,8) {28,40,49,48,48,54,42,48,56,63}");
        assert_eq!(m.get_part2_linalg(), 78);
        //assert_eq!(m.get_part2_tree(), 78);
    }

    #[test]
    fn index7() {
        let m = Machine::from_str("[#.....####] (6) (5,7,8) (0,1,3,4,9) (0,2,4,5,6,9) (1,2,8) (0,4,5,7) (4,6) (0,2,3,7,8,9) (1,5,6,9) (0,2,3) (0,2,3,4,6,7,8,9) (3,4,8) {57,31,44,54,68,54,52,48,62,47}");
        assert_eq!(m.algebraic_solver(1), 129);
    }

    #[test]
    fn index12() {
        let m = Machine::from_str("[#.#####.##] (3,4) (0,1,3,4,5,7,8,9) (1,2,3,4,5,6,7,9) (0,1,2,3,7) (0,1,2,3,5,7,8,9) (0,1,2,3,5,6,7,9) (0,1,3,4,6) (0,4,5,6,7,8,9) (1,3,4,9) (0,3,5,6,7,8) (1,2,3,4,7,8) {51,83,66,96,74,54,52,81,36,64}");
        assert_eq!(m.algebraic_solver(1), 107);
    }

    #[test]
    fn index17() {
        let m = Machine::from_str("[.#...#..] (0,1,5,7) (4,5) (0,6) (0,1,2,5,6,7) (0,1,2,4,6,7) (2,7) (0,2,5,7) (3,4,5) (0,1) {42,29,42,6,29,54,20,52}");
        assert_eq!(m.algebraic_solver(1), 74);
    }

    #[test]
    fn index0() {
        let m = Machine::from_str("[#...##] (0,1,3,4,5) (0,4,5) (1,2,3,4) (0,1,2) {132,30,23,13,121,115}");
        assert_eq!(m.get_part2_linalg(), 138);
    }

    #[test]
    fn index10() {
        let m = Machine::from_str("[#.#...#.] (5,6) (2,3) (1,4,5,6,7) (2,3,6) (1,2,3,4,6,7) (3,4,5,6) (0,1,3,4,7) {19,36,183,222,56,35,55,36}");
        assert_eq!(m.get_part2_linalg(), 237);
    }

    #[test]
    fn index145() {
        let m = Machine::from_str("[.#...###] (0,1,2,3,4,6,7) (2,3,4,5,6) (0,1,3,4,5,6) (0,2,3,4,5,6) (1,3,4,5,6) (2,3,4,5) {22,8,44,51,51,50,38,1}");
        assert_eq!(m.get_part2_linalg(), 51);
    }

    #[test]
    fn index90() {
        let m = Machine::from_str("[.#.##.#.##] (1,7,8) (1,3,8) (0,1,2,3,4,5,6,9) (3,4,7,8,9) (1,2,4,8) (0,1,2,3,4,5,8,9) (0,2,5) (1,2,3,4,5,7,9) (0,1,2,3,4,5,6,8,9) (2,3,7) (5,7) {39,50,45,48,47,56,16,48,57,44}");
        assert_eq!(m.algebraic_solver(1), 91);
    }

    #[test]
    fn index110() {
        let m = Machine::from_str("[.##..#...#] (2,3) (1,3,5,7,8,9) (2,3,4,5,6,7,9) (6,8) (0,1,9) (0,1,2,3,4,7,9) (0,5,9) (0,1,2,3,5,6,7,8) (0,1,2,6,8) (2,3,7,8) (2,3,6,7,9) (0,3,5,6,8,9) (0,1,2,3,6,7,8,9) {90,71,54,75,5,78,87,56,87,98}");
        assert_eq!(m.algebraic_solver(1), 144);
    }
}

