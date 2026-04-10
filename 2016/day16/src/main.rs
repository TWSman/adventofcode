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
    let res = read_contents(&contents, 272);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str, target_len: usize) -> (String, String) {
    let input = cont
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
    let part1 = get_part1(&input, target_len);
    let part2 = get_part1(&input, 35651584);
    (part1, part2)
}

fn process(data: &[u8]) -> Vec<u8> {
    let mut new_data = data.to_owned();
    new_data.push(0);
    for c in data.iter().map(|i| 1 - i).rev() {
        new_data.push(c);
    }
    new_data
}

fn get_part1(input: &[u8], target_len: usize) -> String {
    let mut data = input.to_owned();
    while data.len() <= target_len {
        data = process(&data);
    }
    if target_len < 1000 {
        println!(
            "Data is {}",
            data.iter().map(|i| i.to_string()).collect::<String>()
        );
    }
    data = data[..target_len].to_owned();
    let checksum = checksum(&data, data.len());
    checksum.iter().map(|i| i.to_string()).collect::<String>()
}

fn checksum(input: &[u8], count: usize) -> Vec<u8> {
    let mut output = Vec::new();
    for i in (0..count).step_by(2) {
        if input[i] == input[i + 1] {
            output.push(1);
        } else {
            output.push(0);
        }
    }
    assert_eq!(output.len(), count / 2);
    if output.len() % 2 == 0 {
        checksum(&output, output.len())
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(process(&vec![1]), vec![1, 0, 0]);
        assert_eq!(process(&vec![0]), vec![0, 0, 1]);
        assert_eq!(
            process(&vec![1, 1, 1, 1, 1]),
            vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            process(&vec![1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0]),
            vec![
                1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0,
            ]
        );

        let a = "10000";
        assert_eq!(read_contents(a, 20).0, "01100");
    }

    #[test]
    fn get_checksum() {
        assert_eq!(
            checksum(&vec![1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0], 12),
            vec![1, 0, 0]
        );
    }
}
