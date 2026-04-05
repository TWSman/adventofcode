use clap::Parser;

use std::fs;
use std::ops::Add;
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

// Weapons:    Cost  Damage  Armor
// Dagger        8     4       0
// Shortsword   10     5       0
// Warhammer    25     6       0
// Longsword    40     7       0
// Greataxe     74     8       0
//
// Armor:      Cost  Damage  Armor
// Leather      13     0       1
// Chainmail    31     0       2
// Splintmail   53     0       3
// Bandedmail   75     0       4
// Platemail   102     0       5
//
// Rings:      Cost  Damage  Armor
// Damage +1    25     1       0
// Damage +2    50     2       0
// Damage +3   100     3       0
// Defense +1   20     0       1
// Defense +2   40     0       2
// Defense +3   80     0       3

fn read_contents(cont: &str) -> (i32, i32) {
    let mut boss = Character {
        hp: 0,
        damage: 0,
        armor: 0,
    };
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
            "Armor" => {
                boss.armor = val;
            }
            _ => panic!("Unknown key"),
        }
    }
    get_answer(&boss)
}

fn get_weapons() -> Vec<(i32, Character)> {
    vec![
        // Dagger        8     4       0
        (
            8,
            Character { hp: 0, damage: 4, armor: 0, },
        ),
        // Shortsword   10     5       0
        (
            10,
            Character { hp: 0, damage: 5, armor: 0, },
        ),
        // Warhammer    25     6       0
        (
            25,
            Character { hp: 0, damage: 6, armor: 0, },
        ),
        // Longsword    40     7       0
        (
            40,
            Character { hp: 0, damage: 7, armor: 0, },
        ),
        // Greataxe     74     8       0
        (
            74,
            Character { hp: 0, damage: 8, armor: 0, },
        ),
    ]
}

fn get_rings() -> Vec<(i32, Character)> {
    // Rings:      Cost  Damage  Armor
    vec![
        // Damage +1    25     1       0
        (
            25,
            Character { hp: 0, damage: 1, armor: 0, },
        ),
        // Damage +2    50     2       0
        (
            50,
            Character { hp: 0, damage: 2, armor: 0, },
        ),
        // Damage +3   100     3       0
        (
            100,
            Character { hp: 0, damage: 3, armor: 0,
            },
        ),
        // Defense +1   20     0       1
        (
            20,
            Character { hp: 0, damage: 0, armor: 1,
            },
        ),
        // Defense +2   40     0       2
        (
            40,
            Character { hp: 0, damage: 0, armor: 2,
            },
        ),
        // Defense +3   80     0       3
        (
            80,
            Character { hp: 0, damage: 0, armor: 3,
            },
        ),
        // Rings could be empty
        (
            0,
            Character { hp: 0, damage: 0, armor: 0, },
        ),
        (
            0,
            Character { hp: 0, damage: 0, armor: 0, },
        ),
    ]
}

fn get_armor() -> Vec<(i32, Character)> {
    // Armor:      Cost  Damage  Armor
    vec![
        // Leather      13     0       1
        (
            13,
            Character { hp: 0, damage: 0, armor: 1, },
        ),
        // Chainmail    31     0       2
        (
            31,
            Character { hp: 0, damage: 0, armor: 2,
            },
        ),
        // Splintmail   53     0       3
        (
            53,
            Character { hp: 0, damage: 0, armor: 3, },
        ),
        // Bandedmail   75     0       4
        (
            75,
            Character { hp: 0, damage: 0, armor: 4, },
        ),
        // Platemail   102     0       5
        (
            102,
            Character { hp: 0, damage: 0, armor: 5, },
        ),
        // Its possible to buy no armor
        (
            0,
            Character { hp: 0, damage: 0, armor: 0, },
        ),
    ]
}

fn get_answer(boss: &Character) -> (i32, i32) {
    let weapons = get_weapons();
    let armors = get_armor();
    let rings = get_rings();
    let mut min_cost = 999;
    let mut max_cost = 0;
    for (w_cost, weapon) in &weapons {
        for (a_cost, armor) in &armors {
            for (i, (r1_cost, ring1)) in rings.iter().enumerate() {
                for (j, (r2_cost, ring2)) in rings.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    let mut player = Character::new(100);
                    player = player + *weapon;
                    player = player + *armor;
                    player = player + *ring1;
                    player = player + *ring2;
                    let cost = w_cost + a_cost + r1_cost + r2_cost;
                    if cost > min_cost && cost < max_cost {
                        continue;
                    }
                    if cost < min_cost && fight(&player, boss) {
                        println!("Player won with {cost} gold");
                        min_cost = cost;
                    }

                    if cost > max_cost && !fight(&player, boss) {
                        println!("Player Lost with {cost} gold");
                        max_cost = cost;
                    }
                }
            }
        }
    }
    (min_cost, max_cost)
}

fn fight(player: &Character, boss: &Character) -> bool {
    let mut myhp = player.hp;
    let mut bosshp = boss.hp;
    let player_damage = (player.damage - boss.armor).max(1);
    let boss_damage = (boss.damage - player.armor).max(1);
    loop {
        bosshp -= player_damage;
        if bosshp <= 0 {
            return true;
        }
        myhp -= boss_damage;
        if myhp <= 0 {
            return false;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Character {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Character {
    fn new(hp: i32) -> Self {
        Self {
            armor: 0,
            damage: 0,
            hp,
        }
    }
}

impl Add for Character {
    type Output = Character;
    fn add(self, rhs: Character) -> Character {
        Character {
            armor: self.armor + rhs.armor,
            hp: self.hp + rhs.hp,
            damage: self.damage + rhs.damage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fight() {
        // This is the example case. Should lead to a narrow player victory
        let player = Character {
            hp: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Character {
            hp: 12,
            damage: 7,
            armor: 2,
        };
        assert!(fight(&player, &boss));
        let boss = Character {
            hp: 13,
            damage: 7,
            armor: 2,
        };
        assert!(!fight(&player, &boss));
    }
}
