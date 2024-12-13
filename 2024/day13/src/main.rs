use clap::Parser;
use std::fs;
use regex::Regex;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}




fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn get_equation(ln: &str) -> (i64, i64) {
    let re: Regex = Regex::new(r"Button (A|B)\: X\+([0-9]*), Y\+([0-9]*)").unwrap();
    let res = re.captures(ln);
    dbg!(&ln);
    match res {
        None => {
            panic!("Should not happen");
        },
        Some(m) => {
            let a = m[2].parse::<i64>().unwrap();
            let b = m[3].parse::<i64>().unwrap();
            return (a,b);
        }
    }
    (0,0)
}


fn mat_mul_v(m: ((i64,i64),(i64,i64)), vec: (i64,i64)) -> (i64,i64) {
    (m.0.0 * vec.0 + m.0.1 * vec.1, 
     m.1.0 * vec.0 + m.1.1 * vec.1)
}

//fn mat_mul(a: ((i64,i64),(i64,i64)), b: ((i64,i64),(i64,i64))) -> ((i64,i64),(i64,i64)) {
//}

fn vec_div(vec: (i64, i64), div: i64) -> (i64, i64) {
    (vec.0 / div, vec.1 / div)
}

fn solve(eq1: (i64, i64), eq2: (i64,i64), ans: (i64, i64)) -> Option<(i64, i64)> {
    let a = eq1.0;
    let c = eq1.1;
    let b = eq2.0;
    let d = eq2.1;
    let div = a * d - b * c;
    // Get inverse of
    // (a b)
    // (c d)
    let inv = ((d, -b), (-c, a));
    dbg!(&inv);
    dbg!(&div);
    let tmp = mat_mul_v(inv, ans);
    dbg!(&tmp);
    let solv = vec_div(tmp, div);
    dbg!(&solv);
    println!("{} {}", solv.0 * eq1.0 + solv.1 * eq2.0, solv.0 * eq1.1 + solv.1 * eq2.1);
    if solv.0 * eq1.0 + solv.1 * eq2.0 == ans.0 {
        Some(solv)
    } else {
        None
    }

}

fn read_contents(cont: &str) -> (i64, i64) {
    let re: Regex = Regex::new(r"Prize\: X=([0-9]*), Y=([0-9]*)").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut w = 0;

    //let re: Regex = Regex::new(r"Button A|B\(([0-9]*),([0-9]*)\)").unwrap();
    let mut eq1 = (0,0);
    let mut eq2 = (0,0);
    let mut ans = (0,0);
    for ln in cont.lines() {
        w = (w + 1) % 4;
        match w {
            1 => {
                eq1 = get_equation(&ln);
            }
            2 => {
                eq2 = get_equation(&ln);
            }
            3 => {
                let res = re.captures(&ln);
                match res {
                    None => {
                        panic!("Should not happen");
                    }
                    Some(m) => {
                        let a = m[1].parse::<i64>().unwrap();
                        let b = m[2].parse::<i64>().unwrap();
                        ans = (a,b);
                        println!("Solve");
                        dbg!(eq1);
                        dbg!(eq2);
                        dbg!(ans);
                        match solve(eq1, eq2, ans) {
                            Some(val) => {
                                part1 += val.0 * 3 + val.1;
                            }
                            None => {
                                continue;
                            }
                        }
                    }
                }
            }
            _ => continue
        }

    }
    (part1, part2)

}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(read_contents(&a).0, 480);
        //assert_eq!(read_contents(&a).1, 81);
    }

    #[test] 
    fn solvet() {
        let eq1 = (94, 34);
        let eq2 = (22, 67);
        let ans = (8400, 5400);
        assert_eq!(solve(eq1, eq2, ans), Some((80, 40)));
    }

}
