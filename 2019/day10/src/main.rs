use clap::Parser;
use colored::*;
use num_integer::gcd;
use shared::Vec2D;
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
    let map = read_map(cont);
    let (part1, monitor) = get_part1(&map);
    let part2 = get_part2(monitor.as_ref().unwrap(), map);
    (part1, part2)
}

fn print_grid(grid: &BTreeSet<Vec2D>, monitor: &Vec2D, order: &[(Vec2D, usize)]) {
    let min_x = grid.iter().map(|v| v.x).min().unwrap();
    let max_x = grid.iter().map(|v| v.x).max().unwrap();
    let min_y = grid.iter().map(|v| v.y).min().unwrap();
    let max_y = grid.iter().map(|v| v.y).max().unwrap();

    print!("   ");
    for x in min_x..=max_x {
        if x < 0 {
            print!(" ");
        } else {
            print!("{}", (x / 10).to_string().red());
        }
    }
    println!();
    print!("   ");
    for x in min_x..=max_x {
        if x < 0 {
            print!(" ");
        } else {
            print!("{}", (x % 10).to_string().red());
        }
    }
    println!();

    for y in (min_y..=max_y).rev() {
        print!("{}", format!("{:>3}", y).red());
        for x in min_x..=max_x {
            if (x == monitor.x) && (y == monitor.y) {
                print!("{}", "X".black().on_white());
            } else if grid.contains(&Vec2D { x, y }) {
                if let Some((_a, b)) = order.iter().find(|(v, _)| *v == Vec2D { x, y }) {
                    print!("{}", format!("{}", b % 10).blue().on_white());
                } else {
                    print!("{}", "#".red().on_white());
                    //print!("{}", "#".red().on_white());
                }
            } else {
                print!("{}", ".".white().on_white());
            }
        }
        println!();
    }
}

fn get_part1(map: &BTreeSet<Vec2D>) -> (i64, Option<Vec2D>) {
    let mut max_seen = 0;
    let mut best_monitor = None;
    for monitor in map.iter() {
        let seen = see_count(monitor, map);

        if seen > max_seen {
            println!("New max {} at {:?}", seen, monitor);
            max_seen = seen;
            best_monitor = Some(*monitor);
        }
    }
    (max_seen, best_monitor)
}

fn see_count(monitor: &Vec2D, map: &BTreeSet<Vec2D>) -> i64 {
    // Check how many asteroids the monitor can see
    let mut seen = 0;
    for target in map.iter() {
        if target == monitor {
            continue;
        }
        if can_see(monitor, target, map) {
            seen += 1;
        }
    }
    seen
}

fn get_key(d: i64, n: i64) -> f64 {
    let tmp = (d as f64).atan2(n as f64);
    if tmp > 0.0 {
        tmp
    } else {
        tmp + std::f64::consts::TAU
    }
}

const TARGET: usize = 200;

fn get_part2(monitor: &Vec2D, map: BTreeSet<Vec2D>) -> i64 {
    println!("Getting part2 for monitor: {} {}", monitor.x, monitor.y);
    print_grid(&map, monitor, &[]);

    let max_sep = map
        .iter()
        .map(|v| {
            let tmp = *monitor - *v;
            tmp.x.abs().max(tmp.y.abs())
        })
        .max()
        .unwrap();

    // Define the possible directions in our grid
    let mut possible_directions: Vec<(i64, i64, f64)> = vec![];
    let pi = std::f64::consts::PI;

    // These wouldn't be added otherwise
    possible_directions.push((0, 1, 0.0));
    possible_directions.push((1, 0, pi / 2.0));
    possible_directions.push((-1, 0, 3.0 * pi / 2.0));
    possible_directions.push((0, -1, pi));
    for deno in 1..=max_sep {
        for nume in 1..=max_sep {
            if gcd(deno, nume) == 1 {
                possible_directions.push((nume, deno, get_key(nume, deno)));
                possible_directions.push((nume, -deno, get_key(nume, -deno)));
                possible_directions.push((-nume, -deno, get_key(-nume, -deno)));
                possible_directions.push((-nume, deno, get_key(-nume, deno)));
            }
        }
    }
    possible_directions.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut max_depth = 0;
    let mut layers: Vec<Vec<Vec2D>> = vec![];
    let mut tot_asteroids = 0;
    for (dx, dy, _f) in possible_directions.iter() {
        let mut vec: Vec<Vec2D> = vec![];
        let d = Vec2D { x: *dx, y: *dy };
        let max_count = max_sep / (d.x.abs().max(d.y.abs()));
        for k in 1..=max_count {
            let target = *monitor + d * k;
            if map.contains(&target) {
                vec.push(target);
                tot_asteroids += 1;
            }
        }
        if vec.len() > max_depth {
            max_depth = vec.len();
        }
        if !vec.is_empty() {
            layers.push(vec);
        }
    }
    if tot_asteroids < TARGET {
        return 0;
    }
    let mut removal_order: Vec<(Vec2D, usize)> = vec![];
    let mut ind = 0;
    for i in 0..max_depth {
        for layer in layers.iter_mut() {
            if layer.len() > i {
                ind += 1;
                removal_order.push((layer[i], ind));
            }
        }
    }
    print_grid(&map, monitor, &removal_order[..9]);
    let target = removal_order.get(TARGET - 1).unwrap().0;
    target.x * 100 - target.y
}

