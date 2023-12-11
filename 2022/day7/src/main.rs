use clap::Parser;
use std::fs;
use std::collections::HashMap;
use regex::Regex;
use itertools::{Itertools, Position};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}


fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");
    // In part 1 we add 1 one row/column for each empty one.
    // In other words multiply amount of empty space by 2
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);

}

fn get_new_dir(cur: &str, d: &str) -> String {
    format!("{}+{}", cur, d)
}

fn get_directory_size(dir: &String, directories: &HashMap<String, Vec<String>>, files: &HashMap<String, i64>) -> i64{
    let mut sum: i64 = 0;
    let vec = directories.get(dir).unwrap();
    for v in vec {
        match files.get(v) {
            None => sum += get_directory_size(&v, directories, files),
            Some(s) => sum += s,
        }
    }
    sum
}

fn read_contents(cont: &str) -> (i64, i64) {
    let re1 = Regex::new(r"cd ([a-zA-Z/.]*)").unwrap();
    let mut directories: HashMap<String, Vec<String>> = HashMap::new();
    let mut files: HashMap<String, i64> = HashMap::new();
    let mut current_dir: String = "/".to_string();
    let tmp =  current_dir.clone();
    directories.insert(tmp, vec![]);
    for s in cont.split("$ ") {
        if s == "" {
            continue;
        }
        if &s[..4] == "cd /" {
            continue;
        }
        match &s[..2] {
            "cd" => {
                let dir = re1.captures(&s).unwrap().get(1).unwrap().as_str();
                if dir == ".." {
                    let temp = current_dir.split("+").into_iter().with_position().filter_map(|(p, v)| {
                        match p {
                            Position::Last | Position::Only => None,
                            _ => Some(v)
                        }
                    }).join("+");
                    current_dir = temp;
                } else {
                    let new_dir: String = get_new_dir(&current_dir, dir);
                    if !directories.get(&current_dir).unwrap().contains(&new_dir) {
                        let tmp: String = new_dir.clone().to_owned();
                        directories.get_mut(&current_dir).unwrap().push(tmp.clone());
                    }
                    directories.insert(new_dir.clone(), Vec::new()); 
                    current_dir = new_dir.to_owned();
                }
            },
            "ls" => {
                for v in s.lines().with_position().filter_map(|(p, v)| {
                    match p {
                        Position::First => None,
                        _ => Some(v),
                    }
                })
                    {
                        match v.split(" ").collect_tuple().unwrap() {
                            ("dir", d) => {
                                let new_dir: String = get_new_dir(&current_dir, d);
                                if !directories.get(&current_dir).unwrap().contains(&new_dir) {
                                    let tmp: String = new_dir.clone().to_owned();
                                    directories.get_mut(&current_dir).unwrap().push(tmp.clone());
                                }
                                if !directories.contains_key(&new_dir) {
                                    directories.insert(new_dir.clone(), Vec::new());
                                }
                            }
                            (size, f) => {
                                let f_name: String = get_new_dir(&current_dir, f);
                                files.insert(f_name.clone(), size.parse::<i64>().unwrap());
                                if !directories.get(&current_dir).unwrap().contains(&f_name) {
                                    let tmp: String = f_name.clone().to_owned();
                                    directories.get_mut(&current_dir).unwrap().push(tmp.clone());
                                }

                            }
                        }
                    }
            },
            _ => (),
        }
    }


    let mut dir_sizes: HashMap<String, i64> = HashMap::new();
    for (key, _) in directories.clone().into_iter() {
        dir_sizes.insert(key.clone(), get_directory_size(&key, &directories, &files)); 
    }
    let total_size = 70_000_000;
    let target_size = 30_000_000;
    let root_size = dir_sizes.get("/").unwrap();
    let current_space = total_size - root_size;
    let target_del  = target_size - current_space;
    let ans1 = dir_sizes.clone().into_iter().filter_map(|(_,v)| {
        if v <= 100000 {
            Some(v)
        } else {
            None
        }
    }).sum();

    let ans2: Vec<i64> = dir_sizes.into_iter().filter_map(|(_,v)| {
        if v >= target_del {
            Some(v)
        } else {
            None
        }
    }).sorted().collect();
    (ans1, *ans2.get(0).unwrap())
}

// Total space 70 000 000
// Unused needed 30 000 000
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
        assert_eq!(read_contents(&a).0, 95437);
        assert_eq!(read_contents(&a).1, 24933642);
    }
}
