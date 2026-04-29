use clap::Parser;
use shared::Vec3D;
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
    // 406 is too high
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Particle {
    position: Vec3D,
    velocity: Vec3D,
    acceleration: Vec3D,
}

impl Particle {
    fn new(ln: &str) -> Self {
        let parts = ln.split(">,").collect::<Vec<_>>();
        Self {
            position: get_vec3d(parts[0]),
            velocity: get_vec3d(parts[1]),
            acceleration: get_vec3d(parts[2]),
        }
    }

    fn get_location_at_time(&self, t: i64) -> Vec3D {
        self.position + self.velocity * t + self.acceleration * (t * (t + 1) / 2)
    }
}

fn get_vec3d(part: &str) -> Vec3D {
    let (_, nums) = part.split_once("=<").unwrap();
    let nums = nums
        .trim_end_matches(">")
        .split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    Vec3D {
        x: nums[0],
        y: nums[1],
        z: nums[2],
    }
}

fn get_collision_time(a: &Particle, b: &Particle) -> Option<i64> {
    let x_collision = get_collision_for_axis(a, b, 0);
    let y_collision = get_collision_for_axis(a, b, 1);
    let z_collision = get_collision_for_axis(a, b, 2);
    if x_collision.is_empty() || y_collision.is_empty() || z_collision.is_empty() {
        return None;
    }
    let mut potential = Vec::new();
    for t in &x_collision {
        if y_collision.contains(t) && z_collision.contains(t) {
            potential.push(*t);
        }
    }
    assert!(potential.len() <= 1);
    potential.first().cloned()
}

fn get_collision_for_axis(a: &Particle, b: &Particle, axis: usize) -> Vec<i64> {
    let mut output = Vec::new();
    let (xa, va, aa) = match axis {
        0 => (a.position.x, a.velocity.x, a.acceleration.x),
        1 => (a.position.y, a.velocity.y, a.acceleration.y),
        2 => (a.position.z, a.velocity.z, a.acceleration.z),
        _ => panic!("Invalid axis"),
    };
    let (xb, vb, ab) = match axis {
        0 => (b.position.x, b.velocity.x, b.acceleration.x),
        1 => (b.position.y, b.velocity.y, b.acceleration.y),
        2 => (b.position.z, b.velocity.z, b.acceleration.z),
        _ => panic!("Invalid axis"),
    };

    // We want to solve the equation
    // xa + va*t + aa*t*(t+1)/2 = xb + vb*t + ab*t*(t+1)/2
    // i.e.
    // (aa - ab)*t^2 + (2* (va - vb) + (aa - ab))*t + (xa - xb) = 0
    let polya = aa - ab;
    let polyb = 2 * (va - vb) + (aa - ab);
    let polyc = 2 * (xa - xb);

    if polyc == 0 {
        // Particles start at the same position, so they collide at time 0
        output.push(0);
        return output;
    }
    if polya == 0 && polyb == 0 && polyc == 0 {
        // Particles have exactly the same trajectory
        return output;
    }

    if polya == 0 {
        // Linear case, solution exists if -polyc is divisible by polyb
        if polyb == 0 {
            return output;
        }
        let t = -polyc / polyb;
        if polyc % polyb == 0 && t >= 0 {
            output.push(t);
        }
        return output;
    }

    // Solution will exist if the discriminant is a perfect positive square
    let disc = polyb * polyb - 4 * polya * polyc;
    if disc < 0 {
        // Discriminant is negative, no real solutions
        return output;
    }
    if disc == 0 {
        // Only one potential solution
        let t = -polyb / (2 * polya);
        if polyb % (2 * polya) == 0 && t >= 0 {
            // Make sure t is an integer and non-negative
            output.push(t);
        }
        return output;
    }
    // Otherwise, we have two potential solutions, but we need to check if the discriminant is a perfect square
    let sqrt_disc = (disc as f64).sqrt() as i64;
    if sqrt_disc * sqrt_disc != disc {
        return output;
    }
    // Check both potential solutions
    // They both need to be integers and non-negative
    if (-polyb + sqrt_disc) % (2 * polya) == 0 {
        let t = (-polyb + sqrt_disc) / (2 * polya);
        if t >= 0 {
            output.push(t);
        }
    }
    if (-polyb - sqrt_disc) % (2 * polya) == 0 {
        let t = (-polyb - sqrt_disc) / (2 * polya);
        if t >= 0 {
            output.push(t);
        }
    }
    output
}

