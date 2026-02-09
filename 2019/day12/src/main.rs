use clap::Parser;
use shared::Vec3D;
use std::fs;
use std::time::Instant;
use regex::Regex;
use num_integer::lcm;

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
    let res = read_contents(&contents, 1000);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);

    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_moons(cont: &str) -> Vec<Moon> {
    cont.lines().enumerate().map(|(i,ln)|Moon::new(ln, i)).collect::<Vec<Moon>>()
}

fn read_contents(cont: &str, loops: usize) -> (i64, i64) {
    let moons = read_moons(cont);
    let part1 = get_part1(&moons, loops);
    let part2 = get_part2(&moons, 1_000_000);
    (part1, part2)
}

fn get_part1(moons: &[Moon], steps: usize) -> i64 {
    let mut moons = moons.to_owned();
    for _ in 0..steps {
        let moons2 = moons.clone();
        for moon in moons.iter_mut() {
            moon.apply_gravity(&moons2);
        }
        for moon in moons.iter_mut() {
            moon.propagate();
        }

    }
    for i in 0..steps {
        for moon in &moons {
            print!("{}, {}, {}, ", moon.history[i].x, moon.history[i].y, moon.history[i].z);
        }
        println!();

    }
    moons.iter().map(|m| m.total_energy()).sum()
}

fn get_part2(moons: &Vec<Moon>, max_steps: usize) -> i64 {
    // Solves actual input but is quite slow (took about 10 mins)
    // Actual input had cycles with lengths 160 000 - 180 000
    println!("Starting part 2 with max steps {}, initial State:", max_steps);

    for moon in moons {
        println!("Moon {}: position: {}, {}, {}",
            moon.id,
            moon.position.x,
            moon.position.y,
            moon.position.z,
            );
    }

    assert_eq!(moons.len(), 4);

    let mut moons = moons.clone();
    let mut cycle_found: Vec<Option<i64>> = vec![None, None, None];
    for i_step in 0..max_steps {
        if i_step % 10000 == 0 {
            println!("Step {:>7}: ", i_step);
        }
        let mut back_to_start = [true, true, true, true];
        for i_moon in 0..moons.len() {
            for j in (i_moon+1)..moons.len() {
                let (left, right) = moons.split_at_mut(j);
                Moon::apply_pair(&mut left[i_moon], &mut right[0]);
            }
        }
        for moon in moons.iter_mut() {
            moon.propagate();
            for (i_axis, b) in back_to_start.iter_mut().enumerate() {
                if *b && !moon.back_to_start(if i_axis < 3 {Some(i_axis)} else {None}) {
                    *b = false;
                }
            }
        }

        for (i_axis, b) in back_to_start.iter().take(3).enumerate() {
            if cycle_found[i_axis].is_none() && *b {
                cycle_found[i_axis] = Some(moons[0].history.len() as i64 - 1);
                dbg!(&cycle_found);
                if cycle_found.iter().all(|c| c.is_some()) {
                    println!("All cycles found after {} steps: {:?}", moons[0].history.len() - 1, cycle_found);
                    let lcm = cycle_found.iter().map(|c| c.unwrap()).fold(1, lcm);
                    println!("LCM of cycles is {}", lcm);

                    for j in 0..i_step {
                        for moon in &moons {
                            print!("{}, {}, {}, ", moon.history[j].x, moon.history[j].y, moon.history[j].z);
                        }
                        println!();

                    }

                    return lcm;
                }
            }
        }
    }
    max_steps as i64
}


#[derive(Debug, Clone)]
struct Moon {
    id: usize,
    position: Vec3D,
    velocity: Vec3D,
    history: Vec<Vec3D>,
    prev0: usize,
}

