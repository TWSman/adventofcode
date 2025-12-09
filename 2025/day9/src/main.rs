use clap::Parser;
use std::fs;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use shared::Dir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] struct Args {
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



#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coord {
    x: i64,
    y: i64
}

impl Coord {
    fn from_str(input: &str) -> Self {
        let res: Vec<&str> = input.split(',').collect();
        assert_eq!(res.len(), 2);
        let x = res[0].parse::<i64>().unwrap();
        let y = res[1].parse::<i64>().unwrap();
        Self {
            x,
            y,
        }
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let tiles = read_tiles(cont);
    dbg!(&tiles.len());

    let part1 = get_part1(&tiles);
    let part2 = get_part2(&tiles);
    //let part2 = 0;

    (part1, part2)
}

fn read_tiles(cont: &str) -> Vec<Coord> {
    cont.lines().map(Coord::from_str).collect()
}

fn print_grid(tiles: &BTreeSet<(i64,i64)>, red_tiles: &[Coord], max_x: i64, max_y:i64, a: Option<Coord>, b: Option<Coord>) {
    let nx = usize::try_from(max_x).expect("Should work") + 2;
    let ny = usize::try_from(max_y).expect("Should work") + 2;
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; nx]; ny];
    for x in 0..nx {
        for y in 0..ny {
            if (a.is_some() && x == a.unwrap().x.try_into().unwrap() && y == a.unwrap().y.try_into().unwrap()) || (b.is_some() && x == b.unwrap().x.try_into().unwrap() && y == b.unwrap().y.try_into().unwrap()) {
                grid[y][x] = 'O';
            } else if red_tiles.contains(&Coord {x: x.try_into().unwrap(), y: y.try_into().unwrap()}) {
                grid[y][x] = 'R';
            } else if tiles.contains(&(x as i64, y as i64)) {
                grid[y][x] = '#';
            }
        }
    }
    for ln in grid {
        println!("{}", ln.into_iter().collect::<String>());
    }
}

fn print_loop(tile_loop: &BTreeMap<(i64,i64), Dir>, red_tiles: &[Coord], max_x: i64, max_y:i64, a: Option<Coord>, b: Option<Coord>) {
    let nx = usize::try_from(max_x).expect("Should work") + 2;
    let ny = usize::try_from(max_y).expect("Should work") + 2;
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; nx]; ny];
    for x in 0..nx {
        for y in 0..ny {
            if (a.is_some() && x == a.unwrap().x.try_into().unwrap() && y == a.unwrap().y.try_into().unwrap()) || (b.is_some() && x == b.unwrap().x.try_into().unwrap() && y == b.unwrap().y.try_into().unwrap()) {
                grid[y][x] = 'O';
            } else if tile_loop.contains_key(&(x as i64, y as i64)) {
                match tile_loop.get(&(x as i64, y as i64)) {
                    Some(d) => {
                        grid[y][x] = d.get_char();
                    },
                    None => panic!("Should not happen"),
                }
            } else if red_tiles.contains(&Coord {x: x.try_into().unwrap(), y: y.try_into().unwrap()}) {
                grid[y][x] = 'R';
            }
        }
    }
    for ln in grid {
        println!("{}", ln.into_iter().collect::<String>());
    }
}

fn get_part2(tiles: &[Coord]) -> i64 {
    let n_tiles = tiles.len();
    // Collect pairs of tiles into segments
    let mut segments: Vec<(Coord, Coord)> = Vec::new();
    for i in 0..(n_tiles-1) {
        segments.push((tiles[i],tiles[i+1]));
    }

    // Connect the last to the first
    segments.push((tiles[n_tiles-1], tiles[0]));

    let mut largest_area: i64 = 0;
    for (i,a) in tiles.iter().enumerate() {
        for (j, b) in tiles.iter().enumerate() {
            if i >= j {
                // No need to double check pairs
                continue;
            }
            let candidate_area = ((b.x - a.x).abs() + 1) * ((b.y - a.y).abs() + 1);
            if candidate_area < largest_area {
                // No need to check smaller areas
                continue;
            }
            if check_pair(a, b, &segments) {
                // Check intersection with segments
                continue;
            }
            println!("Found new max ({}, {}) (id {}) and  ({}, {}) (id {}) with area {}", a.x, a.y, i, b.x, b.y, j, candidate_area);
            // Update the largest area
            largest_area = candidate_area;
        }
    }
    largest_area
}

fn check_pair(a: &Coord, b: &Coord, segments: &Vec<(Coord, Coord)>) -> bool {
    // Check if the rectangle defined by a and b intersects any of the segments
    let start_x = a.x.min(b.x);
    let end_x = a.x.max(b.x);
    let start_y = a.y.min(b.y);
    let end_y = a.y.max(b.y);
    for (seg_start, seg_end) in segments {
        if seg_start.x.min(seg_end.x) >= end_x  {
            // Segment is completely to the right
            continue;
        }
        if seg_start.x.max(seg_end.x) <= start_x  {
            // Segment is completely to the right
            continue;
        }
        if seg_start.y.min(seg_end.y) >= end_y  {
            // Segment is completely above the candidate
            continue;
        }
        if seg_start.y.max(seg_end.y) <= start_y  {
            // Segment is completely below the candidate
            continue;
        }
        // Otherwise the segment does intersect the rectangle
        // This means that a and b do not make valid rectangle
        return true;
    }
    false
}

fn get_part1(tiles: &[Coord]) -> i64 {
    tiles.iter().map(|a| {
        tiles.iter().map(|b| {
            ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
        }).max().unwrap_or(0)
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a="7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let tiles = read_tiles(&a);
        assert_eq!(get_part1(&tiles), 50);
        //assert_eq!(read_contents(&a).1, 24);
    }

    #[test]
    fn part2() {
        let a="7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let tiles = read_tiles(&a);
        assert_eq!(get_part2(&tiles), 24);
    }
}

