use clap::Parser;
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
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i32, i32) {
    let programs = parse_programs(cont);

    let part1 = get_part1(&programs);
    let part2 = get_part2(&programs);
    (part1, part2)
}

fn get_visited(programs: &BTreeMap<usize, Program>, start: usize) -> BTreeSet<usize> {
    // Get all programs that are connected to the start
    let mut visited: BTreeSet<usize> = BTreeSet::new();
    let mut queue: Vec<usize> = Vec::new();
    let mut program: usize = start;
    queue.push(program);
    visited.insert(program);
    loop {
        if queue.is_empty() {
            break;
        }
        program = queue.pop().unwrap();

        for connection in &programs.get(&program).unwrap().pipes {
            if visited.contains(connection) {
                continue;
            }
            queue.push(*connection);
            visited.insert(*connection);
        }
    }
    visited
}

fn get_part1(programs: &BTreeMap<usize, Program>) -> i32 {
    let visited = get_visited(programs, 0);
    visited.len() as i32
}

fn get_part2(programs: &BTreeMap<usize, Program>) -> i32 {
    let mut total_visited = BTreeSet::new();
    let mut group_count = 0;
    let keys = programs.keys().cloned().collect::<Vec<_>>();
    for program in programs.values() {
        if program.pipes.is_empty() {
            total_visited.insert(program.id);
            group_count += 1;
        }
    }
    for n in &keys {
        if total_visited.contains(n) {
            continue;
        }
        let visited = get_visited(programs, *n);
        total_visited.extend(visited);
        group_count += 1;
    }
    group_count
}

#[derive(Debug, Clone)]
struct Program {
    id: usize,
    pipes: Vec<usize>,
}

fn parse_programs(cont: &str) -> BTreeMap<usize, Program> {
    let mut programs: BTreeMap<usize, Program> = BTreeMap::new();
    for line in cont.lines() {
        let splits = line.split_whitespace().collect::<Vec<_>>();
        let mut pipes = Vec::new();
        let id = splits[0].parse::<usize>().unwrap();
        for spl in splits.iter().skip(2) {
            let pipe = spl.replace(",", "").parse::<usize>().unwrap();
            if pipe == id {
                // No point adding self connections
                continue;
            }
            if programs.contains_key(&pipe) {
                assert!(programs.get(&pipe).unwrap().pipes.contains(&id));
            }
            pipes.push(pipe);
        }
        programs.insert(id, Program { id, pipes });
    }
    programs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        // All but one programs are connected to 0
        assert_eq!(read_contents(&a).0, 6);
    }

    #[test]
    fn part2() {
        let a = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        // The example contains 2 groups
        assert_eq!(read_contents(&a).1, 2);
    }
}
