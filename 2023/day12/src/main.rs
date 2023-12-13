use clap::Parser;
use std::fs;
use itertools::{Itertools, Position};
use std::cmp::min;
use std::cmp::max;

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
    len: usize, 
}


#[derive(Debug)]
struct Field {
    markers: Vec<Marker>,
    secs: Vec<Section>,
    marker_string: String,
}

fn get_counts(n: i64, counts: &Vec<i64>) -> i64 {
    //dbg!(&n);
    //dbg!(&counts);
    let min_len: i64 = counts.iter().sum::<i64>() + counts.iter().count() as i64 - 1;
    let extra_len = n - min_len;
    if min_len == n {
        //println!("Exactly 1");
        return 1;
    }
    if min_len > n {
        //println!("Not possible");
        return 0;
    }
    if counts.len() == 1 {
        let tmp = n - counts.last().unwrap() + 1;
        //println!("{} possibilities", tmp);
        return tmp
    }
    //dbg!(&extra_len);
    //dbg!(&min_len);
    // Works when extra len is 1
    return match (extra_len, counts.len())  {
        (1,_) => extra_len * (1 + counts.len() as i64),
        (n,2) => (n+1) * (n + 2) / 2,

        (2,3) => 10, // Still need a general formula
        (2,4) => 10, // Not correct!
        
        (3,3) => 10, // Not correct!
        (3,4) => 10, // Not correct!
        
        (4,3) => 10, // Not correct!
        (4,4) => 10, // Not correct!
        (4,5) => 10, // Not correct!
        
        (5,3) => 10, // Not correct!
        _ => panic!("Unknown combo extra N: {}, count: {}", extra_len, counts.len()), 
    }
}

