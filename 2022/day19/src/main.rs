use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use regex::Regex;
use priority_queue::PriorityQueue;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
    // 918 is too low
    println!("Part 2 answer is {}", res.1);  
}


fn get_blueprints(cont: &str) -> Vec<Blueprint> {
    cont.lines().map(|ln| {
        Blueprint::new(ln)
    }).collect()
}


#[derive(Debug, Clone, Copy)]
struct Cost {
    ore: i32,
    clay: i32,
    obsidian: i32,
}

impl Cost {
    fn new() -> Self {
        Self {ore: 0, clay: 0, obsidian: 0}
    }

    fn sum(&self, other: Self) -> Self{
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
        }

    }
}

#[derive(Debug)]
struct Blueprint {
    // Robot costs
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
    id: i64,
}

impl Blueprint {
    fn new(ln: &str) -> Self {
        let (a, b) = ln.split_once(":").unwrap();
        let re_cost = Regex::new(r"([0-9]+) (\w*)").unwrap();

        let id = a.split_whitespace().nth(1).unwrap().parse().unwrap();

        let mut bp = Self {
            ore: Cost { ore: 4, clay: 0, obsidian: 0 },
            clay: Cost { ore: 2, clay: 0, obsidian: 0 },
            obsidian: Cost { ore: 3, clay: 14, obsidian: 0 },
            geode: Cost { ore: 2, clay: 0, obsidian: 7 },
            id: id,
        };
        let re = Regex::new(r"Each (\w*) robot costs ([0-9\w ]*)\.").unwrap();

        for res in re.captures_iter(b) {
            let robot = &res[1];
            let costs = &res[2];
            let mut cost = Cost::new();

            for res_cost in re_cost.captures_iter(costs) {
                match &res_cost[2] {
                    "ore" => {
                        cost.ore = res_cost[1].parse().unwrap();
                    },
                    "clay" => {
                        cost.clay = res_cost[1].parse().unwrap();
                    }
                    "obsidian" => {
                        cost.obsidian = res_cost[1].parse().unwrap();
                    }
                    _ => {
                        dbg!(&res_cost);
                        panic!("Unknown cost")
                    }
                }
            }

            match robot {
                "ore" => {
                    bp.ore = cost;
                }
                "clay" => {
                    bp.clay = cost;
                }
                "obsidian" => {
                    bp.obsidian = cost;
                }
                "geode" => {
                    bp.geode = cost;
                }
                _ => {
                    panic!("Unknown robot")
                }
            }
        }
        bp
    }

    fn quality_level(&self) -> i64 {
        let geodes = self.get_geodes(None);
        self.id * geodes as i64
    }

    fn get_value(&self, state: &State, debug: bool) -> i64 {
        let remaining_time = (24 - state.time) as f64;
        let geode_value = 1.0;
        let geode_robot_value = remaining_time * geode_value;

        // Obsidian is used to make geode robots
        // Obsidian value needs to converted to geode robots. Thus -1.0
        let obsidian_value =if remaining_time > 1.0 {
            (remaining_time - 1.0) * geode_value / (self.geode.obsidian as f64)
        } else {
            0.0
        };
        let obsidian_robot_value = remaining_time * obsidian_value;

        // Clay is used to make obsidian robots
        // Clay needs to first converted to obsidian robots, then to geode robots. Thus -2.0
        let clay_value = if remaining_time > 2.0 {
            obsidian_value * (remaining_time - 2.0) / self.obsidian.clay as f64
        } else {
            0.0
        };
        let clay_robot_value = remaining_time * clay_value;

        // Ore is used for everything. Thus value is a sum of all uses
        // First consider ore used to make geode robots. Through geode robots there is a lag of 1
        // step before value is created
        let mut ore_value = if remaining_time > 1.0 {
            geode_value * (remaining_time - 1.0) / (self.geode.ore as f64)
        } else {
            0.0 as f64
        };
        if remaining_time > 2.0 {
            // Next consider ore used to make obsidian robots. Through obsidian robots there is a
            // lag of 2 steps before value is created
            ore_value += obsidian_value * (remaining_time - 2.0) / (self.obsidian.ore as f64);
        }
        if remaining_time > 3.0 {
            // Next consider ore used to make clay robots. Through clay robots there is a lag of 3
            ore_value += clay_value * (remaining_time - 3.0) / (self.clay.ore as f64);
        }

        if remaining_time > 4.0 {
            // Finally consider ore used to make new ore robots. Through ore robots there is a lag of 4
            ore_value += ore_value * (remaining_time - 4.0) / (self.ore.ore as f64);
        }
        assert!(ore_value >= 0.0);
        let ore_robot_value = 0_f64.max(remaining_time * ore_value);

        if debug {
            dbg!(&remaining_time);
            dbg!(&geode_value);
            dbg!(&geode_robot_value);
            dbg!(&obsidian_value);
            dbg!(&obsidian_robot_value);
            dbg!(&clay_value);
            dbg!(&clay_robot_value);
            dbg!(&ore_value);
            dbg!(&ore_robot_value);
        }

        //dbg!(&ore_robot_value);
        //dbg!(&clay_robot_value);
        //dbg!(&obsidian_robot_value);
        //dbg!(&geode_robot_value);

        (1.0e1 * (state.geode as f64 * geode_value +
        state.obsidian as f64 * obsidian_value +
        state.clay as f64 * clay_value +
        state.ore as f64 * ore_value +
        state.geode_robots as f64 * geode_robot_value +
        state.obsidian_robots as f64 * obsidian_robot_value +
        state.clay_robots as f64 * clay_robot_value +
        state.ore_robots as f64 * ore_robot_value)) as i64
    }

