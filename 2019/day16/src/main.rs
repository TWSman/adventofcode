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
    vec: Vec<i64>,
    len: usize, // Length of the vector
    reversed: bool,
    start_i: usize,
}

impl FFT {
    fn new(ln: &str) -> Self {
        let vec: Vec<i64> = ln
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        let n = vec.len();
        Self {
            vec,
            len: n,
            start_i: 0,
            reversed: false,
        }
    }

    fn get_string(&self, len: i64) -> String {
        if len > 0 {
            self.vec
                .iter()
                .take(len as usize)
                .map(|i| i.to_string())
                .collect()
        } else {
            // Otherwise take the last -len digits
            self.vec
                .iter()
                .rev()
                .take((-len) as usize)
                .rev()
                .map(|i| i.to_string())
                .collect()
        }
    }

    fn get_sub(&self, start_index: usize) -> Self {
        // Get subvector starting from start_index, i.e. skip the first start_index elements
        let vec: Vec<i64> = self
            .vec
            .iter()
            .cycle()
            .skip(start_index)
            .take(self.len - start_index)
            .cloned()
            .collect();
        assert_eq!(vec.len(), self.len - start_index);
        Self {
            vec,
            len: self.len - start_index,
            start_i: start_index,
            reversed: self.reversed,
        }
    }

    fn reverse(&self) -> Self {
        let vec: Vec<i64> = self.vec.iter().rev().cloned().collect();
        Self {
            vec,
            len: self.len,
            start_i: self.start_i,
            reversed: !self.reversed,
        }
    }

    fn repeat_proper(&self, count: usize) -> Self {
        // Repeat vector 'count' times
        let vec = self.vec.repeat(count);
        Self {
            vec,
            len: self.len * count,
            start_i: self.start_i,
            reversed: self.reversed,
        }
    }

    fn transform(&mut self) {
        let si = self.start_i;
        let output = (1..=self.vec.len())
            .map(|i| {
                self.vec
                    .iter()
                    .enumerate()
                    .filter(|(j, _v)| (j + si + 1) / (i + si) % 2 == 1)
                    .map(|(j, v)| {
                        let b = (j + si + 1) / (i + si);
                        if (b / 2).is_multiple_of(2) { *v } else { -v }
                    })
                    .sum::<i64>()
                    .abs()
                    % 10
            })
            .collect::<Vec<i64>>();
        self.vec = output;
    }

    fn transform_part2(&mut self) {
        // Transform vector assuming that we are only interested in the part starting from offset,
        // which is more than half of the total length, so that we can use the fact that the pattern is 0s followed by 1s
        // Transformation matrix will look something like this:
        //
        // 1  0 -1  0  1  0 -1  0  1  0
        // 0  1  1  0  0 -1 -1  0  0  1
        // 0  0  1  1  1  0  0  0 -1 -1
        // 0  0  0  1  1  1  1  0  0  0
        // 0  0  0  0  1  1  1  1  1  0
        // 0  0  0  0  0  1  1  1  1  1
        // 0  0  0  0  0  0  1  1  1  1
        // 0  0  0  0  0  0  0  1  1  1
        // 0  0  0  0  0  0  0  0  1  1
        // 0  0  0  0  0  0  0  0  0  1
        //
        // Where the bottom right quarter is an upper triangular matrix.
        // 1  1  1  1  1
        // 0  1  1  1  1
        // 0  0  1  1  1
        // 0  0  0  1  1
        // 0  0  0  0  1
        //
        let mut output_vec = Vec::new();
        let mut sum = 0;
        // Assumes that the vector is reversed, this way we can easily compute the cumulative sum from the end, which is what we need for the transformation
        assert!(
            self.reversed,
            "Vector should be reversed for transform_part2"
        );
        for v in &self.vec {
            // from N to 1
            sum += v;
            output_vec.push(sum.abs() % 10);
        }
        self.vec = output_vec;
    }
}

fn get_part1(fft: &FFT) -> String {
    let mut fft = fft.clone();
    for _ in 0..100 {
        fft.transform();
    }
    fft.get_string(8)
}

fn get_part2(fft: &FFT) -> String {
    let offset = fft.get_string(7).parse::<usize>().unwrap();

    let mut fft_sub = fft.repeat_proper(10_000).get_sub(offset).reverse();
    for _i in 0..100 {
        fft_sub.transform_part2();
    }
    // The result is now at the end of the vector and reversed,
    // so we need to reverse it back
    fft_sub.get_string(-8).chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "12345678";
        let mut fft = FFT::new(a);

        assert_eq!(fft.get_string(-4), "5678");
        fft.transform();
        assert_eq!(fft.get_string(8), "48226158");
        assert_eq!(fft.get_string(-4), "6158");

        fft.transform();
        assert_eq!(fft.get_string(8), "34040438");

        fft.transform();
        assert_eq!(fft.get_string(8), "03415518");

        fft.transform();
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
        dbg!(a.len());
        let fft = FFT::new(a);
        assert_eq!(get_part2(&fft), "84462026");
    }

    #[test]
    fn sub() {
        let offset = 3;
        let a = "123456789";
        let fft = FFT::new(a);
        assert_eq!(fft.get_sub(offset).get_string(3), "456"); // Should have skipped 3
        // first, i.e. 123
    }
}
