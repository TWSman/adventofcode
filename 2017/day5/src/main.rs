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
    let list = cont
        .lines()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    dbg!(&list);
    let part1 = get_answer(&list, false);
    let part2 = get_answer(&list, true);
    (part1, part2)
}

fn get_answer(list: &[i32], part2: bool) -> i64 {
    let mut vec: Vec<i32> = list.to_vec();
    let mut ind: i32 = 0;
    let n = vec.len() as i32;
    let mut steps = 0;
    loop {
        if ind < 0 {
            return steps;
        }
        if ind >= n {
            return steps;
        }
        let jump = *vec.get(ind as usize).unwrap();
        if part2 && jump >= 3 {
            *vec.get_mut(ind as usize).unwrap() -= 1;
        } else {
            *vec.get_mut(ind as usize).unwrap() += 1;
        }
        steps += 1;
        ind += jump;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts() {
        let a = "0
3
0
1
-3";
        assert_eq!(read_contents(&a).0, 5);
        assert_eq!(read_contents(&a).1, 10);
    }
}
