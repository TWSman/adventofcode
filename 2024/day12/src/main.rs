use clap::Parser;
use std::fs;
use std::io;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type RegionMap = BTreeMap<(i64, i64), Region>;
type PointMap = BTreeMap<(i64, i64), Point>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Debug, Clone, Copy, EnumIter, Eq, PartialEq)]
enum Dir {
    N,
    E,
    S,
    W,
}


impl Dir{
    const fn get_dir(self) -> (i64, i64) {
        match self {
            Self::N => (0, 2),
            Self::E => (2, 0),
            Self::S => (0, -2),
            Self::W => (-2, 0),
        }
    }

    const fn get_next(self) -> Dir {
        match self {
            Self::N => Dir::E,
            Self::E => Dir::S,
            Self::S => Dir::W,
            Self::W => Dir::N,
        }
    }

    const fn get_opposite(self) -> Dir {
        match self {
            Self::N => Dir::S,
            Self::E => Dir::W,
            Self::S => Dir::N,
            Self::W => Dir::E,
        }
    }
}

#[derive(Debug, Clone)]
struct Region {
    points: Vec<(i64, i64)>,
    c: char,
    start: (i64, i64),
    area: u64,
    border: Vec<Border>,
    edges: u64,
}

impl Region {
    const fn new(x: i64, y: i64, c: char) -> Self {
        Self {start: (x,y), c, points: Vec::new(), area: 0, edges: 0, border: Vec::new()}
    }

    fn add_point(&mut self, x: i64, y: i64) {
        self.points.push((x,y));
        self.area += 1;
    }
}


#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    c: char,
    region: Option<(i64, i64)>, // Region is defined by the first point in that region
    directions: Vec<Dir>,
    outside: Vec<Dir>,
}

#[derive(Debug, Clone, Copy)]
struct Border {
    start: (i64, i64),
    end: (i64, i64),
    center: (i64, i64),
    from: (i64, i64), // Which square defined this one
    direction: Dir,
    edge_number: Option<usize>,
}

impl Border {
    fn new(x: i64, y: i64, from: (i64, i64), direction: Dir) -> Self {
        let (start ,end, center) = get_start_end(x,y, direction);
        Self {start, end, center, from, direction, edge_number: None}
    }
}

impl Point {
    const fn new(x: i64, y: i64, c: char) -> Self {
        Self {x, y, c, region: None, directions: Vec::new(), outside: Vec::new()}
    }
}


fn print_field(points: &PointMap, dim: (i64, i64)) {
    let nx = usize::try_from(dim.0).expect("Should work");
    let ny = usize::try_from(dim.1).expect("Should work");
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; nx]; ny];
    for ((x,y),p) in points {
        let i = usize::try_from(*x).expect("x should be nonnegative") / 2;
        let j = usize::try_from(-*y).expect("y should be nonpositive") / 2; 
        grid[j][i] = p.c;
    }
    for ln in grid {
        println!("{}", ln.into_iter().collect::<String>());
    }
}

fn print_region(points: &Vec<(i64, i64)>, borders: &Vec<Border>, c: char, dim: (i64, i64), add_numbers: bool, which: bool) {
    let nx = usize::try_from(dim.0).expect("Should work");
    let ny = usize::try_from(dim.1).expect("Should work");
    let mut grid: Vec<Vec<char>> = vec![vec![' '; nx * 2 + 1]; ny * 2 + 1];
    for i in 0..nx {
        for j in 0..ny {
            grid[i*2+1][j*2+1] = '.';
        }
    }
    let mut ymin = 400;
    let mut ymax = 0;
    let mut xmin = 400;
    let mut xmax = 0;
    for p in points {
        let i = usize::try_from(p.0 + 1).expect("x should be nonnegative");
        let j = usize::try_from(-p.1 + 1).expect("y should be nonpositive"); 
        if j > ymax {
            ymax = j
        } 
        if j < ymin {
            ymin = j
        }
        grid[j][i] = c;
    }
    for b in borders {
        let i = usize::try_from(b.center.0 + 1).expect("x should be nonnegative");
        let j = usize::try_from(-b.center.1 + 1).expect("y should be nonpositive"); 

        if b.edge_number.is_some() & add_numbers {
            let edge_number = b.edge_number.unwrap();
            let a = if which {
                edge_number % 10
            } else {
                edge_number / 10
            };
            grid[j][i] = char::from_digit(a as u32, 10).unwrap();
        } else {
            grid[j][i] = match b.direction {
                Dir::N | Dir::S => '|',
                Dir::W | Dir::E => '-',
            };
        }
        if j > ymax {
            ymax = j;
        } 
        if j < ymin {
            ymin = j;
        }
        if i > xmax {
            xmax = i;
        }
        if i < xmin {
            xmin = i;
        }
    }
    for (i,ln) in grid.iter().enumerate() {
        if i < ymin {
            continue
        }
        if i > ymax {
            continue
        }
        println!("{}", ln.iter().enumerate().skip(xmin).filter_map(|(i,m)| {
            if i <= xmax {
                Some(m)
            } else {
                None
            }
        }).collect::<String>());
    }
}



fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn reorder_border(borders: &[Border]) -> Vec<Border> {
    let num_borders = borders.iter().enumerate().collect::<Vec<(usize, &Border)>>();
    let mut found: BTreeSet<usize> = BTreeSet::new();
    let mut new_vec: Vec<Border> = vec![];
    for (i,b) in &num_borders {
        let mut inturn = b;
        let mut ii = i;
        loop {
            if found.contains(ii) {
                break;
            }
            new_vec.push(**inturn); 
            found.insert(*ii);

            let cc: Vec<_> = num_borders.iter().filter(|(_,m)| {
                // This is a candidate
                (m.start == inturn.end) & (m.direction != inturn.direction.get_opposite())
            }).collect();

            // With complicated borders there can be multiple candidates
            // It seems to be enough the find the border that shares the inside point
            let cc2: Vec<_> = if cc.len() >= 2 {
                cc.iter().filter(|(_,m)| {
                    inturn.from == m.from
                }).collect()
            } else {
                cc.iter().collect()
            };
            assert_eq!(cc2.len(), 1);
            match cc2.first() {
                Some((j,tmp)) => {
                    inturn = tmp;
                    ii = j;
                }
                None => {
                    break;
                }
            }
        }
    }
    new_vec
}

fn get_start_end(x: i64, y: i64, direction: Dir) -> ((i64, i64), (i64, i64), (i64,i64)){
    match direction {
        Dir::W => ((x + 1, y), (x - 1, y), (x,y)),
        Dir::E => ((x - 1, y), (x + 1, y), (x,y)),
        Dir::N => ((x, y - 1), (x, y + 1), (x,y)),
        Dir::S => ((x, y + 1), (x, y - 1), (x,y)),
    }
}

