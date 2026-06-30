use clap::Parser;
use colored::Colorize;
use shared::Vec2D;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i64, i64) {
    let tiles = read_grid(cont);
    let (part1, corners) = get_part1(&tiles);
    let part2 = get_part2(&tiles, &corners);
    (part1, part2)
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    grid: BTreeSet<Vec2D>,
    width: i64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Orientation {
    Identity,
    T90,
    T180,
    T270,
    Flip,
    F90,
    F180,
    F270,
}

impl Tile {
    fn new(id: usize) -> Self {
        Self {
            id,
            grid: BTreeSet::new(),
            width: 0,
        }
    }

    fn len(&self) -> usize {
        self.grid.len()
    }

    fn insert(&mut self, vec: Vec2D) {
        self.width = self.width.max(vec.x).max(vec.y.abs());
        self.grid.insert(vec);
    }

    fn update(&mut self) {
        assert_eq!(self.width % 2, 0);
        let shift = self.width + 1;
        self.grid = self
            .grid
            .iter()
            .map(|v| Vec2D {
                x: 2 * v.x - shift,
                y: 2 * v.y + shift,
            })
            .collect::<BTreeSet<_>>();
    }

    fn transform(&self, orientation: Orientation) -> Self {
        let grid = match orientation {
            Orientation::Identity => &self.grid,
            Orientation::T180 => &self
                .grid
                .iter()
                .map(|v| Vec2D { x: -v.x, y: -v.y })
                .collect::<BTreeSet<_>>(),
            Orientation::Flip => &self
                .grid
                .iter()
                .map(|v| Vec2D { x: -v.x, y: v.y })
                .collect::<BTreeSet<_>>(),
            Orientation::F180 => &self
                .grid
                .iter()
                .map(|v| Vec2D { x: v.x, y: -v.y })
                .collect::<BTreeSet<_>>(),
            Orientation::T90 => &self
                .grid
                .iter()
                .map(|v| Vec2D { x: v.y, y: -v.x })
                .collect::<BTreeSet<_>>(),
            Orientation::T270 => &self
                .grid
                .iter()
                .map(|v| Vec2D { x: -v.y, y: v.x })
                .collect::<BTreeSet<_>>(),
            Orientation::F90 => &self
                .grid
                .iter()
                .map(|v| Vec2D { x: v.y, y: v.x })
                .collect::<BTreeSet<_>>(),
            Orientation::F270 => &self
                .grid
                .iter()
                .map(|v| Vec2D { x: -v.y, y: -v.x })
                .collect::<BTreeSet<_>>(),
        };
        Self {
            grid: grid.clone(),
            id: self.id,
            width: self.width,
        }
    }

    fn get_row(&self, id: usize, reverse: bool) -> Vec<bool> {
        let min_x = -self.width + 1;
        let min_y = -self.width + 1;
        let max_x = self.width - 1;
        let max_y = self.width - 1;

        if reverse {
            match id {
                0 => (min_x..=max_x)
                    .rev()
                    .step_by(2)
                    .map(|x| self.grid.contains(&Vec2D { x, y: max_y }))
                    .collect::<Vec<_>>(),
                1 => (min_y..=max_y)
                    .step_by(2)
                    .map(|y| self.grid.contains(&Vec2D { x: max_x, y }))
                    .collect::<Vec<_>>(),
                2 => (min_x..=max_x)
                    .step_by(2)
                    .map(|x| self.grid.contains(&Vec2D { x, y: min_y }))
                    .collect::<Vec<_>>(),
                3 => (min_y..=max_y)
                    .rev()
                    .step_by(2)
                    .map(|y| self.grid.contains(&Vec2D { x: min_x, y }))
                    .collect::<Vec<_>>(),
                _ => panic!(),
            }
        } else {
            match id {
                0 => (min_x..=max_x)
                    .step_by(2)
                    .map(|x| self.grid.contains(&Vec2D { x, y: max_y }))
                    .collect::<Vec<_>>(),
                1 => (min_y..=max_y)
                    .rev()
                    .step_by(2)
                    .map(|y| self.grid.contains(&Vec2D { x: max_x, y }))
                    .collect::<Vec<_>>(),
                2 => (min_x..=max_x)
                    .rev()
                    .step_by(2)
                    .map(|x| self.grid.contains(&Vec2D { x, y: min_y }))
                    .collect::<Vec<_>>(),
                3 => (min_y..=max_y)
                    .step_by(2)
                    .map(|y| self.grid.contains(&Vec2D { x: min_x, y }))
                    .collect::<Vec<_>>(),
                _ => panic!(),
            }
        }
    }

    fn print_grid(&self, highlight: Option<usize>, monsters: Option<&BTreeSet<Vec2D>>) {
        let min_x = -self.width + 1;
        let min_y = -self.width + 1;
        let max_x = self.width - 1;
        let max_y = self.width - 1;

        for y in (min_y..=max_y).rev() {
            if y % 2 == 0 {
                continue;
            }
            for x in min_x..=max_x {
                let hl = match highlight {
                    Some(0) => y == max_y,
                    Some(1) => x == max_x,
                    Some(2) => y == min_y,
                    Some(3) => x == min_x,
                    _ => false,
                };
                if x % 2 == 0 {
                    continue;
                }
                let l = Vec2D { x, y };
                if self.grid.contains(&l) {
                    if monsters.is_some() && monsters.unwrap().contains(&l) {
                        print!("{}", 'O'.to_string().blue().on_blue());
                    } else if hl {
                        print!("{}", '#'.to_string().black().on_blue());
                    } else {
                        print!("{}", '#'.to_string().black().on_red());
                    }
                } else {
                    print!("{}", '.'.to_string().white().on_black());
                }
            }
            println!();
        }
        println!();
    }
}

fn read_grid(cont: &str) -> BTreeMap<usize, Tile> {
    let mut tiles = BTreeMap::new();
    let mut current_id = 0;
    let mut current_tile = Tile::new(0);
    let mut y = 0;
    for line in cont.lines() {
        if line.starts_with("Tile") {
            if current_tile.len() > 0 {
                current_tile.update();
                tiles.insert(current_id, current_tile);
            }
            current_id = line
                .strip_suffix(':')
                .unwrap()
                .split_once(" ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
            current_tile = Tile::new(current_id);
            y = 0;
            continue;
        }
        if line.is_empty() {
            continue;
        }
        y += 1;
        for (x, c) in line.chars().enumerate() {
            let x = (x + 1) as i64;
            if c == '#' {
                current_tile.insert(Vec2D { x, y: -y });
            }
        }
    }

    current_tile.update();
    tiles.insert(current_id, current_tile);

    tiles
}

fn get_part1(tiles: &BTreeMap<usize, Tile>) -> (i64, Vec<usize>) {
    // Loop through all tiles and check how many possible neighbors they have,
    // Since all borders seem to be unique,
    // there will be exactly 1 or 0 possible neighbors for each side
    // Corner pieces will have 2 neighbors,
    // Side   pieces will have 3 neighbors,
    // Center pieces will have 4 neighbors,
    println!("{} tiles", tiles.len());
    let mut counts = (0, 0, 0); // Corner, side, center
    let mut result = 1;
    let mut corners = Vec::new();
    for (id1, tile) in tiles {
        let mut match_count = 0;
        for side in [0, 1, 2, 3] {
            let row = tile.get_row(side, false);
            for (id2, tile2) in tiles {
                if id1 == id2 {
                    continue;
                }
                if *id2 == 9999 {
                    tile2.print_grid(None, None);
                }
                for side2 in [0, 1, 2, 3, 4, 5, 6, 7] {
                    let row2 = tile2.get_row(side2 % 4, side2 <= 3);
                    if *id2 == 9999 {
                        println!("    Testing {id2} side {side2}:");
                        println!(
                            "    {}",
                            row2.iter()
                                .map(|v| if *v { '#' } else { '.' })
                                .collect::<String>()
                        );
                    }
                    if row2 == row {
                        match_count += 1;
                    } else if *id2 == 9999 {
                        println!("        No match");
                    }
                }
            }
        }
        if match_count == 2 {
            counts.0 += 1;
            result *= id1;
            corners.push(*id1);
        } else if match_count == 4 {
            counts.2 += 1;
        } else if match_count == 3 {
            counts.1 += 1;
        } else {
            println!("{} matches for tile {}", match_count, id1);
            panic!();
        }
    }
    (result as i64, corners)
}

fn get_part2(tiles: &BTreeMap<usize, Tile>, corners: &[usize]) -> i64 {
    // If part1 ran succesfully we know that there is exactly one matching tile and orientation for
    // each side
    let mut coordinates: BTreeMap<(i64, i64), (usize, Orientation)> = BTreeMap::new();

    let mut used = BTreeSet::new();
    let first = corners[0];
    coordinates.insert((0, 0), (first, Orientation::Identity));
    used.insert(first);

    let mut heads = Vec::new();
    for s in [0, 1, 2, 3] {
        heads.push((first, 0, 0, s, Orientation::Identity)); // id, side, which orientation the tile has
    }

    loop {
        if heads.is_empty() {
            break;
        }
        let (id1, x, y, side1, orientation) = heads.pop().unwrap();
        println!("Testing id {id1} side {side1}");
        let (new_x, new_y) = match side1 {
            0 => (x, 1 + y),
            1 => (1 + x, y),
            2 => (x, -1 + y),
            3 => (-1 + x, y),
            _ => panic!(),
        };
        if coordinates.contains_key(&(new_x, new_y)) {
            // Already found this one
            continue;
        }

        let tile1 = tiles.get(&id1).unwrap().transform(orientation);
        let row = tile1.get_row(side1, false);
        for (id2, tile2) in tiles {
            if used.contains(id2) {
                continue;
            }
            for side2 in [0, 1, 2, 3, 4, 5, 6, 7] {
                let row2 = tile2.get_row(side2 % 4, side2 <= 3);
                if *id2 == 9999 {
                    println!("    Testing {id2} side {side2}:");
                    println!(
                        "    {}",
                        row2.iter()
                            .map(|v| if *v { '#' } else { '.' })
                            .collect::<String>()
                    );
                }
                if row2 == row {
                    println!(
                        "        Found a match for {} side {} from {} side {}",
                        id1, side1, id2, side2
                    );
                    //tile1.print_grid(Some(side1), None);
                    //tile2.print_grid(Some((side2) % 4), None);
                    let orientation = match (side1, side2) {
                        (0, 2) | (1, 3) | (2, 0) | (3, 1) => Orientation::Identity,
                        (0, 1) | (1, 2) | (2, 3) | (3, 0) => Orientation::T90,
                        (0, 0) | (1, 1) | (2, 2) | (3, 3) => Orientation::T180,
                        (0, 3) | (1, 0) | (2, 1) | (3, 2) => Orientation::T270,
                        (0, 6) | (1, 5) | (2, 4) | (3, 7) => Orientation::Flip,
                        (0, 7) | (1, 6) | (2, 5) | (3, 4) => Orientation::F90,
                        (0, 4) | (1, 7) | (2, 6) | (3, 5) => Orientation::F180,
                        (0, 5) | (1, 4) | (2, 7) | (3, 6) => Orientation::F270,
                        _ => todo!(),
                    };
                    //tile2
                    //  .transform(orientation)
                    // .print_grid(Some((side1 + 2) % 4), None);

                    coordinates.insert((new_x, new_y), (*id2, orientation));
                    used.insert(*id2);
                    for s in [0, 1, 2, 3] {
                        heads.push((*id2, new_x, new_y, s, orientation));
                    }
                }
            }
        }
    }
    let mut total_grid = Tile::new(0);

    for ((x, y), (tile_id, orientation)) in coordinates.iter() {
        let tile = tiles.get(tile_id).unwrap().transform(*orientation);
        let w = tile.width;
        let min_x = -tile.width + 1;
        let min_y = -tile.width + 1;
        let max_x = tile.width - 1;
        let max_y = tile.width - 1;
        for vec in tile.grid.iter() {
            if vec.x == min_x || vec.x == max_x || vec.y == min_y || vec.y == max_y {
                continue; // Skip borders
            }
            let dx = (vec.x + max_x - 2) / 2;
            let dy = (vec.y + max_y - 2) / 2;
            let loc = Vec2D {
                x: 1 + x * (w - 2) + dx,
                y: 1 + y * (w - 2) + dy,
            };
            total_grid.insert(loc);
        }
    }
    let min_x = total_grid.grid.iter().map(|v| v.x).min().unwrap();
    let min_y = total_grid.grid.iter().map(|v| v.y).min().unwrap();
    total_grid.grid = total_grid
        .grid
        .iter()
        .map(|v| Vec2D {
            x: 1 + v.x - min_x,
            y: -(1 + v.y - min_y),
        })
        .collect::<BTreeSet<_>>();
    total_grid.update();
    total_grid.print_grid(None, None);

    // After the update, the grid spacing is 2 untis
    let monster = [
        Vec2D { x: 0, y: 0 },
        Vec2D { x: 2, y: -2 },
        Vec2D { x: 8, y: -2 },
        Vec2D { x: 10, y: 0 },
        Vec2D { x: 12, y: 0 },
        Vec2D { x: 14, y: -2 },
        Vec2D { x: 20, y: -2 },
        Vec2D { x: 22, y: 0 },
        Vec2D { x: 24, y: 0 },
        Vec2D { x: 26, y: -2 },
        Vec2D { x: 32, y: -2 },
        Vec2D { x: 34, y: 0 },
        Vec2D { x: 36, y: 0 },
        Vec2D { x: 36, y: 2 },
        Vec2D { x: 38, y: 0 },
    ];
    let mut answer = 0;
    for orientation in [
        Orientation::Identity,
        Orientation::T90,
        Orientation::T180,
        Orientation::T270,
        Orientation::Flip,
        Orientation::F90,
        Orientation::F180,
        Orientation::F270,
    ] {
        let tmp = total_grid.transform(orientation);
        let mut found = 0;
        let mut monster_pixels = BTreeSet::new();
        for v in tmp.grid.iter() {
            if monster.iter().all(|m| tmp.grid.contains(&(*v + *m))) {
                for m in monster {
                    monster_pixels.insert(*v + m);
                }
                found += 1;
            }
        }
        if found > 0 {
            tmp.print_grid(None, Some(&monster_pixels));
            println!("Found {} monsters for orientation {:?}", found, orientation);
            answer = total_grid.grid.len() - monster_pixels.len();
        }
    }
    answer as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

        let tiles = read_grid(&a);
        let tile1171 = tiles.get(&1171).unwrap();
        let tile2473 = tiles.get(&2473).unwrap();
        println!("2473:");
        tile2473.print_grid(None, None);
        println!("1171:");
        tile1171.print_grid(None, None);

        for side in [0, 1, 2, 3, 4, 5, 6, 7] {
            let row = tile2473.get_row(side % 4, if side > 3 { false } else { true });
            println!(
                "Side {side}: {}",
                row.iter()
                    .map(|v| if *v { '#' } else { '.' })
                    .collect::<String>()
            );
        }
        //dbg!(&tiles.len());
        ////assert_eq!(tiles.len(), 3);
        assert_eq!(tile1171.get_row(0, false), tile2473.get_row(3, true));
        assert_eq!(read_contents(&a).0, 20899048083289);
    }

    #[test]
    fn part2() {
        let a = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        assert_eq!(read_contents(&a).1, 273);
    }
}
