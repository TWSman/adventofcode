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
    // cqjxmczf is not correct
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (String, String) {
    let input = cont.lines().next().unwrap().to_owned();
    dbg!(&input);
    let part1 = get_next_valid(&input);
    let part2 = get_next_valid(&part1);
    (part1, part2)
}

fn get_next_valid(password: &str) -> String {
    let pass = password
        .chars()
        .map(|c| c as u16 - 'a' as u16)
        .collect::<Vec<u16>>();
    let mut new_pass = pass.clone();
    let n = pass.len();
    let mut ind = 99;
    if new_pass.contains(&CHAR_I) {
        let ind_i = new_pass.iter().position(|&x| x == CHAR_I).unwrap();
        if ind_i < ind {
            ind = ind_i;
        }
        new_pass[ind_i] = CHAR_I + 1;
    }

    if new_pass.contains(&CHAR_L) {
        let ind_l = new_pass.iter().position(|&x| x == CHAR_L).unwrap();
        if ind_l < ind {
            ind = ind_l;
            new_pass[ind_l] = CHAR_L + 1;
        }
    }

    if new_pass.contains(&CHAR_O) {
        let ind_o = new_pass.iter().position(|&x| x == CHAR_O).unwrap();
        if ind_o < ind {
            ind = ind_o;
            new_pass[ind_o] = CHAR_O + 1;
        }
    }

    for (j, v) in new_pass
        .iter_mut()
        .enumerate()
        .take(pass.len())
        .skip(ind + 1)
    {
        println!("Setting {} to 0", j);
        *v = 0;
    }

    let str = new_pass
        .iter()
        .map(|x| *x as u8 + b'a')
        .map(|c| c as char)
        .collect::<String>();
    println!("Modified password {}", str);

    let mut count = 0;
    loop {
        count += 1;
        let mut i = n - 1;
        loop {
            new_pass[i] += 1;
            if new_pass[i] == CHAR_I || new_pass[i] == CHAR_O || new_pass[i] == CHAR_L {
                new_pass[i] += 1;
            }
            if new_pass[i] > CHAR_Z {
                new_pass[i] = 0;
                if i == 0 {
                    break;
                }
                i -= 1;
            } else {
                break;
            }
        }
        let str = new_pass
            .iter()
            .map(|x| *x as u8 + b'a')
            .map(|c| c as char)
            .collect::<String>();
        if count % 100000 == 0 {
            println!("Checking password {}", str);
            println!("Tried {} passwords, still going", count);
        }
        if password_is_valid(&new_pass) {
            println!(
                "Found valid password {} after trying {} passwords",
                str, count
            );
            break;
        }
    }
    new_pass
        .iter()
        .map(|x| *x as u8 + b'a')
        .map(|c| c as char)
        .collect::<String>()
}

const CHAR_I: u16 = 'i' as u16 - 'a' as u16;
const CHAR_O: u16 = 'o' as u16 - 'a' as u16;
const CHAR_L: u16 = 'l' as u16 - 'a' as u16;
const CHAR_Z: u16 = 'z' as u16 - 'a' as u16;

fn password_is_valid(vec: &[u16]) -> bool {
    // 2) Password may not contain the letters i, o, or l
    if vec.contains(&CHAR_I) || vec.contains(&CHAR_O) || vec.contains(&CHAR_L) {
        return false;
    }
    // 1) Password must include one increasing straight of at least three letters
    let mut check1 = false;
    for i in 0..(vec.len() - 2) {
        if vec[i] + 1 == vec[i + 1] && vec[i] + 2 == vec[i + 2] {
            check1 = true;
            break;
        }
    }
    if !check1 {
        return false;
    }

    let mut pairs = 0;
    let mut i = 0;
    loop {
        i += 1;
        if i >= vec.len() {
            break;
        }

        if vec[i - 1] == vec[i] {
            pairs += 1;
            i += 1;
        }
    }
    // 3) Password must containt at least two different non-overlapping pairs
    pairs > 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_next_valid(&"abcdefgh"), "abcdffaa");
        assert_eq!(get_next_valid(&"ghijklmn"), "ghjaabcc");
    }

    fn wrapper(password: &str) -> bool {
        let vec = password
            .chars()
            .map(|c| c as u16 - 'a' as u16)
            .collect::<Vec<u16>>();
        password_is_valid(&vec)
    }

    #[test]
    fn valid() {
        assert!(!wrapper(&"hijklmmn"));
        assert!(!wrapper(&"abbceffg"));
        assert!(!wrapper(&"abbcegjk"));
        assert!(wrapper(&"abcdffaa"));
        assert!(wrapper(&"ghjaabcc"));
    }
}
