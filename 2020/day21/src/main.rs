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

#[derive(Debug, Clone)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
    ingredients_enc: Vec<usize>,
    allergens_enc: Vec<usize>,
}

impl Food {
    fn new(ln: &str) -> Self {
        let (ingredients, allergens) = if !ln.contains('(') {
            (
                ln.split_whitespace()
                    .map(|l| l.to_string())
                    .collect::<Vec<String>>(),
                Vec::new(),
            )
        } else {
            let (a, b) = ln.split_once("(contains ").unwrap();
            let tmp = a
                .split_whitespace()
                .map(|l| l.to_string())
                .collect::<Vec<String>>();
            let tmp2 = b
                .strip_suffix(')')
                .unwrap()
                .split(',')
                .map(|l| l.trim().to_string())
                .collect::<Vec<String>>();
            (tmp, tmp2)
        };
        Self {
            ingredients,
            allergens,
            ingredients_enc: Vec::new(),
            allergens_enc: Vec::new(),
        }
    }

    fn encode(&mut self, mapping: &BTreeMap<String, usize>) {
        for ingredient in self.ingredients.iter() {
            self.ingredients_enc.push(*mapping.get(ingredient).unwrap());
        }
        for allergen in self.allergens.iter() {
            self.allergens_enc.push(*mapping.get(allergen).unwrap());
        }
    }
}

fn main() {
    let args = Args::parse();
    let start = Instant::now();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("\n########################");
    // 192 is too low
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {elapsed:.2?}");
}

fn read_contents(cont: &str) -> (i64, String) {
    let mut foods = cont.lines().map(Food::new).collect::<Vec<_>>();
    let mut mapping = BTreeMap::new();
    let mut counter_mapping = BTreeMap::new();
    let mut i = 0;
    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            if mapping.contains_key(allergen) {
                continue;
            }
            mapping.insert(allergen.clone(), i);
            counter_mapping.insert(i, allergen.clone());
            i += 1;
        }
    }
    for food in foods.iter() {
        for ingredient in food.ingredients.iter() {
            if mapping.contains_key(ingredient) {
                continue;
            }
            mapping.insert(ingredient.clone(), i);
            counter_mapping.insert(i, ingredient.clone());
            i += 1;
        }
    }
    for food in foods.iter_mut() {
        food.encode(&mapping);
    }
    let options = get_preliminary_options(&foods);

    println!("Preliminary mapping:");
    for (key, opts) in &options {
        println!("{}: ", counter_mapping.get(key).unwrap());
        for o in opts {
            println!("    {}", counter_mapping.get(o).unwrap());
        }
    }
    let part1 = get_part1(&options, &foods);
    let part2 = get_part2(&options, &counter_mapping);
    (part1, part2)
}

fn get_preliminary_options(foods: &[Food]) -> BTreeMap<usize, BTreeSet<usize>> {
    let mut all_allergens: BTreeSet<usize> = BTreeSet::new();
    let mut options: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

    // First check which ingredients and allergens are seen together
    for f in foods.iter() {
        for a in f.allergens_enc.iter() {
            for i in f.ingredients_enc.iter() {
                options.entry(*a).or_default().insert(*i);
            }
            all_allergens.insert(*a);
        }
    }

    for allergen in all_allergens.iter() {
        for food in foods.iter() {
            if !food.allergens_enc.contains(allergen) {
                // Skip foods that do not contain the allergen
                continue;
            }
            // Loop over all possible source ingredients of this allergen
            for source in options.get(allergen).unwrap().clone().iter() {
                if !food.ingredients_enc.contains(source) {
                    // This source is not possible the allergen, since the food contains the
                    // allergen but not the source
                    options.get_mut(allergen).unwrap().remove(source);
                }
            }
        }
    }
    options
}

fn get_part1(options: &BTreeMap<usize, BTreeSet<usize>>, foods: &[Food]) -> i64 {
    let all_ingredients: BTreeSet<usize> = foods
        .iter()
        .flat_map(|f| f.ingredients_enc.iter())
        .copied()
        .collect();

    // First check which ingredients are not options for any allergen
    let answers = all_ingredients
        .iter()
        // No allergen contains this ingredient as an option
        .filter(|ing| options.values().all(|opt| !opt.contains(ing)))
        .collect::<BTreeSet<_>>();

    // Then count how many times these ingredients appear in all foods
    foods
        .iter()
        .map(|food| {
            food.ingredients_enc
                .iter()
                .filter(|ing| answers.contains(ing))
                .count() as i64
        })
        .sum()
}

fn get_part2(
    options: &BTreeMap<usize, BTreeSet<usize>>,
    mapping: &BTreeMap<usize, String>,
) -> String {
    // Further analyze to options until there is exactly one source option for each allergen
    let mut options = options.clone();
    let mut final_mapping: BTreeMap<String, String> = BTreeMap::new(); // allergen, ingredient
    loop {
        if final_mapping.len() == options.len() {
            // All mappings have been found
            break;
        }
        let mut changes = Vec::new();
        for (allergen, opts) in options.iter() {
            if opts.len() == 1 {
                // Only one source option left for this allergen
                // Since each ingredient can contain at most one allergen, this ingredient can't be
                // a source for any other allergens
                let ingredient = opts.first().unwrap();
                final_mapping.insert(
                    mapping.get(allergen).unwrap().to_string(),
                    mapping.get(ingredient).unwrap().to_string(),
                );
                changes.push(*ingredient);
            }
        }
        for change in changes {
            for (_, opts) in options.iter_mut() {
                if opts.contains(&change) {
                    opts.remove(&change);
                }
            }
        }
    }
    // BTreeMap should be already sorted alphabetically by key, i.e. by allergen
    final_mapping
        .values()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(read_contents(&a).0, 5);
    }

    #[test]
    fn part2() {
        let a = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(read_contents(&a).1, "mxmxvkd,sqjhc,fvjkl");
    }
}