impl Moon {
    fn new(ln: &str, i: usize) -> Self {
        let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
        if let Some(res) = re.captures(ln) {
            let pos = Vec3D {
                    x: res[1].parse::<i64>().unwrap(),
                    y: res[2].parse::<i64>().unwrap(),
                    z: res[3].parse::<i64>().unwrap()
                };

            Self {
                position: pos,
                velocity: Vec3D { x: 0, y: 0, z: 0 },
                id: i,
                history: vec![pos],
                prev0: 0,
            }
        } else {
            panic!("Bad line: {}", ln);
        }
    }

    fn apply_pair(a: &mut Self, b: &mut Self) {
        let kick_a = a.get_kick(b);
        let kick_b = b.get_kick(a);
        assert!(kick_a.x == -kick_b.x);
        a.velocity = a.velocity + kick_a;
        b.velocity = b.velocity + kick_b;
    }

    fn get_kick_element(dx: i64) -> i64 {
        if dx > 0 {
            1
        } else if dx < 0 {
            -1
        } else {
            0
        }
    }

    // 0 for x, 1 for y, 2 for z, None for all
    fn back_to_start(&self, axis: Option<usize>) -> bool {
        if let Some(a) = axis {
            if a == 0 {
                return self.position.x == self.history[0].x && self.velocity.x == 0;
            } else if a == 1 {
                return self.position.y == self.history[0].y && self.velocity.y == 0;
            } else if a == 2 {
                return self.position.z == self.history[0].z && self.velocity.z == 0;
            } else {
                panic!("Invalid axis: {}", a);
            }
        }
        self.position == self.history[0] && self.velocity == Vec3D { x: 0, y: 0, z: 0 }
    }

    fn potential_energy(&self) -> i64 {
        self.position.manhattan(&Vec3D { x: 0, y: 0, z: 0 })
    }

    fn kinetic_energy(&self) -> i64 {
        self.velocity.manhattan(&Vec3D { x: 0, y: 0, z: 0 })
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn apply_gravity(&mut self, moons: &Vec<Moon>) {
        let mut total_kick = Vec3D{ x: 0, y: 0, z: 0 };
        if self.velocity.manhattan(&Vec3D { x: 0, y: 0, z: 0 }) == 0 && self.history.len() > 1 {
            if self.back_to_start(None) && self.history.len() > 1 {
                println!("Moon {} back to start after {} steps ({} since previous), position: {}, {}, {}",
                    self.id,
                    self.history.len() - 1,
                    self.history.len() - 1 - self.prev0,
                    self.position.x,
                    self.position.y,
                    self.position.z);
            } else  {
                println!("Moon {} has zero velocity after {} steps ({} since previous), position: {}, {}, {}",
                    self.id,
                    self.history.len() - 1,
                    self.history.len() - 1 - self.prev0,
                    self.position.x,
                    self.position.y,
                    self.position.z);
                self.prev0 = self.history.len() - 1;
            }
        }


        for moon in moons {
            if moon.id == self.id {
                continue;
            }
            total_kick = total_kick +  self.get_kick(moon);
        }
        self.velocity = self.velocity + total_kick;
    }

    fn propagate(&mut self) {
        self.position = self.position + self.velocity;
        self.history.push(self.position);
    }

    fn get_kick(&self, other: &Self) -> Vec3D {
        let dx = other.position.x - self.position.x;
        let dy = other.position.y - self.position.y;
        let dz = other.position.z - self.position.z;

        Vec3D {
            x: Self::get_kick_element(dx),
            y: Self::get_kick_element(dy),
            z: Self::get_kick_element(dz),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let moons = read_moons(&a);
        assert_eq!(get_part1(&moons, 10), 179);

        let b = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

        let moons = read_moons(&b);
        assert_eq!(get_part1(&moons, 100), 1940);
    }

    #[test]
    fn part2() {
        let a = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let moons = read_moons(&a);
        assert_eq!(get_part2(&moons, 2900), 2772);

        let b = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

        let moons = read_moons(&b);
        assert_eq!(get_part2(&moons, 1_000_000), 4_686_774_924);
        // 4686774924 = 2 * 2 * 3 * 13 * 13 * 983 * 2351
        // = 983 * 2028 * 2351
            
    }
}
