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
            let a = m[2].parse::<i64>().expect("Should be a number");
            let b = m[3].parse::<i64>().expect("Should be a number");
            (a,b)
        }
    }
}


fn mat_mul_v(m: ((i64,i64),(i64,i64)), vec: (i64,i64)) -> (i64,i64) {
    // Multiply a vector (length 2) with a matrix (2x2)
    (m.0.0 * vec.0 + m.0.1 * vec.1, 
     m.1.0 * vec.0 + m.1.1 * vec.1)
}

fn vec_div(vec: (i64, i64), div: i64) -> (i64, i64) {
    // Divide a vector (length 2) by a scalar
    (vec.0 / div, vec.1 / div)
}

fn solve(button_a: (i64, i64), button_b: (i64,i64), ans: (i64, i64)) -> Option<(i64, i64)> {
    // Solves a system of equations
    let a = button_a.0;
    let b = button_b.0;
    let c = button_a.1;
    let d = button_b.1;
    let div = a * d - b * c;

    // Get inverse matrix
    let inv = ((d, -b), (-c, a));
    let solv = vec_div(mat_mul_v(inv, ans), div);
    if (solv.0 * button_a.0 + solv.1 * button_b.0 == ans.0) & 
        (solv.0 * button_a.1 + solv.1 * button_b.1 == ans.1)
    {
        Some(solv)
    } else {
        None
    }
}

fn get_cost(button_a: (i64, i64), button_b: (i64,i64), ans: (i64, i64)) -> i64 {
    // Calculates the cost of the solution
    // 3 coins for each A button press
    // 1 coin for each B button press
    let solv = solve(button_a, button_b, ans);
    match solv {
        None => 0,
        Some(val) => {
            val.0 * 3 + val.1
        }
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let re: Regex = Regex::new(r"Prize\: X=([0-9]*), Y=([0-9]*)").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut w = 0;

    // This will be added in part2
    let add = 10_000_000_000_000;

    let mut button_a = (0,0);
    let mut button_b = (0,0);
    for ln in cont.lines() {
        w = (w + 1) % 4; // input is in 4 line blocks
        match w {
            // First line gives 
            1 => { button_a = get_equation(ln); }
            2 => { button_b = get_equation(ln); }
            3 => {
                let res = re.captures(ln);
                match res {
                    None => {
                        panic!("No match found");
                    }
                    Some(m) => {
                        let a = m[1].parse::<i64>().unwrap();
                        let b = m[2].parse::<i64>().unwrap();
                        part1 += get_cost(button_a, button_b, (a,b));
                        part2 += get_cost(button_a, button_b, (a + add, b + add));
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
        assert_eq!(read_contents(&a).1, 875318608908);
    }

    #[test] 
    fn solver() {
        let button_a = (94, 34);
        let button_b = (22, 67);
        let ans = (8400, 5400);
        assert_eq!(solve(button_a, button_b, ans), Some((80, 40)));
    }

}
