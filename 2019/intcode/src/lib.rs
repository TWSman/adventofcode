use std::time::Instant;


#[derive(Debug, Clone)]
pub struct Program {
    vals:  Vec<i64>,
    initial_state:  Vec<i64>,
    verbose: usize,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

const OP_VERBOSE: usize = 2;
const STOP_VERBOSE: usize = 1;

impl Program {
    pub fn from_list(initial_state: Vec<i64>) -> Self {
        Program { vals: initial_state.clone(), initial_state: initial_state.clone(), verbose: 1, inputs: Vec::new(), outputs: Vec::new()}
    }

    pub fn new(ln: &str) -> Self {
        let vals = ln.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        Self::from_list(vals)
    }

    pub fn add_input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    pub fn reset(&mut self) {
        self.vals = self.initial_state.clone();
    }

    pub fn get_index(&self, index: usize) -> i64 {
        self.vals[index]
    }

    pub fn set_index(&mut self, index: usize, val: i64) {
        self.vals[index] = val;
    }

    pub fn set_verbose(&mut self, v: usize) {
        self.verbose = v;
    }

    pub fn get_outputs(&self) -> Vec<i64> {
        self.outputs.clone()
    }

    pub fn get_output(&self, index: i64) -> i64 {
        if index >= 0 {
            self.outputs[index as usize]
        } else {
            self.outputs[(self.outputs.len() as i64 + index) as usize]
        }
    }


