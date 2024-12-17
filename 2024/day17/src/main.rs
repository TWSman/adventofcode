use clap::Parser;
use regex::Regex;
use std::fs;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents, true);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

#[derive(Debug)]
struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    instructions: Vec<(Instr, Operand)>,
}

impl Computer {
    fn new() -> Self {
        Self {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            instructions: Vec::new(),
        }
    }

    fn get_operand(&self, operand: Operand) -> i64 {
        match operand {
            Operand::Literal(val) => val,
            Operand::RegA => self.reg_a,
            Operand::RegB => self.reg_b,
            Operand::RegC => self.reg_c,
        }
    }


    fn read_instructions(&mut self, input: &str) {
        let insts: Vec<u64> = input.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
        assert!(insts.len() % 2 == 0);
        for i in 0..(insts.len() / 2) {
            let inst = Instr::new(insts[i * 2]);
            let opcode = if inst.expects_literal() {
                Operand::Literal(insts[i * 2 + 1] as i64)
            }
            else {
                match insts[i * 2 + 1] {
                    4 => Operand::RegA,
                    5 => Operand::RegB,
                    6 => Operand::RegC,
                    val if val < 4 => Operand::Literal(val as i64),
                    _ => panic!("Unknown register")
                }
            };
            self.instructions.push((inst,opcode));
        }
    }

    fn get_part2(&mut self) -> i64 {
        let expected_output_list = self.get_instruction_codes();
        let mut i = 0;
        //
        // Starting with 0 and incrementing by 1 passes the test
        // But this is way too slow to solve the main input
        //
        // Solving part2:
        // solutions that matched until length 15 had the form
        // 169_097_640_975 + x*2**40 + a * 2**18
        // where a was in [1,7,15,31]
        //
        // let mut i = 169_097_640_975 + 7 * i64::pow(2,18);

        loop {
            i += 1;
            // i += i64::pow(2,40); // 
            // Reset state
            self.reg_a = i;
            self.reg_b = 0;
            self.reg_c = 0;
            let output = self.run_instructions(true);
            if output == expected_output_list {
                return i;
            }

            if i > 999_999_999_999_999 {
                return 0;
            }
        }
    }

    fn get_instruction_codes(&self) -> Vec<i64> {
        // Convert the list of instructions to a list of opcodes
        let mut output = Vec::new();
        for (inst, op) in &self.instructions {
            output.push(inst.opcode());
            output.push(op.opcode());
        }
        output
    }

    fn run_instructions(&mut self, part2: bool) -> Vec<i64> {
        // Run the list of instructions until the program halts
        let mut output = Vec::new();
        let mut instruction_pointer = 0;
        let expected_output_list = self.get_instruction_codes();
        let mut expected_output: Option<&i64>;
        loop {
            let (instr, operand) = match &self.instructions.get(instruction_pointer) {
                None => break,
                Some(val) => val,
            };
            let op = self.get_operand(*operand);
            match instr {
                Instr::ADV => {
                    // Divide A by 2^op
                    // Store result to A
                    let num = self.reg_a;
                    let den = i64::pow(2, op as u32);
                    self.reg_a = num / den;
                },
                Instr::BXL => {
                    // Take bitwise XOR of B and op,
                    // Store result to B
                    self.reg_b = self.reg_b ^ op;
                },
                Instr::BST => {
                    // TAke module 8 of op
                    // Store to B
                    self.reg_b = op % 8;
                },
                Instr::BXC => {
                    // Take bitwise XOR of B and C,
                    // store to B
                    self.reg_b = self.reg_b ^ self.reg_c;
                }
                Instr::OUT => {
                    // Take module 8 of op
                    // Push to output
                    if part2 {
                        // If we are solving part2
                        // Exit as soon as we get a mismatch between expected
                        // and actual output
                        expected_output = Some(&expected_output_list[output.len()]);
                        if *expected_output.unwrap() != (op % 8) {
                            return output;
                        }
                    }
                    output.push(op % 8);
                },
                Instr::JNZ => {
                    // If A is not 0 jump to op
                    // i.e. set instruction pointer to op
                    if self.reg_a != 0 {
                        instruction_pointer = op as usize;
                        continue;
                    }
                }
                Instr::BDV => {
                    // Divide A by 2^op
                    // Store result to B
                    let num = self.reg_a;
                    let den = i64::pow(2, op as u32);
                    self.reg_b = num / den;
                },
                Instr::CDV => {
                    // Divide A by 2^op
                    // Store result to C
                    let num = self.reg_a;
                    let den = i64::pow(2, op as u32);
                    self.reg_c = num / den;
                },
            }
            instruction_pointer += 1;
        }
        output
    }
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Literal(i64),
    RegA,
    RegB,
    RegC,
}

