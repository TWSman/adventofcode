use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use priority_queue::PriorityQueue;
use std::fmt::Display;
use core::fmt;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Debug, Clone, Copy, EnumIter)]
enum Dir {
    N,
    E,
    S,
    W,
}


impl Display for Dir{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dir::N => write!(f, "^"),
            Dir::S => write!(f, "v"),
            Dir::W => write!(f, "<"),
            Dir::E => write!(f, ">"),
        }
    }
}

impl Dir{
    const fn get_dir(self) -> (i64, i64) {
        match self {
            Self::N => (0, -1),
            Self::E => (1, 0),
            Self::S => (0, 1),
            Self::W => (-1, 0),
        }
    }
}

struct Map {
    points: BTreeMap<(i64, i64), i64>,
    width: i64,
    height: i64,
}

impl Map {
    fn print_map(&self, path: Option<Vec<(i64,i64)>>, max_t: i64) {
        let nx = usize::try_from(self.width).expect("Should work");
        let ny = usize::try_from(self.height).expect("Should work");
        let mut grid: Vec<Vec<char>> = vec![vec!['.'; nx]; ny];
        if let Some(p) = path {
            for (x,y) in p {
                grid[y as usize][x as usize] = 'O';
            }
        }
        for (r, t) in self.points.iter() {
            if *t >= max_t {
                continue;
            }
            let i = usize::try_from(r.0).expect("x should be nonnegative");
            let j = usize::try_from(r.1).expect("y should be nonnegative"); 
            grid[j][i] = match grid[j][i] {
                '.' => '#',
                '#' => '2',
                _ => '#'
            };
        }
        for ln in grid {
            println!("{}", ln.into_iter().collect::<String>());
        }
    }
    
    fn is_empty(&self, x1: i64, y1: i64, t: i64) -> bool {
        if x1 < 0 || y1 < 0 || x1 >= self.width || y1 >= self.height {
            return false
        }
        match self.points.get(&(x1, y1)) { 
            None => true,
            Some(val) => *val >= t
        }
    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct PathHead {
    x: i64,
    y: i64,
    length: i64,
    history: Vec<(i64,i64)>,
}

impl PathHead {
    fn new(x: i64,
        y: i64,
        length: i64,
        history: Vec<(i64,i64)>
    ) -> Self {
        Self {x,
            y,
            length,
            history,
        }
    }
}

impl Display for PathHead {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "length: {}, at ({}, {})", self.length, self.x, self.y)
    }
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents, 71, 71, 1024);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {},{}", part2.0, part2.1);
}

fn read_map(cont: &str, height: i64, width: i64) -> Map {
    let points = cont.lines().enumerate().map(|(i,l)| {
        let mut it = l.split(',');
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        ((x, y), i as i64)
    }).collect::<BTreeMap<(i64, i64), i64>>();
    Map {points, height, width}
}


fn taxicab(x1: i64, y1: i64, x2: i64, y2: i64) -> i64{
    i64::abs(x1-x2) + i64::abs(y1-y2)
}

fn read_contents(cont: &str, height: i64, width: i64, max_t: i64) -> (i64, (i64, i64)) {
    let map = read_map(cont, height, width);
    map.print_map(None, max_t);
    let part1 = find_path(&map, max_t, true);
    let mut a = max_t + 1;
    let mut b = (&map.points.len() - 1) as i64;
    let mut c;
    let ind;
    println!("Start binary search");
    loop {
        c = (a + b) / 2;
        if c == a {
            ind = a;
            break;
        }
        if c == b {
            ind = b;
            break;
        }
        println!("Trying {}", c);
        if find_path(&map, c, false) == 0 {
            println!("No path found for {}", c);
            b = c; // copy c to b
        } else {
            println!("Path found for {}", c);
            a = c; // Copy c to a
        }
    }
    let tmp = map.points.iter().filter_map(|(k,v)| {
        if *v == ind {
            Some(*k)
        } else {
            None
        }
    }).collect::<Vec<_>>();
    dbg!(&tmp);
    find_path(&map, ind - 1, true);
    let part2 = tmp.first().unwrap();
    (part1, *part2)
    
}

fn find_path(map: &Map, max_t: i64, save_path: bool) -> i64 {
    //map.print_map(max_t);

    let mut paths = PriorityQueue::new();
    let mut already_found: BTreeMap<(i64, i64), i64> = BTreeMap::new();

    // Start point is top left corner (0,0)
    let start = PathHead::new(0,0, 0, Vec::new());
    // Target is bottom right corner
    let target = (map.width - 1, map.height - 1);

    let h  = taxicab(0, target.0, 0, target.1);
    already_found.insert((start.x, start.y), 0);

    let _ = &paths.push(start, -h);

    loop {
        let (path, _priority) = match paths.pop() {
            None => {
                return 0;
            },
            Some(p) => p,
        };

        if (path.x == target.0) & (path.y == target.1) {
            // We found the end
            if save_path {
                map.print_map(Some(path.history.clone()), max_t);
            }
            return path.length
        }

        for d in Dir::iter() {
            let new_x = path.x + d.get_dir().0;
            let new_y = path.y + d.get_dir().1;
            let key = (new_x, new_y);
            let h  = taxicab(new_x, target.0, new_y, target.1);
            if let Some(v) =  already_found.get(&key) {
                if v > &path.length {
                    continue
                };
            }
            if map.is_empty(new_x, new_y, max_t) {
                let mut new_vec;
                if save_path {
                    new_vec = path.history.clone();
                    new_vec.push((new_x, new_y));
                } else {
                    new_vec = Vec::new();
                }
                let new_path = PathHead::new(new_x, new_y, path.length + 1, new_vec);
                match already_found.get(&key) {
                    Some(v) if v < &new_path.length => (),
                    _ => {
                        already_found.insert(key, new_path.length);
                        paths.push(new_path.clone(), -(new_path.length + h));
                    },
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(read_contents(&a,7,7, 12).0, 22);
        assert_eq!(read_contents(&a,7,7, 12).1, (6,1));
    }

}
