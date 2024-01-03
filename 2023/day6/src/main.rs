fn main() {
    let input_part1 = "Time:        47     70     75     66
Distance:   282   1079   1147   1062";

    let input_part2 = "Time:        47707566
Distance:   282107911471062";

    println!("Part 1 answer is {}", read_contents(input_part1));
    println!("Part 2 answer is {}", read_contents(input_part2));
}

const ACC: f64 = 1.0; // Acceleration is 1m/s/s
// Must have this win margin, end result will always be a whole number.
// Thus the exact value does not matter
const MARGIN: f64 = 0.5;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}


fn second_degree(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant: f64 = b * b - 4.0 * a * c;
    // We assume that here the polynomials always have two solutions.
    // This means that the discriminant must be positive.
    // In the context of the task, this assumes that there is a way to win every race.
    // No solutions would mean that a race is impossible to win
    assert!(discriminant > 0.0);
    let sol2 = 0.5 * (-b - f64::sqrt(discriminant)) / a;
    let sol1 = 0.5 * (-b + f64::sqrt(discriminant)) / a;
    // In this task a is always negative, so sol1 should be smaller
    assert!(sol1 < sol2);
    (sol1, sol2)
}

fn analyze_race(r: &Race) -> i64 {
    // Td = Time used for traveling
    // Td = Time used for acceleration
    // v = Speed achieved, a = acceleration
    // D = Target distance (record distance + a minimal win margin)
    // Distance traveled is v * Td
    // Speed is Ta * a
    // Td is remaining distance Td = T - Ta
    // Thus Distance traveled is d= Ta * a * (T - Ta) = -a * Ta^2  + a * T * Ta 
    // Extra distance is d - D = -a * Ta^2 + a * T * Ta -D
    // Second degree polynomial, A * x^2 + B * x + C
    // A = -a = 1; B = a * T = T; C = -D
    // Solutions are
    // [- B +- sqrt(B^2 - 4 * A * C)] / [2A]
    let a = -1.0 * ACC;
    let b = ACC * r.time as f64;
    let c = -1.0 * (r.distance as f64 + MARGIN);
    // Every race should have 2 solutions
    let (sol1, sol2) = second_degree(a, b, c);
    let n1 = sol1.ceil() as i64;
    let n2 = sol2.floor() as i64;
    n2 - n1 + 1
}

fn read_contents(cont: &str) -> i64 {
    let lines: Vec<&str> = cont.lines().collect();
    let times: Vec<i64> = lines[0].split_whitespace().filter_map(|m| { match m.parse::<i64>()
        { Ok(val) => Some(val),
            Err(_) => None,
        }
    }).collect();
    dbg!(&times);
    let distances: Vec<i64> = lines[1].split_whitespace().filter_map(|m| { match m.parse::<i64>()
        { Ok(val) => Some(val),
            Err(_) => None,
        }
    }).collect();
    assert_eq!(distances.len(), times.len());
    let n = distances.len();
    let races: Vec<Race> = (0..n).map(|i| {Race {time: times[i], distance: distances[i]}}).collect();
    //dbg!(&distances)
    let results: Vec<i64> = races.iter().map(analyze_race).collect();
    return results.iter().product();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a: &str = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(read_contents(&a), 288);
        let b: &str = "Time:      71530
Distance:  940200";
        assert_eq!(read_contents(&b), 71503);
    }
    #[test]
    fn range() {
        let r1 = Race {time: 7, distance: 9};
        let r2 = Race {time: 15, distance: 40};
        let r3 = Race {time: 30, distance: 200};
        assert_eq!(analyze_race(&r1), 4);
        assert_eq!(analyze_race(&r2), 8);
        assert_eq!(analyze_race(&r3), 9);
    }
}
