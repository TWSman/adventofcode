use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // let filename: &str = "input.txt";
    read_file(filename);
    read_file2(filename);
}

// Lower case a: 97
// Upper case a: 65
// Example 
fn get_priority(c: char) -> i32 {
    let i = c as i32;
    if i < 97 {
        i - 64 + 26
    } else {
        i - 96
    }

}

fn read_file(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    let lines = contents.split("\n");

    let mut total = 0;
    for ln in lines {
        if ln == "" {
            continue
        }
        let n = ln.len();
        let chars: Vec<char> = ln.chars().collect();
        let left: HashSet<char> = HashSet::from_iter(chars[0..n/2].iter().cloned());
        let right: HashSet<char> = HashSet::from_iter(chars[n/2..].iter().cloned());
        let common: Vec<&char> = left.intersection(&right).collect();
        let priori = get_priority(*common[0]);
        total += priori;
    }
    println!("Total Sum: {total}");
}

fn read_file2(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut total = 0;
    for lns in lines.chunks(3) {
        if lns.len() < 3 {
            continue;
        }
        let first: HashSet<char> = HashSet::from_iter(lns[0].chars());
        let second: HashSet<char> = HashSet::from_iter(lns[1].chars());
        let third: HashSet<char> = HashSet::from_iter(lns[2].chars());
        let inter: HashSet<_> = first.intersection(&second).cloned().collect();
        let common: Vec<&char> = third.intersection(&inter).collect();
        let priori = get_priority(*common[0]);
        // let common_set: HashSet<char> = HashSet::from_iter(common.iter());
        total += priori;
    }
    println!("Total Sum: {total}");
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_a() {
        let a = "a".chars().nth(0).unwrap();
        let z = "z".chars().nth(0).unwrap();
        assert_eq!(get_priority(a), 1);
        assert_eq!(get_priority(z), 26);
    }

    #[test]
    fn test_capital() {
        let a = "A".chars().nth(0).unwrap();
        let z = "Z".chars().nth(0).unwrap();
        assert_eq!(get_priority(a), 27);
        assert_eq!(get_priority(z), 52);
    }
}
