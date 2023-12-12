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
        dbg!(&self.marker_string);
        dbg!(&counts);
        if counts.len() == 0 {
            //dbg!("No counts left");
            return 1;
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
                        return dbg!(sub_field.get_options_alt(&sub_counts));
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
                    return dbg!(sub_field.get_options_alt(&counts));
                }
                _ => (),
            }
        }
        let first_sec = self.secs.iter().next().unwrap();
        match first_sec.marker {
            Marker::Broken => {
                //dbg!("Self starts with a broken marker");
                //dbg!("Remove First group and create a subfield");
                let n = self.markers.len();
                //dbg!(&c1);
                //dbg!(&n);
                if n < c1 {
                    return 0;
                }
                if n > c1 {
                    let sub_field = Field::new(self.marker_string.get(c1+1..n).unwrap(), true);
                    //dbg!(&sub_field);
                    let sub_counts = counts[1..counts.len()].to_vec();
                    //dbg!(&sub_counts);
                    return dbg!(sub_field.get_options_alt(&sub_counts));
                } else {
                    return 1;
                }
            }
            Marker::Fixed =>{
                dbg!("Self starts with a fixed marker");
                dbg!("Remove First section");
                let l = first_sec.len as usize;
                let sub_field = Field::new(self.marker_string.get(l..n).unwrap(), true);
                let tmp = sub_field.get_options_alt(&counts);
                dbg!(&sub_field);
                dbg!(tmp);
                return tmp;
            }
            Marker::Unknown => {
                println!("Found unknown");
                let q = dbg!(self.unknown());
                let p = dbg!(self.possible());
                let mut p1 = max(q, c1) - c1 + 1;
                println!("{} group size", &c1);
                println!("{} unknowns", &q);
                println!("{} unknowns/broken ones", &p);
                println!("{} Possible starts", &p1);
                println!("next count: {}", &c1);
                
                match self.markers.get(q) {
                    Some(Marker::Broken) => {p1 += 1;}
                    _ => (),
                }

                // Not enough unknown / Broken
                if p < c1 {
                    return 0;
                }
                let mut sum: i64 =0 ;
                if p == c1 { // Exactly enough possibilities, remove first group
                    if n > c1 + 1{
                        let sub_field = Field::new(self.marker_string.get(c1+1..n).unwrap(), true);
                        assert_eq!(sub_field.get_len(), self.get_len() - (c1+1));
                        let sub_counts = counts[1..counts.len()].to_vec();
                        let tmp = sub_field.get_options_alt(&sub_counts);
                        return tmp;
                    } else {
                        return 1;
                    }
                }
                // Multiple possibilities
                // ?????
                for i in 0..p1 {
                    dbg!(&i);
                    dbg!(&n);
                    dbg!(&c1);
                    dbg!(i+c1);
                    match self.markers.get(i + c1) {
                        Some(Marker::Broken) => {
                            println!("Found broken, nonvalid option");
                            continue;
                        }
                        _ => (),
                    }
                    if n >= c1 + 1 {
                        match self.marker_string.get(c1 + i + 1..n) {
                            Some(v) => {
                                let sub_field = Field::new(v, true);
                                assert!(sub_field.get_len() < self.get_len());
                                dbg!(&sub_field.marker_string);
                                let sub_counts = counts[1..counts.len()].to_vec();
                                let tmp = sub_field.get_options_alt(&sub_counts);
                                println!("Got {}", tmp);
                                sum += tmp; 
                            },
                            None =>  {
                                if (counts.len() == 0) {
                                    sum += 1;
                                    dbg!(&sum);
                                }
                            }
                        }
                    } else if counts.len() > 0{
                        sum += 0;
                    }
                    dbg!(&sum);
                }
                //match self.marker_string.get(p1..n) {
                //    Some(v) => {
                //        let sub_field = Field::new(v, true);
                //        assert!(sub_field.get_len() < self.get_len());
                //        let tmp = sub_field.get_options_alt(&counts);
                //        println!("Got {}", tmp);
                //        sum += tmp; 
                //        dbg!(&sum);
                //    }
                //    None => (),
                //}
                return sum;
            }
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
    dbg!(&input);
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
    dbg!(&field);
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

    //#[test]
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
        //assert_eq!(read_line("?###???????? 3,2,1 "      ,1),10);

        assert_eq!(read_line("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3", 1), 1);
        assert_eq!(read_line("???.### 1,1,3"            ,5), 1);
        //assert_eq!(read_line(".??..??...?##. 1,1,3"     ,5), 16384);
        //assert_eq!(read_line("?#?#?#?#?#?#?#? 1,3,1,6"  ,5), 1);
        //assert_eq!(read_line("????.#...#... 4,1,1 "     ,5), 16);
        //assert_eq!(read_line("????.######..#####. 1,6,5",5), 2500);
        //assert_eq!(read_line("?###???????? 3,2,1 "      ,5), 506250);
    }
}
