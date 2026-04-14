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

fn get_part1(target_elves: usize) -> usize {
    let mut pow2 = 1;
    // Find nearest power of 2 less tan or equal to target_elves
    while pow2 * 2 <= target_elves {
        pow2 *= 2;
    }

    // Powers of 2 seem result in the first elf getting the pressent
    if pow2 == target_elves {
        return 1;
    }

    let mut elf_i = 1;
    for _ in (pow2 + 1)..=target_elves {
        // Elf index increases by 2 each time
        elf_i += 2;
    }
    elf_i
}

#[allow(dead_code)]
fn get_part2_full(target_elves: usize) -> usize {
    println!("Target elves: {}", target_elves);
    let mut elves = (1..=target_elves).collect::<Vec<usize>>();
    let mut elf_i = 0; // Index in array
    while elves.len() > 1 {
        //let elf_name = elves.get(elf_i).unwrap();
        //println!("Elf #{} has turn", elf_name);
        let to_remove = (elf_i + elves.len() / 2) % elves.len(); // Counting from the current elf
        //let remove_name = elves.get(to_remove).unwrap();
        //println!("Elf #{} ({}) tries to steal from elf at index {}, i.e. Elf #{}", elf_name, elf_i, to_remove, remove_name);
        elves.remove(to_remove);
        if to_remove > elf_i {
            elf_i += 1;
        }
        if elf_i >= elves.len() {
            elf_i = 0;
        }
    }
    elves[0]
}

fn get_part2(target_elves: usize) -> usize {
    let mut pow3 = 1;
    while pow3 * 3 <= target_elves {
        pow3 *= 3;
    }
    // If target elves is a power of three, then the last elf gets the presents
    if pow3 == target_elves {
        return pow3;
    }
    println!("Closest power of three: {}", pow3);
    if target_elves < 2 * pow3 {
        // Until 2 * pow2 elf index increases by 1 (starting from 0) when elf count is increased by one
        // Number of increases is target_elves - pow
        return target_elves - pow3;
    }
    if target_elves == 2 * pow3 {
        // 2 times a power of three results in the elf at n / 2 getting the presents
        return target_elves / 2;
    }

    // Otherwise, elf index increases by 2 when elf count is increased by one
    (target_elves - 2 * pow3) * 2 + pow3
}

fn read_contents(cont: &str) -> (usize, usize) {
    let elves = cont.trim().parse::<usize>().unwrap();
    let part1 = get_part1(elves);
    let part2 = get_part2(elves);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(get_part1(5), 3);
        assert_eq!(get_part1(6), 5);
        assert_eq!(get_part1(15), 15);
        assert_eq!(get_part1(16), 1);

        assert_eq!(get_part1(531441), 14307);
    }

    #[test]
    fn part2() {
        assert_eq!(get_part2(3), 3);
        assert_eq!(get_part2(4), 1);
        assert_eq!(get_part2(5), 2);
        assert_eq!(get_part2(6), 3);
        assert_eq!(get_part2(7), 5);
        assert_eq!(get_part2(8), 7);
        assert_eq!(get_part2(9), 9);
        assert_eq!(get_part2(10), 1);
        assert_eq!(get_part2(11), 2);
        assert_eq!(get_part2(12), 3);
        assert_eq!(get_part2(13), 4);
        assert_eq!(get_part2(14), 5);
        assert_eq!(get_part2(15), 6);
        assert_eq!(get_part2(16), 7);
        assert_eq!(get_part2(17), 8);
        assert_eq!(get_part2(18), 9);
        assert_eq!(get_part2(19), 11);
        assert_eq!(get_part2(20), 13);
        assert_eq!(get_part2(21), 15);
        assert_eq!(get_part2(22), 17);
        assert_eq!(get_part2(23), 19);
        assert_eq!(get_part2(24), 21);
        assert_eq!(get_part2(25), 23);
        assert_eq!(get_part2(26), 25);
        assert_eq!(get_part2(27), 27);
        assert_eq!(get_part2(28), 1);
        assert_eq!(get_part2(29), 2);
        assert_eq!(get_part2(30), 3);
        assert_eq!(get_part2(31), 4);
        assert_eq!(get_part2(32), 5);
        assert_eq!(get_part2(33), 6);
        assert_eq!(get_part2(34), 7);
        assert_eq!(get_part2(35), 8);
        assert_eq!(get_part2(36), 9);
        assert_eq!(get_part2(37), 10);
        assert_eq!(get_part2(38), 11);
        assert_eq!(get_part2(39), 12);
        assert_eq!(get_part2(40), 13);

        assert_eq!(get_part2(27), 27); // Powers of three seem to result in the final elf
        assert_eq!(get_part2(54), 27); // Twice the power of three results in n / 2 i.e. the
        // previous power of 3 (from the elf count)
        assert_eq!(get_part2(81), 81); // Powers of three seem to result in the final elf
        assert_eq!(get_part2(243), 243);
        assert_eq!(get_part2(6561), 6561);
        assert_eq!(get_part2(19683), 19683);
        assert_eq!(get_part2(531441), 531441);

        assert_eq!(get_part2(30), 3); // 3 ** 3 +3
        assert_eq!(get_part2(84), 3);

        assert_eq!(get_part2(33), 6); // 3 ** 3 +6
        assert_eq!(get_part2(87), 6);

        assert_eq!(get_part2(36), 9); // 3 ** 3 + 9
        assert_eq!(get_part2(90), 9); // 3 ** 4 + 9

        assert_eq!(get_part2(39), 12); // 3 ** 3 + 12
        assert_eq!(get_part2(93), 12); // 3 ** 4 + 12
    }
}
