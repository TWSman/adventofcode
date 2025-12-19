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


fn get_solved() -> BTreeMap<usize,i32> {
    BTreeMap::from([
        (000, 138),
        (001, 38),
        (002, 52),
        (003, 64),
        (004, 62),
        (005, 107),
        (006, 199),
        (007, -1),
        (008, 78),
        (009, 39),
        (010, 237),
        (011, 81),
        (012, -1),
        (013, 83),
        (014, -1),
        (015, 219),
        (016, 81),
        (017, 77),
        (018, -1),
        (019, 60),
        (020, -1),
        (021, 91),
        (022, 180),
        (023, -1),
        (024, 222),
        (025, 95),
        (026, 66),
        (027, 213),
        (028, 8),
        (029, 191),
        (030, 39),
        (031, 215),
        (032, 26),
        (033, 82),
        (034, 68),
        (035, 60),
        (036, 53),
        (037, -1),
        (038, 56),
        (039, 57),
        (040, 57),
        (041, 193),
        (042, 37),
        (043, 228),
        (044, -1),
        (045, 48),
        (046, 26),
        (047, 66),
        (048, 93),
        (049, 14),
        (050, 42),
        (051, -1),
        (052, 156),
        (053, -1),
        (054, -1),
        (055, 212),
        (056, 206),
        (057, 141),
        (058, -1),
        (059, -1),
        (060, 171),
        (061, 87),
        (062, -1),
        (063, 220),
        (064, -1),
        (065, 45),
        (066, 72),
        (067, 53),
        (068, 58),
        (069, 34),
        (070, 191),
        (071, 74),
        (072, 50),
        (073, 259),
        (074, -1),
        (075, 90),
        (076, 80),
        (077, 55),
        (078, -1),
        (079, 115),
        (080, -1),
        (081, 212),
        (082, 60),
        (083, 149),
        (084, 50),
        (085, 44),
        (086, 73),
        (087, 63),
        (088, 76),
        (089, -1),
        (090, -1),
        (091, 19),
        (092, 147),
        (093, 80),
        (094, 73),
        (095, -1),
        (096, 59),
        (097, 84),
        (098, 142),
        (099, 287),
        (100, 90),
        (101, 55),
        (102, -1),
        (103, 67),
        (104, 74),
        (105, 264),
        (106, 85),
        (107, 11),
        (108, 150),
        (109, -1),
        (110, -1),
        (111, 96),
        (112, 199),
        (113, 32),
        (114, 39),
        (115, 46),
        (116, 11),
        (117, -1),
        (118, 185),
        (119, 56),
        (120, -1),
        (121, 178),
        (122, 317),
        (123, 82),
        (124, 116),
        (125, 236),
        (126, -1),
        (127, 56),
        (128, 147),
        (129, -1),
        (130, -1),
        (131, 232),
        (132, 90),
        (133, 136),
        (134, -1),
        (135, 118),
        (136, 110),
        (137, 201),
        (138, 80),
        (139, -1),
        (140, 195),
        (141, 161),
        (142, -1),
        (143, 126),
        (144, 57),
        (145, 51),
        (146, -1),
        (147, 70),
        (148, 237),
        (149, 229),
        (150, 22),
        (151, -1),
        (152, -1),
        (153, -1),
        (154, -1),
        (155, 212),
        (156, 34),
        (157, 55),
        (158, 100),
        (159, 11),
        (160, 87),
        (161, 56),
        (162, 194),
        (163, 63),
        (164, 216),
        (165, 28),
        (166, 76),
        (167, 59),
        (168, 233),
        (169, 138),
        (170, 227),
        (171, 43),
        (172, -1),
        (173, 21),
        (174, 52),
        (175, 77),
        (176, 68),
        (177, 199),
        (178, 99),
        (179, 22),
        (180, -1),
        (181, 55),
        (182, 103),
        (183, -1),
        (184, 93),
        (185, -1),
        (186, 72),
    ])
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
        println!("Processing machine with {} buttons and {} lights", self.buttons.len(), self.n_lights);
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
                            println!("Removing equations {} and {}", i, j);
                            min_val = self.solve_linalg(&self.joltages, vec![i,j]);
                            if min_val > 0 {
                                println!("Found solution by removing equations {} and {}", i, j);
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

    fn solve_linalg(&self, target: &Vec<i32>, drop_indices: Vec<usize>) -> i64 {
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
                    dbg!(&v);
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


    fn get_part2_fixed(&self) -> (BTreeMap<usize, i32>, Vec<(usize,usize,i32)>) {
        let matrix: Vec<Vec<i32>> = transpose(self.buttons.iter().map(|b| b.vector(self.n_lights)).collect());

        let mut eqsys = EquationSystem::new(self.joltages.clone(), matrix.clone());
        eqsys.solve();

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
        
        let mut min_sum = 99999;
        let mut best_solution: Option<Vec<Option<i32>>> = None;
        if pairs.len() > 0 {
            let (i1,i2,target) = pairs[0];
            println!("  X{} + X{} = {}", i1, i2, target);
            for v1 in 0..=target {
                let v2 = target - v1;
                println!("    Trying X{} = {}, X{} = {}", i1, v1, i2, v2);
                let mut new_sys = eqsys.clone();
                new_sys.add_solution((i1,v1));
                new_sys.solve();
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
        }

        let fixed_map = if best_solution.is_some() {
                best_solution.unwrap().iter().enumerate().filter_map(|(i,v)| {
                match v {
                    Some(x) => Some((i, *x)),
                    None => None,
                }
            }).collect()
        } else {
            eqsys.solution.iter().enumerate().filter_map(|(i,v)| {
                match v {
                    Some(x) => Some((i, *x)),
                    None => None,
                }
            }).collect()
        };
        (fixed_map, pairs)
    }

    // Set priority to n_presses + abs(diff)
    fn get_part2_tree(&self) -> i32 {
        // Brute force solution to part2
        // Works but is too slow
        //
        // Get buttons whose activation count is fixed
        let (fixed_indices, _pairs) = self.get_part2_fixed();
        // This should not happen here
        //assert!(pairs.len() == 0);
        //dbg!(&fixed_indices);
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
            println!("Found solution with {} presses", press_count);
            return press_count;
        }
        else {
            //dbg!(&starting_sum);
            //dbg!(&self.joltages);
            //panic!();
        }
        let diff_sum = self.get_diff_sum(&starting_sum);
        //println!("Starting sum:");
        //dbg!(&starting_sum);
        //println!("Diff sum: {}", diff_sum);
        //match pairs.get(0) {
        //    Some((i1,i2,target)) =>  {
        //        fixed_indices.insert(*i1, -1);
        //        fixed_indices.insert(*i2, -1);
        //        for v1 in 0..=*target {
        //            let v2 = target - v1;
        //            let new_head = add_to_vector(&starting_head, *i1, v1);
        //            let new_head = add_to_vector(&new_head, *i2, v2);
        //            let new_sum = self.get_sum(&new_head);
        //            let diff_sum = self.get_diff_sum(&new_sum);
        //            let prio: i32 = diff_sum + new_head.iter().sum::<i32>() + 1;
        //            heads.push(new_head, -prio);
        //            }
        //        }
        //    None => {}
        //}
        let prio = press_count + diff_sum;
        heads.push(starting_head, -prio);
        loop {
            if heads.len() == 0 {
                break -1;
            }
            let (button_activation, _prio) = heads.pop().unwrap();

            loop_count += 1;
            if loop_count > 100000 {
                println!("Processed {} heads so far.... Quit", loop_count);
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
                        continue;
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

    fn get_part2_tree_alt(&self) -> i32 {
        // Alternative version that keeps track of the joltage activation
        // instead of button activations
        //println!("Processing machine with {} buttons and {} lights", self.buttons.len(), self.n_lights);
        let (fixed_indices, _pairs) = self.get_part2_fixed();
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

#[derive(Debug, Clone)]
struct EquationSystem {
    target: Vec<i32>,
    vectors: Vec<Vec<i32>>,
    solution: Vec<Option<i32>>,
    is_duplicate: Vec<bool>,
}

impl EquationSystem {
    fn new(target: Vec<i32>, vectors: Vec<Vec<i32>>) -> Self {
        let n = vectors.len();
        assert_eq!(target.len(), vectors.len());
        let buttons = vectors[0].len();
        for v in vectors.iter() {
            assert_eq!(v.len(), buttons);
        }
        Self {
            target: target,
            vectors: vectors,
            solution: vec![None; buttons],
            is_duplicate: vec![false; n],
        }
    }

    fn remove_duplicates(&mut self) {
        let mut unique_vectors: Vec<Vec<i32>> = Vec::new();
        let mut unique_targets: Vec<i32> = Vec::new();
        for (i,v) in self.vectors.iter().enumerate() {
            if !unique_vectors.contains(v) {
                unique_vectors.push(v.clone());
                unique_targets.push(self.target[i]);
            } else {
                self.is_duplicate[i] = true;
            }
        }
    }

    fn get_unique(&self) -> (Vec<Vec<i32>>, Vec<i32>) {
        let targets: Vec<i32> = self.target.iter().enumerate().filter_map(|(i,v)| {
            if !self.is_duplicate[i] && self.target[i] != 0 {
                Some(*v)
            } else {
                None
            }
        }).collect();
        let vectors = self.vectors.iter().enumerate().filter_map(|(i,v)| {
            if !self.is_duplicate[i] && self.target[i] != 0 {
                Some(v.clone())
            } else {
                None
            }
        }).collect();
        (vectors, targets)
    }

    fn check_vec(&self, vec: &Vec<i32>, target: i32) -> Option<(usize, i32)> {
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
        for (vj,v) in self.vectors.iter_mut().enumerate() {
            if v[ix] != 0 {
                let coeff = v[ix];
                assert_eq!(coeff, 1);
                self.target[vj] -= coeff * x;
                v[ix] = 0;
            }
        }
    }

    fn solve(&mut self) {
        loop {
            let mut new_solution: Option<(usize,i32)> = None;
            for (vi,v) in self.vectors.iter().enumerate() {
                new_solution = self.check_vec(v, self.target[vi]);
                if new_solution.is_some() {
                    break;
                }
            }

            if new_solution.is_none() {
                //println!("Try Next Step");
                for (ai, a) in self.vectors.iter().enumerate() {
                    if new_solution.is_some() {
                        //println!("Breaking outer loop");
                        break;
                    }
                    if self.is_duplicate[ai] {
                        continue;
                    }
                    for (bi, b) in self.vectors.iter().enumerate() {
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
                println!("Try combinations");
                let (vec_remaining, target_remaining)  = self.get_unique();
                let combos = 3_i32;
                let i_max = combos.pow(vec_remaining.len() as u32);
                println!("{i_max} combinations");
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

            if new_solution.is_none() {
                break;
            } else {
                let (ix, x) = new_solution.unwrap();
                self.add_solution((ix, x));
            }
            self.remove_duplicates();
            let (matx, sol) = self.get_unique();
            if sol.len() == 0 {
                // No unknowns left
                break;
            }
            for i in 0..matx.len() {
                println!("{:?} = {}", matx[i], sol[i]);
            }
            //#dbg!(&self.get_unique());
        }
    }

}

fn read_contents(cont: &str) -> (i64, i64) {
    let machines: Vec<Machine> = cont.lines().map(|ln| {
        Machine::from_str(ln)
    }).collect();
    //dbg!(&machines);
    let part1 = machines.iter().map(|m| m.get_part1() as i64).sum();
    let solved_cases = get_solved();
    let mut solved = 0;
    let mut new_solved = 0;
    let part2 = machines.iter().enumerate().map(|(i,m)| {
      //println!("Processing machine {} / {}", i, machines.len() );
      //println!("{}", m);
      //m.get_part2_tree() as i64
        if solved_cases[&i] != -1 {
            println!("Skipping machine {} as already solved", i);
            solved += 1;
            return 0;
        }
        let res = m.get_part2_linalg() as i64;
        if res == -2 {
            println!("Could not find solution for machine {}", i);
            panic!();
        }
        if res > 0 {
            solved += 1;
            new_solved += 1;
            println!("index {i:03}: part2: {res}");
        }
        //if res == -1 && i != 7 && i != 12 && i != 14 {
        //    dbg!(&m);
        //    println!("Could not find solution for machine {}", i);
        //    panic!();
        //}
        //println!("index {i:03}: part2: {res}");
        res
    }).sum();

    println!("Solved {}/{} machines, new solves: {}", solved, machines.len(), new_solved);

    //let a = machines.iter().map(|m| (m.n_lights, m.buttons.len())).collect::<Vec<_>>();
    //println!("lights, buttons");
    //for aa in a.iter() {
    //    println!("{}, {}", aa.0, aa.1);
    //}
    //let part2 = machines.iter().enumerate().collect::<Vec<_>>().par_iter().map(|(i,m)| {
    //    println!("Processing machine {} / {}", i, machines.len() );
    //    //println!("{}", m);
    //    let res = m.get_part2_linalg() as i64;
    //    println!("index {i}: part2: {res}");
    //    res
    //}).sum();

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

        sys.solve();
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
        sys.solve();
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

    #[test]
    fn index7() {
        let m = Machine::from_str("[#.....####] (6) (5,7,8) (0,1,3,4,9) (0,2,4,5,6,9) (1,2,8) (0,4,5,7) (4,6) (0,2,3,7,8,9) (1,5,6,9) (0,2,3) (0,2,3,4,6,7,8,9) (3,4,8) {57,31,44,54,68,54,52,48,62,47}");
        assert_eq!(m.get_part2_linalg(), 78);
    }

    #[test]
    fn index12() {
        let m = Machine::from_str("[#.#####.##] (3,4) (0,1,3,4,5,7,8,9) (1,2,3,4,5,6,7,9) (0,1,2,3,7) (0,1,2,3,5,7,8,9) (0,1,2,3,5,6,7,9) (0,1,3,4,6) (0,4,5,6,7,8,9) (1,3,4,9) (0,3,5,6,7,8) (1,2,3,4,7,8) {51,83,66,96,74,54,52,81,36,64}");
        assert_eq!(m.get_part2_linalg(), 1);
    }

    #[test]
    fn index18() {
        let m = Machine::from_str("[.#...#..] (0,1,5,7) (4,5) (0,6) (0,1,2,5,6,7) (0,1,2,4,6,7) (2,7) (0,2,5,7) (3,4,5) (0,1) {42,29,42,6,29,54,20,52}");
        assert_eq!(m.get_part2_linalg(), 74);
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
        // There is a bug where the searched solution does not actually satisfy all equations
        let m = Machine::from_str("[.#.##.#.##] (1,7,8) (1,3,8) (0,1,2,3,4,5,6,9) (3,4,7,8,9) (1,2,4,8) (0,1,2,3,4,5,8,9) (0,2,5) (1,2,3,4,5,7,9) (0,1,2,3,4,5,6,8,9) (2,3,7) (5,7) {39,50,45,48,47,56,16,48,57,44}");
        assert_eq!(m.get_part2_linalg(), 51);
    }






// Index 14:
//"[####.#...#] (2,3,4,5,6,8) (2,6) (0,4,7) (2,5,7) (0,2,3,5,7,9) (0,1,2,4,8) (0,1,4,5,6,9) (1,4,7) (2) (0,1,2,4,5,9) (1,3,4,7,9) (0,2,3,5,8,9) (1,4,5,6,7,9) {52,60,56,45,84,77,64,37,34,73}";
    //
    // Index 18
//[.#...#..] (0,1,5,7) (4,5) (0,6) (0,1,2,5,6,7) (0,1,2,4,6,7) (2,7) (0,2,5,7) (3,4,5) (0,1) {42,29,42,6,29,54,20,52}

}