fn can_see(monitor: &Vec2D, target: &Vec2D, map: &BTreeSet<Vec2D>) -> bool {
    let d = *target - *monitor;
    let dx = d.x;
    let dy = d.y;
    if dx == 0 {
        let mn_y = target.y.min(monitor.y) + 1;
        let mx_y = target.y.max(monitor.y);
        return !(mn_y..mx_y).any(|y| map.contains(&Vec2D { x: monitor.x, y }));
    }
    if dy == 0 {
        let mn_x = target.x.min(monitor.x) + 1;
        let mx_x = target.x.max(monitor.x);
        return !(mn_x..mx_x).any(|x| map.contains(&Vec2D { x, y: monitor.y }));
    }

    let gcd = gcd(dx.abs(), dy.abs());
    if gcd == 1 {
        return true;
    }
    let d0 = Vec2D {
        x: dx / gcd,
        y: dy / gcd,
    };
    !(1..gcd).any(|i| map.contains(&(d0 * i + *monitor)))
}

fn read_map(cont: &str) -> BTreeSet<Vec2D> {
    cont.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Vec2D {
                        x: x as i64,
                        y: -(y as i64),
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = ".#..#
.....
#####
....#
...##";
        let map = read_map(&a);
        assert!(!can_see(
            &Vec2D { x: 3, y: -4 },
            &Vec2D { x: 1, y: 0 },
            &map
        ));
        assert!(can_see(&Vec2D { x: 3, y: -4 }, &Vec2D { x: 4, y: 0 }, &map));
        assert!(can_see(
            &Vec2D { x: 3, y: -4 },
            &Vec2D { x: 0, y: -2 },
            &map
        ));
        assert!(can_see(
            &Vec2D { x: 3, y: -4 },
            &Vec2D { x: 1, y: -2 },
            &map
        ));
        assert!(can_see(
            &Vec2D { x: 3, y: -4 },
            &Vec2D { x: 2, y: -2 },
            &map
        ));
        assert!(can_see(
            &Vec2D { x: 3, y: -4 },
            &Vec2D { x: 3, y: -2 },
            &map
        ));
        assert!(can_see(
            &Vec2D { x: 3, y: -4 },
            &Vec2D { x: 4, y: -2 },
            &map
        ));
        assert!(can_see(
            &Vec2D { x: 3, y: -4 },
            &Vec2D { x: 4, y: -3 },
            &map
        ));
        assert!(can_see(
            &Vec2D { x: 3, y: -4 },
            &Vec2D { x: 4, y: -4 },
            &map
        ));

        assert!(!can_see(
            &Vec2D { x: 1, y: 0 },
            &Vec2D { x: 4, y: -3 },
            &map
        ));

        assert_eq!(see_count(&Vec2D { x: 3, y: 4 }, &map), 8);
        assert_eq!(see_count(&Vec2D { x: 1, y: 0 }, &map), 7);
        assert_eq!(see_count(&Vec2D { x: 4, y: 0 }, &map), 7);

        assert_eq!(read_contents(a).0, 8);

        let b = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        assert_eq!(read_contents(b).0, 33);

        let c = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

        assert_eq!(read_contents(c).0, 35);

        let d = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        assert_eq!(read_contents(d).0, 210);
    }

    #[test]
    fn part2() {
        // Used to test removal order. Doesn't actually contain 200 asteroids, but the ordering was
        // given in the problem statement, so it's good enough for testing
        let _a = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
        //assert_eq!(read_contents(a).1, 1);

        let a = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        assert_eq!(read_contents(a).1, 802);
    }
}
