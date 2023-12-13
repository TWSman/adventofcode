use clap::Parser;
use std::fs;
use std::cmp::min;
use std::iter::zip;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");

    let res1 = read_contents(&contents);
    println!("Part 1 answer is {}", res1.0);
    println!("Part 2 answer is {}", res1.1);

}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Marker {
    Rock, //#
    Ash, //.
}

impl Marker {
    fn new(c: char) -> Marker {
        match c {
            '#' => Marker::Rock,
            '.' => Marker::Ash,
            v => panic!("Invalid character '{}'", v),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    markers: Vec<Marker>,
}

impl Line {
    fn new() -> Line {
        Line {markers: Vec::new()}
    }

    fn push(&mut self, m: Marker) {
        self.markers.push(m);
    }

    fn len(&self) -> usize {
        self.markers.len()
    }

    fn almost_equal(&self, other: &Line) -> bool {
        assert_eq!(self.len(), other.len());
        let diffs = zip(self.markers.iter(), other.markers.iter()).filter(|(x,y)| { x != y }).count();
        if diffs == 1 {
            true
        } else {
            false
        }
    }
}

fn search2(lines: &Vec<Line>) -> Option<i64> {
    let n = lines.len();
    // Loop over all possible reflection lines
    for i in 0..n {
        let mut diffs = 0;
        let max_i = min(n - i - 1, i + 1);
        // Move away from reflection line 1 step at a time
        for j in 0..max_i {
            if lines[i + 1 + j] == lines[i - j] {
                continue
            } else if lines[i+1+j].almost_equal(&lines[i-j]) {
                diffs += 1;
            } else {
                diffs = 2;
                break;
            }
        }
        if diffs == 1 {
            return Some(i as i64 + 1);
        }
    }
    None
}

fn search(lines: &Vec<Line>) -> Option<i64> {
    // Find potential candidates, 
    let matching: Vec<usize> = lines.windows(2).enumerate().filter_map(|(i,m)| {
        if m[0] == m[1] {
            Some(i)
        } else {
            None
        }
    }).collect();
    if matching.len() == 0 {
        return None;
    } 
    for i in matching {
        let max_i = min(lines.len() - i - 1, i + 1);
        if (0..max_i).all(|j| {
            lines[i + 1 + j] == lines[i - j]
        }) {
            return Some(i as i64 + 1);
        }
    }
    return None
}

fn read_contents(cont: &str) -> (i64, i64) {
    let blocks = cont.split("\n\n");
    let mut sum1 = 0;
    let mut sum2 = 0;
    for b in blocks {
        let v = read_block(b);
        sum1 += v.0;
        sum2 += v.1;
    }
    (sum1, sum2)
}

fn read_block(cont: &str) -> (i64, i64) {
    let line_width = cont.lines().next().expect("Should be at least 1 line").len() + 1;
    let mut lines: Vec<Line> = Vec::new();
    let mut columns: Vec<Line> = Vec::new();
    for (i,c) in cont.chars().enumerate() {
        if c == '\n' {
            continue;
        }
        let x = i % line_width;
        let y = i / line_width;
        let m = Marker::new(c);
        match lines.get_mut(y) {
            Some(v) => {
                v.push(m.clone());
            },
            None => {
                let mut line = Line::new();
                line.push(m.clone());
                lines.push(line);
            }
        }
        match columns.get_mut(x) {
            Some(v) => {
                v.push(m.clone());
            },
            None => {
                let mut col = Line::new();
                col.push(m.clone());
                columns.push(col);
            }
        }
    }
    let mut sum1 = 0;
    match search(&columns) {
        None => (),
        Some(v) => {
            sum1 += v
        }, 
    }

    match search(&lines) {
        None => (),
        Some(v) => {
            sum1 += 100 * v
        },
    }

    let mut sum2 = 0;
    match search2(&columns) {
        None => (),
        Some(v) => {
            sum2 += v
        },
    }
    match search2(&lines) {
        None => (),
        Some(v) => {
            sum2 += 100 * v
        },
    }
    (sum1, sum2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks() {
        let a = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        let b = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let c = "##..###.###..
##.#.#..##..#
..#.#...#.#.#
######.#.####
##..#.##....#
##.##.#..#.#.
#####....#.#.
###.########.
#######..#.##
..####..##..#
......#.###..
......#.###..
..####..#...#";

        assert_eq!(read_block(&a).0, 5);
        assert_eq!(read_block(&b).0, 400);
        assert_eq!(read_block(&c).0, 1);

    }

    #[test]
    fn blocks2() {
        let a = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        let b = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let c = "##..###.###..
##.#.#..##..#
..#.#...#.#.#
######.#.####
##..#.##....#
##.##.#..#.#.
#####....#.#.
###.########.
#######..#.##
..####..##..#
......#.###..
......#.###..
..####..#...#";

        assert_eq!(read_block(&a).1, 300);
        assert_eq!(read_block(&b).1, 100);
        //assert_eq!(read_block(&c).0, 1);

    }

    #[test]
    fn almost_equal() {
        let mut a = Line::new();
        let mut b = Line::new();
        let mut c = Line::new();

        a.push(Marker::Rock);
        b.push(Marker::Rock);
        c.push(Marker::Rock);

        a.push(Marker::Rock);
        b.push(Marker::Rock);
        c.push(Marker::Rock);

        a.push(Marker::Rock);
        b.push(Marker::Ash);
        c.push(Marker::Ash);

        a.push(Marker::Ash);
        b.push(Marker::Ash);
        c.push(Marker::Rock);

        assert!(a.almost_equal(&b));
        assert!(!a.almost_equal(&a));
        assert!(!a.almost_equal(&c));
    }


    #[test]
    fn conts() {
        let c = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(read_contents(&c).0, 405);
        //assert_eq!(read_contents(&c).1, 400);
    }
}
