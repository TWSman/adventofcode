use clap::Parser;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn check_numbers(rules: &HashMap<usize, HashSet<usize>>, numbers: &Vec<usize>) -> Option<usize> {
    // Check a list of numbers and see that it follows all rules of rules
    // Iterate over the list
    for i in 0..numbers.len() {
        let a = numbers[i];
        // Convert earlier numbers to a set
        let number_set: HashSet<usize> = HashSet::from_iter(numbers[..i].to_owned());
        match rules.get(&a) {
            None => continue,
            Some(val) => {
                let inter = number_set.intersection(val).collect::<HashSet<&usize>>();
                if inter.is_empty() {
                    // We want the intersection to be empty
                    // This means that there are no incorrect orderings
                    continue;
                }
                // If at any time the intersection is not empty this number list is not valid
                return None;
            }
        }
    }

    let center = numbers
        .get(numbers.len() / 2)
        .expect("Center value should exist");
    Some(*center)
}

fn reorder_numbers(rules: &HashMap<usize, HashSet<usize>>, numbers: &Vec<usize>) -> usize {
    let n = numbers.len();
    // Convert number list to a set
    let number_set: HashSet<usize> = HashSet::from_iter(numbers.to_owned());

    // Create a map to store order values
    let mut ordering: HashMap<usize, usize> = HashMap::new();

    for l in numbers {
        // Count the number of rules 'l' has
        // Only count those that concern other numbers in this list
        // It seems that this count exactly matches the ordering
        // If 'l' is supposed to be first/second in a list of 6 numbers, this intersection should include
        // 5/4 values
        // If 'l' is supposed to be last this intersection will be empty (no numbers should come
        // after 'l')
        match rules.get(l) {
            None => _ = ordering.insert(0, *l),
            Some(val) => _ = ordering.insert(val.intersection(&number_set).count(), *l),
        }
    }
    // Number at index 0 should come last
    // This means that there are no numbers that should come after that number
    // Reverse wouldn't actually matter since the center number will be the same
    let new_numbers = (0..n)
        .rev()
        .map(|i| ordering.get(&i).unwrap().to_owned())
        .collect::<Vec<usize>>();
    // Get center value
    let center = *new_numbers.get(n / 2).expect("Center value should exist");
    center
}

fn read_contents(cont: &str) -> (usize, usize) {
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    for line in cont.lines() {
        if line.contains('|') {
            // Read lines of form a|b
            // This means that a should always come before b
            let res = line
                .split('|')
                .filter_map(|m| m.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            // Either retrieve the set of rules associated with this number or add a new empty set
            let set = rules.entry(res[0]).or_default();
            _ = set.insert(res[1]);
        } else if line.contains(',') {
            // Alternatively convert line to a list of numbers
            let numbers = line
                .split(',')
                .filter_map(|m| m.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            // Check if the list follows our rules, or reorder the list to follow
            // Lists that follow the rules count towards part1,
            // lists that don't follow the rules count towards part2
            check_numbers(&rules, &numbers).map_or_else(
                || part2 += reorder_numbers(&rules, &numbers),
                |val| part1 += val,
            );
        }
    }

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let a = "29|13
47|13
47|29
47|53
47|61
53|13
53|29
61|13
61|29
61|53
75|13
75|29
75|47
75|53
75|61
97|13
97|29
97|47
97|53
97|61
97|75

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(read_contents(&a).0, 143);
        assert_eq!(read_contents(&a).1, 123);
    }
}
