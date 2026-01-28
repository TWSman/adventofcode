use std::time::Instant;
use num_enum::TryFromPrimitive;


#[derive(Debug, Clone)]
pub struct Program {
    // The main internal state
    vals:  Vec<i64>, // Main memory of the program
    pointer: usize, // Instruction pointer
    relative_base: i64,

    // Other helpful stuff, related to I/O, verbosity and resetting memory
    input_pointer: usize,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
    verbose: usize,
    initial_state: Vec<i64>,
}

const OP_VERBOSE: usize = 2;
const STOP_VERBOSE: usize = 1;

impl Program {
    pub fn from_list(initial_state: Vec<i64>) -> Self {
        Program {
            vals: initial_state.clone(),
            pointer: 0,
            relative_base: 0,
            input_pointer: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
            verbose: 1,
            initial_state: initial_state.clone(),
        }
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
        self.inputs.clear();
        self.outputs.clear();
        self.pointer = 0;
        self.input_pointer = 0;
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

    pub fn run_until_stop(&mut self) {
        loop {
            let res = self.run();
            if res.is_none() {
                break;
            }
        }
    }


    pub fn run(&mut self) -> Option<i64> {
        let start = Instant::now();
        if self.verbose >= 1 {
            println!("\nStarting execution");
        }
        loop {
            let opcode = self.vals[self.pointer];
            let op = Operation::new((opcode % 100).try_into().unwrap()).unwrap();

            let mods = vec![(self.vals[self.pointer] / 100) % 10,
                (self.vals[self.pointer] / 1000) % 10,
                (self.vals[self.pointer] / 10000) % 10];

            if self.verbose >= 2 {
                println!();
                dbg!(self.pointer, op, opcode, &mods);
            }

            let (inputs, outputs, total) = op.get_parameters();
            assert!(outputs <= 1); // This implementation only supports one output parameter
            let param: Vec<i64> = (self.pointer+1..self.pointer+1+inputs).enumerate().map(|(i, pi)| {
                if mods[i] == 0 {
                    self.vals[self.vals[pi as usize] as usize]
                } else {
                    self.vals[pi as usize]
                }
            }).collect();
            let out_ind: usize = if outputs == 1 {
                self.vals[self.pointer + 1 + inputs].try_into().expect("Output index must be positive")
            } else {
                0
            };
            if self.verbose >= 2 {
                dbg!(&param);
                dbg!(&out_ind);
            }

            match op {
                Operation::Sum => {
                    if self.verbose >= OP_VERBOSE {
                        println!("Adding {} and {} into *{}", param[0], param[1], out_ind);
                    }
                    self.vals[out_ind] = param[0] + param[1];
                },
                Operation::Product => {
                    if self.verbose >= OP_VERBOSE {
                        println!("Multiplying {} and {} into *{}", param[0], param[1], out_ind);
                    }
                    self.vals[out_ind] = param[0] * param[1];
                },

                Operation::Input => {
                    let input = self.inputs[self.input_pointer];
                    self.input_pointer += 1;
                    if self.verbose >= 1 {
                        println!("Read input: {} to *{}", input, out_ind);
                    }
                    self.vals[out_ind] = input;
                }

                Operation::Output => {
                    if self.verbose >= 1 {
                        println!("Program Outputs: {}", param[0]);
                    }
                    self.outputs.push(param[0]);
                    self.pointer += 2; // Advance pointer before returning. Otherwise pointer would
                    // not change
                    return Some(param[0]);
                }

                Operation::JumpIfTrue => {
                    if param[0] != 0 {
                        self.pointer = param[1].try_into().unwrap();
                        continue; // Continue avoids advancing the pointer below
                    }
                }

                Operation::JumpIfFalse => {
                    if param[0] == 0 {
                        self.pointer = param[1].try_into().unwrap();
                        continue; // Continue avoids advancing the pointer below
                    }
                }

                Operation::LessThan => {
                    if param[0] < param[1] {
                        self.vals[out_ind] = 1;
                    } else {
                        self.vals[out_ind] = 0;
                    }
                }
                Operation::Equals => {
                    if param[0] == param[1] {
                        self.vals[out_ind] = 1;
                    } else {
                        self.vals[out_ind] = 0;
                    }
                }

                Operation::AdjustRelativeBase => {
                    self.relative_base += param[0];
                },
                Operation::Stop => {
                    let elapsed = start.elapsed();
                    if self.verbose >= STOP_VERBOSE {
                        println!("Stopping execution after {:.2?}", elapsed);
                    }
                    return None;
                },
            }
            // Unless there was a jump, advance the pointer
            self.pointer += total;
        }
    }
}

#[repr(usize)]
#[derive(Debug, TryFromPrimitive, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Sum = 1, // 1
    Product = 2, // 2
    Input = 3, // 3
    Output = 4, // 4
    JumpIfTrue = 5, // 5
    JumpIfFalse = 6, // 6
    LessThan = 7, // 7
    Equals = 8, // 8
    AdjustRelativeBase = 9, // 9
    Stop = 99,
}

impl Operation {
    fn new(code: usize) -> Option<Self> {
        Self::try_from(code).ok()
    }

