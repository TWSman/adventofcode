use clap::Parser;
use std::fs;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use regex::Regex;
use priority_queue::PriorityQueue;

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
    // 2419 is too low
}

fn read_contents(cont: &str) -> (i64, i64) {
    let valves = get_valves(cont);
    let part1 = get_part1(&valves);
    //let part2 = get_part2(&valves);
    let part2 = 0;
    (part1,part2)
}

fn get_valves(cont: &str) -> BTreeMap<String, Valve> {
    let mut valves: BTreeMap<String, Valve> = cont.lines().map(Valve::new).map(|v| {
        (v.id.clone(), v)
    }).collect();
    let valves_filter: BTreeSet<String> = valves.iter().filter_map(|(i, v)| {
        if v.flow_rate > 0 {
            Some(i.to_string())
        } else {
            None
        }
    }).collect();
    let conn_a = get_connections("AA", &valves);
    valves.get_mut("AA").unwrap().connections = conn_a;
    for iv in &valves_filter {
        let conn = get_connections(iv, &valves);
        let valve = valves.get_mut(iv).unwrap();
        valve.connections = conn;
    }
    valves
}

fn get_connections(start: &str, valves: &BTreeMap<String, Valve>) -> Vec<(String, i64, i64)> {
    let mut already_visited: BTreeSet<String> = BTreeSet::new();
    let max_open = valves.iter().filter(|(_i,v)| v.flow_rate > 0).count();
    let mut queue: PriorityQueue<(String, Vec<String>), i64> = PriorityQueue::new();
    queue.push((start.to_string(), vec![start.to_string()]), 0);
    let mut new_vec: Vec<(String, i64, i64)> = Vec::new();
    loop {
        if new_vec.len() == max_open {
            break;
        }
        if queue.is_empty() {
            break;
        }
        let ((loc, hist), prio) = queue.pop().unwrap();
        let dist = prio * -1;
        let valve = valves.get(&loc).unwrap();
        if valve.flow_rate > 0 && loc != start {
            new_vec.push((loc, valve.flow_rate, dist));
        }
        for l in &valve.options {
            if l == "Open" {
                continue;
            }
            if already_visited.contains(l) {
                continue;
            }
            let mut new_hist = hist.clone();
            new_hist.push(l.to_string());
            
            already_visited.insert(l.to_string());
            let new_dist = dist + 1;
            queue.push((l.to_string(), new_hist), -1 * new_dist);
        }
    }
    new_vec.sort_by_key(|v| v.1);
    new_vec.reverse();
    if start != "AA" {
        assert_eq!(new_vec.len(), max_open - 1);
    }
    new_vec
}

fn get_part1(valves: &BTreeMap<String, Valve>) -> i64 {
    let start = String::from("AA");
    // time, total flow, location, opened valves
    let mut q: PriorityQueue<(i64, i64, String, Vec<String>), i64> = PriorityQueue::new();
    // Remaining time x potential
    let prio = 30 * 100;
    q.push((0, 0, start, Vec::new()), prio);
    let mut max_pressure = 0;
    let max_open = valves.iter().filter(|(_i,v)| v.flow_rate > 0).count();

    loop {
        if q.is_empty() {
            break;
        }
        let ((time, pressure, loc, open), _prio) = q.pop().unwrap();
        if pressure > max_pressure {
            max_pressure = pressure;
        }
        if open.len() == max_open {
            break;
        }
        for (t, flow_rate, distance) in &valves.get(&loc).unwrap().connections {
            if open.contains(&t) {
                //println!("{t} already opened");
                continue;
            }
            let mut new_open = open.clone();
            new_open.push(t.to_string());
            // 'distance' minutes to move to new valve
            // plus 1 minute to open that valve
            let new_time = time + 1 + distance;
            if new_time > 30 {
                // No time to open valve
                continue;
            }
            let remaining_time = 30 - new_time;
            let added_pressure = remaining_time * flow_rate;
            let new_pressure = pressure + added_pressure;
            //if new_pressure > max_pressure {
            //    max_pressure = new_pressure;
            //}
            let prio = remaining_time * 100 + new_pressure;
            q.push((new_time, new_pressure, t.to_string(), new_open), prio);
        }
    }
    max_pressure
}


fn get_part1_old(valves: &BTreeMap<String, Valve>) -> i64 {
    let start = String::from("AA");
    // time, total flow, location, opened valves
    let mut q: PriorityQueue<(i64, i64, String, Vec<String>), i64> = PriorityQueue::new();
    let prio = 30 * 100;
    let mut opensets: BTreeMap<String, (i64, i64)> = BTreeMap::new();
    q.push((0, 0, start, Vec::new()), prio);
    let mut max_pressure = 0;
    let max_open = valves.iter().filter(|(_i,v)| v.flow_rate > 0).count();
    dbg!(&max_open);
    loop {
        if q.is_empty() {
            break;
        }
        let ((time, pressure, loc, open), _prio) = q.pop().unwrap();
        if pressure > max_pressure {
            if pressure == 1716 {
                dbg!(&open);
                dbg!(&time);
                dbg!(&pressure);
            }
            max_pressure = pressure;
        }
        if time == 30 {
            // No more time left
            break;
        }
        if open.len() == max_open {
            continue;
        }
        let time_remaining = 29 - time;
        let new_time = time + 1;

        // Options are:
        // Open current valve
        // Move to one of the tunnels
        for option in &valves.get(&loc).unwrap().options {
            if option == "Open" && !open.contains(&loc) {
                let flow_rate = valves.get(&loc).unwrap().flow_rate;
                if flow_rate > 0 {
                    let mut o = open.clone();
                    o.sort();
                    let summary: String = o.iter().fold(String::from(""), |acc ,x| acc.to_string() + x);
                    let added_pressure = time_remaining * flow_rate;
                    let new_pressure = pressure + added_pressure;
                    if opensets.contains_key(&summary) {
                        // This set of open valves has already been added, but with a
                        // better configuration
                        let a = opensets.get(&summary).unwrap();
                        if a.0 > new_pressure && a.1 > time_remaining {
                            continue;
                        }
                    }
                    opensets.insert(summary, (new_pressure, time_remaining));
                    let mut new_open = open.clone();
                    new_open.push(loc.clone());
                    let prio = (time_remaining - 1) * 100 + new_pressure; // Prioritize highest possible pressure
                    q.push((new_time, new_pressure, loc.clone(), new_open), prio);
                }
            } else if option != "Open" {
                let new_loc = option.to_string();
                // Just move
                let prio = (time_remaining - 1) * 100 + pressure;
                q.push((new_time, pressure, new_loc, open.clone()), prio);
            }
        }
    }
    max_pressure
}

