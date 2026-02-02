use clap::Parser;
use std::fs;
use shared::Dir;
use shared::Vec2D;

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
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
}



struct PathSegment {
    dir: Dir,
    distance: i64,
}

impl PathSegment {
    fn new(str: &str) -> Self {
        let (a,b) = str.split_at(1);
        let dir = match a {
            "U" => Dir::N,
            "R" => Dir::E,
            "D" => Dir::S,
            "L" => Dir::W,
            _ => panic!("Bad direction"),
        };

        let distance = b.parse::<i64>().unwrap();
        Self {
            dir,
            distance
        }
    }
    fn get_vec(&self) -> Vec2D {
        self.dir.get_dir_true_vec() * self.distance
    }
}

#[derive(Debug)]
struct Wire {
    segments: Vec<Segment>,
    // Start, end, direction and distance from start (for the start of this segment)
}

#[derive(Debug)]
struct Segment {
    start: Vec2D,
    end: Vec2D,
    dir: Dir,
    distance_from_start: i64,
}

impl Wire {
    fn new(ln: &str) -> Self {
        let path = ln.split(',').map(PathSegment::new).collect::<Vec<PathSegment>>();
        let mut segments = vec![];
        let mut loc = Vec2D{x:0,y:0};
        let mut distance = 0;
        for aa in path {
            let new_loc = loc + aa.get_vec();
            segments.push(Segment{start: loc, end: new_loc, dir: aa.dir, distance_from_start: distance});
            distance += aa.distance;
            loc = new_loc;
        }
        Self {
            segments,
        }

    }

    fn get_crossings(&self, other: &Self) -> Vec<(Vec2D, i64)> {
        let mut crossings = vec![];
        for seg_a in &self.segments {
            for seg_b in &other.segments {
                if let Some(res) = get_crossing(seg_a, seg_b) {
                    crossings.push(res);
                }
            }
        }
        crossings
    }
}

fn get_crossing(seg_a: &Segment, seg_b: &Segment) -> Option<(Vec2D, i64)> {
    if seg_a.dir == seg_b.dir || seg_a.dir == seg_b.dir.opposite() {
        return None; // Parallel lines
    }
    let (horizontal, vertical) = if seg_a.dir == Dir::N || seg_a.dir == Dir::S {
        (seg_b, seg_a)
    } else {
        (seg_a, seg_b)
    };

    assert_eq!(horizontal.start.y, horizontal.end.y);
    assert_eq!(vertical.start.x, vertical.end.x);

    let (x_min, x_max, x_vert) = if horizontal.dir == Dir::E {
        (horizontal.start.x, horizontal.end.x, vertical.start.x)
    } else {
        (horizontal.end.x, horizontal.start.x, vertical.start.x)
    };


    let (y_min, y_max, y_hori) = if vertical.dir == Dir::N {
        (vertical.start.y, vertical.end.y, horizontal.start.y)
    } else {
        (vertical.end.y, vertical.start.y, horizontal.start.y)
    };
    assert!(x_min < x_max);
    assert!(y_min < y_max);

    if y_max < y_hori || y_min > y_hori {
        return None;
    }

    if x_max < x_vert || x_min > x_vert {
        return None;
    }

    if y_hori == 0 && x_vert == 0 {
        return None;
    }

    let d = (vertical.distance_from_start + (y_hori - vertical.start.y).abs()) +
            (horizontal.distance_from_start + (x_vert - horizontal.start.x).abs());

    Some((Vec2D{x: x_vert, y: y_hori}, d))
}

fn read_contents(cont: &str) -> (i64, i64) {
    let wires = cont.lines().map(Wire::new).collect::<Vec<Wire>>();

    assert_eq!(wires.len(), 2);
    let wire_a = &wires[0];
    let wire_b = &wires[1];
    let crossings = wire_a.get_crossings(wire_b);

    let part1 = crossings.iter().map(|(c,_)| c.manhattan(&Vec2D{x:0,y:0})).min().unwrap_or(0);
    let part2 = crossings.iter().map(|(_,d)| *d).min().unwrap_or(0);

    (part1, part2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {

        let a = "R8,U5,L5,D3
U7,R6,D4,L4";
        assert_eq!(read_contents(a).0, 6);
        assert_eq!(read_contents(a).1, 30);

        let a = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

        assert_eq!(read_contents(a).0, 159);
        assert_eq!(read_contents(a).1, 610);
        let b = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(read_contents(b).0, 135);
        assert_eq!(read_contents(b).1, 410);
    }

}
