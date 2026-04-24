use clap::Parser;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let start = Instant::now();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i64, i64) {
    let start_values = cont
        .lines()
        .map(|l| l.split_whitespace().last().unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let generators = vec![
        Generator::new(start_values[0], 16807, 4), // Generator A
        Generator::new(start_values[1], 48271, 8), // Generator B
    ];
    let part1 = get_part1(&generators);
    let part2 = get_part2(&generators);
    (part1, part2)
}

#[derive(Debug, Clone, Copy)]
struct Generator {
    previous: i64,
    seed: i64,
    check: i64,
}

impl Generator {
    fn new(start_value: i64, seed: i64, check: i64) -> Self {
        Self {
            previous: start_value,
            seed,
            check,
        }
    }

    fn next(&mut self) -> i64 {
        let val = self.previous * self.seed % 2147483647;
        self.previous = val;
        val
    }

    fn next_string(&mut self) -> String {
        let val = self.next();
        format!("{val:016b}") // Return the binary representation of the value
    }

    fn next_string_part2(&mut self) -> String {
        let mut val = self.next();
        while val % self.check != 0 {
            val = self.next();
        }
        format!("{val:016b}") // Return the binary representation of the value
    }
}

fn get_part1(generators: &[Generator]) -> i64 {
    let mut gen_a = generators[0];
    let mut gen_b = generators[1];
    let mut sum = 0;
    for _ in 0..40_000_000 {
        let a = gen_a.next_string();
        let b = gen_b.next_string();
        if a[a.len() - 16..] == b[b.len() - 16..] {
            sum += 1;
        }
    }
    sum
}

fn get_part2(generators: &[Generator]) -> i64 {
    let mut gen_a = generators[0];
    let mut gen_b = generators[1];
    let mut sum = 0;
    for _ in 0..5_000_000 {
        let a = gen_a.next_string_part2();
        let b = gen_b.next_string_part2();
        if a[16.max(a.len()) - 16..] == b[16.max(b.len()) - 16..] {
            sum += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Generator A starts with 65
Generator B starts with 8921";
        let mut generator = Generator::new(65, 16807, 4);
        assert_eq!(generator.next(), 1092455);
        assert_eq!(generator.next(), 1181022009);
        assert_eq!(generator.next(), 245556042);
        assert_eq!(generator.next(), 1744312007);
        assert_eq!(generator.next(), 1352636452);

        let mut generator = Generator::new(8921, 48271, 8);
        assert_eq!(generator.next_string(), "11001101010101101001100110111");
        assert_eq!(generator.next_string(), "1001001100010001000010110001000");
        assert_eq!(read_contents(a).0, 588);
    }

    #[test]
    fn part2() {
        let a = "Generator A starts with 65
Generator B starts with 8921";
        let mut generator = Generator::new(65, 16807, 4);
        assert_eq!(
            generator.next_string_part2(),
            "1010000100111111001100000100100"
        );
        assert_eq!(
            generator.next_string_part2(),
            "1110110101111001011111010110000"
        );
        assert_eq!(read_contents(a).1, 309);
    }
}
