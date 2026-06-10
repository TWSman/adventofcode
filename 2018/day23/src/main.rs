use clap::Parser;
use regex::Regex;
use shared::Vec3D;
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
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

#[derive(Debug, Clone)]
struct NanoBot {
    loc: Vec3D,
    radius: i64,
}

impl NanoBot {
    fn new(ln: &str) -> Self {
        let re = Regex::new(r"(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
        let caps = re.captures(ln).unwrap();
        Self {
            loc: Vec3D {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
                z: caps[3].parse().unwrap(),
            },
            radius: caps[4].parse().unwrap(),
        }
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let bots = cont.lines().map(NanoBot::new).collect::<Vec<_>>();
    let part1 = get_part1(&bots);
    let part2 = get_part2(&bots);
    (part1, part2)
}

fn get_part1(bots: &[NanoBot]) -> i64 {
    let bot0 = bots.iter().max_by(|a, b| a.radius.cmp(&b.radius)).unwrap();
    bots.iter()
        .filter(|bot| bot.loc.manhattan(&bot0.loc) <= bot0.radius)
        .count() as i64
}

fn get_part2(bots: &[NanoBot]) -> i64 {
    // This logic makes some assumptions:
    // 1) Best location is within range from the bot with most connections
    // 2) The volume that gives the maximum number of connections is large enough such that
    //  the logic will hit that volume during the first iteration with a coarse interval
    let mut bots_by_loc: Vec<(NanoBot, usize)> = Vec::new();
    for bot1 in bots {
        let c = bots
            .iter()
            .filter(|bot2| bot1.loc.manhattan(&bot2.loc) <= bot2.radius)
            .count();
        bots_by_loc.push((bot1.clone(), c));
    }

    bots_by_loc.sort_by_key(|v| v.1);
    let (best, c) = bots_by_loc.last().unwrap();

    let mut max_c = *c;
    let mut min_x = best.loc.x - best.radius;
    let mut min_y = best.loc.y - best.radius;
    let mut min_z = best.loc.z - best.radius;

    let mut max_x = best.loc.x + best.radius;
    let mut max_y = best.loc.y + best.radius;
    let mut max_z = best.loc.z + best.radius;

    let mut max_d = max_x - min_x;
    max_d = max_d.max(max_y - min_y);
    max_d = max_d.max(max_z - min_z);
    let mut d: i64 = 1;
    loop {
        if d * 1000 > max_d {
            break;
        }
        d *= 10;
    }
    let mut best;
    loop {
        let mut valid: Vec<Vec3D> = Vec::new();
        for x in (min_x..=max_x).step_by(d as usize) {
            for y in (min_y..=max_y).step_by(d as usize) {
                for z in (min_z..=max_z).step_by(d as usize) {
                    let loc = Vec3D { x, y, z };
                    let c = bots
                        .iter()
                        .filter(|bot2| loc.manhattan(&bot2.loc) <= bot2.radius)
                        .count();
                    if c == max_c {
                        valid.push(loc);
                    } else if c > max_c {
                        valid = vec![loc];
                        max_c = c;
                    }
                }
            }
        }
        valid.sort_by(|a, b| (a.x + a.y + a.z).cmp(&(b.x + b.y + b.z)));
        best = *valid.first().unwrap();
        if d == 1 {
            break;
        }
        max_x = best.x + d;
        min_x = best.x - d;

        max_y = best.y + d;
        min_y = best.y - d;

        max_z = best.z + d;
        min_z = best.z - d;
        d /= 10;
    }
    best.x + best.y + best.z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,0,0>, r=4
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";
        assert_eq!(read_contents(&a).0, 7);
    }

    #[test]
    fn part2() {
        let a = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";
        assert_eq!(read_contents(&a).1, 36);
    }
}
