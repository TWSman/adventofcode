use clap::Parser;
use std::collections::BTreeMap;
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
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i64, i64) {
    let list = read_passports(cont);
    let part1 = get_part1(&list);
    let part2 = get_part2(&list);
    (part1, part2)
}

#[derive(Debug)]
struct Passport {
    map: BTreeMap<Field, String>,
}

#[derive(Ord, PartialOrd, Debug, Eq, PartialEq)]
enum Field {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

impl Field {
    fn new(ln: &str) -> Self {
        match ln {
            "byr" => Field::BirthYear,
            "iyr" => Field::IssueYear,
            "eyr" => Field::ExpirationYear,
            "hgt" => Field::Height,
            "hcl" => Field::HairColor,
            "ecl" => Field::EyeColor,
            "pid" => Field::PassportID,
            "cid" => Field::CountryID,
            _ => panic!("Unknown key: {ln}"),
        }
    }

    fn validate_data(&self, value: &String) -> bool {
        match self {
            Field::BirthYear => validate_number(value, 1920, 2002),
            Field::IssueYear => validate_number(value, 2010, 2020),
            Field::ExpirationYear => validate_number(value, 2020, 2030),
            Field::Height => {
                if value.ends_with("cm") {
                    if value.len() != 5 {
                        false
                    } else {
                        validate_number(&value[..3], 150, 193)
                    }
                } else if value.ends_with("in") {
                    if value.len() != 4 {
                        false
                    } else {
                        validate_number(&value[..2], 59, 76)
                    }
                } else {
                    false
                }
            }
            Field::HairColor => value.starts_with('#'),
            Field::EyeColor => {
                for a in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"] {
                    if a == value {
                        return true;
                    }
                }
                false
            }
            Field::PassportID => value.len() == 9 && value.parse::<u32>().is_ok(),
            Field::CountryID => true,
        }
    }
}

fn validate_number(input: &str, min: i32, max: i32) -> bool {
    match input.parse::<i32>() {
        Err(_) => false,
        Ok(val) => val >= min && val <= max,
    }
}

impl Passport {
    fn valid_part1(&self) -> bool {
        for key in [
            Field::BirthYear,
            Field::IssueYear,
            Field::ExpirationYear,
            Field::Height,
            Field::HairColor,
            Field::EyeColor,
            Field::PassportID,
        ] {
            if !self.map.contains_key(&key) {
                return false;
            }
        }
        true
    }

    fn valid_part2(&self) -> bool {
        for key in [
            Field::BirthYear,
            Field::IssueYear,
            Field::ExpirationYear,
            Field::Height,
            Field::HairColor,
            Field::EyeColor,
            Field::PassportID,
        ] {
            match self.map.get(&key) {
                None => return false, // Key must exist
                Some(val) if !key.validate_data(val) => {
                    return false; // Matching data must be valid
                }
                _ => continue,
            }
        }
        true // Everything is fine
    }
}

fn read_passports(cont: &str) -> Vec<Passport> {
    let mut out = Vec::new();
    let mut map = BTreeMap::new();
    for line in cont.lines() {
        if line.is_empty() {
            out.push(Passport { map });
            map = BTreeMap::new();
            continue;
        }
        for tmp in line.split_whitespace() {
            let (key, val) = tmp.split_once(':').unwrap();
            map.insert(Field::new(key), val.to_string());
        }
    }
    out.push(Passport { map });
    out
}

fn get_part1(list: &[Passport]) -> i64 {
    list.iter().filter(|p| p.valid_part1()).count() as i64
}

fn get_part2(list: &[Passport]) -> i64 {
    list.iter().filter(|p| p.valid_part2()).count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(read_contents(&a).0, 2);
    }

    #[test]
    fn part2() {
        let a = "2020".to_string();
        assert!(validate_number(&a, 2020, 2030));
        assert!(!validate_number(&a, 2021, 2030));
        let b = "a2020".to_string();
        assert!(!validate_number(&b, 2020, 2030));

        let b = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let list = read_passports(&b);
        assert!(!list[0].valid_part2());
        assert!(!list[1].valid_part2());
        assert!(!list[2].valid_part2());
        assert!(!list[3].valid_part2());
        assert!(list[5].valid_part2());
        assert!(list[6].valid_part2());
        assert!(list[7].valid_part2());
        assert!(list[8].valid_part2());
    }
}
