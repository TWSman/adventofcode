use clap::Parser;
use std::fs;
use std::cmp::Ordering;

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
    let mut pairs: Vec<(MaybeList, MaybeList)> = Vec::new();
    let mut a: Option<MaybeList> = None;
    let mut b: Option<MaybeList> = None;
    for line in cont.lines() {
        if line.is_empty() {
            pairs.push((a.unwrap(), b.unwrap()));
            a = None;
            b = None;
        } else if a.is_none() {
            a = Some(MaybeList::new(line));
        } else {
            b = Some(MaybeList::new(line));
        }
    }
    pairs.push((a.unwrap(), b.unwrap()));
    let part1 = pairs.iter().enumerate().map(|(i,(a,b))| {
        if a.compare(b) == Ordering::Less {
            (i + 1) as i64
        } else {
            0
        }
    }
    ).sum();
    let mut full_list: Vec<(i64, &MaybeList)> = Vec::new();
    let test1 = MaybeList::new("[[2]]");
    let test2 = MaybeList::new("[[6]]");
    full_list.push((0, &test1));
    full_list.push((1, &test2));
    let mut id = 2;
    dbg!(pairs.len());
    for (a,b) in &pairs {
        full_list.push((id, a));
        full_list.push((id + 1, b));
        id += 2;
    }
    full_list.sort_by_key(|v| v.1);
    let part2 = full_list.iter().enumerate().filter_map(|(i, (id, _v))| {
        if *id <= 1{
            Some(i as i64 + 1)
        } else {
            None
        }
    }).product();

    (part1, part2)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum MaybeList {
    Integer(i64),
    List(Vec<MaybeList>),
}


impl Ord for MaybeList {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

impl PartialOrd for MaybeList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl MaybeList {
    fn new(ln: &str) -> MaybeList {
        assert!(ln.starts_with('['));
        if ln == "[]" {
            return MaybeList::List(Vec::new());
        }
        assert!(!ln.is_empty(), );
        let n = ln.len() - 1;
        let inns = &ln[1..n];
        let mut numbers: String = String::new();
        let chars: Vec<char> = inns.chars().collect();
        let mut accum: String = String::new();
        let mut list: Vec<MaybeList> = Vec::new();
        let mut open_brackets = 0;
        let mut close_brackets = 0;
        for c in chars {
            if !accum.is_empty() {
                if c == '[' {
                    open_brackets += 1;
                }
                if c == ']' {
                    close_brackets += 1;
                }
                accum.push(c);
                if close_brackets == open_brackets {
                    list.push(MaybeList::new(&accum));
                    accum = String::new();
                }
                continue;
            }
            if c == '[' {
                accum.push(c);
                open_brackets += 1;
                continue;
            }
            if c.is_ascii_digit() {
                numbers.push(c);
            } else if !numbers.is_empty() {
                assert_eq!(c, ',');
                let num = numbers.parse::<i64>().unwrap();
                list.push(MaybeList::Integer(num));
                numbers = String::new();
            }
        }
        if !numbers.is_empty() {
            let num = numbers.parse::<i64>().unwrap();
            list.push(MaybeList::Integer(num));
        }
        MaybeList::List(list)
    }

    fn compare(&self, other: &MaybeList) -> Ordering {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => {
                a.cmp(b)
            }
            (Self::List(a), Self::List(b)) => {
                let na = a.len();
                let nb = b.len();
                let n = na.min(nb);
                for i in 0..n {
                    match a[i].compare(&b[i]) {
                        Ordering::Equal => continue,
                        Ordering::Less => {
                            return Ordering::Less;
                        }
                        Ordering::Greater => {
                            return Ordering::Greater;
                        }
                    }
                }
                na.cmp(&nb)
            }
            (Self::List(_a), Self::Integer(b)) => {
                let new_b = MaybeList::List(vec![MaybeList::Integer(*b)]);
                self.compare(&new_b)
            }

            (Self::Integer(a), Self::List(_b)) => {
                let new_a = MaybeList::List(vec![MaybeList::Integer(*a)]);
                new_a.compare(other)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a ="[1,1,3,1,1]
[1,1,11,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(read_contents(&a).0, 13);
        assert_eq!(read_contents(&a).1, 140);
    }
}
