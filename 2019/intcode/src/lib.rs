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
    step_counter: usize,
}

const DEBUG_VERBOSE: usize = 3;
const OP_VERBOSE: usize = 2;
const STOP_VERBOSE: usize = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramState {
    Running,
    Output(i64),
    WaitingForInput,
    Stopped,
    Unknown,
}

impl Program {
    pub fn from_list(initial_state: Vec<i64>) -> Self {
        let mut stat = initial_state.clone();
        stat.resize(4096, 0);
        Program {
            vals: stat.clone(),
            pointer: 0,
            relative_base: 0,
            input_pointer: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
            verbose: 1,
            initial_state: stat,
            step_counter: 0,
        }
    }

    pub fn new(ln: &str) -> Self {
        let vals = ln.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        Self::from_list(vals)
    }

    pub fn add_input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    pub fn get_input_pointer(&self) -> usize {
        self.input_pointer
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

    pub fn get_inputs(&self) -> Vec<i64> {
        self.inputs.clone()
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
            let res = self.run(None);
            if res == ProgramState::Stopped {
                break;
            }
        }
    }

    pub fn run(&mut self, steps: Option<usize>) -> ProgramState {
        let start = Instant::now();
        if self.verbose >= 1 && self.step_counter == 0 {
            println!("\nStarting execution");
        }
        let mut step_counter = 0;
        loop {
            step_counter += 1;
            if let Some(st) = steps {
                if step_counter > st {
                    return ProgramState::Running
                }
            }
            self.step_counter += 1;
            if self.verbose >= OP_VERBOSE {
                println!("\n-- Step {} --", self.step_counter);
            }
            let opcode = self.vals[self.pointer];
            let op = Operation::new((opcode % 100).try_into().unwrap()).unwrap();

            let mods = vec![(self.vals[self.pointer] / 100) % 10,
                (self.vals[self.pointer] / 1000) % 10,
                (self.vals[self.pointer] / 10000) % 10,
                (self.vals[self.pointer] / 100000) % 10,
            ];



            if self.verbose >= DEBUG_VERBOSE {
                println!();
                dbg!(self.pointer, op, opcode, &mods);
            }

            if self.verbose >= OP_VERBOSE {
                println!("Executing operation: {:?} at pointer {}", op, self.pointer);
            }

            let (inputs, outputs, total) = op.get_parameters();
            assert!(outputs <= 1); // This implementation only supports one output parameter
            let param: Vec<i64> = (self.pointer+1..self.pointer+1+inputs).enumerate().map(|(i, pi)| {
                if mods[i] == 0 {
                    self.vals[self.vals[pi as usize] as usize]
                } else if mods[i] == 1 {
                    self.vals[pi as usize]
                } else {
                    // Relative mode
                    self.vals[(self.relative_base + self.vals[pi as usize]) as usize]
                }
            }).collect();

            let output_mode = mods[inputs];
            let out_ind: usize = if outputs == 0 {
                0
            } else if  output_mode == 0 {
                self.vals[self.pointer + 1 + inputs].try_into().expect("Output index must be positive")
            } else if output_mode == 2 {
                let tmp = self.vals[self.pointer + 1 + inputs];
                (self.relative_base + tmp).try_into().expect("Output index must be positive")
            } else {
                panic!("Invalid output mode");
            };

            if self.verbose >= DEBUG_VERBOSE {
                dbg!(&param);
                dbg!(&out_ind);
            }

            match op {
                Operation::Sum => {
                    if self.verbose >= OP_VERBOSE {
                        println!("Adding {} and {} = {} into *{}", param[0], param[1], param[0] + param[1], out_ind);
                    }
                    self.vals[out_ind] = param[0] + param[1];
                },
                Operation::Product => {
                    if self.verbose >= OP_VERBOSE {
                        println!("Multiplying {} and {} = {} into *{}", param[0], param[1], param[0] * param[1], out_ind);
                    }
                    self.vals[out_ind] = param[0] * param[1];
                },

                Operation::Input => {
                    if self.input_pointer >= self.inputs.len() {
                        if self.verbose >= STOP_VERBOSE {
                            println!("No input available, pausing execution");
                        }
                        return ProgramState::WaitingForInput;
                    }
                    let input = self.inputs[self.input_pointer];
                    self.input_pointer += 1;
                    if self.verbose >= STOP_VERBOSE {
                        println!("Read input: {} to *{}", input, out_ind);
                    }
                    self.vals[out_ind] = input;
                }

                Operation::Output => {
                    if self.verbose >= STOP_VERBOSE {
                        println!("Program Outputs: {}", param[0]);
                    }
                    self.outputs.push(param[0]);
                    self.pointer += 2; // Advance pointer before returning. Otherwise pointer would
                    // not change
                    return ProgramState::Output(param[0]);
                }

                Operation::JumpIfTrue => {
                    if self.verbose >= OP_VERBOSE {
                        println!("Checking if {} != 0", param[0]);
                    }
                    if param[0] != 0 {
                        if self.verbose >= OP_VERBOSE {
                            println!("Jumping to {}", param[1]);
                        }
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
                    if self.verbose >= OP_VERBOSE {
                        println!("Checking if {} < {}", param[0], param[1]);
                    }
                    if param[0] < param[1] {
                        self.vals[out_ind] = 1;
                    } else {
                        self.vals[out_ind] = 0;
                    }
                }
                Operation::Equals => {
                    if self.verbose >= OP_VERBOSE {
                        println!("Checking if {} == {}", param[0], param[1]);
                    }
                    if param[0] == param[1] {
                        self.vals[out_ind] = 1;
                    } else {
                        self.vals[out_ind] = 0;
                    }
                }

                Operation::AdjustRelativeBase => {
                    if self.verbose >= OP_VERBOSE {
                        println!("Adjusting relative base from {} by {}", self.relative_base, param[0]);
                    }
                    self.relative_base += param[0];
                },
                Operation::Stop => {
                    let elapsed = start.elapsed();
                    if self.verbose >= STOP_VERBOSE {
                        println!("Stopping execution after {:.2?} and {} steps", elapsed, self.step_counter);
                    }
                    return ProgramState::Stopped;
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
        assert_eq!(p.vals[..5], vec![1,0,0,3,99]);

        // Does a sum and a product
        let b = "1,9,10,3,2,3,11,0,99,30,40,50";
        let mut p = Program::new(b);
        assert_eq!(p.vals[..12], vec![1,9,10,3,2,3,11,0,99,30,40,50]);
        p.run_until_stop();
        assert_eq!(p.vals[..12], vec![3500,9,10,70,2,3,11,0,99,30,40,50]);

        // Basic sum
        let mut p = Program::new("1,0,0,0,99");
        p.run_until_stop();
        assert_eq!(p.vals[..5], vec![2,0,0,0,99]);
        
        // Basic product
        let mut p = Program::new("2,3,0,3,99");
        p.run_until_stop();
        assert_eq!(p.vals[..5], vec![2,3,0,6,99]);

        // Basic product
        let mut p = Program::new("2,4,4,5,99,0");
        p.run_until_stop();
        assert_eq!(p.vals[..6], vec![2,4,4,5,99,9801]);
        
        //  Another sum
        let mut p = Program::new("1,1,1,4,99,5,6,0,99");
        p.run_until_stop();
        assert_eq!(p.vals[..9],  vec![30,1,1,4, 2,5,6,0,99]);
    }

    #[test] 
    fn reset() {
        // Does a sum and a product
        let b = "1,9,10,3,2,3,11,0,99,30,40,50";
        let mut p = Program::new(b);
        assert_eq!(p.vals[..12], vec![1,9,10,3,2,3,11,0,99,30,40,50]);
        p.run_until_stop();
        assert_eq!(p.vals[..12], vec![3500,9,10,70,2,3,11,0,99,30,40,50]);

        p.add_input(42);
        assert_eq!(p.inputs, vec![42]);
        p.reset();
        assert_eq!(p.vals[..12], vec![1,9,10,3,2,3,11,0,99,30,40,50]);
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
        assert_eq!(p1.run(None), ProgramState::Output(0));

        let mut p2 = p.clone();
        p2.add_input(7);
        p2.run_until_stop();
        assert_eq!(p2.get_outputs(), vec![1]);

        p2.reset();
        p2.add_input(7);
        assert_eq!(p2.run(None), ProgramState::Output(1));

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
        assert_eq!(p.run(None), ProgramState::Output(1125899906842624));

        // Multiplies 34915192 by itself and outputs the result
        let mut p = Program::from_list(vec![1102,34915192,34915192,7,4,7,99,0]);
        assert_eq!(p.run(None), ProgramState::Output(34915192 * 34915192));
    }

    #[test]
    fn inputs() {
        let mut p = Program::from_list(vec![3,5,3,6,99,0,0,0,0,0,0,0,0]);
        p.add_input(42); // Input 42, should go to 5
        p.add_input(1729); // Input 1729, should go to 6
        p.run(None);
        assert_eq!(p.vals[5], 42);
        assert_eq!(p.vals[6], 1729);
    }

    #[test]
    fn relative_output() {
        // Checks that having a relative mode output works correctly
        // i.e. Output index is given in relative mode
        let mut p = Program::from_list(vec![
            1101,0,3,1000,
            109,994,
            209,6,
            9,1000,
            203,0,
            99]);

        p.set_verbose(2);
        p.add_input(1);

        // 1101,0,3,1000,
        p.run(Some(1));
        assert_eq!(p.vals[1000], 3);
        assert_eq!(p.pointer, 4);

        // 109,994, Adjust relative base, absolute, 994
        p.run(Some(1));
        assert_eq!(p.relative_base, 994);
        assert_eq!(p.pointer, 6);

        // 209,6, Adjust relative base, relative, 6 (i.e. at 1000)
        p.run(Some(1));
        assert_eq!(p.relative_base, 997);
        assert_eq!(p.pointer, 8);

        //    209,6, Adjust relative base, relative, 3, which is *1000, which is 3
        p.run(Some(1));
        assert_eq!(p.relative_base, 1000);
        assert_eq!(p.pointer, 10);
        //    203,0, Read Input, relative mode to *1000
        p.run(Some(1));
        assert_eq!(p.inputs, vec![1]);
        assert_eq!(p.pointer, 12);
        assert_eq!(p.vals[1000], 1);

        // Next should be stop
        assert_eq!(p.run(None), ProgramState::Stopped);
        // Pointer shouldn't have moved
        assert_eq!(p.pointer, 12);
    }


}
