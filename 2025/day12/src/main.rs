use clap::Parser;
use std::fs;
use std::collections::BTreeMap;


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
    println!("Part 1 answer is {}", res);
}

#[derive(Debug)]
struct Grid {
    size: (i64, i64),
    shapes: Vec<i64>,
}

#[derive(Debug, Clone)]
struct Shape {
    pattern: Vec<bool>
}

impl Shape {
    fn from_lines(lines: &str) -> (usize, Self) {
        let mut id = 0;
        let mut pattern = Vec::new();
        for ln in lines.lines() {
            for c in ln.chars() {
                match c {
                    '#' => pattern.push(true),
                    '.' => pattern.push(false),
                    d if d.is_ascii_digit() => id = d.to_digit(10).unwrap(),
                    _ => {},
                }
            }
        }
        (id as usize, Self {pattern})
    }

    fn get_area(&self) -> usize {
        self.pattern.iter().filter(|&&b| b).count()
    }
}

impl Grid {
    fn get_area(&self) -> i64 {
        self.size.0 * self.size.1
    }

    fn shapes_sum(&self, shapes: &BTreeMap<usize, Shape>) -> i64 {
        self.shapes.iter().enumerate().map(|(i,c)| {
            c * shapes[&i].get_area() as i64
        }).sum()
    }

    fn get_possible_count(&self) -> i64 {
        (self.size.0 / 3) * (self.size.1 / 3)
    }

    fn get_count(&self) -> i64 {
        self.shapes.iter().sum()
    }

    fn read_line(ln: &str) -> Option<Self> {
        let (grid,shapes);
        match ln.split_once(':') {
            None => {
                return None;
            },
            Some((a,b)) =>  {
                grid = a;
                shapes = b;
            }
        };
        if shapes.trim().is_empty() {
            return None;
        }

        let (x,y) = grid.split_once('x').unwrap();
        Some(Self {size: (x.trim().parse().unwrap(),
            y.trim().parse().unwrap()),
            shapes: shapes.split_whitespace().map(|n| n.trim().parse().unwrap()).collect(),
        })
    }
}

fn read_contents(cont: &str) -> i64 {
    // This logic will determine the maximum possible answer and the minimum possible answer
    //

    let mut grids: Vec<Grid> = Vec::new();
    let mut shapes: BTreeMap<usize,Shape> = BTreeMap::new();
    let mut combined: String = "".to_string();
    for line in cont.lines() {
        if line.contains(':') {
            if line.contains('x') {
                grids.push(Grid::read_line(line).unwrap());
            } else {
                combined.push_str(line);
            }
        } else if line.contains("#") {
            combined.push_str(line);
        } else if !combined.is_empty() {
            let (id,shape) = Shape::from_lines(&combined);
            shapes.insert(id, shape);
            combined = "".to_string();
        }
            
    }

    println!("{} grids", grids.len());
    println!("{} shapes", shapes.len());

    // Just check if the combined area of the shapes is smaller than the grid area
    // This is the maximum number of valid grids
    let maximum_answer = grids.iter().filter(|g|
        g.get_area() >= g.shapes_sum(&shapes)
    ).count() as i64;

    // Check if the number of full 3x3 blocks in the grid is enough fit all shapes
    // This will give the minimum number of valid grids
    let minimum_answer = grids.iter().filter(|g|
        g.get_possible_count() >= g.get_count()
    ).count() as i64;

    assert_eq!(minimum_answer, maximum_answer);
    
    minimum_answer
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        // Target: Count how many grids can fit the shapes given
        let a = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        assert_eq!(read_contents(&a), 2);
    }

    #[test]
    fn shape() {

        let a = "0:
###
###
###";

        let b = "1:
###
.##
..#";
        let (id,shape) = Shape::from_lines(&a);
        dbg!(&shape);
        assert_eq!(id, 0);
        assert_eq!(shape.pattern.len(), 9);
        assert_eq!(shape.pattern, vec![true;9]);
        assert_eq!(shape.get_area(), 9);

        let (id,shape) = Shape::from_lines(&b);
        dbg!(&shape);
        assert_eq!(id, 1);
        assert_eq!(shape.pattern.len(), 9);
        assert_eq!(shape.get_area(), 6);
    }

    #[test]
    fn grid() {
        let a = "4x4: 0 0 0 0 2 0";
        let grid = Grid::read_line(&a).unwrap();
        dbg!(&grid);
        assert_eq!(grid.size.0, 4);
        assert_eq!(grid.size.1, 4);
        assert_eq!(grid.shapes[0], 0);
        assert_eq!(grid.shapes[4], 2);
    }
}