impl Operand {
    fn opcode(&self) -> i64 {
        match self {
            Operand::Literal(val) => *val,
            Operand::RegA => 4,
            Operand::RegB => 5,
            Operand::RegC => 6,
        }
    }
}


impl Instr { 
    fn new(i: u64) -> Self {
        match i {
            0 => Instr::ADV,
            1 => Instr::BXL,
            2 => Instr::BST,
            3 => Instr::JNZ,
            4 => Instr::BXC,
            5 => Instr::OUT,
            6 => Instr::BDV,
            7 => Instr::CDV,
            _ => panic!("Unknown instruction")
        }
    }

    fn opcode(&self) -> i64 {
        match self {
            Instr::ADV => 0,
            Instr::BXL => 1,
            Instr::BST => 2,
            Instr::JNZ => 3,
            Instr::BXC => 4,
            Instr::OUT => 5,
            Instr::BDV => 6,
            Instr::CDV => 7,
        }
    }

    fn expects_literal(&self) -> bool {
        match self {
            Instr::BXL => true,
            Instr::JNZ => true,
            _ => false
        }
    }
}

fn read_contents(cont: &str, dopart2: bool) -> (String, i64) {
    let re = Regex::new(r"Register ([ABC]): ([0-9]*)").unwrap();
    let re2 = Regex::new(r"Program: (.*)").unwrap();
    let mut computer = Computer::new();
    for ln in cont.lines() {
        match re.captures(ln) {
            Some (val) => {
                if val[1] == *"A" {
                    computer.reg_a = val[2].parse::<i64>().unwrap();
                }
                if val[1] == *"B" {
                    computer.reg_b = val[2].parse::<i64>().unwrap();
                }
                if val[1] == *"C" {
                    computer.reg_c = val[2].parse::<i64>().unwrap();
                }
                continue;
            } ,
            None => {},
        }
        match re2.captures(ln) {
            Some(val) => {
                    computer.read_instructions(&val[1]);
                },
            None => {continue;},
        }
    }
    let output = computer.run_instructions(false);

    let part1= output.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(",");
    let part2 = if dopart2 {
        computer.get_part2()
    } else {
        0
    };
    (part1, part2)
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part1() {
        // example data for part1
        let  a = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(read_contents(&a, false).0, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2() {
        // example data for part2  (slightly different from part1)
        let  a = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(read_contents(&a, true).1, 117440);
    }

    #[test]
    fn compute1() {
        let mut computer = Computer::new();
        computer.reg_c = 9;
        computer.instructions.push((Instr::BST, Operand::RegC));
        let _ = computer.run_instructions(false);
        assert_eq!(computer.reg_b, 1);
    }

    #[test]
    fn compute2() {
        let mut computer = Computer::new();
        computer.reg_a = 10;
        computer.read_instructions("5,0,5,1,5,4");
        assert_eq!(computer.get_instruction_codes(), [5,0,5,1,5,4]);
        let output = computer.run_instructions(false);
        assert_eq!(output, [0,1,2]);
    }

    #[test]
    fn compute3() {
        let mut computer = Computer::new();
        computer.reg_a = 2024;
        computer.read_instructions("0,1,5,4,3,0");
        assert_eq!(computer.get_instruction_codes(), [0,1,5,4,3,0]);
        let output = computer.run_instructions(false);
        assert_eq!(output, [4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(computer.reg_a, 0);
    }

    #[test]
    fn compute4() {
        let mut computer = Computer::new();
        computer.reg_b = 29;
        computer.read_instructions("1,7");
        let _ = computer.run_instructions(false);
        assert_eq!(computer.reg_b, 26);
    }

    #[test]
    fn compute5() {
        let mut computer = Computer::new();
        computer.reg_b = 2024;
        computer.reg_c = 43690;
        computer.read_instructions("4,0");
        let _ = computer.run_instructions(false);
        assert_eq!(computer.reg_b, 44354);
    }
}