fn get_part1(particles: &[Particle]) -> i64 {
    let mut min_acc = 9999;
    let mut min_idx = 0;
    let zero = Vec3D { x: 0, y: 0, z: 0 };
    for (i, p) in particles.iter().enumerate() {
        let acc = p.acceleration.manhattan(&zero);
        if acc < min_acc {
            min_acc = acc;
            min_idx = i;
        } else if acc == min_acc {
            let min_particle = &particles[min_idx];
            let vela = (p.acceleration * 100 + p.velocity).manhattan(&zero);
            let velb = (min_particle.acceleration * 100 + min_particle.velocity).manhattan(&zero);
            if vela < velb {
                min_idx = i;
            } else if vela == velb {
                return 0;
            }
        }
    }
    min_idx as i64
}

fn get_part2(particles: &[Particle]) -> i64 {
    let removed = get_removed(particles);
    (particles.len() - removed.len()).try_into().unwrap()
}

fn get_removed(particles: &[Particle]) -> BTreeSet<usize> {
    let mut collisions: BTreeMap<i64, Vec<(usize, usize)>> = BTreeMap::new();
    for (ia, pa) in particles.iter().enumerate() {
        for (ib, pb) in particles.iter().enumerate().skip(ia + 1) {
            if pa.position == pb.position {
                collisions.entry(0).or_default().push((ia, ib));
                continue;
            }
            if let Some(t) = get_collision_time(pa, pb) {
                assert!(t > 0);
                assert!(pa.get_location_at_time(t) == pb.get_location_at_time(t));
                collisions.entry(t).or_default().push((ia, ib));
            }
        }
    }
    let mut removed: BTreeSet<usize> = BTreeSet::new();
    for (_t, v) in collisions.iter() {
        let mut new_removed = BTreeSet::new();
        for (ia, ib) in v {
            if !removed.contains(ia) && !removed.contains(ib) {
                new_removed.insert(*ia);
                new_removed.insert(*ib);
            }
        }
        removed.extend(new_removed.iter());
    }
    //println!("Removed {} particles", removed.len());
    removed
}

fn read_contents(cont: &str) -> (i64, i64) {
    let particles: Vec<Particle> = cont.lines().map(Particle::new).collect();
    let part1 = get_part1(&particles);
    let part2 = get_part2(&particles);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";
        assert_eq!(read_contents(&a).0, 0);
    }

    #[test]
    fn part2() {
        let a = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";
        assert_eq!(read_contents(&a).1, 1);
    }

    #[test]
    fn collision() {
        let a = Particle {
            position: Vec3D { x: -6, y: 0, z: 0 },
            velocity: Vec3D { x: 3, y: 0, z: 0 },
            acceleration: Vec3D { x: 0, y: 0, z: 0 },
        };
        let b = Particle {
            position: Vec3D { x: -4, y: 0, z: 0 },
            velocity: Vec3D { x: 2, y: 0, z: 0 },
            acceleration: Vec3D { x: 0, y: 0, z: 0 },
        };
        let c = Particle {
            position: Vec3D { x: -2, y: 0, z: 0 },
            velocity: Vec3D { x: 1, y: 0, z: 0 },
            acceleration: Vec3D { x: 0, y: 0, z: 0 },
        };
        assert_eq!(get_collision_for_axis(&a, &b, 0), Some(2));
        assert_eq!(get_collision_for_axis(&a, &b, 1), Some(0));
        assert_eq!(get_collision_for_axis(&a, &b, 2), Some(0));
        assert_eq!(get_collision_time(&a, &b), Some(2));

        assert_eq!(get_collision_for_axis(&a, &c, 0), Some(2));
        assert_eq!(get_collision_for_axis(&a, &c, 1), Some(0));
        assert_eq!(get_collision_for_axis(&a, &c, 2), Some(0));
        assert_eq!(get_collision_time(&a, &c), Some(2));
    }
}