    pub fn run(&mut self) {
        let start = Instant::now();
        if self.verbose >= 1 {
            println!("\nStarting execution");
        }
        let mut pointer: usize = 0;
        let mut input_pointer: usize = 0;
        loop {
            let opcode = self.vals[pointer] % 100;
            let op = Operation::new(opcode.try_into().unwrap());
            let mod1 = (self.vals[pointer] / 100) % 10;
            let mod2 = (self.vals[pointer] / 1000) % 10;
            let mod3 = (self.vals[pointer] / 10000) % 10;

            if self.verbose >= 2 {
                dbg!(pointer, opcode, mod1, mod2, mod3);
            }

            match op {
                Some(Operation::Sum) => {

                    let in1 = self.vals[pointer+1] as usize;
                    let in2 = self.vals[pointer+2] as usize;
                    let out = self.vals[pointer+3] as usize;

                    let a = if mod1 == 0 { self.vals[in1] } else { in1 as i64 };
                    let b = if mod2 == 0 { self.vals[in2] } else { in2 as i64 };
                    if self.verbose >= OP_VERBOSE {
                        println!("Adding {} and {} into *{}", a, b, out);
                    }
                    self.vals[out] = a + b;
                    pointer += 4;
                },
                Some(Operation::Product) => {
                    let in1 = self.vals[pointer+1] as usize;
                    let in2 = self.vals[pointer+2] as usize;
                    let out = self.vals[pointer+3] as usize;
                    let a = if mod1 == 0 { self.vals[in1] } else { in1 as i64 };
                    let b = if mod2 == 0 { self.vals[in2] } else { in2 as i64 };
                    if self.verbose >= OP_VERBOSE {
                        println!("Multiplying {} and {} into *{}", a, b, out);
                    }
                    self.vals[out] = a * b;
                    pointer += 4;
                },

                Some(Operation::Input) => {
                    let input = self.inputs[input_pointer];
                    input_pointer += 1;
                    let out = self.vals[pointer+1] as usize;
                    self.vals[out] = input;
                    pointer += 2;
                }
                Some(Operation::Output) => {
                    let out = self.vals[pointer+1];
                    let out_val = if mod1 == 0 {self.vals[out as usize]} else { out};
                    println!("Output: {}", out_val);
                    self.outputs.push(out_val);
                    pointer += 2;
                }

                Some(Operation::JumpIfTrue) => {
                    let ind1 = self.vals[pointer + 1];
                    let ind2 = self.vals[pointer + 2];
                    let val1 = if mod1 == 0 { self.vals[ind1 as usize] } else { ind1};
                    let val2 = if mod2 == 0 { self.vals[ind2 as usize] } else { ind2};
                    if val1 != 0 {
                        pointer = val2 as usize;
                    } else {
                        pointer += 3;
                    }
                }

                Some(Operation::JumpIfFalse) => {
                    let ind1 = self.vals[pointer + 1];
                    let ind2 = self.vals[pointer + 2];
                    let val1 = if mod1 == 0 { self.vals[ind1 as usize] } else { ind1};
                    let val2 = if mod2 == 0 { self.vals[ind2 as usize] } else { ind2};
                    if val1 == 0 {
                        pointer = val2 as usize;
                    } else {
                        pointer += 3;
                    }
                }

                Some(Operation::LessThan) => {
                    let ind1 = self.vals[pointer+1] as usize;
                    let ind2 = self.vals[pointer+2] as usize;
                    let out = self.vals[pointer+3] as usize;

                    let val1 = if mod1 == 0 { self.vals[ind1] } else { ind1 as i64 };
                    let val2 = if mod2 == 0 { self.vals[ind2] } else { ind2 as i64 };
                    if val1 < val2 {
                        self.vals[out] = 1;
                    } else {
                        self.vals[out] = 0;
                    }
                    pointer += 4;
                }
                Some(Operation::Equals) => {
                    let ind1 = self.vals[pointer+1] as usize;
                    let ind2 = self.vals[pointer+2] as usize;
                    let out = self.vals[pointer+3] as usize;

                    let val1 = if mod1 == 0 { self.vals[ind1] } else { ind1 as i64 };
                    let val2 = if mod2 == 0 { self.vals[ind2] } else { ind2 as i64 };
                    if val1 == val2 {
                        self.vals[out] = 1;
                    } else {
                        self.vals[out] = 0;
                    }
                    pointer += 4;
                }

                Some(Operation::Stop) => {
                    let elapsed = start.elapsed();
                    if self.verbose >= STOP_VERBOSE {
                        println!("Stopping execution after {:.2?}", elapsed);
                    }
                    break;
                },
                None => panic!("Unknown opcode {}", opcode),
            }
        }
    }
}

enum Operation {
    Sum,
    Product,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Stop,
}

impl Operation {
    fn new(code: usize) -> Option<Self> {
        match code {
            1 => Some(Operation::Sum),
            2 => Some(Operation::Product),
            3 => Some(Operation::Input),
            4 => Some(Operation::Output),
            5 => Some(Operation::JumpIfTrue),
            6 => Some(Operation::JumpIfFalse),
            7 => Some(Operation::LessThan),
            8 => Some(Operation::Equals),
            99 => Some(Operation::Stop),
            _ => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let a = "1,0,0,3,99";
        let p = Program::new(a);
        assert_eq!(p.vals, vec![1,0,0,3,99]);

        // Does a sum and a product
        let b = "1,9,10,3,2,3,11,0,99,30,40,50";
        let mut p = Program::new(b);
        assert_eq!(p.vals, vec![1,9,10,3,2,3,11,0,99,30,40,50]);
        p.run();
        assert_eq!(p.vals, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
        

        // Basic sum
        let mut p = Program::new("1,0,0,0,99");
        p.run();
        assert_eq!(p.vals, vec![2,0,0,0,99]);
        
        // Basic product
        let mut p = Program::new("2,3,0,3,99");
        p.run();
        assert_eq!(p.vals, vec![2,3,0,6,99]);

        // Basic product
        let mut p = Program::new("2,4,4,5,99,0");
        p.run();
        assert_eq!(p.vals, vec![2,4,4,5,99,9801]);
        
        //  Another sum
        let mut p = Program::new("1,1,1,4,99,5,6,0,99");
        p.run();
        assert_eq!(p.vals,  vec![30,1,1,4, 2,5,6,0,99]);
    }

    #[test]
    fn equals() {
        // Position mode
        let p = Program::from_list(vec![3,9,8,9,10,9,4,9,99,-1,8]);
        let mut p1 = p.clone();
        p1.add_input(8);
        p1.run();
        assert_eq!(p1.get_outputs(), vec![1]);
        
        let mut p2 = p.clone();
        p2.add_input(7);
        p2.run();
        assert_eq!(p2.get_outputs(), vec![0]);


        // Immediate mode
        let p = Program::from_list(vec![3,3,1108,-1,8,3,4,3,99]);
        let mut p1 = p.clone();
        p1.add_input(8);
        p1.run();
        assert_eq!(p1.get_outputs(), vec![1]);

        let mut p2 = p.clone();
        p2.add_input(7);
        p2.run();
        assert_eq!(p2.get_outputs(), vec![0]);
        
    }

    #[test]
    fn less_than() {
        // Position mode
        let p = Program::from_list(vec![3,9,7,9,10,9,4,9,99,-1,8]);
        let mut p1 = p.clone();
        p1.add_input(7);
        p1.run();
        assert_eq!(p1.get_outputs(), vec![1]);
        
        let mut p2 = p.clone();
        p2.add_input(8);
        p2.run();
        assert_eq!(p2.get_outputs(), vec![0]);

        let p = Program::from_list(vec![3,3,1107,-1,8,3,4,3,99]);
        let mut p1 = p.clone();
        p1.add_input(7);
        p1.run();
        assert_eq!(p1.get_outputs(), vec![1]);
        
        let mut p2 = p.clone();
        p2.add_input(8);
        p2.run();
        assert_eq!(p2.get_outputs(), vec![0]);
    }

    #[test]
    fn jump() {
        let p = Program::from_list(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);
        let mut p1 = p.clone();
        p1.add_input(0);
        p1.run();
        assert_eq!(p1.get_outputs(), vec![0]);

        let mut p2 = p.clone();
        p2.add_input(7);
        p2.run();
        assert_eq!(p2.get_outputs(), vec![1]);

        let p = Program::from_list(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);
        let mut p1 = p.clone();
        p1.add_input(0);
        p1.run();
        assert_eq!(p1.get_outputs(), vec![0]);

        let mut p2 = p.clone();
        p2.add_input(7);
        p2.run();
        assert_eq!(p2.get_outputs(), vec![1]);
    }

    #[test]
    fn day5(){
        // Example from day 5, part2
        let p = Program::from_list(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
        let mut p1 = p.clone();
        p1.add_input(0);
        p1.run();
        assert_eq!(p1.get_outputs(), vec![999]);

        let mut p1 = p.clone();
        p1.add_input(9);
        p1.run();
        assert_eq!(p1.get_outputs(), vec![1001]);

        let mut p1 = p.clone();
        p1.add_input(8);
        p1.run();
        assert_eq!(p1.get_outputs(), vec![1000]);
    }
}

