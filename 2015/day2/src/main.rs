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

fn read_contents(cont: &str) -> (i32, i32) {
    let presents = cont.lines().map(Present::new).collect::<Vec<_>>();
    let part1 = get_part1(&presents);
    let part2 = get_part2(&presents);
    (part1, part2)
}

struct Present {
    length: i32,
    width: i32,
    height: i32,
}

impl Present {
    fn new(ln: &str) -> Self {
        let parts = ln
            .split('x')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        Self {
            length: parts[0],
            width: parts[1],
            height: parts[2],
        }
    }

    fn volume(&self) -> i32 {
        self.length * self.height * self.width
    }

    fn part1(&self) -> i32 {
        // Part1 is the surface area of the present plus the area of the smallest side
        let area_a = self.length * self.width;
        let area_b = self.width * self.height;
        let area_c = self.height * self.length;
        // Surface area includes 2 faces of each type, so we multiply by 2 and then add the smallest area
        2 * (area_a + area_b + area_c) + area_a.min(area_b).min(area_c)
    }

    fn part2(&self) -> i32 {
        // Part2 is the smallest perimeter of any one face plus the cubic volume of the present
        let half_perimeter_a = self.length + self.width;
        let half_perimeter_b = self.length + self.height;
        let half_perimeter_c = self.width + self.height;
        2 * half_perimeter_a.min(half_perimeter_b).min(half_perimeter_c) + self.volume()
    }
}

fn get_part1(vec: &[Present]) -> i32 {
    vec.iter().map(|r| r.part1()).sum()
}

fn get_part2(vec: &[Present]) -> i32 {
    vec.iter().map(|r| r.part2()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(read_contents("2x3x4").0, 58);
        assert_eq!(read_contents("1x1x10").0, 43);
    }

    #[test]
    fn part2() {
        assert_eq!(read_contents("2x3x4").1, 34);
        assert_eq!(read_contents("1x1x10").1, 14);
    }
}