    fn get_geodes(&self, optimal: Option<Vec<Action>>) -> i32 {
        dbg!(&self);
        let mut queue: PriorityQueue<State, i64> = PriorityQueue::new();
        let start_state = State::new();
        //queue.push(State::new(), self.get_value(&start_state, false));
        queue.push(start_state, 0);

        let mut max_geodes: i32 = 0;
        let mut max_time: i32 = 0;

        let mut loop_count = 0;

        loop {
            if queue.is_empty() {
                println!("Queue empty after {loop_count} loops");
                break;
            }
            //dbg!(queue.len());
            let (state, prio) = queue.pop().unwrap();
            loop_count += 1;
            if false {
                if loop_count > 20 {
                    dbg!(&queue);
                    break;
                }
                println!();
            }
            assert_eq!(state.time as usize, state.history.len());
            let mut debug = false;
            if let Some(ref opt) = optimal {
                let sub_target = opt[0..state.time as usize].to_vec();
                if sub_target == state.history {
                    println!("Time: {}", state.time + 1);
                    debug = true;
                }
            }; 
            //dbg!(&state);
            
            // 1 minute passed
            let mut new_state = state.clone();
            new_state.forward();
            max_time = max_time.max(new_state.time);
            //if loop_count % 100000 == 0 {
            //    dbg!(&loop_count, &max_geodes, &max_time, &queue.len());
            //    //dbg!(&state);
            //    //dbg!(&state.time, &loop_count, &queue.len(), &max_geodes);
            //}

            if new_state.time == 24 {
                // Reached time limit
                if new_state.geode > max_geodes {
                    dbg!(&new_state);
                    max_geodes = new_state.geode;
                    dbg!(&loop_count, &max_geodes);
                }
                else if new_state.geode < max_geodes - 1 {
                    println!("Pruned at time limit with geodes: {}", new_state.geode);
                    dbg!(&queue.len());
                    dbg!(&new_state);
                    dbg!(&prio);
                    self.get_value(&state, true);
                    break;
                }
                continue;
            }

            
            for action in Action::iter() {
                if debug {
                    dbg!(&action);
                }
                // It seems to obay to just build one robot at a time
                let mut new_head = new_state.clone();
                match action {
                    Action::BuildGeode => {
                        if !state.can_afford(&self.geode) {
                            if debug {
                                println!("Can't afford geode robots");
                            }
                            continue;
                        }
                        if debug {
                            println!("Build geode robot");
                        }
                        new_head.build(self.geode); // subtracts cost
                        new_head.geode_robots += 1;
                    }
                    Action::BuildObsidian => {
                        if !state.can_afford(&self.obsidian) {
                            if debug {
                                println!("Can't afford obsidian robots");
                            }
                            continue;
                        }
                        new_head.build(self.obsidian); // subtracts cost
                        new_head.obsidian_robots += 1;
                    }
                    Action::BuildClay => {
                        if !state.can_afford(&self.clay) {
                            if debug {
                                println!("Can't afford clay robots");
                            }
                            continue;
                        }
                        new_head.build(self.clay); // subtracts cost
                        new_head.clay_robots += 1;
                    }
                    Action::BuildOre => {
                        if state.clay_robots > 0 {
                            // No point building ore robots after starting to build clay ones
                            if debug {
                                println!("Already started building clay robots");
                            }
                            continue;
                        }
                        if !state.can_afford(&self.ore) {
                            if debug {
                                println!("Can't afford ore robots");
                            }
                            continue;
                        }
                        new_head.build(self.ore); // subtracts cost
                        new_head.ore_robots += 1;
                    }
                    Action::Wait => {
                        // Do nothing
                    }
                }
                new_head.history.push(action);
                let val = self.get_value(&new_head, false);
                if val > 0 {
                    if debug {
                        dbg!(&action, &val);
                    }
                    let prio = new_head.get_prio();
                    queue.push(new_head, prio);
                    if action == Action::BuildGeode || action == Action::BuildObsidian {
                        // Always build either a geode or obsidian robot if possible
                        break;
                    }
                }
            }

            /*
            let opts = 2_i32.pow(4); // 4 options: build geode, obsidian, clay, ore robot, or nothing
            for i_opt in 1..opts { // Skip 0 option (do nothing)
                let mut total_cost = Cost::new();
                let mut new_head = new_state.clone();
                let mut potential = true;
                if i_opt & 0b1000 != 0 {
                    // Build geode robot
                    total_cost = total_cost.sum(self.geode);
                    new_head.geode_robots += 1;
                    potential = false;
                }
                if i_opt & 0b0100 != 0 {
                    if new_head.time >= 22 {
                        // No point building obsidian robots at the end
                        continue;
                    }
                    // Build obsidian robot
                    total_cost = total_cost.sum(self.obsidian);
                    new_head.obsidian_robots += 1;
                    potential = false;
                }
                if i_opt & 0b0010 != 0 {
                    if new_head.time >= 21 {
                        // No point building clay robots at the end
                        continue;
                    }
                    // Build clay robot
                    if new_head.clay_robots > 6 {
                        continue;
                    }
                    total_cost = total_cost.sum(self.clay);
                    new_head.clay_robots += 1;
                }
                if i_opt & 0b0001 != 0 {
                    if new_head.time >= 20 {
                        // No point building ore robots at the end
                        continue;
                    }
                    if new_head.ore_robots >= 3 {
                        // No point building more ore robots than max cost
                        continue;
                    }
                    // Build ore robot
                    total_cost = total_cost.sum(self.ore);
                    new_head.ore_robots += 1;
                }
                if !state.can_afford(&total_cost) {
                    //println!("Cannot afford option {:04b} at time {}", i_opt, new_state.time);
                    continue;
                }
                //println!("Adding head for option {:04b} at time {}", i_opt, new_state.time);
                new_head.ore -= total_cost.ore;
                new_head.clay -= total_cost.clay;
                new_head.obsidian -= total_cost.obsidian;
                do_nothing = do_nothing && potential;
                //dbg!(&new_head);
                let val = self.get_value(&new_head, false);
                if val > 0 {
                    queue.push(new_head, val);
                }
            }
            // Also consider the option of building nothing
            // This only makes sense if we can't build obsidian or geode robots
            if do_nothing {
                //println!("Adding do-nothing head at time {}", new_state.time);
                new_state.history.push(Action::Wait);
                let val = self.get_value(&new_state, false);
                if val > 0 {
                    queue.push(new_state, val);
                }
            }
            */
        }
        max_geodes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    time: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    //history: BTreeMap<i32, Action>, 
    history: Vec<Action>, 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum Action {
    BuildGeode,
    BuildObsidian,
    BuildClay,
    BuildOre,
    Wait,
}

impl State {
    fn can_afford(&self, cost: &Cost) -> bool {
        self.ore >= cost.ore &&
        self.clay >= cost.clay &&
        self.obsidian >= cost.obsidian
    }

    fn forward(&mut self) {
        self.time += 1;
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }

    fn build(&mut self, cost: Cost) {
        //dbg!(&cost);
        assert!(self.can_afford(&cost));
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
    }

    fn get_prio(&self) -> i64 {
        let t_rem = 24 -self.time;
        let base = 4_i32;
        (self.geode * base.pow(8) +
        t_rem * base.pow(7) +
        self.geode_robots * base.pow(6) +
        self.obsidian * base.pow(5) + 
        self.obsidian_robots * base.pow(4) +
        self.clay * base.pow(3) + 
        self.clay_robots * base.pow(2) + 
        //self.obsidian * self.ore * base.pow(3) +
        self.ore * base.pow(1) +
        self.ore_robots).into()

    }

    fn new() -> Self {
        Self {
            time: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1, // We start with one ore robot
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            //history: BTreeMap::new(),
            history: Vec::new(),
        }

    }
}


fn read_contents(cont: &str) -> (i64, i64) {
    let blueprints = get_blueprints(cont);
    dbg!(&blueprints);

    let part1 = get_part1(&blueprints);
    let part2 = get_part2(&blueprints);
    (part1, part2)
}

fn get_part1(blueprints: &Vec<Blueprint>) -> i64 {
    blueprints.iter().map(|bp| {
        bp.quality_level()
    }).sum()
}

fn get_part2(blueprints: &Vec<Blueprint>) -> i64 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {

        let a ="Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

        let blueprints = get_blueprints(a);
        dbg!(&blueprints);

        let optimal2 = vec![
            // Optimal strategy for the default robot
            Action::Wait,
            Action::Wait,
            Action::BuildOre,
            Action::Wait,
            Action::BuildOre,
            Action::BuildClay,
            Action::BuildClay,
            Action::BuildClay,
            Action::BuildClay,
            Action::BuildClay,
            Action::BuildObsidian,
            Action::BuildClay,
            Action::BuildObsidian,
            Action::BuildObsidian,
            Action::BuildObsidian,
            Action::Wait,
            Action::BuildObsidian,
            Action::BuildGeode,
            Action::BuildObsidian,
            Action::BuildGeode,
            Action::BuildObsidian,
            Action::BuildGeode,
            Action::BuildObsidian,
            Action::Wait,
        ];


        assert_eq!(blueprints[1].get_geodes(Some(optimal2)), 12);
        assert_eq!(blueprints[0].get_geodes(None), 9);

        assert_eq!(read_contents(&a).0, 33);
    }

    #[test]
    fn ex2() {

        // This is the optimal strategy for example 2 (One of the optimals)
        let bp = Blueprint::new("Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
        let mut state = State::new();

        // Optimal strategy for blueprint 2:
        // Minute 1:
        state.forward();
        assert!(bp.get_value(&state, false) > 0);
        // Minute 2:
        state.forward();
        assert!(bp.get_value(&state, false) > 0);
        // Minute 3:
        // Build ore robot
        state.build(bp.ore);
        state.forward();
        state.ore_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 4:
        state.forward();
        assert!(bp.get_value(&state, false) > 0);
        // Minute 5:
        // Build ore robot
        state.build(bp.ore);
        state.forward();
        state.ore_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 6:
        // Build clay robot
        dbg!(&state);
        state.build(bp.clay);
        state.forward();
        state.clay_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 7:
        // Build clay robot
        state.build(bp.clay);
        state.forward();
        state.clay_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 8:
        // Build clay robot
        state.build(bp.clay);
        state.forward();
        state.clay_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 9:
        // Build clay robot
        state.build(bp.clay);
        state.forward();
        state.clay_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 10:
        // Build clay robot
        state.build(bp.clay);
        state.forward();
        state.clay_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 11:
        // Build obsidian robot
        state.build(bp.obsidian);
        state.forward();
        state.obsidian_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 12:
        // Build clay robot
        state.build(bp.clay);
        state.forward();
        state.clay_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 13:
        // Build obsidian robot
        state.build(bp.obsidian);
        state.forward();
        state.obsidian_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 14:
        // Build obsidian robot
        state.build(bp.obsidian);
        state.forward();
        state.obsidian_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 15:
        // Build obsidian robot
        state.build(bp.obsidian);
        state.forward();
        state.obsidian_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 16:
        // Wait 
        state.forward();
        assert!(bp.get_value(&state, false) > 0);
        // Minute 17:
        // Build obsidian robot
        state.build(bp.obsidian);
        state.forward();
        state.obsidian_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 18:
        // Build geode robot
        state.build(bp.geode);
        state.forward();
        state.geode_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 19:
        // Build Obsidian robot
        state.build(bp.obsidian);
        state.forward();
        state.obsidian_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 20:
        // Build geode robot
        state.build(bp.geode);
        state.forward();
        state.geode_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 21:
        // Build Obsidian robot
        state.build(bp.obsidian);
        state.forward();
        state.obsidian_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 22:
        // Build Geode robot
        state.build(bp.geode);
        state.forward();
        state.geode_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 23:
        // Build Obsidian robot
        state.build(bp.obsidian);
        state.forward();
        state.obsidian_robots += 1;
        assert!(bp.get_value(&state, false) > 0);
        // Minute 24:
        // Just wait
        state.forward();
        dbg!(&state);
        dbg!(bp.get_value(&state, true));
        assert_eq!(state.time, 24);
        assert_eq!(state.geode, 12);
    }
}



