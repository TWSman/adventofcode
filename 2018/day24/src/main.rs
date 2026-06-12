use clap::Parser;
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
    // 15147 is too low
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

#[derive(Debug, Clone)]
struct Group {
    id: usize,
    units: i64,
    hp: i64,
    damage: i64,
    damage_type: Element,
    ep: i64,
    initiative: usize,
    weakness: Vec<Element>,
    immune: Vec<Element>,
    faction: Faction,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Faction {
    Immune,
    Infection,
    Tie,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Element {
    Fire,
    Bludgeon,
    Radiation,
    Cold,
    Slash,
}

impl Element {
    fn new(ln: &str) -> Self {
        match ln.replace([',', ';'], "").as_str() {
            "fire" => Self::Fire,
            "radiation" => Self::Radiation,
            "bludgeoning" => Self::Bludgeon,
            "cold" => Self::Cold,
            "slashing" => Self::Slash,
            _ => panic!(),
        }
    }
}

impl Group {
    fn new(ln: &str, faction: Faction, id: usize) -> Self {
        let re = Regex::new(r"(\d+) units each with (\d+) hit points( \(.*\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
        let caps = re.captures(ln).unwrap();
        let mut weakness = Vec::new();
        let mut immune = Vec::new();
        if caps.get(3).is_some() {
            let tmp = &caps[3]
                .strip_prefix(" (")
                .unwrap()
                .strip_suffix(')')
                .unwrap();
            let mut v: &mut Vec<Element> = &mut weakness;
            for t in tmp.split_whitespace() {
                if t == "to" {
                    continue;
                }
                if t == "immune" {
                    v = &mut immune;
                    continue;
                }
                if t == "weak" {
                    v = &mut weakness;
                    continue;
                }
                v.push(Element::new(t));
            }
        }
        let units = caps[1].parse().unwrap();
        let damage = caps[4].parse().unwrap();
        Self {
            id,
            units,
            hp: caps[2].parse().unwrap(),
            damage,
            damage_type: Element::new(&caps[5]),
            initiative: caps[6].parse().unwrap(),
            immune,
            weakness,
            faction,
            ep: units * damage,
        }
    }

    fn calculate_damage(&self, damage: i64, damage_type: Element) -> i64 {
        if self.immune.contains(&damage_type) {
            0
        } else if self.weakness.contains(&damage_type) {
            damage * 2
        } else {
            damage
        }
    }

    fn take_damage(&mut self, damage: i64) {
        let units_lost = damage / self.hp;
        self.units = (self.units - units_lost).max(0);
        self.ep = self.units * self.damage;
    }
}

fn read_groups(cont: &str) -> Vec<Group> {
    let mut faction: Option<Faction> = None;
    let mut groups = Vec::new();
    let mut id = 0;
    for line in cont.lines() {
        if line.starts_with("Immune System:") {
            faction = Some(Faction::Immune);
            continue;
        }
        if line.starts_with("Infection") {
            faction = Some(Faction::Infection);
            continue;
        }
        if line.len() > 10 {
            groups.push(Group::new(line, faction.unwrap(), id));
            id += 1;
        }
    }
    groups
}

fn read_contents(cont: &str) -> (i64, i64) {
    let groups = read_groups(cont);
    let part1 = get_part1(&groups);
    let part2 = get_part2(&groups);
    (part1, part2)
}

fn get_part1(groups: &[Group]) -> i64 {
    let (groups, _winner) = run_battle(groups);
    groups.iter().map(|g| g.units).sum()
}

fn run_battle(groups: &[Group]) -> (Vec<Group>, Faction) {
    let mut groups = groups.to_owned();
    let mut groups_by_faction = BTreeMap::new();
    for faction in [Faction::Infection, Faction::Immune] {
        let c = groups.iter().filter(|g| g.faction == faction).count();
        groups_by_faction.insert(faction, c);
    }
    let mut destroyed = BTreeSet::new();
    loop {
        let units_now = groups.iter().map(|g| g.units).sum::<i64>();
        groups.sort_by(|a, b| b.ep.cmp(&a.ep).then(b.initiative.cmp(&a.initiative)));
        //dbg!(&groups);
        let mut targets: BTreeSet<usize> = BTreeSet::new();
        // Attacker, target, initiative of attacker
        let mut target_by: Vec<(usize, usize, usize)> = Vec::new();
        for g in &groups {
            if destroyed.contains(&g.id) {
                continue;
            }
            let mut max = None;
            for g2 in &groups {
                if destroyed.contains(&g2.id)
                    || g.id == g2.id
                    || targets.contains(&g2.id)
                    || g2.faction == g.faction
                {
                    continue;
                }
                let damage = g2.calculate_damage(g.ep, g.damage_type);
                match max {
                    Some((_, d, _ep)) if damage > d => {
                        max = Some((g2.id, damage, g2.ep));
                    }
                    Some((_, d, ep)) if damage == d && g2.ep > ep => {
                        max = Some((g2.id, damage, g2.ep));
                    }
                    None if damage > 0 => {
                        max = Some((g2.id, damage, g2.ep));
                    }
                    _ => {}
                }
            }
            if let Some(m) = max {
                targets.insert(m.0);
                target_by.push((g.id, m.0, g.initiative));
            }
        }
        target_by.sort_by(|a, b| b.2.cmp(&a.2)); // Sort by initiative
        for (i_attack, i_target, _ini) in target_by.iter() {
            if destroyed.contains(i_attack) {
                continue;
            }
            let g_attack = groups.iter().find(|g| g.id == *i_attack).unwrap();
            let g_target = groups.iter().find(|g| g.id == *i_target).unwrap();
            let damage = g_target.calculate_damage(g_attack.ep, g_attack.damage_type);
            let t = groups.iter_mut().find(|g| g.id == *i_target).unwrap();
            t.take_damage(damage);
            if t.units == 0 {
                *groups_by_faction.get_mut(&t.faction).unwrap() -= 1;
                destroyed.insert(*i_target);
            }
        }
        // Check if one faction has been destroyed
        if *groups_by_faction.get(&Faction::Infection).unwrap() == 0
            || *groups_by_faction.get(&Faction::Immune).unwrap() == 0
        {
            break;
        }

        // Check if any units were killed
        // The battle might result in a stalemate where neither side can damage the other
        let units_after = groups.iter().map(|g| g.units).sum::<i64>();
        if units_after == units_now {
            return (Vec::new(), Faction::Tie);
        }
    }
    let winner = groups.iter().find(|g| g.units > 0).unwrap().faction;
    (groups, winner)
}

fn get_part2(groups: &[Group]) -> i64 {
    // Binary search the find the correct boost
    let mut a = 0;
    let mut b = 2000;
    let mut count = 0;
    loop {
        let mut g2 = groups.to_owned();
        let boost = (a + b) / 2;
        if boost == a || boost == b {
            break;
        }
        for g in g2.iter_mut() {
            if g.faction == Faction::Immune {
                g.damage += boost;
                g.ep = g.damage * g.units;
            }
        }
        let (g2, winner) = run_battle(&g2);
        if winner == Faction::Immune {
            count = g2.iter().map(|g| g.units).sum();
            b = boost;
        } else {
            a = boost;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_group() {
        let a = "17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2";
        let g = Group::new(&a, Faction::Immune, 0);
        assert_eq!(g.hp, 5390);
        assert_eq!(g.id, 0);
        assert_eq!(g.units, 17);
        assert_eq!(g.damage, 4507);
        assert_eq!(g.initiative, 2);
        assert_eq!(g.damage_type, Element::Fire);
        assert!(g.weakness.contains(&Element::Radiation));
    }

    #[test]
    fn part1() {
        let a = "
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        assert_eq!(read_contents(&a).0, 5216);
    }

    #[test]
    fn part2() {
        let a = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 6077 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 1595 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        assert_eq!(read_contents(&a).0, 51);
        println!("Done");

        let a = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        assert_eq!(read_contents(&a).1, 51);
    }
}