    fn get_parameters(&self) -> (usize, usize, usize) {
        // Get the size of the operation in memory (how much pointer should advance)
        match self {
            Self::Sum | Self::Product | Self::Equals | Self::LessThan  => (2,1,4), // Two inputs, one output
            Self::Output => (1,0,2), // One input, no output (to memory)
            Self::Input => (0,1,2), // No inputs, one output
            Self::AdjustRelativeBase => (1,0,2), // One input, adjusts relative base
            Self::JumpIfTrue | Self::JumpIfFalse => (2,0,3), // Two inputs, no outputs
            Self::Stop => (0,0,1),
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
        p.run_until_stop();
        assert_eq!(p.vals, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
        

        // Basic sum
        let mut p = Program::new("1,0,0,0,99");
        p.run_until_stop();
        assert_eq!(p.vals, vec![2,0,0,0,99]);
        
        // Basic product
        let mut p = Program::new("2,3,0,3,99");
        p.run_until_stop();
        assert_eq!(p.vals, vec![2,3,0,6,99]);

        // Basic product
        let mut p = Program::new("2,4,4,5,99,0");
        p.run_until_stop();
        assert_eq!(p.vals, vec![2,4,4,5,99,9801]);
        
        //  Another sum
        let mut p = Program::new("1,1,1,4,99,5,6,0,99");
        p.run_until_stop();
        assert_eq!(p.vals,  vec![30,1,1,4, 2,5,6,0,99]);
    }

    #[test] 
    fn reset() {
        // Does a sum and a product
        let b = "1,9,10,3,2,3,11,0,99,30,40,50";
        let mut p = Program::new(b);
        assert_eq!(p.vals, vec![1,9,10,3,2,3,11,0,99,30,40,50]);
        p.run_until_stop();
        assert_eq!(p.vals, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);

        p.add_input(42);
        assert_eq!(p.inputs, vec![42]);
        p.reset();
        assert_eq!(p.vals, vec![1,9,10,3,2,3,11,0,99,30,40,50]);
        assert_eq!(p.inputs, vec![]);
    }

    #[test]
    fn equals() {
        // Position mode
        let p = Program::from_list(vec![3,9,8,9,10,9,4,9,99,-1,8]);
        let mut p1 = p.clone();
        p1.add_input(8);
        p1.run_until_stop();
        assert_eq!(p1.get_outputs(), vec![1]);
        
        let mut p2 = p.clone();
        p2.add_input(7);
        p2.run_until_stop();
        assert_eq!(p2.get_outputs(), vec![0]);


        // Immediate mode
        let p = Program::from_list(vec![3,3,1108,-1,8,3,4,3,99]);
        let mut p1 = p.clone();
        p1.add_input(8);
        p1.run_until_stop();
        assert_eq!(p1.get_outputs(), vec![1]);

        let mut p2 = p.clone();
        p2.add_input(7);
        p2.run_until_stop();
        assert_eq!(p2.get_outputs(), vec![0]);
        
    }

    #[test]
    fn less_than() {
        // Position mode
        let p = Program::from_list(vec![3,9,7,9,10,9,4,9,99,-1,8]);
        let mut p1 = p.clone();
        p1.add_input(7);
        p1.run_until_stop();
        assert_eq!(p1.get_outputs(), vec![1]);
        
        let mut p2 = p.clone();
        p2.add_input(8);
        p2.run_until_stop();
        assert_eq!(p2.get_outputs(), vec![0]);

        let p = Program::from_list(vec![3,3,1107,-1,8,3,4,3,99]);
        let mut p1 = p.clone();
        p1.add_input(7);
        p1.run_until_stop();
        assert_eq!(p1.get_outputs(), vec![1]);
        
        let mut p2 = p.clone();
        p2.add_input(8);
        p2.run_until_stop();
        assert_eq!(p2.get_outputs(), vec![0]);
    }

    #[test]
    fn jump() {
        let p = Program::from_list(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);
        let mut p1 = p.clone();
        p1.add_input(0);
        p1.run_until_stop();
        assert_eq!(p1.get_outputs(), vec![0]);

        p1.reset();
        p1.add_input(0);
        assert_eq!(p1.run(), Some(0));

        let mut p2 = p.clone();
        p2.add_input(7);
        p2.run_until_stop();
        assert_eq!(p2.get_outputs(), vec![1]);

        p2.reset();
        p2.add_input(7);
        assert_eq!(p2.run(), Some(1));

        let p = Program::from_list(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);
        let mut p1 = p.clone();
        p1.add_input(0);
        p1.run_until_stop();
        assert_eq!(p1.get_outputs(), vec![0]);

        let mut p2 = p.clone();
        p2.add_input(7);
        p2.run_until_stop();
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
        p1.run_until_stop();
        assert_eq!(p1.get_outputs(), vec![999]);

        let mut p1 = p.clone();
        p1.add_input(9);
        p1.run_until_stop();
        assert_eq!(p1.get_outputs(), vec![1001]);

        let mut p1 = p.clone();
        p1.add_input(8);
        p1.run_until_stop();
        assert_eq!(p1.get_outputs(), vec![1000]);
    }

    #[test]
    fn extra_memory() {
        // This should produce a copy of itself as output
        let mut p = Program::from_list(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
        p.set_verbose(2);
        p.run_until_stop();
        assert_eq!(p.get_outputs(), vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
    }

    #[test]
    fn large_numbers() {
        let mut p = Program::from_list(vec![104,1125899906842624,99]);
        assert_eq!(p.run(), Some(1125899906842624));

        // Multiplies 34915192 by itself and outputs the result
        let mut p = Program::from_list(vec![1102,34915192,34915192,7,4,7,99,0]);
        assert_eq!(p.run(), Some(34915192 * 34915192));
    }

    #[test]
    fn inputs() {
        let mut p = Program::from_list(vec![3,5,3,6,99,0,0,0,0,0,0,0,0]);
        p.add_input(42); // Input 42, should go to 5
        p.add_input(1729); // Input 1729, should go to 6
        p.set_verbose(0);
        p.run();
        assert_eq!(p.vals[5], 42);
        assert_eq!(p.vals[6], 1729);
    }
}
