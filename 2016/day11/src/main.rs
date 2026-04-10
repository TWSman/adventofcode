use clap::Parser;
use priority_queue::PriorityQueue;
use regex::Regex;
use std::collections::BTreeMap;
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
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i32, i32) {
    let state = read_state(cont);
    let part1 = find_route(&state);
    let mut state2 = state.clone();
    assert!(state2.steps == 0);
    let j = state2.things.len() / 2;

    // Elerium
    state2.things.insert((Thing::Chip, j), 1);
    state2.things.insert((Thing::Generator, j), 1);

    // Dilithium
    state2.things.insert((Thing::Chip, j + 1), 1);
    state2.things.insert((Thing::Generator, j + 1), 1);
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
            if !translations.contains_key(&chip) {
                translations.insert(chip.clone(), j);
                j += 1;
            }
            let ind = *translations.get(&chip).unwrap();
            things.insert((Thing::Chip, ind), (i + 1) as i32);
        }
        for generator in floor_generators {
            if !translations.contains_key(&generator) {
                translations.insert(generator.clone(), j);
                j += 1;
            }
            let ind = *translations.get(&generator).unwrap();
            things.insert((Thing::Generator, ind), (i + 1) as i32);
        }
    }
    State {
        elevator: 1, // Elevator starts on ground floor
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
    println!("Find route for {} things", start.things.len());
    let mut queue = PriorityQueue::new();
    let prio = start.get_priority();
    queue.push(start.clone(), prio);
    let mut min_steps = 9999;
    let mut visited: BTreeMap<String, i32> = BTreeMap::new();
    let mut loop_count = 0;
    let mut skipped = 0;
    let things = start.things.len();
    loop {
        loop_count += 1;
        if queue.is_empty() {
            return min_steps;
        }
        let (state, _) = queue.pop().unwrap();

        if state.is_finished() && state.steps < min_steps {
            min_steps = state.steps;
            println!("New minimum found in {loop_count} loops: {min_steps}");
            println!(
                "Loop count: {}, queue size: {}, visited size: {}, skipped: {}, steps: {}, min steps: {}",
                loop_count,
                queue.len(),
                visited.len(),
                skipped,
                state.steps,
                min_steps,
            );
            return min_steps;
        }
        if state.steps > min_steps {
            println!("Too many steps, stopping search");
            return min_steps;
        }
        if state.steps >= min_steps {
            continue;
        }
        if loop_count % 10_000 == 0 {
            let things4 = state.things.iter().filter(|(_, loc)| **loc == 4).count();
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
            if up_down == -1 && !state.floor_allowed(new_elevator) {
                //println!("This move is not allowed");
                continue;
            }
            for thing in &stuff_on_floor {
                //println!("Moving {:?} from floor {} to floor {}", thing, current_floor, new_elevator);
                let mut new_state = state.clone();
                new_state.elevator = new_elevator;
                new_state.things.insert(*thing, new_elevator);
                new_state.steps += 1;
                let prio = new_state.get_priority();
                let hash = new_state.get_hash();
                if new_state.is_valid() {
                    if !visited.contains_key(&hash)
                        || *visited.get(&hash).unwrap() > new_state.steps
                    {
                        visited.insert(hash, new_state.steps);
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
                        if !visited.contains_key(&hash)
                            || *visited.get(&hash).unwrap() > new_state.steps
                        {
                            visited.insert(hash, new_state.steps);
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
    things: BTreeMap<(Thing, usize), i32>,
    steps: i32,
}

impl State {
    fn floor_allowed(&self, floor: i32) -> bool {
        let things1 = self.things.iter().filter(|(_, loc)| **loc == 1).count() as i32;
        if floor > 2 {
            return true;
        }
        if floor == 1 {
            if things1 == 0 {
                // No point in moving stuff to floor 1 if there is nothing there
                return false;
            } else {
                return true;
            }
        }
        assert_eq!(floor, 2);
        let things2 = self.things.iter().filter(|(_, loc)| **loc == 2).count() as i32;
        if things2 == 0 && things1 == 0 {
            return false;
        }
        true
    }

    fn get_hash(&self) -> String {
        let mut output = format!("ele_{}", self.elevator);
        let mut vec = Vec::new();
        // Find pairs of generator and chip
        for (thing, loc) in self.things.iter() {
            if thing.0 == Thing::Generator {
                let loc_chip = self.things.get(&(Thing::Chip, thing.1)).unwrap();
                vec.push((loc, loc_chip));
            }
        }
        // Pairs get sorted by just location
        // This way swapping two pairs does not change the hash
        vec.sort();
        for (v1, v2) in vec {
            output.push_str(&format!("_{}{}", v1, v2));
        }
        output
    }

    fn is_valid(&self) -> bool {
        for (thing, loc) in self.things.iter() {
            // Check all chips, skip others
            if thing.0 != Thing::Chip {
                continue;
            }
            if self.things.get(&(Thing::Generator, thing.1)).unwrap() == loc {
                // This chip is with its generator, it's safe
                continue;
            }
            // This chip is not with its generator, check if there are other generators on the same floor
            if self
                .things
                .iter()
                .filter(|((t, _), f)| *t == Thing::Generator && *f == loc)
                .count()
                > 0
            {
                return false;
            }
        }
        //println!("Is valid");
        true
    }

    fn get_priority(&self) -> i32 {
        // More steps -> smaller priority
        let things3 = self.things.iter().filter(|(_, loc)| **loc == 3).count() as i32;
        let things2 = self.things.iter().filter(|(_, loc)| **loc == 2).count() as i32;
        let things1 = self.things.iter().filter(|(_, loc)| **loc == 1).count() as i32;
        let priority = self.steps + 6 * things1 + 4 * things2 + 2 * things3;
        -priority
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
