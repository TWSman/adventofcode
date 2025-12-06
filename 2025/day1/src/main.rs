use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}


fn read_contents(cont: &str) -> (usize, usize) {
    // Store moves in a vector, where negative values denote moving left and positive values denote
    // moving right
    let moves: Vec<i64> = cont.lines().map(|ln| {
        match ln.chars().next().unwrap() {
            'R' => ln[1..].parse::<i64>().unwrap(),
            'L' => -ln[1..].parse::<i64>().unwrap(),
            _ => panic!()
        }
    }).collect();

    let mut state: i64 = 50;
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    for mv in moves {
        let state_old = state;
        // Add number of clicks to state
        state += mv;
        if state < 0 {
            // Underflow
            let increase = usize::try_from(state / -100).unwrap();

            part2 += increase;
            if state_old != 0 {
                part2 += 1;
            }
        }
        if state == 0 {
            // We moved to 0 naturally
            part2 += 1;
        }
        if state >= 100 {
            // Overflow
            let increase = usize::try_from(state / 100).unwrap();
            part2 += increase;
        }

        // 
        state = state.rem_euclid(100);
        if state == 0 {
            part1 += 1;
        }
    }
    (part1, part2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(read_contents(&a).0, 3);
        assert_eq!(read_contents(&a).1, 6);

        let c= "R50
L200
R200
";
        assert_eq!(read_contents(&c).1, 5);
    }
}