impl Field {
    fn new(v: &str, skim: bool) -> Field{
        let mut vv: &str = &v.clone();
        if skim {
            vv = vv.trim_matches('.');
        }
        //dbg!(&vv);
        let mut f = Field {marker_string: vv.to_string(), secs: Vec::new(), markers: vv.chars().map(|c| {Marker::new(c)}).collect()};
        let mut prev = Marker::None;
        let mut sec_len: usize = 0;
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

    fn possible(&self) -> usize {
        self.secs.iter().map(|m| {
            match m.marker {
                Marker::Unknown | Marker::Broken => Some(m.len),
                Marker::Fixed => None,
                _ => panic!("Panic"),
            }
        }).while_some().sum()
    }

    fn unknown(&self) -> usize {
        self.secs.iter().map(|m| {
            match m.marker {
                Marker::Unknown => Some(m.len),
                Marker::Fixed | Marker::Broken => None,
                _ => panic!("Panic"),
            }
        }).while_some().sum()
    }

    fn get_sections(&self) -> Vec<i64> {
        self.secs.iter().filter_map(|m| {
            match m.marker {
                Marker::Broken => Some(m.len as i64),
                _ => None,
            }
        }).collect()
    }

    fn get_options(&self) -> Vec<Vec<i64>> {
        get_options(&self.marker_string)
    }

    fn get_len(&self) -> usize {
        self.markers.len()
    }

    fn get_options_alt(&self, counts: &Vec<i64>) -> i64 {
        //dbg!(&self.marker_string);
        //dbg!(&counts);
        if counts.len() == 0 {
            //dbg!("No counts left");
            return 1;
        }
        if self.secs.len() == 1 {
            let sec = self.secs.last().unwrap();
            if sec.marker == Marker::Unknown {
                return get_counts(sec.len as i64, &counts);
            }
        }
        let min_len: i64 = counts.iter().sum::<i64>() as i64 + counts.len() as i64 - 1;
        if (self.markers.len() as i64) < min_len {
            //println!("No enough markers left to fill counts");
            return 0;
        }
        if self.markers.len() as i64 == min_len {
            //dbg!("Only 1 way to fill");
            //dbg!(&counts);
            //dbg!(&self.markers);
            return 1;
        }
        let c1 = *counts.iter().next().unwrap() as usize;
        let c_last = *counts.iter().last().unwrap() as usize;
        let n = self.markers.len();
        if self.secs.len() > 1 {
            let last_sec = self.secs.iter().last().unwrap();
            match last_sec.marker {
                Marker::Broken => {
                    //dbg!("Self ends with a broken marker");
                    let n = self.markers.len();
                    //dbg!(&n);
                    if n < c_last {
                        //dbg!("HEY");
                        return 0;
                    }
                    if n > c_last {
                        //dbg!(&self);
                        //dbg!(&c_last);
                        //dbg!(&n);
                        //dbg!(&(n-c_last));
                        //
                        //dbg!("Remove last group ({}) and create a subfield", counts[counts.len()-1]);
                        let sub_field = Field::new(self.marker_string.get(0..n-c_last-1).unwrap(), true);
                        let sub_counts = counts[0..counts.len()-1].to_vec();
                        //dbg!(&sub_counts);
                        return sub_field.get_options_alt(&sub_counts);
                    } else {
                        //dbg!("Return 1");
                        return 1;
                    }
                },
                Marker::Fixed =>{
                    //dbg!("Self ends with a fixed marker");
                    //dbg!("Remove last section");
                    let l = last_sec.len as usize;
                    //dbg!(&l);
                    //dbg!(&n);
                    let sub_field = Field::new(self.marker_string.get(0..n-l).unwrap(), true);
                    //dbg!(&sub_field.marker_string);
                    return sub_field.get_options_alt(&counts);
                }
                _ => (),
            }
        }
        let first_sec = self.secs.iter().next().unwrap();
        match first_sec.marker {
            Marker::Broken => {
                //println!("Self starts with a broken marker");
                let n = self.markers.len();
                if n < c1 {
                    return 0;
                }
                if n > c1 {
                    match self.markers.get(c1) {
                        Some(Marker::Broken) => {
                            //println!("Following marker is Broken");
                            return 0;
                        },
                        _ => (),
                    }
                    let sub_field = Field::new(self.marker_string.get(c1+1..n).unwrap(), true);
                    let sub_counts = counts[1..counts.len()].to_vec();
                    return sub_field.get_options_alt(&sub_counts);
                } else {
                    return 1;
                }
            }
            Marker::Fixed =>{
                //dbg!("Self starts with a fixed marker");
                //bg!("Remove First section");
                let l = first_sec.len as usize;
                let sub_field = Field::new(self.marker_string.get(l..n).unwrap(), true);
                let tmp = sub_field.get_options_alt(&counts);
                //dbg!(&sub_field);
                //dbg!(tmp);
                return tmp;
            }
            Marker::Unknown => {
                //println!("Split in two");
                let f1 = Field::new(&self.marker_string.replacen("?", "#", 1), true);
                let f2 = Field::new(&self.marker_string.replacen("?", ".", 1), true);
                return f1.get_options_alt(&counts) + f2.get_options_alt(&counts);
            },
            _ => {panic!("HEY");},
        }
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
        let f = Field::new(&markers, true);
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
    let (field, mut counts) = match input.split_whitespace().with_position().map(|(p,v)| {
        match p {
            Position::First => {
                RetType::Type1(Field::new(&(0..repeat).map(|_| v).join("?"), true))
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
    dbg!(&field.marker_string);
    dbg!(&counts);
    // let options = field.get_options();
    // options.iter().filter(|m| { m == &&counts}).count() as i64
    field.get_options_alt(&counts)
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
        //assert_eq!(read_contents(&a, 1), 21);
        //assert_eq!(read_contents(&a, 5), 21);
    }

    #[test]
    fn counts() {
        assert_eq!(get_counts(5, &vec![5]), 1);
        assert_eq!(get_counts(5, &vec![4]), 2);
        assert_eq!(get_counts(5, &vec![3]), 3);
        assert_eq!(get_counts(5, &vec![2]), 4);
        assert_eq!(get_counts(5, &vec![1]), 5);

        assert_eq!(get_counts(5, &vec![2,2]), 1);
        assert_eq!(get_counts(5, &vec![2,1]), 3);

        assert_eq!(get_counts(6, &vec![2,1,1]), 1); // No extra
        assert_eq!(get_counts(7, &vec![2,1,1]), 4); // 1 extra space
        assert_eq!(get_counts(8, &vec![2,2,1]), 4); // 1 extra space

        assert_eq!(get_counts(5, &vec![1,1]), 6); // 2 extra space
        // 2 groups, 2 extra space => 6 
        // First moves: 3 options
        // Second moves: 1, 2 or 3 options
        // x.x..
        // x..x.
        // .x.x.
        // x...x
        // .x..x
        // ..x.x
        //
        assert_eq!(get_counts(6, &vec![1,1]), 10); // 2 extra space
        // x.x...
        // x..x..
        // .x.x..
        // x...x.
        // .x..x.
        // ..x.x.
        // x....x
        // .x...x
        // ..x..x
        // ...x.x
        //
        assert_eq!(get_counts(7, &vec![1,1]), 15); // 2 extra space
        // x.x.... 1 + 2 +3 +4 +5=
        // x..x...
        // .x.x...
        //
        // x...x..
        // .x..x..
        // ..x.x..
        //
        // x....x.
        // .x...x.
        // ..x..x.
        // ...x.x.
        //
        // x.....x
        // .x....x
        // ..x...x
        // ...x..x
        // ....x.x

        assert_eq!(get_counts(8, &vec![2,1,1]), 10); // 2 extra space
        // 3 groups, 2 extra space => 9
        // First moves: 3 options
        // Second moves: 1, 2 or 3 options
        // Third moves: 1, 2 or 3 options
        //
        // 1 + (1+2) + (1+2+3)
        // xx.x.x.. 1
        //
        //  xx.x..x.
        //  xx..x.x. 2
        //
        //  .xx.x.x. 1
        //          =3
        //
        //  xx.x...x 1
        //
        //  xx..x..x 2
        //  .xx.x..x
        //
        //  xx...x.x 3
        //  .xx..x.x
        //  ..xx.x.x
        //          = 6
    }

    #[test]
    fn field_empty() {
        let f = Field::new("......", true);
        let counts: Vec<i64> = vec![];
        assert_eq!(f.get_options_alt(&counts), 1);
    }
//
    #[test]
    fn field_ready() {
        let f = Field::new("...###", true);
        let counts: Vec<i64> = vec![3];
        assert_eq!(f.get_options_alt(&counts), 1);
    }

    #[test]
    fn field_simple() {
        let f = Field::new("...???", true);
        let counts: Vec<i64> = vec![3];
        assert_eq!(f.get_options_alt(&counts), 1);
        let counts: Vec<i64> = vec![1];
        assert_eq!(f.get_options_alt(&counts), 3);
    }

    #[test]
    fn field1() {
        let f = Field::new("??", true);
        let counts: Vec<i64> = vec![1];
        assert_eq!(f.get_options_alt(&counts), 2);
    }

    #[test]
    fn field() {
        let f = Field::new("??..??", true);
        let counts: Vec<i64> = vec![1,1];
        assert_eq!(f.get_options_alt(&counts), 4);
    }

    #[test]
    fn field2() {
        let f = Field::new("###????", true);
        let counts: Vec<i64> = vec![3,2];
        assert_eq!(f.get_options_alt(&counts), 2);

        let f = Field::new("?###????", true);
        let counts: Vec<i64> = vec![3,2];
        assert_eq!(f.get_options_alt(&counts), 2);

        let f = Field::new("?????", true);
        let counts: Vec<i64> = vec![2,1];
        assert_eq!(f.get_options_alt(&counts), 3);

        let f = Field::new("?###??????", true);
        let counts: Vec<i64> = vec![3,2,1];
        assert_eq!(f.get_options_alt(&counts), 3);
    }

    #[test]
    fn line(){
        assert_eq!(read_line("###.### 3,3"            ,1), 1);

        assert_eq!(read_line("???.### 1,1,3"            ,1), 1);
        assert_eq!(read_line(".??..??...?##. 1,1,3"     ,1), 4);
        assert_eq!(read_line("?#?#?#?#?#?#?#? 1,3,1,6"  ,1), 1);
        assert_eq!(read_line("????.#...#... 4,1,1 "     ,1), 1);
        assert_eq!(read_line("????.######..#####. 1,6,5",1), 4);
        assert_eq!(read_line("?###???????? 3,2,1 "      ,1),10);

        //assert_eq!(read_line("..???#??.?????? 4,3",      5), 10);
        //assert_eq!(read_line("##??#??#?..??? 9,1,1",     5), 10);

        //assert_eq!(read_line("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3", 1), 1);
        assert_eq!(read_line("???.### 1,1,3"            ,5), 1);
        assert_eq!(read_line(".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##. 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3", 1), 16384);
        assert_eq!(read_line(".??..??...?##. 1,1,3"     ,5), 16384);
        assert_eq!(read_line("?#?#?#?#?#?#?#? 1,3,1,6"  ,5), 1);
        assert_eq!(read_line("????.#...#... 4,1,1 "     ,5), 16);
        assert_eq!(read_line("????.######..#####. 1,6,5",5), 2500);
        assert_eq!(read_line("?###???????? 3,2,1 "      ,5), 506250);
    }
}
