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
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
}


fn read_line(input: &str) -> i64 {
    let mut field: Field = Field::new("");
    let mut counts: Vec<i64> = Vec::new();
    for (p,v) in input.split_whitespace().with_position() {
        match p {
            Position::First => {
                field = Field::new(v);
            },
            Position::Last => {
                counts = v.split(",").map(|m| {m.parse::<i64>().unwrap()}).collect();
            }
            _ => panic!("HEY"),
        }
    }
    let options = field.get_options();
    options.iter().filter(|m| { m == &&counts}).count() as i64
}

fn read_contents(cont: &str) -> (i64, i64) {
    
    (cont.lines().enumerate().map(|(i, l)| {
        if i % 10 == 0 {
            println!("{}", i);
        }
        read_line(&l)}).sum(), 0)
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
        assert_eq!(read_contents(&a).0, 21);
    }

    #[test]
    fn line(){
        assert_eq!(read_line("???.### 1,1,3"            ), 1);
        assert_eq!(read_line(".??..??...?##. 1,1,3"     ), 4);
        assert_eq!(read_line("?#?#?#?#?#?#?#? 1,3,1,6"  ), 1);
        assert_eq!(read_line("????.#...#... 4,1,1 "     ), 1);
        assert_eq!(read_line("????.######..#####. 1,6,5"), 4);
        assert_eq!(read_line("?###???????? 3,2,1 "      ),10);
    }
}