fn read_map(cont: &str) -> (PointMap, RegionMap, (i64, i64)) {
    let height = cont.lines().count();
    let width = cont.lines().next().unwrap().chars().count();
    let dim = (i64::try_from(width).unwrap(), i64::try_from(height).unwrap());
    let mut points: PointMap = cont.lines().enumerate().flat_map(|(i, ln)| {
            let y = -i64::try_from(2 * i).unwrap();
            ln.chars().enumerate().map(move |(j, c)| {
                let x = i64::try_from(2 * j).unwrap();
                ((x, y), Point::new(x, y, c))
            })
        }).collect::<PointMap>();

    let points_snapshot = points.clone();
    for p in points.values_mut() {
        let x = p.x;
        let y = p.y;
        for dir in Dir::iter() {
            let d = dir.get_dir();
            match points_snapshot.get(&(x + d.0, y + d.1)) {
                Some(val) if val.c == p.c => {
                    p.directions.push(dir);
                }
                _ => {
                    p.outside.push(dir);
                },
            }
        }
    }

    print_field(&points, dim);
    let mut regions: RegionMap = BTreeMap::new();
    for row in 0..height {
        for col in 0..width {
            let (x,y) = (2 * col as i64, -(2 * row as i64));
            let p = points.get_mut(&(x, y)).unwrap();
            if p.region.is_some() {
                // This already belongs to a region
                continue;
            }
            let region = (x, y);
            p.region = Some(region);
            let mut reg = Region::new(x, y, p.c);
            reg.add_point(x, y);

            // Start looking from the region origin
            let mut heads: Vec<Point> = vec![p.clone()];
            let mut visited: BTreeSet<(i64,i64)> = BTreeSet::new();
            visited.insert((x,y));
            // Loop until this region has been exhausted
            loop {
                match heads.pop() {
                    None => {
                        // Stop if there are no more heads
                        break;
                    },
                    Some(head) => {
                        for dir in &head.outside {
                            let b = Border::new(head.x + dir.get_dir().0 / 2,
                                    head.y + dir.get_dir().1 / 2,
                                    (head.x, head.y),
                                    dir.get_next(),
                                );
                            reg.border.push(b);
                        }
                        for dir in &head.directions {
                            let (dx,dy) = dir.get_dir();
                            if visited.contains(&(head.x  + dx, head.y + dy)) {
                                // If we have already visited this head, we can stop
                                continue;
                            }
                            let new_head = points.get_mut(&(head.x  + dx, head.y + dy)).unwrap();
                            // Keep track of the region for each point
                            new_head.region = Some(region);
                            reg.add_point(new_head.x, new_head.y);
                            visited.insert((new_head.x, new_head.y));
                            heads.push(new_head.clone());
                        }
                    }
                }
            }
            regions.insert(
                region,
                reg);
        }
    }
    (points, regions, dim)
}

fn get_part1(regions: &RegionMap) -> u64 {
    regions.iter().map(|(_i, m)| m.area * m.border.len() as u64).sum()
}

fn get_part2(regions: &mut RegionMap, dim: (i64, i64), print: bool) -> u64 {
    let res = regions.iter_mut().map(|(_i, m)| {
        let mut edges = 0;
        let mut prev = m.border.last().expect("Border should exist").clone();
        let mut border = reorder_border(&m.border);
        let mut start = border.first().unwrap().clone();
        for b in &mut border {
            if b.end == start.start {
                // We made a loop
                if b.direction == start.direction {
                    // This is to avoid double counting edges
                    continue;
                }
            }
            if prev.end != b.start {
                // We jump to a new sub border
                start = *b;
            }
            if !((prev.end == b.start) & (prev.direction == b.direction)) {
                edges += 1;
                b.edge_number = Some(edges as usize);
            }
            prev = *b;
        }

        m.border = border;
        m.edges = edges;
        // There must an even number of edges
        assert!(edges % 2 == 0);
        edges * m.area
    }).sum();

    if !print {
        return res
    }

    // One by one print all the regions for debuggin
    for (j,(i, f)) in regions.iter().enumerate().skip(33) {
        if f.border.len() == 4 {
            continue;
        }
        if f.edges <= 6 {
            continue;
        }
        print_region(&f.points, &f.border, f.c, dim, false, true);
        print_region(&f.points, &f.border, f.c, dim, true, true);
        print_region(&f.points, &f.border, f.c, dim, true, false);
        println!("{}: {} {}",j + 1, i.0, i.1);
        println!("{} edges", f.edges);
        println!("{} borders", f.border.len());

        println!("Press Enter to continue to the next iteration, or type 'exit' to quit:");
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
    //let (_i, f) = regions.first_key_value().unwrap();
    res
}

fn read_contents(cont: &str) -> (u64, u64) {
    let (_points, mut regions, dim) = read_map(cont);
    (get_part1(&regions), get_part2(&mut regions, dim, false))
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "AAAA
BBCD
BBCC
EEEC";

        // Should have A: 4, B: 4, C: 8, D: 4
        assert_eq!(read_contents(&a).0, 140);
        assert_eq!(read_contents(&a).1, 80);
        let c = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        assert_eq!(read_contents(&c).0, 1184);
        assert_eq!(read_contents(&c).1, 368);

        let b = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(read_contents(&b).0, 1930);
        assert_eq!(read_contents(&b).1, 1206);
    }
}
