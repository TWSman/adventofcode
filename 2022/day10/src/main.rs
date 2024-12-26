use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {res}");

}

enum Op {
    Noop,
    Addx(i64),
}

impl Op {
    fn new(ln: &str) -> Self {
        if ln.starts_with("noop") {
            Op::Noop
        } else {
            Op::Addx(ln[5..].parse::<i64>().unwrap())
        }
    }
}
struct Computer {
    cycle: i64,
    register: i64,
    strength_sum: i64,
    lit_pixels: Vec<char>,
}
 
impl Computer {
    fn new() -> Self {
        Self { cycle:0, register: 1, strength_sum: 0, lit_pixels: Vec::new()}
    }

    fn push_cycle(&mut self) {
        // During cycle 1, we draw to column 0
        let col = self.cycle % 40;
        self.cycle += 1;
        if (self.cycle + 20) % 40 == 0 {
            self.strength_sum += self.cycle * self.register;
        }
        if i64::abs(col - self.register) <= 1 {
            self.lit_pixels.push('#');
        } else {
            self.lit_pixels.push('.');
        }
    }

    fn print_results(&self) {
        for chunk in self.lit_pixels.chunks(40) {
            println!("{}", chunk.iter().collect::<String>());
        }
    }

    fn process(&mut self, op: &Op) {
        match op {
            Op::Noop => {
                self.push_cycle();
            },
            Op::Addx(val) => {
                self.push_cycle();
                self.push_cycle();
                self.register += val;
            }
        }
    }

}

fn read_contents(cont: &str) -> i64 {
    let operations = cont.lines().map(|ln| {
        Op::new(ln)
    }).collect::<Vec<Op>>();
    let mut comp = Computer::new();
    for op in operations {
        comp.process(&op);
    }
    comp.print_results();
    comp.strength_sum

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {

        let a = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(read_contents(&a), 13140);
        //assert_eq!(read_contents(&a).1, 1);

    }

}
