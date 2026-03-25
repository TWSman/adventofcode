use clap::Parser;
use itertools::Itertools;
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
    let mut packages = cont
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    // Sort to decreasing weight
    packages.sort_by(|a, b| b.cmp(a));
    dbg!(&packages.len());
    let part1 = get_part1(&packages);
    let part2 = get_part2(&packages);
    (part1, part2)
}

fn get_part1(packages: &[i64]) -> i64 {
    println!("Part 1:");
    find_min_qe(packages, 3, true)
}

fn get_part2(packages: &[i64]) -> i64 {
    println!("\n\nPart 2:");
    find_min_qe(packages, 4, true)
}

fn find_min_qe(packages: &[i64], groups: usize, main: bool) -> i64 {
    let cumulative_weight = packages
        .iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    let n = packages.len();
    let weight_sum = packages.iter().sum::<i64>();
    assert_eq!(weight_sum % groups as i64, 0);
    let target_weight = weight_sum / groups as i64;

    let minimum_needed = cumulative_weight
        .iter()
        .position(|&x| x >= target_weight)
        .unwrap()
        + 1;
    if main {
        println!("Need at least {minimum_needed} packages");
    }
    let mut count = minimum_needed;
    loop {
        let mut min_qe = i64::MAX;
        for comb in packages.iter().combinations(count) {
            if comb.iter().copied().sum::<i64>() != target_weight {
                continue;
            }
            let qe = comb.iter().copied().product::<i64>();
            if qe < min_qe {
                if main {
                    println!("New minimum QE found: {qe} with:\n    {:?}", comb);
                }
                // Check if the remaining packages can be split into 2 groups of target weight
                let remaining_packages = packages
                    .iter()
                    .filter(|p| !comb.contains(p))
                    .copied()
                    .collect::<Vec<_>>();
                if groups > 2 {
                    if main {
                        println!(
                            "    Checking if the remaining packages can be split into {} groups",
                            groups - 1
                        );
                    }
                    let sub_minqe = find_min_qe(&remaining_packages, groups - 1, false);
                    assert!(
                        sub_minqe != 0,
                        "Could not find a valid partition for the remaining packages"
                    )
                }
                min_qe = qe;
            }
        }
        if min_qe != i64::MAX {
            if main {
                println!("Minimum QE for {count} packages is {min_qe}");
            }
            return min_qe;
        }
        if main {
            println!("No valid combination found for {count} packages");
        }
        count += 1;
        if count > n {
            break;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let a = "1
2
3
4
5
7
8
9
10
11";
        assert_eq!(read_contents(&a).0, 99);
        assert_eq!(read_contents(&a).1, 44);
    }
}
