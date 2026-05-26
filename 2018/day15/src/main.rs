use clap::Parser;
use colored::Colorize;
use priority_queue::PriorityQueue;
use shared::Dir;
use shared::Vec2D;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

#[allow(dead_code)]
fn wait_for_enter() {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("\n########################");
    println!("Part 1 answer is {:?}", res.0);
    println!("Part 2 answer is {:?}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> (i32, i32) {
    let grid = read_grid(cont);
    grid.print_grid();
    let part1 = get_part1(&mut grid.clone(), 0);
    let part2 = get_part2(&grid);
    (part1, part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Object {
    Empty,
    Wall,
    Elf,
    Goblin,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec2D, Object>,
    actors: Vec<Actor>,
}

impl Grid {
    fn print_grid(&self) {
        let (min_x, max_x) = (
            self.grid.keys().map(|v| v.x).min().unwrap(),
            self.grid.keys().map(|v| v.x).max().unwrap(),
        );

        let (min_y, max_y) = (
            self.grid.keys().map(|v| v.y).min().unwrap(),
            self.grid.keys().map(|v| v.y).max().unwrap(),
        );

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let loc = Vec2D { x, y };
                match self.grid.get(&loc) {
                    Some(Object::Wall) => {
                        print!("{}", "#".blue().on_black());
                    }
                    Some(Object::Empty) => {
                        print!("{}", ".".white().on_black());
                    }
                    Some(Object::Elf) => {
                        print!("{}", "E".green().on_black());
                    }
                    Some(Object::Goblin) => {
                        print!("{}", "G".red().on_black());
                    }
                    None => {
                        print!("{}", ".".white().on_black());
                    }
                }
            }
            println!();
        }
    }

    fn evolve(&mut self, verbose: usize) -> bool {
        self.actors
            .sort_by(|a, b| b.loc.y.cmp(&a.loc.y).then(a.loc.x.cmp(&b.loc.x)));
        let n = self.actors.len();
        for i1 in 0..n {
            let mut no_targets = true;
            for i2 in 0..n {
                if i1 == i2 {
                    continue;
                }
                if self.actors[i2].hp <= 0 {
                    continue;
                }
                if self.actors[i1].creature != self.actors[i2].creature {
                    no_targets = false;
                }
            }
            let actor = self.actors.get_mut(i1).unwrap();
            if verbose > 0 {
                println!("\nChecking {:?} at {:?}", actor.creature, actor.loc);
            }
            if no_targets {
                println!("    No targets found");
                self.actors.retain(|act| act.hp > 0);
                return false;
            }
            if actor.hp <= 0 {
                continue;
            }
            let creature1 = &actor.creature.clone();
            let loc1 = &actor.loc.clone();
            let attack = actor.attack;
            let target = Self::find_target(&self.grid, actor.loc, actor.creature, verbose);
            if target.is_none() {
                if verbose > 0 {
                    println!("    {:?} at {} sees no targets", creature1, loc1);
                }
                continue;
            }
            let target = target.unwrap();
            if target.3 > 0 {
                if verbose > 0 {
                    println!("    {:?} at {} moves to {}", creature1, loc1, target.1);
                }
                self.grid.insert(actor.loc, Object::Empty);
                actor.loc = target.1;
                self.grid.insert(actor.loc, actor.creature);
            } else if verbose > 0 {
                println!("    {:?} at {} already at range", creature1, loc1);
            }
            if target.3 <= 1 {
                if verbose > 1 {
                    println!("Trying to Attack");
                }
                let mut options = Vec::new();
                for i2 in 0..n {
                    if i2 == i1 {
                        continue;
                    }
                    let (actor, actor2) = if i1 < i2 {
                        let (left, right) = self.actors.split_at_mut(i2);
                        (&mut left[i1], &mut right[0])
                    } else {
                        let (left, right) = self.actors.split_at_mut(i1);
                        (&mut right[0], &mut left[i2])
                    };
                    for dir in [Dir::N, Dir::W, Dir::E, Dir::S] {
                        let new_loc = actor.loc + dir.get_dir_true_vec();
                        if new_loc == actor2.loc
                            && actor2.hp > 0
                            && actor2.creature != actor.creature
                        {
                            options.push((i2, actor2.hp, actor2.loc));
                        }
                    }
                }
                //options.sort_by_key(|opt| opt.1);
                options.sort_by(|a, b| {
                    a.1.cmp(&b.1)
                        .then(b.2.y.cmp(&a.2.y))
                        .then(a.2.x.cmp(&b.2.x))
                });
                if options.len() > 1 && verbose > 0 {
                    println!("    {:?} at {} sees multiple targets", creature1, loc1);
                    for opt in options.iter() {
                        println!("         {} has {} hp", opt.2, opt.1);
                    }
                }

                let (i, _hp, _loc) = options.first().unwrap();

                let actor2 = &mut self.actors[*i];
                let creature2 = &actor2.creature.clone();
                let loc2 = &actor2.loc.clone();

                if verbose > 0 {
                    println!(
                        "    {:?} at {} attacks {:?} at {}",
                        creature1, loc1, creature2, loc2
                    );
                }
                actor2.hp -= attack;
                if verbose > 1 {
                    dbg!(&actor2);
                }
                if actor2.hp <= 0 {
                    if verbose > 0 {
                        println!("Target dies");
                    }
                    self.grid.insert(actor2.loc, Object::Empty);
                }
            }
        }
        //false
        true
    }

    fn find_target(
        grid: &BTreeMap<Vec2D, Object>,
        loc: Vec2D,
        attacker: Object,
        verbose: usize,
    ) -> Option<(Vec2D, Vec2D, Vec2D, usize)> {
        if verbose > 1 {
            println!("Finding targets");
        }
        let mut queue: PriorityQueue<(Vec2D, i32, Vec2D), i32> = PriorityQueue::new();
        queue.push((loc, 0, loc), 0); // Current location, steps, first step
        let mut options: Vec<(Vec2D, Vec2D, Vec2D, usize)> = Vec::new(); // Target square, first square, target creature loc, steps
        let mut target_steps = 99;
        loop {
            if queue.is_empty() {
                if !options.is_empty() {
                    options.sort_by(|a, b| {
                        b.0.y
                            .cmp(&a.0.y)
                            .then(a.0.x.cmp(&b.0.x))
                            .then(b.1.y.cmp(&a.1.y))
                            .then(a.1.x.cmp(&b.1.x))
                    });
                    let out = *options.first().unwrap();
                    if verbose > 1 {
                        println!(
                            "Choose Target at {:?}, from square {:?} with first step to {:?}",
                            out.2, out.0, out.1
                        );
                    }
                    return Some(out);
                } else {
                    //println!("No targets");
                    return None;
                }
            }
            let ((loc, steps, first_loc), _prio) = queue.pop().unwrap();
            if steps > target_steps {
                continue;
            }
            for dir in [Dir::N, Dir::W, Dir::E, Dir::S] {
                let new_loc = loc + dir.get_dir_true_vec();
                match grid.get(&new_loc) {
                    Some(Object::Wall) | None => continue,
                    Some(Object::Empty) => {
                        queue.push(
                            (
                                new_loc,
                                steps + 1,
                                if steps == 0 { new_loc } else { first_loc },
                            ),
                            -steps,
                        );
                    }
                    Some(obj) if attacker != *obj => {
                        if verbose > 1 {
                            println!(
                                "Found possible target {:?} at {} from {} with {} steps",
                                obj, new_loc, loc, steps
                            );
                        }
                        options.push((loc, first_loc, new_loc, steps as usize));
                        target_steps = steps;
                    }
                    _ => continue,
                }
            }
        }
    }
}

fn get_part1(grid: &mut Grid, verbose: usize) -> i32 {
    let mut tick = 0;
    loop {
        if verbose > 0 {
            println!("Tick: {tick}");
            grid.print_grid();
            for actor in grid.actors.iter() {
                println!("{:} {:?}: {}", actor.loc, actor.creature, actor.hp);
            }
        }
        //wait_for_enter();
        if !grid.evolve(verbose) {
            grid.print_grid();
            if verbose > 0 {
                println!("Nothing happened");
            }
            break;
        }
        tick += 1;
    }
    let hp_sum = grid.actors.iter().map(|a| a.hp).sum::<i32>();
    println!("Fight ended after {tick} ticks with {hp_sum} hp remaining");
    if verbose > 0 {
        println!("Remaining actors:");
        dbg!(&grid.actors);
    }

    tick * grid.actors.iter().map(|a| a.hp).sum::<i32>()
}

fn get_part2(grid: &Grid) -> i32 {
    let mut attack = 3;
    let elf_start = grid
        .actors
        .iter()
        .filter(|act| act.creature == Object::Elf)
        .count();
    println!("Start with {elf_start} elves");
    loop {
        let mut grid = grid.clone();
        attack += 1;
        if attack > 200 {
            break;
        }
        println!("Testing attack power {attack}");
        for actor in grid.actors.iter_mut() {
            if actor.creature == Object::Elf {
                actor.attack = attack;
            }
        }
        let res = get_part1(&mut grid, 0);
        let elf_count = grid
            .actors
            .iter()
            .filter(|act| act.creature == Object::Elf)
            .count();
        println!("Finished with {elf_count} elves");
        if elf_count == elf_start {
            return res;
        }
    }
    0
}

#[derive(Debug, Clone)]
struct Actor {
    loc: Vec2D,
    creature: Object,
    hp: i32,
    attack: i32,
}

impl Actor {
    fn new(loc: Vec2D, creature: Object) -> Self {
        Self {
            loc,
            creature,
            hp: 200,
            attack: 3,
        }
    }
}

fn read_grid(cont: &str) -> Grid {
    let mut actors: Vec<Actor> = Vec::new();
    let grid = cont
        .lines()
        .enumerate()
        .fold(BTreeMap::new(), |mut grid, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let loc = Vec2D {
                    x: x as i64,
                    y: -(y as i64),
                };
                let obj = match c {
                    '.' => Object::Empty,
                    '#' => Object::Wall,
                    'E' => {
                        actors.push(Actor::new(loc, Object::Elf));
                        Object::Elf
                    }
                    'G' => {
                        actors.push(Actor::new(loc, Object::Goblin));
                        Object::Goblin
                    }
                    c => {
                        dbg!(&c);
                        todo!()
                    }
                };
                grid.insert(loc, obj);
            });
            grid
        });
    Grid { grid, actors }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving() {
        let a = "#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";
        let mut grid = read_grid(a);
        grid.print_grid();
        let res = grid.evolve(0);
        assert!(res);
        grid.print_grid();
        assert_eq!(grid.actors[0].loc, Vec2D { x: 2, y: -1 });
        assert_eq!(grid.actors[1].loc, Vec2D { x: 4, y: -2 });
        assert_eq!(grid.actors[2].loc, Vec2D { x: 6, y: -1 });
        assert_eq!(grid.actors[3].loc, Vec2D { x: 2, y: -4 });
        assert_eq!(grid.actors[4].loc, Vec2D { x: 4, y: -3 });
        assert_eq!(grid.actors[5].loc, Vec2D { x: 7, y: -3 });
        assert_eq!(grid.actors[6].loc, Vec2D { x: 1, y: -6 });
        assert_eq!(grid.actors[7].loc, Vec2D { x: 4, y: -6 });
        assert_eq!(grid.actors[8].loc, Vec2D { x: 7, y: -6 });

        let res = grid.evolve(0);
        grid.print_grid();
        assert!(res);
        assert_eq!(grid.actors[0].loc, Vec2D { x: 3, y: -1 });
        assert_eq!(grid.actors[1].loc, Vec2D { x: 5, y: -1 });
        assert_eq!(grid.actors[2].loc, Vec2D { x: 4, y: -2 });
        assert_eq!(grid.actors[3].loc, Vec2D { x: 4, y: -3 });
        assert_eq!(grid.actors[4].loc, Vec2D { x: 6, y: -3 });
        assert_eq!(grid.actors[5].loc, Vec2D { x: 2, y: -3 });
        assert_eq!(grid.actors[6].loc, Vec2D { x: 1, y: -5 });
        assert_eq!(grid.actors[7].loc, Vec2D { x: 4, y: -5 });
        assert_eq!(grid.actors[8].loc, Vec2D { x: 7, y: -5 });

        let res = grid.evolve(0);
        grid.print_grid();
        // AFter 3 rounds
        assert!(res);
        assert_eq!(grid.actors[0].loc, Vec2D { x: 3, y: -2 });
        assert_eq!(grid.actors[1].loc, Vec2D { x: 5, y: -2 });
        assert_eq!(grid.actors[2].loc, Vec2D { x: 4, y: -2 });
        assert_eq!(grid.actors[3].loc, Vec2D { x: 3, y: -3 });
        assert_eq!(grid.actors[4].loc, Vec2D { x: 4, y: -3 });
        assert_eq!(grid.actors[5].loc, Vec2D { x: 5, y: -3 });
        assert_eq!(grid.actors[6].loc, Vec2D { x: 1, y: -4 });
        assert_eq!(grid.actors[7].loc, Vec2D { x: 4, y: -4 });
        assert_eq!(grid.actors[8].loc, Vec2D { x: 7, y: -5 });
    }

    #[test]
    fn part1() {
        let a = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        let mut grid = read_grid(a);
        assert_eq!(get_part1(&mut grid, 1), 27730);

        let a = "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";
        let mut grid = read_grid(a);
        // Should end after 37 rounds with 982 hp left
        assert_eq!(get_part1(&mut grid, 1), 37 * 982);

        let a = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        let mut grid = read_grid(a);
        // Should end after 46 rounds with 859 hp left
        assert_eq!(get_part1(&mut grid, 1), 39514);

        let a = "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        let mut grid = read_grid(a);
        // Should end after 35 rounds with 793 hp left
        assert_eq!(get_part1(&mut grid, 0), 27755);

        let a = "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";
        let mut grid = read_grid(a);
        // Should end after 54 rounds with 536 hp left
        assert_eq!(get_part1(&mut grid, 0), 28944);

        let a = "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        let mut grid = read_grid(a);
        // Should end after 20 rounds with 937 hp left
        assert_eq!(get_part1(&mut grid, 1), 18740);
    }

    #[test]
    fn part2() {
        let a = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        let mut grid = read_grid(a);
        for actor in grid.actors.iter_mut() {
            if actor.creature == Object::Elf {
                actor.attack = 15;
            }
        }

        assert_eq!(get_part1(&mut grid, 1), 4988);

        let grid = read_grid(a);
        assert_eq!(get_part2(&grid), 4988);
    }
}
