use clap::Parser;
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

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn new(str: &str) -> Self {
        let (name, res) = str.split_once(':').unwrap();
        let mut out = Self {
            name: name.to_string(),
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        };
        for a in res.trim().split(',') {
            let (key, value) = a.trim().split_once(' ').unwrap();
            let val = value.parse::<i32>().unwrap();
            match key {
                "capacity" => {
                    out.capacity = val;
                }
                "durability" => {
                    out.durability = val;
                }
                "flavor" => {
                    out.flavor = val;
                }
                "texture" => {
                    out.texture = val;
                }
                "calories" => {
                    out.calories = val;
                }
                _ => {
                    panic!("Unknown key")
                }
            }
        }
        out
    }
}

fn get_cookie(vec: &Vec<(&Ingredient, i32)>) -> i32 {
    let capacity_sum: i32 = vec.iter().map(|(i, c)| c * i.capacity).sum::<i32>().max(0);
    let durability_sum: i32 = vec
        .iter()
        .map(|(i, c)| c * i.durability)
        .sum::<i32>()
        .max(0);
    let flavor_sum: i32 = vec.iter().map(|(i, c)| c * i.flavor).sum::<i32>().max(0);
    let texture_sum: i32 = vec.iter().map(|(i, c)| c * i.texture).sum::<i32>().max(0);
    capacity_sum * durability_sum * flavor_sum * texture_sum
}

fn read_contents(cont: &str) -> (i32, i32) {
    let ingredients = cont.lines().map(Ingredient::new).collect::<Vec<_>>();
    println!("{} ingredients", ingredients.len());
    let part1 = get_answer(&ingredients, false);
    let part2 = get_answer(&ingredients, true);
    (part1, part2)
}

fn get_answer(ingredients: &[Ingredient], part2: bool) -> i32 {
    let n = ingredients.len();
    let mut counts: Vec<(&Ingredient, i32)> = ingredients.iter().map(|c| (c, 0)).collect();
    let mut max_score = 0;

    // There are 100 ^ (n-1) to have a count of 0-100 for the first (n-1) ingredients
    // These include options which have a sum larger than 100
    let option_count = 100_i32.pow(u32::try_from(n - 1).unwrap());
    let mut tested = 0;
    for opt in 0..option_count {
        let mut o = opt;
        let mut x_sum = 0;
        let mut c;
        for count in counts.iter_mut().take(n - 1) {
            (c, o) = (o % 100, o / 100);
            x_sum += c;
            count.1 = c;
        }
        if x_sum > 100 {
            // If the sum is larger than 100, this is not valid cookie
            continue;
        }
        counts[n - 1].1 = 100 - x_sum;
        if part2 {
            let calorie_sum: i32 = counts.iter().map(|(i, c)| c * i.calories).sum();
            if calorie_sum != 500 {
                continue;
            }
        }
        tested += 1;
        let cookie = get_cookie(&counts);
        if cookie > max_score {
            dbg!(&cookie);
            max_score = cookie;
        }
    }
    println!("Tested {tested} options");
    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        assert_eq!(read_contents(&a).0, 62842880);
        assert_eq!(read_contents(&a).1, 57600000);
    }
}
