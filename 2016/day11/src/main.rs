use clap::Parser;
use priority_queue::PriorityQueue;
use regex::Regex;
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
    // 67 is too high (or at least wrong)
    // 65 is too high (or at least wrong)
    // 64 is wrong
    // 63 is wrong
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i32, i32) {
    let state = read_state(cont);
    let part1 = find_route(&state);
    let mut state2 = state.clone();
    assert!(state2.steps == 0);
    let j = state2.things.len();
    state2.things.insert((Thing::Chip, j + 1), 1);
    state2.things.insert((Thing::Generator, j + 1), 1);

    state2.things.insert((Thing::Chip, j + 2), 1);
    state2.things.insert((Thing::Generator, j + 2), 1);
    let part2 = find_route(&state2);
    (part1, part2)
}

fn read_state(cont: &str) -> State {
    let re_microchip = Regex::new(r"(\w+)-compatible microchip").unwrap();
    let re_generator = Regex::new(r"(\w+) generator").unwrap();
    let mut translations: BTreeMap<String, usize> = BTreeMap::new();
    let mut things = BTreeMap::new();
    let mut j = 0;
    for (i, ln) in cont.lines().enumerate() {
        println!("Floor: {}", 1 + i);
        if ln.contains("nothing") {
            continue;
        }
        let floor_microchips = re_microchip
            .captures_iter(ln)
            .map(|cap| cap[1].to_string())
            .collect::<Vec<_>>();
        let floor_generators = re_generator
            .captures_iter(ln)
            .map(|cap| cap[1].to_string())
            .collect::<Vec<_>>();
        for chip in floor_microchips {
            //chips.insert(chip, (i + 1) as i32);
            if !translations.contains_key(&chip) {
                translations.insert(chip.clone(), j);
                j += 1;
            }
            let ind = *translations.get(&chip).unwrap();
            things.insert((Thing::Chip, ind), (i + 1) as i32);
        }
        for generator in floor_generators {
            //generators.insert(generator, (i + 1) as i32);
            if !translations.contains_key(&generator) {
                translations.insert(generator.clone(), j);
                j += 1;
            }
            let ind = *translations.get(&generator).unwrap();
            things.insert((Thing::Generator, ind), (i + 1) as i32);
        }
    }
    State {
        elevator: 1,
        things,
        steps: 0,
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
enum Thing {
    Generator,
    Chip,
}

fn find_route(start: &State) -> i32 {
    let mut queue = PriorityQueue::new();
    let prio = start.get_priority();
    queue.push(start.clone(), prio);
    let mut min_steps = 9999;
    let mut visited: BTreeSet<String> = BTreeSet::new();
    let mut loop_count = 0;
    let mut skipped = 0;
    loop {
        loop_count += 1;
        if queue.is_empty() {
            return min_steps;
        }
        let (state, _) = queue.pop().unwrap();
        if state.is_finished() && state.steps < min_steps {
            min_steps = state.steps;
            println!("New minimum found in {loop_count} loops: {min_steps}");
            //return min_steps;
            continue;
        }
        if state.steps > min_steps + 20 {
            println!("Too many steps, stopping search");
            return min_steps;
        }
        if state.steps >= min_steps {
            continue;
        }
        if loop_count % 10_000 == 0 {
            let things4 = state.things.iter().filter(|(_, loc)| **loc == 4).count();
            let things = state.things.len();
            println!(
                "Loop count: {}, queue size: {}, visited size: {}, skipped: {}, steps: {}, min steps: {}, top floor: {} / {}",
                loop_count,
                queue.len(),
                visited.len(),
                skipped,
                state.steps,
                min_steps,
                things4,
                things
            );
        }
        for up_down in [-1, 1] {
            let current_floor = state.elevator;
            let stuff_on_floor: Vec<(Thing, usize)> = state
                .things
                .iter()
                .filter(|(_, loc)| **loc == current_floor)
                .map(|(t, _)| *t)
                .collect();
            let new_elevator = state.elevator + up_down;
            if !(1..=4).contains(&new_elevator) {
                continue;
            }
            for thing in &stuff_on_floor {
                //println!("Moving {:?} from floor {} to floor {}", thing, current_floor, new_elevator);
                // Check moving any chip
                let mut new_state = state.clone();
                new_state.elevator = new_elevator;
                new_state.things.insert(*thing, new_elevator);
                new_state.steps += 1;
                let prio = new_state.get_priority();
                let hash = new_state.get_hash();
                if new_state.is_valid() {
                    if !visited.contains(&hash) {
                        //println!("Moving {:?} from floor {} to floor {}", thing, current_floor, new_elevator);
                        visited.insert(hash);
                        queue.push(new_state, prio);
                    } else {
                        skipped += 1;
                    }
                }
                for thing2 in &stuff_on_floor {
                    if thing == thing2 {
                        continue;
                    }
                    //println!("Moving chips {} and {} from floor {} to floor {}", chip.0, chip2.0, current_floor, new_elevator);
                    let mut new_state = state.clone();
                    new_state.elevator = new_elevator;
                    new_state.things.insert(*thing, new_elevator);
                    new_state.things.insert(*thing2, new_elevator);
                    new_state.steps += 1;
                    let prio = new_state.get_priority();
                    let hash = new_state.get_hash();
                    //println!("Moving {:?} and {:?}from floor {} to floor {}", thing, thing2, current_floor, new_elevator);
                    if new_state.is_valid() {
                        if !visited.contains(&hash) {
                            visited.insert(hash);
                            queue.push(new_state, prio);
                        } else {
                            skipped += 1;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    elevator: i32,
    //chips: BTreeMap<String, i32>,
    //generators: BTreeMap<String, i32>,
    things: BTreeMap<(Thing, usize), i32>,
    steps: i32,
}

impl State {
    fn get_hash(&self) -> String {
        let mut output = format!("ele_{}", self.elevator);
        for (thing, loc) in self.things.iter() {
            if thing.0 == Thing::Generator {
                output.push_str(&format!("g{}", loc));
            }
        }

        for (thing, loc) in self.things.iter() {
            if thing.0 == Thing::Chip {
                output.push_str(&format!("c{}", loc));
            }
        }
        output
    }

    fn is_valid(&self) -> bool {
        for (thing, loc) in self.things.iter() {
            if thing.0 != Thing::Chip {
                continue;
            }
            if self.things.get(&(Thing::Generator, thing.1)).unwrap_or(&0) != loc {
                // This chip is not with its generator, check if there are other generators on the same floor
                for (_, gen_loc) in self
                    .things
                    .iter()
                    .filter(|((t, _), _)| *t == Thing::Generator)
                {
                    if gen_loc == loc {
                        //println!("Invalid state: chip {:<10} is on floor {} with generator {:<10}", chip, loc, gene);
                        return false;
                    }
                }
            }
        }
        //println!("Is valid");
        true
    }

    fn get_priority(&self) -> i32 {
        //let things = self.things.len() as i32;
        // More steps -> smaller priority
        let mut priority = -4 * self.steps;
        // More stuff at loc = 4 -> Higher priority
        //let things4 = self.things.iter().filter(|(_,loc)| **loc == 4).count() as i32;
        let things3 = self.things.iter().filter(|(_, loc)| **loc == 3).count() as i32;
        let things2 = self.things.iter().filter(|(_, loc)| **loc == 2).count() as i32;
        let things1 = self.things.iter().filter(|(_, loc)| **loc == 1).count() as i32;
        priority -= 36 * things1 + 16 * things2 + 12 * things3;
        priority
    }

    fn is_finished(&self) -> bool {
        for (_, loc) in self.things.iter() {
            if *loc != 4 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";
        assert_eq!(read_contents(&a).0, 11);
    }
}
