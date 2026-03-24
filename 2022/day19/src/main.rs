use clap::Parser;
use std::fs;
use regex::Regex;
use priority_queue::PriorityQueue;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use colored::Colorize;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}


fn main() {
    let args = Args::parse();
    let start = Instant::now();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);  
    assert_eq!(res.0, 1356);
    println!("Part 2 answer is {}", res.1);  
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}


fn get_blueprints(cont: &str) -> Vec<Blueprint> {
    cont.lines().map(|ln| {
        Blueprint::new(ln)
    }).collect()
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Cost {
    ore: i32,
    clay: i32,
    obsidian: i32,
}

impl Cost {
    fn new() -> Self {
        Self {ore: 0, clay: 0, obsidian: 0}
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum Action {
    BuildGeode,
    BuildObsidian,
    BuildClay,
    BuildOre,
    Wait,
}

const INPUT_GEODES: [(i32, i32); 30] = [
    (1,3),
    (2,13),
    (3,0),
    (4,0),
    (5,0),
    (6,10),
    (7,0),
    (8,7),
    (9,0),
    (10,2),
    (11,0),
    (12,0),
    (13,1),
    (14,12),
    (15,1),
    (16,0),
    (17,0),
    (18,0),
    (19,0),
    (20,0),
    (21,7),
    (22,1),
    (23,0),
    (24,15),
    (25,1),
    (26,3),
    (27,8),
    (28,1),
    (29,1),
    (30,3),
];


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
        // Quality level is needed for part 1 answer
        // Quality level is the product of geode count blueprint ID
        let geodes = self.get_geodes(PART1_CONFIG, None);
        println!("Blueprint {}: Geodes: {}, Quality level: {}", self.id, geodes, self.id * geodes as i64);
        assert_eq!(self.id as i32, INPUT_GEODES[self.id as usize - 1].0);
        assert_eq!(geodes, INPUT_GEODES[self.id as usize - 1].1);
        self.id * geodes as i64
    }

    fn get_value(&self, max_time: i32, state: &State, debug: bool) -> f64 {
        let remaining_time = (max_time - state.time) as f64;
        let geode_value = 1.0;
        let geode_robot_value = remaining_time * geode_value;

        // Obsidian
        let obsidian_turns_needed = (self.geode.obsidian - state.obsidian) as f64 / state.obsidian_robots.max(1) as f64;
        let obsidian_turns_needed_plus1 = (self.geode.obsidian - state.obsidian) as f64 / (state.obsidian_robots.max(1) + 1) as f64;
        let obsidian_turns_needed_plus2 = (self.geode.obsidian - state.obsidian) as f64 / (state.obsidian_robots.max(1) + 2) as f64;
        if debug {
            dbg!(&self.geode.obsidian);
            dbg!(&obsidian_turns_needed);
        }
        let mut obsidian_robot_value = 0_f64.max(remaining_time - obsidian_turns_needed - 1.0) * 1.0;
        if obsidian_robot_value < 0.1 {
            obsidian_robot_value += 0_f64.max(remaining_time - obsidian_turns_needed_plus1 - state.clay_robots as f64 / self.obsidian.clay as f64) / 2.0; // What happens if we can build 2
        }
        if obsidian_robot_value < 0.1 {
            obsidian_robot_value += 0_f64.max(remaining_time - obsidian_turns_needed_plus2 - 2.0 * state.clay_robots as f64 / self.obsidian.clay as f64) / 3.0; // What happens if we could build 3
        }



        let clay_turns_needed = (self.obsidian.clay - state.clay) as f64 / state.clay_robots.max(1) as f64;
        let clay_robot_value = 0_f64.max(remaining_time - clay_turns_needed / 2.0 - self.geode.obsidian as f64 - 2.0) * 1.0;

        let ore_turns_needed = (self.clay.ore - state.ore) as f64 / state.ore_robots.max(1) as f64;
        let ore_robot_value = 0_f64.max(remaining_time - ore_turns_needed - self.obsidian.clay as f64 - self.geode.obsidian as f64 - 3.0) * 1.0;
        
        let clay_value = 0.0;
        let ore_value = 0.0;
        let obsidian_value = 0.0;

        if debug {
            println!("Remaining time:  {}
Unit Values:
Geode:     {:.02} * {} = {:.02}
Obsidian:  {:.02} * {} = {:.02}
Clay:      {:.02} * {} = {:.02}
Ore:       {:.02} * {} = {:.02}

Robot Values:
Geode:     {:.02} * {} = {:.02}
Obsidian:  {:.02} * {} = {:.02}
Clay:      {:.02} * {} = {:.02}
Ore:       {:.02} * {} = {:.02}",
                remaining_time,
                geode_value,
                state.geode,
                state.geode as f64 * geode_value,
                obsidian_value,
                state.obsidian,
                state.obsidian as f64 * obsidian_value,
                clay_value,
                state.clay,
                state.clay as f64 * clay_value,
                ore_value,
                state.ore,
                state.ore as f64 * ore_value,


                geode_robot_value,
                state.geode_robots,
                state.geode_robots as f64 * geode_robot_value,

                obsidian_robot_value,
                state.obsidian_robots,
                state.obsidian_robots as f64 * obsidian_robot_value,

                clay_robot_value,
                state.clay_robots,
                state.clay_robots as f64 * clay_robot_value,
                ore_robot_value,
                state.ore_robots,
                state.ore_robots as f64 * ore_robot_value
            );
        }

        (1.0e0 * (state.geode as f64 * geode_value +
        state.obsidian as f64 * obsidian_value +
        state.clay as f64 * clay_value +
        state.ore as f64 * ore_value +
        state.geode_robots as f64 * geode_robot_value +
        state.obsidian_robots as f64 * obsidian_robot_value +
        state.clay_robots as f64 * clay_robot_value +
        state.ore_robots as f64 * ore_robot_value)) as f64
    }

    fn get_value_old(&self, state: &State, debug: bool) -> f64 {
        // Calculate the expected value of current state
        //
        let remaining_time = (24 - state.time) as f64;
        let geode_value = 1.0;
        let geode_robot_value = remaining_time * geode_value;

        // Obsidian is used to make geode robots
        // Obsidian value needs to converted to geode robots. Thus -1.0
        let obsidian_value = if remaining_time > 1.0 + self.geode.obsidian as f64 {
            (remaining_time - 1.0 - self.geode.obsidian as f64) * geode_value / (self.geode.obsidian as f64)
        } else {
            0.0
        };
        let obsidian_robot_value = (remaining_time - 2.0 - self.geode.obsidian as f64) * obsidian_value;

        // Clay is used to make obsidian robots
        // Clay needs to first converted to obsidian robots, then to geode robots. Thus -4.0
        let clay_value = if remaining_time > 3.0 + self.obsidian.clay as f64 {
            obsidian_value * (remaining_time - 3.0 - self.obsidian.clay as f64) / self.obsidian.clay as f64
        } else {
            0.0
        };
        let clay_robot_value = 0_f64.max((remaining_time - 4.0 - self.obsidian.clay as f64) * clay_value);

        // Ore is used for everything. Thus value is a sum of all uses
        // First consider ore used to make geode robots. Through geode robots there is a lag of 2
        // steps before value is created

        // Only consider ore used to make clay robots
        let mut ore_value = if remaining_time > 5.0 + self.clay.ore as f64 {
            // Next consider ore used to make clay robots. Through clay robots there is a lag of 6
            clay_robot_value * (remaining_time - 5.0 - self.clay.ore as f64) / (self.clay.ore as f64)
        } else {
            0.0
        };

        if false {
            if remaining_time > 4.0 {
                // Next consider ore used to make obsidian robots. Through obsidian robots there is a
                // lag of 4 steps before value is created
                ore_value += obsidian_robot_value * (remaining_time - 4.0) / (self.obsidian.ore as f64);
            }
            // Finally ore into geode robots
            if remaining_time > 2.0 {
                ore_value += geode_robot_value * (remaining_time - 2.0) / (self.geode.ore as f64)
            }
        }


        //if remaining_time > 8.0 {
        //    // Finally consider ore used to make new ore robots. Through ore robots there is a lag of 8
        //    ore_value += ore_value * (remaining_time - 8.0) / (self.ore.ore as f64);
        //}
        //ore_value /= 4.0;
        assert!(ore_value >= 0.0);
        let ore_robot_value = 0_f64.max((remaining_time - 6.0 - self.clay.ore as f64) * ore_value);

        if debug {
            println!("Remaining time:  {}
Unit Values:
Geode:           {:.02}/{:.02}
Obsidian value:  {:.02}/{:.02}
Clay value:      {:.02}/{:.02}
Ore value:       {:.02}/{:.02}

State Values:
Geode:           {:.02}/{:.02}
Obsidian value:  {:.02}/{:.02}
Clay value:      {:.02}/{:.02}
Ore value:       {:.02}/{:.02}",
                remaining_time,
                geode_value,
                geode_robot_value,
                obsidian_value,
                obsidian_robot_value,
                clay_value,
                clay_robot_value,
                ore_value,
                ore_robot_value,
                state.geode as f64 * geode_value,
                state.geode_robots as f64 * geode_robot_value,
                state.obsidian as f64 * obsidian_value,
                state.obsidian_robots as f64 * obsidian_robot_value,
                state.clay as f64 * clay_value,
                state.clay_robots as f64 * clay_robot_value,
                state.ore as f64 * ore_value,
                state.ore_robots as f64 * ore_robot_value
            );
                    
        }

        //dbg!(&ore_robot_value);
        //dbg!(&clay_robot_value);
        //dbg!(&obsidian_robot_value);
        //dbg!(&geode_robot_value);

        (1.0e0 * (state.geode as f64 * geode_value +
        state.obsidian as f64 * obsidian_value +
        state.clay as f64 * clay_value +
        state.ore as f64 * ore_value +
        state.geode_robots as f64 * geode_robot_value +
        state.obsidian_robots as f64 * obsidian_robot_value +
        state.clay_robots as f64 * clay_robot_value +
        state.ore_robots as f64 * ore_robot_value)) as f64
    }

    fn run_manual(&self) -> i32 {
        let mut state = State::new();
        loop {
            state.print(Some(self));
            println!("Choose option:");
            if state.time == 24 {
                println!("Time limit reached. Final geode count: {}", state.geode);
                break state.geode;
            }
            for (i,action) in Action::iter().enumerate() {
                let out = match action {
                    Action::BuildGeode => {
                        format!("{}: Build Geode, ({} ore and {} obsidian)", i+1, self.geode.ore, self.geode.obsidian)
                    },
                    Action::BuildOre =>{
                        format!("{}: Build Ore, ({} ore)", i+1, self.ore.ore)
                    },
                    Action::BuildObsidian => {
                        format!("{}: Build Obsidian, ({} ore and {} clay)", i+1, self.obsidian.ore, self.obsidian.clay)
                    },
                    Action::BuildClay => {
                        format!("{}: Build Clay, ({} ore)", i+1, self.clay.ore)
                    }
                    Action::Wait => {
                        format!("{}: Wait", i+1)
                    }
                    _ => continue,
                };
                if state.can_afford_action(action, self) {
                    println!("{}", out.green());
                } else {
                    println!("{}", out.red());
                }
            }
            state.forward();
            let input = std::io::stdin().lines().next().unwrap().unwrap();
            //let action = options.iter().find(|opt| opt.0.to_string() == input).unwrap().1;
            match input.trim() {
                "1" => {
                    if state.can_afford_action(Action::BuildGeode, self) {
                        state.build(self.geode);
                        state.geode_robots += 1;
                    } else {
                        panic!("Cannot afford geode robot");
                    }
                },
                "2" => {
                    if state.can_afford_action(Action::BuildObsidian, self) {
                        state.build(self.obsidian);
                        state.obsidian_robots += 1;
                    } else {
                        panic!("Cannot afford obsidian robot");
                    }
                },
                "3" => {
                    if state.can_afford_action(Action::BuildClay, self) {
                        state.build(self.clay);
                        state.clay_robots += 1;
                    } else {
                        panic!("Cannot afford clay robot");
                    }
                },
                "4" => {
                    if state.can_afford_action(Action::BuildOre, self) {
                        state.build(self.ore);
                        state.ore_robots += 1;
                    } else {
                        panic!("Cannot afford ore robot");
                    }
                }
                _ => continue,
                                
            }
        }
    }

    fn get_geodes_heuristic(&self) -> i32 {
        let mut max_geodes = 0;
        for target_ore_robots in 1..=3 {
            for target_clay_robots in 1..=6 {
                let geodes = self.get_geodes_single(target_ore_robots, target_clay_robots, false);
                if geodes > max_geodes {
                    println!("Target ore robots: {}, Target clay robots: {}, Geodes: {}", target_ore_robots, target_clay_robots, geodes);
                    max_geodes = max_geodes.max(geodes);
                }
            }
        }
        max_geodes
    }

    fn get_geodes_single(&self, target_ore_robots: i32, target_clay_robots: i32, debug: bool) -> i32 {
        let mut state = State::new();
        for i in 0..24 {
            if debug {
                println!("\nMinute {}: ", i + 1);
            }
            let options = state.get_options(self, true);
            if options.len() > 1 {
                println!("Options: {:?}", options);
            }
            if state.can_afford_action(Action::BuildGeode, self) {
                if debug {println!("Building geode robot")};
                state.perform_action(Action::BuildGeode, self, true);
                continue;
            }
            if state.can_afford_action(Action::BuildObsidian, self) {
                if debug {println!("Building obsidian robot")};
                state.perform_action(Action::BuildObsidian, self, true);
                continue;
            }
            if state.ore_robots < target_ore_robots {
                if state.can_afford_action(Action::BuildOre, self){
                    if debug {println!("Building ore robot")};
                    state.perform_action(Action::BuildOre, self, true);
                    continue;
                }
            } else if state.can_afford_action(Action::BuildClay, self) && state.clay_robots < target_clay_robots {
                if debug {println!("Building clay robot")};
                state.perform_action(Action::BuildClay, self, true);
                continue;
            }
            if debug {
                println!("Waiting");
            }
            state.perform_action(Action::Wait, self, true);
        }
        state.geode
    }
    
    fn get_geodes_value_based(&self, optimal: Option<Vec<Action>>) -> i32 {
        let mut state_queue: Vec<(State, f64, Action, usize)> = Vec::new();
        let start_state = State::new();
        let val = self.get_value(24, &start_state, false);
        state_queue.push((start_state, val, Action::Wait, 0));
        const KEEP: usize = 10;
        for i in 1..=23 {
            println!("Minute {i}");
            let mut new_queue = Vec::new();
            let mut optimal_val = -10.0;
            let mut optimal_history = None;
            for (j, (state, val, prev_opt, j_prev)) in state_queue.iter().take(KEEP).enumerate() {
                let options: Vec<(Action, f64)> = state.get_options(self, false);
                if let Some(ref opt) = optimal {
                    let sub_target = opt[0..(i-1) as usize].to_vec();
                    assert_eq!(sub_target.len(), state.history.len());
                    if sub_target == state.history {
                        let optimal_option = opt[i as usize - 1];
                        println!("Optimal: {:?}", opt[i as usize - 1]);
                        //println!("Optimal sub target: {:?}", sub_target);
                        
                        let mut new_state = state.clone();
                        new_state.perform_action(optimal_option, self, true);
                        optimal_val = self.get_value(24, &new_state, true);
                        println!("Optimal option value: {:.02}", optimal_val);
                        optimal_history = Some(opt[0..i as usize].to_vec());
                    }
                }
                for opt in options {
                    if opt.1 < 0.0 {
                        continue; // skip negative values
                    }
                    let mut new_state = state.clone();
                    new_state.perform_action(opt.0, self, true);
                    new_queue.push((new_state, opt.1, opt.0, j));
                }
            }
            if optimal_val < -999.0 {
                panic!("Optimal value not found");
            } 
            new_queue.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            if let Some(opt) = optimal_history && KEEP < 100 {
                for (ii,(s, v, o, j)) in new_queue.iter().enumerate() {
                    assert_eq!(s.history.len(), opt.len());
                    if s.history == opt {
                        let out = format!("{}: {:.01} {:?} (optimal)", ii, v, s.history);
                        println!("{}", out.red().on_white());
                    } else if s.history[0..opt.len()-1] == opt[0..opt.len()-1] {
                        let out = format!("{}: {:.01} {:?} (was optimal)", ii, v, s.history);
                        println!("{}", out.black().on_white());
                    } else {
                        println!("{ii}: {:.01} {:?}", v, s.history);
                    }
                }
            }
            println!("Best option: {:?}", new_queue[0].2);
            new_queue[0].0.print(Some(self));
            //if _i > 5 {
            //    println!("New queue: {}", new_queue.len());
            //    for (s, v, o, j) in new_queue.iter() {
            //        println!("{:?}: {:.02} from {}", o, v, j);
            //    }
            //}
            println!();
            state_queue = new_queue;
        }
        state_queue[0].0.geode + state_queue[0].0.geode_robots
    }

    fn get_geodes(&self, config: Config, optimal: Option<Vec<Action>>) -> i32 {
        let mut queue: PriorityQueue<State, i64> = PriorityQueue::new();
        let start_state = State::new();
        //queue.push(State::new(), self.get_value(24, &start_state, false));
        queue.push(start_state, 0);

        let mut max_geodes: i32 = 0;
        let mut max_time: i32 = 0;

        let mut loop_count = 0;

        let mut best_state = State::new();

        loop {
            if queue.is_empty() {
                //println!();
                println!("Queue empty after {loop_count} loops");
                //println!("Max geodes: {max_geodes}");
                //println!("Best state:");
                //println!("Geode robots: {} (bought at {})", best_state.geode_robots, best_state.last_geode_bought);
                //println!("Obsidian robots: {} (bought at {})", best_state.obsidian_robots, best_state.last_obsidian_bought);
                //println!("Clay robots: {} (bought at {})", best_state.clay_robots, best_state.last_clay_bought);
                println!("Ore robots: {} (bought at {})", best_state.ore_robots, best_state.last_ore_bought);
                break;
            }
            //dbg!(queue.len());
            let (state, _prio) = queue.pop().unwrap();
            loop_count += 1;
            //assert_eq!(state.time as usize, state.history.len());
            let mut debug = false;
            if let Some(ref opt) = optimal {
                let sub_target = opt[0..state.time as usize].to_vec();
                if sub_target == state.history {
                    println!("Time: {}", state.time + 1);
                    debug = true;
                    println!("Optimal: {:?}", opt[state.time as usize]);
                }
            }; 
            //dbg!(&state);
            
            // 1 minute passed
            max_time = max_time.max(state.time);

            if state.time == config.max_time - 1 {
                // Reached time limit
                let mut new_head = state.clone();
                new_head.forward();
                if new_head.geode > max_geodes {
                    max_geodes = new_head.geode;
                    best_state = new_head.clone();
                                    
                }
                continue;
            }

            
            for action in Action::iter() {
                if debug {
                    dbg!(&action);
                }
                // It seems to be enough to just build one robot at a time
                let mut new_head = state.clone();
                new_head.forward();

                if !state.can_afford_action(action, self) {
                    if debug {
                        println!("Can't afford action {:?}", action);
                    }
                    continue;
                }
                match action {
                    Action::BuildGeode => {
                        if debug {
                            println!("Build geode robot");
                        }
                        new_head.build(self.geode); // subtracts cost
                        new_head.geode_robots += 1;
                    }
                    Action::BuildObsidian => {
                        if state.time > config.obsidian_stop_time {
                            continue;
                        }
                        new_head.build(self.obsidian); // subtracts cost
                        new_head.obsidian_robots += 1;
                    }
                    Action::BuildClay => {
                        if state.time > config.clay_stop_time {
                            continue;
                        }
                        if state.obsidian_robots > config.max_obsidian_robots_clay { // This needs to be at least 2
                            continue;
                        }
                        new_head.build(self.clay); // subtracts cost
                        new_head.clay_robots += 1;
                    }
                    Action::BuildOre => {
                        if state.time > config.ore_stop_time {
                            continue;
                        }
                        if state.clay_robots >  config.max_clay_robots_ore {
                            // No point building ore robots after starting to build clay ones
                            if debug {
                                println!("Already started building clay robots");
                            }
                            continue;
                        }
                        if state.ore_robots > config.max_ore_robots { 
                            continue;
                        }
                        new_head.build(self.ore); // subtracts cost
                        new_head.ore_robots += 1;
                    }
                    Action::Wait => {
                        // Do nothing
                    }
                }
                //new_head.perform_action(action, self, false);
                new_head.history.push(action);
                let val = self.get_value(24, &new_head, false);
                if true {
                    if debug {
                        dbg!(&action, &val);
                    }
                    let prio = new_head.get_prio();
                    queue.push(new_head, prio);
                }
            }
        }
        max_geodes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Config {
    max_time: i32,
    max_ore_robots: i32,
    max_obsidian_robots_clay: i32,
    max_clay_robots_ore: i32,
    ore_stop_time: i32,
    clay_stop_time: i32,
    obsidian_stop_time: i32,
}

const PART1_CONFIG: Config = Config {
    max_time: 24,
    max_ore_robots: 3, // Must be at least 3
    max_obsidian_robots_clay: 3, // Must be at least 2
    max_clay_robots_ore: 1, // Must be at least 1
    ore_stop_time: 9,
    clay_stop_time: 19,
    obsidian_stop_time: 20, // Must be at least
};

const PART2_CONFIG: Config = Config {
    max_time: 32,
    max_ore_robots: 5,

    max_obsidian_robots_clay: 5,
    max_clay_robots_ore: 5,

    ore_stop_time: 10,
    clay_stop_time: 15,
    obsidian_stop_time: 23,
};

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
    last_ore_bought: i32,
    last_clay_bought: i32,
    last_obsidian_bought: i32,
    last_geode_bought: i32,
    //history: BTreeMap<i32, Action>, 
    history: Vec<Action>, 
}

impl State {
    fn can_afford(&self, cost: &Cost) -> bool {
        self.ore >= cost.ore &&
        self.clay >= cost.clay &&
        self.obsidian >= cost.obsidian
    }

    fn can_afford_action(&self, action: Action, blueprint: &Blueprint) -> bool {
        match action {
            Action::BuildGeode => self.can_afford(&blueprint.geode),
            Action::BuildObsidian => self.can_afford(&blueprint.obsidian),
            Action::BuildClay => self.can_afford(&blueprint.clay),
            Action::BuildOre => self.can_afford(&blueprint.ore),
            Action::Wait => true,
        }
    }

    fn get_options(&self, blueprint: &Blueprint, debug: bool) -> Vec<(Action, f64)> {
        let mut options = Vec::new();
        for action in Action::iter() {
            if self.can_afford_action(action, blueprint) {
                let mut new_state = self.clone();
                new_state.perform_action(action, blueprint, false);
                options.push((action, blueprint.get_value(24, &new_state, debug)));
            }
        }
        options
    }

    fn print_options(&self, blueprint: &Blueprint) {
        println!("Options:");
        for action in Action::iter() {
            if self.can_afford_action(action, blueprint) {
                println!("{:?}", action);
            }
        }
    }

    fn perform_action(&mut self, action: Action, blueprint: &Blueprint, store_history: bool) {
        assert!(self.can_afford_action(action, blueprint));
        self.forward();
        match action {
            Action::BuildGeode => {
                self.build(blueprint.geode);
                self.geode_robots += 1;
                self.last_geode_bought = self.time;
            },
            Action::BuildObsidian => {
                self.build(blueprint.obsidian);
                self.obsidian_robots += 1;
                self.last_obsidian_bought = self.time;
            },
            Action::BuildClay => {
                self.build(blueprint.clay);
                self.clay_robots += 1;
                self.last_clay_bought = self.time;
            },
            Action::BuildOre => {
                self.build(blueprint.ore);
                self.ore_robots += 1;
                self.last_ore_bought = self.time;
            },
            Action::Wait => {
                // Do nothing
            }
        }
        if store_history {
            self.history.push(action);
        }
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

    fn print(&self, blueprint: Option<&Blueprint>) {
        let value = if let Some(bp) = blueprint {
            bp.get_value(24, self, false)
        } else {
            0.0
        };
        println!("Time:            {}\nOre:             {}\nClay:            {}\nObsidian:        {}\nGeodes:          {}\nOre robots:      {}\nClay robots:     {}\nObsidian robots: {}\nGeode robots:    {}\nvalue:           {:.02}",
            self.time, self.ore, self.clay, self.obsidian, self.geode, self.ore_robots, self.clay_robots, self.obsidian_robots, self.geode_robots, value);
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
            last_ore_bought: 0,
            last_clay_bought: 0,
            last_obsidian_bought: 0,
            last_geode_bought: 0,
            //history: BTreeMap::new(),
            history: Vec::new(),
        }

    }
}


fn read_contents(cont: &str) -> (i64, i64) {
    let blueprints = get_blueprints(cont);
    //dbg!(&blueprints);

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
    blueprints.iter().take(3).map(|bp| {
        let geodes = bp.get_geodes(PART2_CONFIG, None) as i64;
        println!("Blueprint {}: Geodes: {}, Product: {}", bp.id, geodes, geodes * bp.id);
        geodes * bp.id
    }).product()
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

        let optimal1 = [
            Action::Wait, // 1
            Action::Wait, // 2
            Action::BuildClay, // 3
            Action::Wait, // 4
            Action::BuildClay, // 5
            Action::Wait, // 6
            Action::BuildClay, // 7
            Action::Wait, // 8
            Action::Wait, // 9
            Action::Wait, // 10
            Action::BuildObsidian, // 11
            Action::BuildClay, // 12
            Action::Wait, // 13
            Action::Wait, // 14
            Action::BuildObsidian, // 15
            Action::Wait, // 16
            Action::Wait, // 17
            Action::BuildGeode, // 18
            Action::Wait, // 19
            Action::Wait, // 20
            Action::BuildGeode, // 21
            Action::Wait, // 22
            Action::Wait, // 23
            Action::Wait, // 24
        ];

        assert_eq!(blueprints[1].get_geodes_value_based(Some(optimal2.clone())), 12);

        //assert_eq!(blueprints[0].get_geodes_single(1, 4, true), 9);
        assert_eq!(blueprints[1].get_geodes(PART1_CONFIG, Some(optimal2)), 12);
        assert_eq!(blueprints[0].get_geodes(PART1_CONFIG, None), 9);

        //assert_eq!(blueprints[0].get_geodes_heuristic(), 9);

        assert_eq!(read_contents(&a).0, 33);
    }

    #[test]
    fn part2() {
        let a ="Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

        let blueprints = get_blueprints(a);

        let optimal1 = vec![
            Action::Wait, // 1
            Action::Wait, // 2
            Action::Wait, // 3
            Action::Wait, // 4
            Action::BuildOre, // 5
            Action::Wait, // 6
            Action::BuildClay, // 7
            Action::BuildClay, // 8
            Action::BuildClay, // 9
            Action::BuildClay, // 10
            Action::BuildClay, // 11
            Action::BuildClay, // 12
            Action::BuildClay, // 13
            Action::BuildObsidian, // 14
            Action::Wait, // 15
            Action::BuildObsidian, // 16
            Action::BuildObsidian, // 17
            Action::Wait, // 18
            Action::BuildObsidian, // 19 
            Action::BuildGeode, // 20
            Action::BuildObsidian, // 21
            Action::BuildGeode, // 22
            Action::BuildGeode, // 23
            Action::BuildGeode, // 24
            Action::Wait, // 25
            Action::BuildGeode, // 26
            Action::BuildGeode, // 27
            Action::Wait, // 28
            Action::BuildGeode, // 29
            Action::BuildGeode, // 30
            Action::BuildGeode, // 31
            Action::Wait, // 32
        ];
        //assert_eq!(blueprints[0].get_geodes(PART2_CONFIG, Some(optimal1)), 56);
        assert_eq!(blueprints[0].get_geodes(PART2_CONFIG, None), 56);
        //assert_eq!(blueprints[1].get_geodes(PART2_CONFIG, None), 62);
    }

    #[test]
    fn input1() {
        // Blueprint 1 from inputs
        let a = "Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 9 obsidian.";
        let bp = Blueprint::new(a);
        assert_eq!(bp.ore, Cost { ore: 3, clay: 0, obsidian: 0 });
        //bp.run_manual();
        //assert_eq!(bp.get_geodes_heuristic(), 3);
        assert_eq!(bp.get_geodes(PART1_CONFIG, None), 3);
    }

    #[test]
    fn ex2() {

        // This is the optimal strategy for example 2 (One of the optimals)
        let bp = Blueprint::new("Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
        let mut state = State::new();

        let optimal = vec![
            // Optimal strategy for the default robot
            // Minute 1:
            Action::Wait,
            // Minute 2:
            Action::Wait,
            // Minute 3:
            // Build ore robot
            Action::BuildOre,
            // Minute 4:
            Action::Wait,
            // Minute 5:
            Action::BuildOre,
            // Minute 6:
            Action::BuildClay,
            // Minute 7:
            Action::BuildClay,
            // Minute 8:
            Action::BuildClay,
            // Minute 9:
            // Build clay robot
            Action::BuildClay,
            // Minute 10:
            // Build clay robot
            Action::BuildClay,
            // Minute 11:
            // Build obsidian robot
            Action::BuildObsidian,
            // Minute 12:
            // Build clay robot
            Action::BuildClay,
            // Minute 13:
            // Build obsidian robot
            Action::BuildObsidian,
            // Minute 14:
            // Build obsidian robot
            Action::BuildObsidian,
            // Minute 15:
            // Build obsidian robot
            Action::BuildObsidian,
            // Minute 16:
            // Wait 
            Action::Wait,
            // Minute 17:
            // Build obsidian robot
            Action::BuildObsidian,
            // Minute 18:
            // Build geode robot
            Action::BuildGeode,
            // Minute 19:
            // Build Obsidian robot
            Action::BuildObsidian,
            // Minute 20:
            // Build geode robot
            Action::BuildGeode,
            // Minute 21:
            // Build Obsidian robot
            Action::BuildObsidian,
            // Minute 22:
            // Build Geode robot
            Action::BuildGeode,
            // Minute 23:
            // Build Obsidian robot
            Action::BuildObsidian,
            // Minute 24:
            // Just wait
            Action::Wait,
        ];

        for action in optimal {
            println!("\n\n");
            println!("Performed action: {:?}", action);
            state.perform_action(action, &bp, false);
            state.print(Some(&bp));
            println!();
            assert!(bp.get_value(24, &state, true) > 0.0);
        }
        //dbg!(&state.history);
        assert_eq!(state.time, 24);
        assert_eq!(state.geode, 12);
    }
}



