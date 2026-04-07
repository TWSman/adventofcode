use clap::Parser;
use std::collections::BTreeMap;
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

fn read_contents(cont: &str) -> (String, String) {
    let n = cont.lines().next().unwrap().len();
    let mut counters: Vec<BTreeMap<char, i32>> = vec![BTreeMap::new(); n];

    for line in cont.lines() {
        for (i, counter) in counters.iter_mut().enumerate() {
            let c = line.chars().nth(i).unwrap();
            if !counter.contains_key(&c) {
                counter.insert(c, 1);
            } else {
                *counter.get_mut(&c).unwrap() += 1;
            }
        }
    }
    let mut part1 = String::new();
    let mut part2 = String::new();
    for counter in counters.iter() {
        let mut counts = counter
            .iter()
            .map(|(c, count)| (*c, *count))
            .collect::<Vec<_>>();
        counts.sort_by_key(|(c, count)| (-*count, *c));
        part1.push(counts[0].0);
        part2.push(counts.last().unwrap().0);
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";
        assert_eq!(read_contents(&a).0, "easter");
        assert_eq!(read_contents(&a).1, "advent");
    }
}
