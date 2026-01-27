use clap::Parser;
use std::fs;

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
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}


fn read_contents(cont: &str) -> (i64, i64) {
    let masses = cont.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let part1 = masses.iter().map(|m| get_fuel(*m)).sum();
    let part2 = masses.iter().map(|m| get_fuel_total(*m)).sum();
    (part1, part2)
}

fn get_fuel(mass: i64) -> i64 {
    (mass / 3 - 2).max(0)
}

fn get_fuel_total(mass: i64) -> i64 {
    let mut m = mass;
    let mut total = 0;
    loop {
        let fuel = get_fuel(m);
        total += fuel;
        if fuel == 0 {
            return total;
        }
        m = fuel;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_fuel(12), 2);
        assert_eq!(get_fuel(14), 2);
        assert_eq!(get_fuel(1969), 654);
        assert_eq!(get_fuel(100756), 33583);

        let a = "12
14
1969
100756";

        assert_eq!(read_contents(a).0, 33583 + 654 + 2 + 2);
    }

    #[test]
    fn part2() {
        assert_eq!(get_fuel_total(12), 2);
        assert_eq!(get_fuel_total(14), 2);
        assert_eq!(get_fuel_total(1969), 966);
        assert_eq!(get_fuel_total(100756), 50346);

        let a = "12
14
1969
100756";

        assert_eq!(read_contents(a).1, 50346 + 966 + 2 + 2);

    }
}
