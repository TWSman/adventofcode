use clap::Parser;
use std::fs;
use regex::Regex;
use std::io;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn from_str(ln: &str) -> Self {
        let re: Regex = Regex::new(r"p=([0-9]*),([0-9]*) v=(-?[0-9]*),(-?[0-9]*)").unwrap();
        match re.captures(ln) {
            None => {
                panic!("Should not happen");
            } 
            Some(m) => {
                let x = m[1].parse::<i64>().expect("Should be a number");
                let y = m[2].parse::<i64>().expect("Should be a number");
                let vx = m[3].parse::<i64>().expect("Should be a number");
                let vy = m[4].parse::<i64>().expect("Should be a number");
                Self {x, y, vx, vy}
            }
        }
    }

    fn take_steps(&self, steps: i64, width: i64, height: i64) -> Robot {
        let x = (self.x + self.vx * steps).rem_euclid(width);
        let y = (self.y + self.vy * steps).rem_euclid(height);
        Self {x, y, vx: self.vx, vy: self.y}
    }
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents, WIDTH, HEIGHT);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn get_robots(cont: &str) -> Vec<Robot> {
    let robots: Vec<Robot> = cont.lines().map(|ln| Robot::from_str(ln)).collect();
    robots
}


fn print_field(robots: &Vec<Robot>, dim: (i64, i64)) {
    let nx = usize::try_from(dim.0).expect("Should work");
    let ny = usize::try_from(dim.1).expect("Should work");
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; nx]; ny];
    for r in robots {
        let i = usize::try_from(r.x).expect("x should be nonnegative");
        let j = usize::try_from(r.y).expect("y should be nonnegative"); 
        grid[j][i] = match grid[j][i] {
            '.' => '1',
            '1' => '2',
            _ => '#'
        };
    }
    for ln in grid {
        println!("{}", ln.into_iter().collect::<String>());
    }
}

fn read_contents(cont: &str, width: i64, height: i64) -> (i64, i64) {
    let steps = 100;
    let robots = get_robots(cont);
    let robots2: Vec<Robot> = robots.iter().map(|m| m.take_steps(steps, width ,height)).collect();
    let limx = width / 2;
    let limy = height / 2;
    dbg!(&limx);
    dbg!(&limy);
    // Top left
    let q1 = robots2.iter().filter(|m| (m.x < limx) & (m.y < limy)).count();
    // Bottom Left
    let q2 = robots2.iter().filter(|m| (m.x < limx) & (m.y > limy)).count();
    // Bottom Right
    let q3 = robots2.iter().filter(|m| (m.x > limx) & (m.y > limy)).count();
    // Top Right
    let q4 = robots2.iter().filter(|m| (m.x > limx) & (m.y < limy)).count();
    let part1 = q1 * q2 * q3 * q4;
    // Somekind of structures seem to happen at 
    // 5531, 5632, 5733 (loop of 101)
    // 5580, 5683, 5686 (loop of 103)
    //
    // 103 loop starts from 18
    // 101 loop starts from 77
    let mut part2 = 0;
    for i in 0..10403 {
        // If i has form 
        // 77 + 101 * n => (i -77 ) % 101 == 0
        // 18 + 103 * m => (i - 18) % 103 == 0
        // This should be an answer
        if (i - 77)% 101 != 0 {
            continue
        }
        if (i - 18)% 103 != 0 {
            continue
        }
        part2 = i;
        let robo = robots.iter().map(|m| m.take_steps(i, width, height)).collect::<Vec<Robot>>();
        print_field(&robo, (width, height));
        println!("{i}: Press Enter to continue to the next iteration, or type 'exit' to quit:");
        // Read input from the user
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read input");
        // Trim the input and check for the exit condition
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting the loop.");
            break;
        }
    }
    (part1 as i64, part2)

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let a = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(read_contents(&a, 11, 7).0, 12);
    }

}
