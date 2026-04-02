use clap::Parser;

use itertools::Itertools;
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
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_cities(cont: &str) -> (BTreeMap<String, usize>, BTreeMap<(usize, usize), i32>) {
    let mut cities: BTreeMap<String, usize> = BTreeMap::new();
    let mut distances: BTreeMap<(usize, usize), i32> = BTreeMap::new();
    let mut i = 0;
    for line in cont.lines() {
        let (cit, dist) = line.split_once(" = ").unwrap();
        let (from, to) = cit.split_once(" to ").unwrap();
        if !cities.contains_key(from) {
            cities.insert(from.to_string(), i);
            i += 1;
        }
        if !cities.contains_key(to) {
            cities.insert(to.to_string(), i);
            i += 1;
        }
        let from_i = cities.get(from).unwrap();
        let to_i = cities.get(to).unwrap();
        let d = dist.parse::<i32>().unwrap();
        distances.insert((*from_i, *to_i), d);
        distances.insert((*to_i, *from_i), d);
    }
    (cities, distances)
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn read_contents(cont: &str) -> (i32, i32) {
    let (cities, distances) = read_cities(cont);
    println!("{} cities", cities.len());
    println!("{} potential routes", factorial(cities.len()));
    let (part1, part2) = analyze_list(&cities, &distances);
    (part1, part2)
}

fn analyze_list(
    cities: &BTreeMap<String, usize>,
    distances: &BTreeMap<(usize, usize), i32>,
) -> (i32, i32) {
    let mut min_distance = i32::MAX;
    let mut max_distance = i32::MIN;
    let city_names: Vec<usize> = cities.values().cloned().collect();
    for city_list in city_names.iter().permutations(city_names.len()) {
        let total_distance = city_list
            .windows(2)
            .map(|w| {
                let from = w[0];
                let to = w[1];
                *distances.get(&(*from, *to)).unwrap()
            })
            .sum::<i32>();
        if total_distance < min_distance {
            min_distance = total_distance;
        }
        if total_distance > max_distance {
            max_distance = total_distance;
        }
    }
    (min_distance, max_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        assert_eq!(read_contents(a).0, 605);
        assert_eq!(read_contents(a).1, 982);
    }
}
