use clap::Parser;
use std::fs;
use indexmap::IndexMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();

    let mut contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");
    let res1 = part1(&contents);
    println!("Part 1 answer is {}", res1);

    // Logic in part2() breaks if there is a trailing newline character
    if contents.ends_with('\n') {
        contents.pop();
    }
    let res2 = part2(&contents);
    println!("Part 2 answer is {}", res2);
}

fn hash(input: &str) -> u64 {
    input.chars().filter_map(|c| {
        match c { 
            '\n' => None,
            c => Some(c as u64),
        }
    }).fold(0, |acc, x| {17 * (acc + x) % 256})
}


#[derive(Debug)]
struct Box {
    number: i64,
    lens_list: IndexMap<String, i64>,
}

impl Box {
    fn new(i: i64) -> Box {
        Box {number: i, lens_list: IndexMap::new()} 
    }

    fn delete(&mut self, lab: String) {
        self.lens_list.shift_remove(&lab);
    }

    fn insert(&mut self, lab: String, lens: i64) {
        self.lens_list.insert(lab, lens);
    }

    fn power(&self) -> i64 {
        self.lens_list.iter().enumerate().map(|(i, (_lab, f))| {
            (self.number + 1) * ((i as i64) + 1) * f
        }).sum()
    }

    fn len(&self) -> usize {
        self.lens_list.len()
    }
}

struct Boxes {
    box_list: Vec<Box>
}

impl Boxes {
    fn new() -> Boxes {
        Boxes {box_list: (0..256).map(|i| Box::new(i)).collect()}
    }

    fn get_box(&mut self, i: usize) -> &mut Box {
        self.box_list.get_mut(i).unwrap()
    }

    fn power(&self) -> i64 {
        self.box_list.iter().map(|b| {
            b.power()
        }).sum()
    }
}

fn part1(cont: &str) -> u64 {
    cont.split(",").map(|s| {
        hash(&s)
    }).sum()
}


fn part2(cont: &str) -> i64 {
    let mut boxes = Boxes::new();
    for s in cont.split(",") {
        let (label, f): (&str, i64) = 
        match s.chars().last().unwrap() {
            c if c.is_numeric() => {
                (&s[..s.len() -2], c.to_digit(10).unwrap() as i64)
            }
            '-' => (&s[..s.len() - 1], -1),
            v => {panic!("Something unexpected {}", v);},
        };
        let hash = hash(&label);
        let boks = boxes.get_box(hash as usize);
        if f == -1 {
            boks.delete(label.to_string());
        } else {
            boks.insert(label.to_string(), f);
        }
    }
    boxes.power()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(&a), 1320);
        assert_eq!(part2(&a), 145);
    }

        
    #[test]
    fn box_power() {
        let mut b = Box::new(0);
        b.insert("rn".to_string(), 1);
        b.insert("cm".to_string(), 2);
        dbg!(&b);
        assert_eq!(b.power(), 5);
    }

    #[test]
    fn hash_test() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }
}
