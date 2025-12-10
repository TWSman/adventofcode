use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::fmt::Display;
use core::fmt;
use regex::Regex;

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


#[derive(Debug, Clone, Copy)]
enum Light {
    On,
    Off
}

#[derive(Debug, Clone, Copy)]
struct Button {
    changes: i32 // 
}

impl Button {
    fn from_str(ln: &str) -> Self {
        let mut a = 0;
        for b in ln.split(",") {
            a += 2_i32.pow(b.parse::<u32>().unwrap())
        }
        Self {changes: a}
    }
}

#[derive(Debug)]
struct Machine {
    n_lights: usize,
    target_lights: i32, // Target as binary mask
    buttons: Vec<Button>,
    joltages: Vec<i32>,
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
        println!("Testing machine with target {:b} and {} buttons", self.target_lights, self.buttons.len());
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

    fn get_part2(&self) -> i32 {
        0
    }
}


fn read_contents(cont: &str) -> (i64, i64) {
    let machines: Vec<Machine> = cont.lines().map(|ln| {
        Machine::from_str(ln)
    }).collect();
    dbg!(&machines);
    let part1 = machines.iter().map(|m| m.get_part1() as i64).sum();
    let part2 = 0;
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
    fn part2() {

        let m = Machine::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        dbg!(&m);
        assert_eq!(m.get_part2(), 10);

        let a="[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(read_contents(&a).1, 33);
    }

}

