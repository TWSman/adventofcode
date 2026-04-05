use clap::Parser;

use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::fs;
use std::time::Instant;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
    let mut boss = Character::new(0);
    for line in cont.lines() {
        let (key, value) = line.split_once(':').unwrap();
        let val = value.trim().parse::<i32>().unwrap();
        match key {
            "Hit Points" => {
                boss.hp = val;
            }
            "Damage" => {
                boss.damage = val;
            }
            _ => panic!("Unknown key"),
        }
    }
    let mut player = Character::new(50);
    player.mana = 500;
    let part1 = run_fight(&player, &boss, false);
    let part2 = run_fight(&player, &boss, true);
    (part1, part2)
}

#[derive(EnumIter, Debug, PartialEq, Eq)]
enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

fn run_fight(player: &Character, boss: &Character, part2: bool) -> i32 {
    let mut queue = PriorityQueue::new();
    queue.push((*player, *boss, 0), Reverse(0));
    loop {
        if queue.is_empty() {
            println!("Queue is empty");
            return 0;
        }
        let ((player, boss, mana_used), _prio) = queue.pop().unwrap();
        println!("Mana used: {mana_used}");
        let mut player = player;
        let mut boss = boss;
        if part2 {
            player.hp -= 1;
            if player.hp <= 0 {
                continue;
            }
        }
        player.apply_effects();
        if boss.poison_effect > 0 {
            boss.hp -= 3;
            boss.poison_effect -= 1;
        }
        if boss.hp <= 0 {
            println!("Boss at 0 health after poison");
            return mana_used;
        }
        for spell in Spell::iter() {
            let mut new_player = player;
            let mut new_boss = boss;
            let mut mana = mana_used;
            match spell {
                Spell::Missile => {
                    new_boss.hp -= 4;
                    new_player.mana -= 53;
                    mana += 53;
                    if new_boss.hp <= 0 {
                        return mana;
                    }
                }
                Spell::Drain => {
                    new_boss.hp -= 2;
                    new_player.hp += 2;
                    new_player.mana -= 73;
                    mana += 73;
                    if new_boss.hp <= 0 {
                        return mana;
                    }
                }
                Spell::Shield => {
                    if new_player.shield_effect > 0 {
                        continue;
                    }
                    new_player.shield_effect = 6;
                    new_player.mana -= 113;
                    mana += 113;
                }
                Spell::Poison => {
                    if new_boss.poison_effect > 0 {
                        continue;
                    }
                    new_boss.poison_effect = 6;
                    new_player.mana -= 173;
                    mana += 173;
                }

                Spell::Recharge => {
                    if new_player.recharge_effect > 0 {
                        continue;
                    }
                    new_player.recharge_effect = 5;
                    new_player.mana -= 229;
                    mana += 229;
                }
            }
            if new_player.mana < 0 {
                // Tried to cast a spell without enough mana
                println!("Not enough mana to cast {spell:?}");
                continue;
            }
            new_player.apply_effects();
            if new_boss.poison_effect > 0 {
                new_boss.poison_effect -= 1;
                new_boss.hp -= 3;
                if new_boss.hp <= 0 {
                    println!("Boss at 0 health after poison");
                    return mana;
                }
            }
            let boss_dmg = if new_player.shield_effect > 0 {
                (new_boss.damage - 7).max(1)
            } else {
                new_boss.damage
            };
            new_player.hp -= boss_dmg;
            if new_player.hp <= 0 {
                println!("Player is dead ({spell:?})");
                continue;
            }
            println!("Cast {spell:?}, mana used: {mana}");
            //dbg!(&new_boss);
            queue.push((new_player, new_boss, mana), Reverse(mana));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Character {
    hp: i32,
    damage: i32,
    mana: i32,
    shield_effect: i32,
    recharge_effect: i32,
    poison_effect: i32,
}

impl Character {
    fn new(hp: i32) -> Self {
        Self {
            damage: 0,
            mana: 0,
            hp,
            shield_effect: 0,
            recharge_effect: 0,
            poison_effect: 0,
        }
    }

    fn apply_effects(&mut self) {
        if self.shield_effect > 0 {
            self.shield_effect -= 1;
        }
        if self.recharge_effect > 0 {
            self.recharge_effect -= 1;
            self.mana += 101;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut boss = Character::new(13);
        boss.damage = 8;
        let mut player = Character::new(10);
        player.mana = 250;
        // optimal is to first cast poison, then missile
        assert_eq!(run_fight(&player, &boss, false), 173 + 53);
        boss.hp = 14;
        // Optimal: Recharge, Shield, Drain, Poison, Missile
        assert_eq!(run_fight(&player, &boss, false), 229 + 113 + 73 + 173 + 53);
    }
}
