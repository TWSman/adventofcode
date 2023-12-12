use clap::Parser;
use std::fs;
//use std::collections::HashSet;
use itertools::{Itertools, Position};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}


#[derive(Debug, PartialEq, Eq, Clone)]
enum Marker {
    Unknown,
    Fixed,
    Broken,
    None,
}

impl Marker {
    fn new(c: char) -> Marker {
        match c {
            '?' => Marker::Unknown,
            '#' => Marker::Broken,
            '.' => Marker::Fixed,
            _ => panic!("Unknown character"),
        }
    }
}

#[derive(Debug, Clone)]
struct Section {
    marker: Marker,
    len: i64, 
}


#[derive(Debug)]
struct Field {
    markers: Vec<Marker>,
    secs: Vec<Section>,
    marker_string: String,
}

impl Field {
    fn new(v: &str) -> Field{
        let mut f = Field {marker_string: v.to_string(), secs: Vec::new(), markers: v.chars().map(|c| {Marker::new(c)}).collect()};
        let mut prev = Marker::None;
        let mut sec_len: i64 = 0;
        for m in &f.markers {
            if (m == &prev) | (prev == Marker::None) {
                sec_len += 1;
            }
            else {
                f.secs.push(Section {marker: prev, len: sec_len});
                sec_len = 1;
            }
            prev = m.clone();
        }
        f.secs.push(Section {marker: prev, len: sec_len});
        // let tmp = markers.to_owned();
        // Field {secs: secs.clone(), markers: tmp.clone()}
        f
    }

    fn get_sections(&self) -> Vec<i64> {
        self.secs.iter().filter_map(|m| {
            match m.marker {
                Marker::Broken => Some(m.len),
                _ => None,
            }
        }).collect()
    }

    fn get_options(&self) -> Vec<Vec<i64>> {
        get_options(&self.marker_string)
    }
}

fn get_options(markers: &String) -> Vec<Vec<i64>> {
    let n: usize = markers.chars().map(|c| {
        match c {
            '?' => 1,
            _ => 0,
        }
    }).sum();

    if n == 0 {
        let f = Field::new(&markers);
        return vec![f.get_sections()];
    } else {
        let marker1 = markers.replacen("?", "#", 1);
        let marker2 = markers.replacen("?", ".", 1);
        let mut vec1 = get_options(&marker1);
        let mut vec2 = get_options(&marker2);
        vec1.append(&mut vec2);
        vec1
    }
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");
    // In part 1 we add 1 one row/column for each empty one.
    // In other words multiply amount of empty space by 2
    let res1 = read_contents(&contents, 1);
    println!("Part 1 answer is {}", res1);

    let res2 = read_contents(&contents, 5);
    println!("Part 2 answer is {}", res2);
}


enum RetType {
    Type1(Field),
    Type2(Vec<i64>),
}

fn read_line(input: &str, repeat: usize) -> i64 {
    dbg!(&input);
    let (field, mut counts) = match input.split_whitespace().with_position().map(|(p,v)| {
        match p {
            Position::First => {
                RetType::Type1(Field::new(
                    &(0..repeat).map(|_| v).join("?")
                        ))
            },
            Position::Last => {
                RetType::Type2(v.split(",").map(|m| {m.parse::<i64>().unwrap()}).collect())
            }
            _ => panic!("Unknown position"),
        }
    }).collect_tuple().unwrap() {
            (RetType::Type1(x), RetType::Type2(y)) => (x,y),
            _ => panic!("HEY"),
        };

    let c = counts.clone();
    for _ in 0..(repeat-1) {
        counts.append(&mut c.clone());
    }
    let options = field.get_options();

    options.iter().filter(|m| { m == &&counts}).count() as i64
}

fn read_contents(cont: &str, repeat: usize) -> i64 {
    cont.lines().enumerate().map(|(i, l)| {
        if i % 2 == 0 {
            println!("{}", i);
        }
        read_line(&l, repeat)}).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(read_contents(&a, 1), 21);
        //assert_eq!(read_contents(&a, 5), 21);
    }

    #[test]
    fn line(){
        assert_eq!(read_line("???.### 1,1,3"            ,1), 1);
        assert_eq!(read_line(".??..??...?##. 1,1,3"     ,1), 4);
        assert_eq!(read_line("?#?#?#?#?#?#?#? 1,3,1,6"  ,1), 1);
        assert_eq!(read_line("????.#...#... 4,1,1 "     ,1), 1);
        assert_eq!(read_line("????.######..#####. 1,6,5",1), 4);
        assert_eq!(read_line("?###???????? 3,2,1 "      ,1),10);

        assert_eq!(read_line("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3", 1), 1);
        assert_eq!(read_line("???.### 1,1,3"            ,5), 1);
        //assert_eq!(read_line(".??..??...?##. 1,1,3"     ,5), 16384);
        //assert_eq!(read_line("?#?#?#?#?#?#?#? 1,3,1,6"  ,5), 1);
        //assert_eq!(read_line("????.#...#... 4,1,1 "     ,5), 16);
        //assert_eq!(read_line("????.######..#####. 1,6,5",5), 2500);
        //assert_eq!(read_line("?###???????? 3,2,1 "      ,5), 506250);
    }
}
