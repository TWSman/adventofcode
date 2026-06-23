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

fn read_contents(cont: &str) -> (i64, i64) {
    let mut iter = cont.lines();
    let target = iter.next().unwrap().parse::<i64>().unwrap();
    let busses = iter
        .next()
        .unwrap()
        .split(',')
        .map(|ln| ln.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let part1 = get_part1(target, &busses);
    let part2 = get_part2(&busses);
    (part1, part2)
}

fn get_part1(target: i64, busses: &[Option<i64>]) -> i64 {
    let busses = busses.iter().filter_map(|c| *c).collect::<Vec<_>>();
    let (bus_id, wait) = busses
        .iter()
        .map(|b| (b, b - target % b))
        .min_by_key(|c| c.1)
        .unwrap();
    bus_id * wait
}

fn get_part2(busses: &[Option<i64>]) -> i64 {
    // Remove 'x' buses and add index to others
    let busses = busses
        .iter()
        .enumerate()
        .filter_map(|(i, c)| c.map(|v| (i as i64, v)))
        .collect::<Vec<_>>();

    // Keep track of the product of all ids so far
    // Once we have a single known solution, all other solutions can be found by adding multiples of
    // product
    let mut product = 1;

    // Keep track of the last accepted solution
    // Without any limitations the best timestamp is 0
    let mut timestamp = 0;

    for b in &busses {
        // Generate more solutions and pick the next tone that also solves for the next id
        timestamp = (0..)
            .map(|n| timestamp + n * product)
            .find(|n| (b.0 + n) % b.1 == 0)
            .unwrap();
        product *= b.1;
    }
    timestamp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "939
7,13,x,x,59,x,31,19";
        assert_eq!(read_contents(&a).0, 295);
    }

    #[test]
    fn part2() {
        let a = "939
7,13,x,x,59,x,31,19";
        assert_eq!(read_contents(&a).1, 1068781);

        let a = "0
17,x,13,19";
        assert_eq!(read_contents(&a).1, 3417);

        let a = "0
67,7,59,61";
        assert_eq!(read_contents(&a).1, 754018);

        let a = "0
67,x,7,59,61";
        assert_eq!(read_contents(&a).1, 779210);

        let a = "0
67,7,x,59,61";
        assert_eq!(read_contents(&a).1, 1261476);
    }
}
