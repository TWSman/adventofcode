use clap::Parser;
use std::fs;
use std::collections::BTreeSet;

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


fn get_cubes(cont: &str) -> BTreeSet<(i32, i32, i32)> {
    cont.lines().map(|ln| {
        let x = ln.split(',').map(|x|
            x.parse::<i32>().unwrap()
        ).collect::<Vec<i32>>();
        assert_eq!(x.len(), 3);
        (x[0], x[1], x[2])
    }).collect::<BTreeSet<(i32, i32, i32)>>()
}

fn read_contents(cont: &str) -> (i64, i64) {
    let cubes = get_cubes(cont);

    let part1 = get_part1(&cubes);
    let part2 = get_part2(&cubes);
    (part1, part2)
}


fn vec_sum(a: (i32, i32, i32), b: (i32, i32, i32), multi: i32) -> (i32, i32, i32)  {
    // Add two 3D vectors with a multiplier
    (a.0 + b.0 * multi, a.1 + b.1 * multi, a.2 + b.2 * multi)
}

fn get_part1(cubes: &BTreeSet<(i32, i32, i32)>) -> i64 {
    println!("Calculating part 1 with {} cubes",  cubes.len());
    let opt: Vec<(i32, i32, i32)> = vec![
        ( 0, 0, 1),
        ( 0, 0,-1),
        ( 0, 1, 0),
        ( 0,-1, 0),
        ( 1, 0, 0),
        (-1, 0, 0),
    ];
    cubes.iter().map(|c| {
        opt.iter().map(|o| {
            let s = vec_sum(*c, *o, 1);
            if cubes.contains(&s) {
                0
            } else {
                1
            }
        }).sum::<i64>()
    }).sum()
}

fn get_part2(cubes: &BTreeSet<(i32, i32, i32)>) -> i64 {
    let mut cubes = cubes.clone();
    let opt: Vec<(i32, i32, i32)> = vec![
        ( 0, 0, 1),
        ( 0, 0,-1),
        ( 0, 1, 0),
        ( 0,-1, 0),
        ( 1, 0, 0),
        (-1, 0, 0),
    ];

    let max_x = cubes.iter().map(|c| c.0).max().unwrap();
    let min_x = cubes.iter().map(|c| c.0).min().unwrap();

    let max_y = cubes.iter().map(|c| c.1).max().unwrap();
    let min_y = cubes.iter().map(|c| c.1).min().unwrap();

    let max_z = cubes.iter().map(|c| c.2).max().unwrap();
    let min_z = cubes.iter().map(|c| c.2).min().unwrap();

    let size_x = max_x - min_x + 1;
    let size_y = max_y - min_y + 1;
    let size_z = max_z - min_z + 1;
    //dbg!((min_x, max_x, min_y, max_y, min_z, max_z));
    //dbg!((size_x, size_y, size_z));
    println!("Volume: {}", size_x * size_y * size_z);

    // Check all empty cubes
    let mut open_cubes: BTreeSet<(i32,i32,i32)> = BTreeSet::new();

    // For each empty cube try to find a way out (to min/max x/y/z)
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                if cubes.contains(&(x,y,z)) {
                    continue;
                }
                if open_cubes.contains(&(x,y,z)) {
                    continue;
                }

                // Dijkstra style search for a way out
                let mut heads: Vec<(i32, i32, i32)> = Vec::new();
                heads.push((x,y,z));
                let mut closed = true;

                // Keep track of other checked cubes
                // They will all be closed if and only if the starting point is closed
                let mut checked: Vec<(i32, i32, i32)> = Vec::new();
                loop {
                    if heads.is_empty() {
                        break;
                    }
                    let s = heads.pop().unwrap();
                    if s.0 < 0 || s.1 < 0 || s.2 < 0 {
                        closed = false;
                        break;
                    }
                    if s.0 > max_x || s.1 > max_y || s.2 > max_z {
                        closed = false;
                        break;
                    }

                    checked.push(s);

                    for o in &opt {
                        let sum = vec_sum(s, *o, 1);
                        if cubes.contains(&sum) {
                            continue;
                        }
                        if checked.contains(&sum) {
                            continue;
                        }
                        heads.push(sum);
                    }
                }


                if closed {
                    println!("Adding cube at {:?} as it's closed, and {} others", (x,y,z), checked.len() - 1);
                    for c in checked {
                        cubes.insert(c);
                    }
                } else {
                    for c in checked {
                        open_cubes.insert(c);
                    }
                }
            }
        }
    }

    // Now that the insides are filled, we can reuse part 1 to get the answer
    get_part1(&cubes)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a ="1,1,1
2,1,1";
        assert_eq!(read_contents(&a).0, 10);

        let a ="2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

        assert_eq!(read_contents(&a).0, 64);
    }
    
    #[test]
    fn part2() {
        let a ="2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

        assert_eq!(read_contents(&a).1, 58);

    }
}
