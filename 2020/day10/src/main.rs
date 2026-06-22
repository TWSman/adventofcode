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

fn get_part1(jolts: &[i64]) -> i64 {
    let mut diff1 = 0;
    let mut diff3 = 1; // Final diff is always 3
    let mut prev = -1;
    for jolt in jolts {
        if prev == -1 {
            prev = *jolt;
            continue;
        }
        if jolt - prev == 1 {
            diff1 += 1;
        } else if jolt - prev == 3 {
            diff3 += 1;
        } else if jolt - prev != 2 {
            panic!();
        }
        prev = *jolt;
    }
    diff1 * diff3
}

fn get_part2(jolts: &[i64]) -> i64 {
    let mut paths: BTreeMap<i64, i64> = BTreeMap::new();
    for jolt in jolts.iter().rev() {
        if paths.is_empty() {
            // Loop starts from the highest joltage. There is only one path to the target
            // And the highest jotlage is the only direct access to the target
            paths.insert(*jolt, 1);
            continue;
        }
        let mut tmp = 0;
        for (b, count) in paths.iter() {
            if *b <= jolt + 3 {
                tmp += count;
            }
        }
        paths.insert(*jolt, tmp);
    }
    *paths.get(&0).unwrap()
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut jolts: Vec<i64> = cont.lines().map(|c| c.parse::<i64>().unwrap()).collect();
    jolts.push(0);
    jolts.sort();
    let part1 = get_part1(&jolts);
    let part2 = get_part2(&jolts);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(read_contents(&a).0, 35);
        let b = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(read_contents(&b).0, 220);
    }

    #[test]
    fn part2() {
        let a = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(read_contents(&a).1, 8);
        let b = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(read_contents(&b).1, 19208);
    }
}