fn get_part2(valves: &BTreeMap<String, Valve>) -> i64 {
    let start = String::from("AA");
    // time, total flow, mylocation, elephant location, opened valves
    let mut q: PriorityQueue<(i64, i64, String, String, Vec<String>), i64> = PriorityQueue::new();
    let prio = 26 * 100;
    let mut opensets: BTreeMap<String, (i64, i64)> = BTreeMap::new();
    q.push((0, 0, start.clone(), start, Vec::new()), prio);
    let mut max_pressure = 0;
    let max_open = valves.iter().filter(|(_i,v)| v.flow_rate > 0).count();
    dbg!(&max_open);
    loop {
        if q.is_empty() {
            break;
        }
        let ((time, pressure, my_loc, e_loc, open), _prio) = q.pop().unwrap();
        if pressure > max_pressure {
            max_pressure = pressure;
            dbg!(&my_loc);
            dbg!(&e_loc);
            dbg!(&open);
            dbg!(&time);
            dbg!(&max_pressure);
        }
        if time == 26 {
            // No more time left
            break;
        }
        if open.len() == max_open {
            continue;
        }
        let time_remaining = 25 - time;
        let new_time = time + 1;

        // Options are:
        // Open current valve
        // Move to one of the tunnels
        for my_option in &valves.get(&my_loc).unwrap().options {
            for e_option in &valves.get(&e_loc).unwrap().options {
                if my_option == "Open" && e_option == "Open" && my_loc == e_loc {
                    // No point in both opening the same valve
                    continue;
                }
                let mut new_open = open.clone();
                let mut new_pressure = pressure;
                let mut my_new_loc = my_loc.clone();
                let mut e_new_loc = e_loc.clone();
                if my_option == "Open" && !open.contains(&my_loc) {
                    let flow_rate = valves.get(&my_loc).unwrap().flow_rate;
                    if flow_rate > 0 {
                        let added_pressure = time_remaining * flow_rate;
                        new_pressure += added_pressure;
                        new_open.push(my_loc.clone());
                    }
                } else if my_option != "Open" {
                    // Just move
                    my_new_loc =  my_option.to_string();
                } else {
                    // My option is open but this valve is already open
                    continue;
                }
                if e_option == "Open" && !open.contains(&e_loc) {
                    let flow_rate = valves.get(&e_loc).unwrap().flow_rate;
                    if flow_rate > 0 {
                        let added_pressure = time_remaining * flow_rate;
                        new_pressure += added_pressure;
                        new_open.push(e_loc.clone());
                    }
                } else if e_option != "Open" {
                    // Just move
                    e_new_loc =  e_option.to_string();
                } else {
                    // Elephant option is open but this valve is already open
                    continue;
                }

                //if new_open.len() > open.len() {
                //    // At least one new opened valve
                //    let mut o = new_open.clone();
                //    o.sort();
                //    let summary: String = o.iter().fold(String::from(""), |acc ,x| acc.to_string() + x);
                //    if opensets.contains_key(&summary) {
                //        // This set of open valves has already been added, but with a
                //        // better configuration
                //        let a = opensets.get(&summary).unwrap();
                //        if a.0 > new_pressure && a.1 > time_remaining {
                //            continue;
                //        }
                //    }
                //    opensets.insert(summary, (new_pressure, time_remaining));
                //}


                let prio = (time_remaining - 1) * 100 + new_pressure;
                q.push((new_time, new_pressure, my_new_loc, e_new_loc, new_open), prio);
            }
        }
    }
    max_pressure
}




#[derive(Debug, Clone)]
struct Valve {
    id: String,
    options: Vec<String>,
    connections: Vec<(String, i64, i64)>, //id, flow rate, distance
    flow_rate: i64,
}
 
impl Valve {
    fn new(ln: &str) -> Self {
        let re = Regex::new(r"Valve ([A-Z]*) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)").unwrap();
        let res = re.captures(ln).unwrap();
        let id = res[1].to_string();
        let flow_rate = res[2].parse::<i64>().unwrap();
        let mut options = res[3].split(',').map(|a| a.trim().to_string()).collect::<Vec<String>>();
        if flow_rate > 0 {
            options.push(String::from("Open"));
        }
        Self {id, options, flow_rate, connections: Vec::new()}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a ="Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        let valves = get_valves(&a);
        assert_eq!(get_part1(&valves), 1651);
        assert_eq!(get_part1_old(&valves), 1651);
    }

    #[test]
    fn part2() {
        let a ="Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        let valves = get_valves(&a);
        assert_eq!(get_part2(&valves), 1707);
    }


}
