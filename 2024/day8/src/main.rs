use clap::Parser;
use std::fs;
use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Debug, Copy, Clone)]
struct Antenna {
    x: i64,
    y: i64,
    val: char,
}

type CoordSet = HashSet<(i64, i64)>;

impl Antenna {
    fn new(x:i64, y:i64, val: char) -> Self {
        Self { x, y, val }
    }

    fn coord(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

fn print_field(antennas: &Vec<&Antenna>, dim: (i64, i64)) {
    let nx = usize::try_from(dim.0).expect("Should work");
    let ny = usize::try_from(dim.1).expect("Should work");
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; nx]; ny];
    for ant in antennas {
        let i = usize::try_from(ant.x).expect("x should be nonnegative");
        let j = usize::try_from(-ant.y).expect("y should be nonpositive"); 
        grid[j][i] = ant.val;
    }
    for ln in grid {
        println!("{}", ln.into_iter().collect::<String>());
    }
}



fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    // 230 is too hight for part1
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn sum_vec(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn scale_vec(a: (i64, i64), i: i64) -> (i64, i64) {
    (a.0 * i, a.1 * i)
}


fn check_in_map(a: (i64, i64), dim: (i64,i64)) -> bool {
    !((a.0 < 0) | (a.1 > 0) | (a.0 >= dim.0) | (a.1 <= -dim.1))
}


fn read_map(cont: &str) -> Vec<Antenna> {
    let antennas: Vec<Antenna> = cont.lines().enumerate().flat_map(|(i, ln)| {
            let y = -i64::try_from(i).unwrap();
            ln.chars().enumerate().map(move |(j, c)| {
                let x = i64::try_from(j).unwrap();
                match c {
                    '.' => None,
                    val => Some(Antenna::new(x,y,val)),
                }
            })
        }).flatten()
    .collect::<Vec<Antenna>>();
    antennas
}

fn get_antinodes(cont: &str) -> Vec<(i64,i64)> {
    let antinodes: Vec<(i64, i64)> = cont.lines().enumerate().flat_map(|(i, ln)| {
            let y = -i64::try_from(i).unwrap();
            ln.chars().enumerate().map(move |(j, c)| {
                let x = i64::try_from(j).unwrap();
                match c {
                    '#' => Some((x,y)),
                    _ => None,
                }
            })
        }).flatten()
    .collect::<Vec<(i64,i64)>>();
    antinodes
}

fn dbg_pair(a: &Antenna, b: &Antenna, dim: (i64, i64), pot: (i64,i64)) {
    println!("Checking ({0}, {1}), ({2} {3})", a.x, a.y, b.x, b.y);
    println!("Result ({}, {}) is valid", pot.0, pot.1);
    let pot_ant = Antenna::new(pot.0, pot.1, '#');
    let ants = vec![a, b, &pot_ant];
    
    print_field(&ants, dim);
}

fn get_part1(a: &Antenna, b: &Antenna, dim: (i64, i64)) -> Option<(i64, i64)> {
    // dx points from b to a
    let dx = (a.x - b.x, a.y - b.y);
    // Move step dx from a
    let pot = sum_vec(a.coord(), dx);

    if check_in_map(pot, dim) {
        //dbg_pair(a, b, dim, pot);
        Some(pot)
    } else {
        //println!("({}, {}) is outside", sum.0, sum.1);
        None
    }
}

fn get_part2(a: &Antenna, b: &Antenna, dim: (i64, i64)) -> Vec<(i64, i64)> {
    println!("Checking ({0}, {1}), ({2} {3})", a.x, a.y, b.x, b.y);
    
    // dx points from b to a
    let dx = (a.x - b.x, a.y - b.y);
    // Move step dx from a

    let mut rets: Vec<(i64,i64)> = Vec::new();

    let mut i = 0;
    loop {
        let pot = sum_vec(a.coord(), scale_vec(dx, i));
        if check_in_map(pot, dim) {
            println!("Result ({}, {}) is valid", pot.0, pot.1);
            rets.push(pot);
        } else {
            println!("Result ({}, {}) is outside map", pot.0, pot.1);
            break;
        }
        i += 1;
    }
    rets
}



fn read_contents(cont: &str) -> (usize, usize) {
    let (antinodes1, antinodes2) = analyze_antinodes(cont);
    let part1 = antinodes1.len();
    let part2 = antinodes2.len();
    (part1 , part2)

}

fn analyze_antinodes(cont: &str) -> (CoordSet, CoordSet) {
    let height = i64::try_from(cont.lines().count()).unwrap();
    let width = i64::try_from(cont.lines().next().expect("First line should exist").len()).unwrap();
    println!("Width is {width}");
    println!("Height is {height}");
    let dim = (width, height);
    let antennas = read_map(cont);
    // dbg!(&antennas);
    let chars: HashSet<char> = antennas.iter().map(|m| m.val).collect();
    // dbg!(&chars);
    let mut antinodes1: CoordSet = HashSet::new();
    let mut antinodes2: CoordSet = HashSet::new();
    for c in chars {
        println!("Checking {c}");
        // dbg!(&c);
        let ant = antennas.iter().filter(|m| m.val == c).collect::<Vec<&Antenna>>();
        //dbg!(&ant);
        for i in 0..ant.len() {
            for j in 0..ant.len() {
                if i != j {
                    let a = ant.get(i).expect("Should exist");
                    let b = ant.get(j).expect("Should exist");
                    let _ = match get_part1(a, b, dim) {
                        Some(val) => antinodes1.insert(val),
                        None => false,
                    };
                    let rets = get_part2(a,b,dim);
                    for r in rets {
                        antinodes2.insert(r);
                    }
                }
            }
        }
    }
    (antinodes1, antinodes2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full() {
        let a = 
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(read_contents(&a).0, 14);
        assert_eq!(read_contents(&a).1, 34);
    }

    #[test]
    fn antinodes() {
        let a = 
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let t = "......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....#.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.";
        let ant: Vec<(i64, i64)> = get_antinodes(&t);
        let antin = analyze_antinodes(&a).0;
        dbg!(&ant);
        dbg!(&antin);
        let ant2: HashSet<&(i64,i64)> = HashSet::from_iter(ant.iter());
        for a in &antin {
            dbg!(&a);
            assert!(ant2.contains(&a));
        }
        println!("Found ones were correct");
        for a in &ant {
            if antin.contains(&a) {
                {}
            } else {
                println!("({}, {}) is missing", a.0, a.1);
            }
            //assert!(antin.contains(&a));
        }
        assert_eq!(ant.len(), antin.len());
    }
    // (4,-2) is missing
    // This results from (6, -5) and (8, -8)

    #[test]
    fn map() {
        let a = 
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(read_map(&a).len(), 7);
    }

    #[test]
    fn part1() {
        let dim = (12, 12);
        let a = Antenna::new(6,-5, 'A');
        let b = Antenna::new(8,-8, 'A');
        assert!(get_part1(&a, &b, dim).is_some());
        assert!(get_part1(&b, &a, dim).is_some());
    }

    #[test]
    fn part2() {
        let dim = (12,12);
        let a = Antenna::new(6,-6, 'A');
        let b = Antenna::new(5,-5, 'A');
        assert_eq!(get_part2(&a, &b, dim).len(), 6);
    }

    #[test]
    fn print() {
        let a = 
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let height = a.lines().count() as i64;
        let width = a.lines().next().expect("First line should exist").len() as i64;
        println!("Width is {}", width);
        println!("Height is {}", height);
        let dim = (width, height);
        let antennas = read_map(&a);
        //print_field(&antennas, dim);
        //assert_eq!(2,3);
    }
}
