use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use shared::Vec2D;
use regex::Regex;
use std::cmp::Ordering;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents, 2_000_000, 4_000_000);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}

fn read_contents(cont: &str, row: i64, max_coord: i64) -> (i64, i64) {
    let sensors = cont.lines().map(Sensor::new).collect::<Vec<_>>();
    let part1 = get_part1(&sensors, row);
    let part2 = get_part2(&sensors, max_coord);
    (part1,part2)
}

fn get_part1(sensors: &Vec<Sensor>, row: i64) -> i64 {
    let mut set: BTreeSet<Range> = BTreeSet::new();
    for sensor in sensors {
        let ydist = sensor.coord.distance_to_y(row);
        let x = sensor.coord.x;
        if ydist > sensor.distance {
            // Too far from the target row
            continue;
        }
        let remaining_space = sensor.distance - ydist;
        let mut new_range = Range::new(x - remaining_space, x + remaining_space);
        let mut to_drop: Vec<Range> = Vec::new();

        for r in &set {
            let a = new_range.combine_with(r);
            if let Some(b) = a {
                new_range = b;
                to_drop.push(*r);
            }
        }
        for d in to_drop {
            set.remove(&d);
        }
        set.insert(new_range);
    }
    let sum: i64 = set.iter().map(Range::length).sum();
    let mut overlaps: BTreeSet<i64> = BTreeSet::new();
    for sensor in sensors {
        if sensor.beacon.y != row {
            continue;
        }
        for range in &set {
            if sensor.beacon.x >= range.start && sensor.beacon.x <= range.end {
                overlaps.insert(sensor.beacon.x);
            }
        }
    }
    sum - overlaps.len() as i64
}

fn get_part2(sensors: &[Sensor], max_coord: i64) -> i64 {
    // This is somehwat slow (runs in a few minutes) but works
    let mut ranges: BTreeMap<i64, BTreeSet<Range>> = BTreeMap::new();
    let mut rows_to_skip: BTreeSet<i64> = BTreeSet::new();
    let comp_range = Range::new(0, max_coord);
    for ii in 0..=max_coord {
        ranges.insert(ii, BTreeSet::new());
    }
    dbg!(&sensors.len());
    for (i,sensor) in sensors.iter().enumerate() {
        dbg!(&i);
        let x = sensor.coord.x;
        let y = sensor.coord.y;
        let dist = sensor.distance;
        let y_max = y + dist;
        let y_min = y - dist;
        for yy in y_min..=y_max {
            // No reason to check this row
            if yy < 0 || yy > max_coord || rows_to_skip.contains(&yy) {
                continue
            }
            let ydist = sensor.coord.distance_to_y(yy);
            assert!(ydist >= 0);
            let rangeset = ranges.get(&yy).unwrap();
            let remaining_space = sensor.distance - ydist;
            let start = 0.max(x - remaining_space);
            let end = max_coord.min(x + remaining_space);
            let mut new_range = Range::new(start, end);
            let mut to_drop: Vec<Range> = Vec::new();

            for r in rangeset {
                let a = new_range.combine_with(r);
                if let Some(b) = a {
                    to_drop.push(*r);
                    new_range = b;
                }
            }
            let rangeset = ranges.get_mut(&yy).unwrap();
            for d in to_drop {
                rangeset.remove(&d);
            }
            if new_range == comp_range {
                rows_to_skip.insert(yy);
            }
            rangeset.insert(new_range);
        }
    }
    for (y,rangeset) in ranges {
        if rangeset.len() == 2 {
            let mut it = rangeset.iter();
            let a = it.next().unwrap();
            let b = it.next().unwrap();
            assert_eq!(b.start, a.end + 2);
            let x = i64::midpoint(a.end, b.start);
            return x * 4_000_000 + y;
        }
    }
    0
}


#[derive(Debug, Clone, Copy, Eq, PartialEq )]
struct Range {
    start: i64,
    end: i64,
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.end < other.start {
            Ordering::Less
        } else if self.start > other.end {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
 
impl Range {
    fn new(start: i64, end: i64) -> Self {
        Self {start, end}
    }
     
    fn length(&self) -> i64 {
        self.end - self.start + 1
    }

    fn combine_with(&self, other: &Self) -> Option<Self> {
        // If there is a match, return combined Range
        // Otherwise return None
        if other.start > self.end + 1
        || self.start > other.end + 1 {
            None
        } else {
            Some(Self{start: self.start.min(other.start), end: self.end.max(other.end)})
        }

    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Sensor {
    coord: Vec2D,
    beacon: Vec2D,
    distance: i64,
}
 
impl Sensor {
    fn new(ln: &str) -> Self {
        let re = Regex::new(r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();
        let res = re.captures(ln).unwrap();
        let coord = Vec2D::new(
            res[1].parse::<i64>().unwrap(),
            res[2].parse::<i64>().unwrap(),
            );
        let beacon = Vec2D::new(
            res[3].parse::<i64>().unwrap(),
            res[4].parse::<i64>().unwrap(),
            );
        let distance = coord.manhattan(&beacon);
        Self {coord, beacon, distance}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range() {
        let a = Range::new(10,12);
        let b = Range::new(14,16);

        let c = Range::new(11,17);
        let d = Range::new(12,17);
        let e = Range::new(13,17);
        assert_eq!(a.combine_with(&b), None);
        assert_eq!(b.combine_with(&a), None);
        assert_eq!(a.combine_with(&a), Some(a));

        assert_eq!(a.combine_with(&c), Some(Range::new(10,17)));
        assert_eq!(a.combine_with(&d), Some(Range::new(10,17)));
        assert_eq!(a.combine_with(&e), Some(Range::new(10,17)));

        assert_eq!(c.combine_with(&a), Some(Range::new(10,17)));
        assert_eq!(d.combine_with(&a), Some(Range::new(10,17)));
        assert_eq!(e.combine_with(&a), Some(Range::new(10,17)));
    }
    #[test]
    fn part1() {
        let a ="Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(read_contents(&a, 10, 20).0, 26);
    }

    #[test]
    fn part2() {
        let a ="Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(read_contents(&a, 10, 20).1, 56000011);
    }

}
