use clap::Parser;
use std::fs;
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
}

impl Region {
    const fn new(x: i64, y: i64, c: char) -> Self {
        Self {start: (x,y), c, points: Vec::new(), area: 0, border: Vec::new()}
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

#[derive(Debug, Clone)]
struct Border {
    start: (i64, i64),
    end: (i64, i64),
    direction: Dir,
}

impl Border {
    fn new(x: i64, y: i64, direction: Dir) -> Self {
        let (start ,end) = get_start_end(x,y, direction);
        Self {start, end, direction}
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


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    // 873472 is too high
    // 872936 also too high
    println!("Part 2 answer is {part2}");
}

fn reorder_border(borders: &Vec<Border>) -> Vec<Border> {
    dbg!(&borders.len());
    let mut found: BTreeSet<usize> = BTreeSet::new();
    let mut new_vec: Vec<Border> = vec![];
    for (i,b) in borders.iter().enumerate() {
        let mut next = b;
        let mut ii = i;
        loop {
            if found.contains(&ii) {
                break;
            }
            new_vec.push(next.clone()); 
            found.insert(ii);
            match borders.iter().enumerate().filter(|(j,m)| {
                (m.start == next.end) & (m.direction != next.direction.get_opposite())
            }).next() {
                Some((j,tmp)) => {
                        next = tmp;
                        ii = j;
                    }
                None => {
                        continue;
                }
            }
        }
    }
    new_vec
}

fn locate_border_loc(border: &Border, borders: &Vec<Border>) -> usize {
    for i in 0..borders.len() {
        let b = &borders[i];
        if b.start == border.end {
            // Should be put before b
            return i;
        }
    }
    // Otherwise put this at the end
    return borders.len();
}

// If border location is (0, 1) and direction West
// Start is (1,1), end is (-1, 1)


// If border location is (1, 0) and direction North
// Start is (1, -1)
// End is (1, 1)
fn get_start_end(x: i64, y: i64, direction: Dir) -> ((i64, i64), (i64, i64)){
    match direction {
        Dir::W => ((x + 1, y), (x - 1, y)),
        Dir::E => ((x - 1, y), (x + 1, y)),
        Dir::N => ((x, y - 1), (x, y + 1)),
        Dir::S => ((x, y + 1), (x, y - 1)),
    }
}

fn read_map(cont: &str) -> (PointMap, RegionMap) {
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

            let mut heads: Vec<Point> = vec![p.clone()];
            let mut visited: BTreeSet<(i64,i64)> = BTreeSet::new();
            visited.insert((x,y));
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
    (points, regions)
}

fn get_part1(regions: &RegionMap) -> u64 {
    regions.iter().map(|(_i, m)| m.area * m.border.len() as u64).sum()
}

fn get_part2(regions: &RegionMap) -> u64 {
    regions.iter().map(|(_i, m)| {
        let mut edges = 0;
        let mut prev = m.border.last().expect("Border should exist").clone();
        let border = reorder_border(&m.border);
        for b in border {
            if !((prev.end == b.start) & (prev.direction == b.direction)) {
                edges += 1;
            }
            prev = b.clone();
        }
        dbg!(&edges);
        if edges % 2 == 0 {
            //assert!(edges % 2 == 0);
            edges * m.area
        } else {
            (edges - 1) * m.area
        }
    }).sum()
}

fn read_contents(cont: &str) -> (u64, u64) {
    let (_points, regions) = read_map(cont);
    //dbg!(&regions);
    (get_part1(&regions), get_part2(&regions))
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
