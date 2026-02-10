use clap::Parser;
use std::collections::BTreeSet;
use std::fs;
use std::time::Instant;
use ndarray::prelude::*;
use ndarray_linalg::Solve;

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
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (String, String) {
    let fft = FFT::new(cont);
    let part1 = get_part1(&fft);
    let part2 = get_part2(&fft);
    (part1, part2)
}

#[derive(Clone, Debug)]
struct FFT {
    vec: Vec<i64>
}

impl FFT {
    fn new(ln: &str) -> Self {
        let vec = ln.trim().chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
        Self { vec }
    }

    fn get_string(&self, len: usize) -> String {
        self.vec.iter().take(len).map(|i| i.to_string()).collect()
    }

    fn repeat(&self, count: usize) -> Self {
        // Repeat vector 'count' times
        let vec = self.vec.repeat(count);
        Self {
            vec
        }
    }

    fn transform(&self) -> Self {
        let base_pattern = [0, 1, 0, -1];
        let mut output = Vec::new();
        for i in 1..=self.vec.len() {
            let mut sum = 0;
            //println!("i: {}", i);
            for (j,v) in self.vec.iter().enumerate() {
                let seg = ((j + 1) / i) % 4;
                //println!("     j: {}, seg: {}, multi: {}", j, seg, base_pattern[seg]);
                sum += v * base_pattern[seg];
            }
            output.push(sum.abs() % 10);
        }
        Self {
            vec: output
        }
    }

    fn get_part2_answer(&self) -> String {
        dbg!(&self.vec[..7]);
        let offset = self.vec[..7].iter().map(|i| char::from_digit(*i as u32, 10).unwrap()).collect::<String>().parse::<usize>().unwrap();
        dbg!(&offset);
        self.vec.iter().skip(offset).take(8).map(|i| i.to_string()).collect()
    }
}


fn get_part1(fft: &FFT) -> String {
    let mut fft = fft.clone();
    for i in 0..100 {
        fft = fft.transform();
        println!("After {} phases: {}", i + 1, fft.get_string(8));
    }
    fft.get_string(8)
}

fn get_part2(fft: &FFT) -> String {
    let fft = fft.repeat(10000);
    fft.get_part2_answer()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "12345678";
        let mut fft = FFT::new(a);
        fft = fft.transform();
        assert_eq!(fft.get_string(8), "48226158");

        fft = fft.transform();
        assert_eq!(fft.get_string(8), "34040438");
        
        fft = fft.transform();
        assert_eq!(fft.get_string(8), "03415518");

        fft = fft.transform();
        assert_eq!(fft.get_string(8), "01029498");

        let a = "80871224585914546619083218645595";

        let fft = FFT::new(a);
        assert_eq!(get_part1(&fft), "24176176");
        
        let fft = FFT::new("19617804207202209144916044189917");
        assert_eq!(get_part1(&fft), "73745418");
        
        let fft = FFT::new("69317163492948606335995924319873");
        assert_eq!(get_part1(&fft), "52432133");
    }

    #[test]
    fn part2() {
        let a = "03036732577212944063491565474664";
        let fft = FFT::new(a);
        assert_eq!(get_part2(&fft), "84462026");
    }
}
