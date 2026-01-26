use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Snafu {
    DoubleMinus,
    Minus,
    Zero,
    One,
    Two
}

impl Snafu {
    fn from_char(c: char) -> Self {
        match c {
            '=' => Snafu::DoubleMinus,
            '-' => Snafu::Minus,
            '0' => Snafu::Zero,
            '1' => Snafu::One,
            '2' => Snafu::Two,
            _ => panic!("Invalid Snafu char {}", c)
        }
    }

    fn from_int(i: i64) -> Self {
        match i {
            -2 => Snafu::DoubleMinus,
            -1 => Snafu::Minus,
            0 => Snafu::Zero,
            1 => Snafu::One,
            2 => Snafu::Two,
            _ => panic!("Invalid Snafu int {}", i)
        }
    }

    fn get_char(&self) -> char {
        match self {
            Snafu::DoubleMinus => '=',
            Snafu::Minus => '-',
            Snafu::Zero => '0',
            Snafu::One => '1',
            Snafu::Two => '2',
        }
    }

    fn get_val(&self) -> i64 {
        match self {
            Snafu::DoubleMinus => -2,
            Snafu::Minus => -1,
            Snafu::Zero => 0,
            Snafu::One => 1,
            Snafu::Two => 2,
        }
    }
}

struct SnafuNumber {
    vals: Vec<Snafu>, // Least significant digit first
}

impl SnafuNumber {
    fn new(input: &str) -> Self {
        let vals = input.chars().rev().map(Snafu::from_char).collect();
        Self {
            vals,
        }
    }

    fn get_decimal(&self) -> i64 {
        let mut res = 0;
        let mut base = 1;
        for c in self.vals.iter() {
            res += c.get_val() * base;
            base *= 5;
        }
        res
    }

    fn str(&self) -> String {
        self.vals.iter().rev().map(|c| c.get_char()).collect()
        
    }

    fn from_decimal(val: i64) -> Self {
        let mut vals = Vec::new();
        let mut a = val;
        let mut c;
        while a != 0 {
            c = a % 5;
            let c = (c + 2) % 5 - 2;
            assert!((-2..=2).contains(&c));
            a -= c;
            a /= 5;
            vals.push(Snafu::from_int(c));
        }
        Self {
            vals,
        }
    }
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res);  
}



fn read_contents(cont: &str) -> String {
    let numbers: Vec<SnafuNumber> = cont.lines().map(SnafuNumber::new).collect();
    let normals = numbers.iter().map(|v| v.get_decimal()).collect::<Vec<i64>>();
    let sum = normals.iter().sum::<i64>();
    SnafuNumber::from_decimal(sum).str()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snafu() {
        assert_eq!(SnafuNumber::new("1=-0-2").get_decimal(), 1747);
        assert_eq!(SnafuNumber::new("2=0=").get_decimal(), 198);
        assert_eq!(SnafuNumber::new("1=-0-2").str(), "1=-0-2");
        assert_eq!(SnafuNumber::from_decimal(198).str(), "2=0=");
        assert_eq!(SnafuNumber::from_decimal(1747).str(), "1=-0-2");

        let vals = [
            ("1=-0-2",     1747),
            ("12111",      906),
            ("2=0=",      198),
            ("21",       11),
            ("2=01",      201),
            ("111",       31),
            ("20012",     1257),
            ("112",       32),
            ("1=-1=",      353),
            ("1-12",      107),
            ("12",        7),
            ("1=",        3),
            ("122",       37),
        ];

        for (a,b) in vals.iter() {
            assert_eq!(SnafuNumber::new(a).get_decimal(), *b);
            assert_eq!(SnafuNumber::from_decimal(*b).str(), a.to_string());
        }


    }

    #[test]
    fn part1() {
        let a ="1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
        assert_eq!(read_contents(&a), "2=-1=0");
    }
    
}
