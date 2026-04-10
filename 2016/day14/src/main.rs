use clap::Parser;
use colored::Colorize;
use memoize::memoize;
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
    let cont = cont.trim();
    let part1 = get_part1(cont);
    let part2 = get_part2(cont);

    (part1, part2)
}

#[memoize]
fn wrapper(salt: String, j: i64) -> Vec<char> {
    let inp = format!("{}{}", salt, j);
    format!("{:x}", md5::compute(inp)).chars().collect()
}

#[memoize]
fn wrapper2(salt: String, j: i64) -> Vec<char> {
    let mut inp = format!("{}{}", salt, j);
    for _ in 0..2017 {
        inp = format!("{:x}", md5::compute(inp));
    }

    inp.to_string().chars().collect()
}

#[allow(dead_code)]
fn print_hash(hash: Vec<char>, c: char) {
    // Print the hash, with the char c highlighted in red
    print!("    ");
    for c2 in hash {
        if c2 == c {
            print!("{}", c2.to_string().red());
        } else {
            print!("{}", c2);
        }
    }
    println!();
}

fn get_part1(salt: &str) -> i64 {
    let mut key_index = -1;
    let mut found = 0;
    let mut indices = vec![];
    loop {
        key_index += 1;
        let hash = wrapper(salt.to_string(), key_index);
        let mut valid = None;
        // Look for a triplet
        for i in 0..(hash.len() - 2) {
            if (hash[i] == hash[i + 1]) && (hash[i] == hash[i + 2]) {
                valid = Some(hash[i]);
                break;
            }
        }
        if valid.is_none() {
            continue;
        }
        for offset in (key_index + 1)..=(key_index + 1000) {
            let hash_next = wrapper(salt.to_string(), offset);
            if check_five(&hash_next, valid.unwrap()) {
                found += 1;
                indices.push(key_index);
                if found == 64 {
                    return key_index;
                }
                break;
            }
        }
    }
}

fn get_part2(salt: &str) -> i64 {
    let mut key_index = -1;
    let mut found = 0;
    let mut indices = vec![];
    loop {
        key_index += 1;
        let hash = wrapper2(salt.to_string(), key_index);
        let mut valid = None;
        for i in 0..(hash.len() - 2) {
            // Look for a triplet
            if (hash[i] == hash[i + 1]) && (hash[i] == hash[i + 2]) {
                valid = Some(hash[i]);
                break;
            }
        }
        if valid.is_none() {
            continue;
        }
        for offset in (key_index + 1)..=(key_index + 1000) {
            let hash_next = wrapper2(salt.to_string(), offset);
            if check_five(&hash_next, valid.unwrap()) {
                // let hash_next = wrapper2(salt.to_string(), offset);
                //print_hash(hash, valid.unwrap());
                //print_hash(hash_next, valid.unwrap());
                found += 1;
                indices.push(key_index);
                println!("Found {found} keys, last one is {}", key_index);
                if found == 64 {
                    return key_index;
                }
                break;
            }
        }
    }
}

fn check_five(hash: &[char], c: char) -> bool {
    for i in 0..(hash.len() - 4) {
        if (hash[i] == c)
            && (hash[i + 1] == c)
            && (hash[i + 2] == c)
            && (hash[i + 3] == c)
            && (hash[i + 4] == c)
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_part1(&"abc"), 22728);
    }

    #[test]
    fn part2() {
        assert_eq!(
            wrapper2("abc".to_string(), 0),
            "a107ff634856bb300138cac6568c0f24"
                .chars()
                .collect::<Vec<char>>()
        );
        assert_eq!(get_part2(&"abc"), 22551);
    }
}
