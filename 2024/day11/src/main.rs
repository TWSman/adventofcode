use std::iter::successors;
use memoize::memoize;
use num_format::{Locale, ToFormattedString};

fn main() {
    let input = "77 515 6779622 6 91370 959685 0 9861";
    let (part1, part2) = read_contents(input);
    println!("Part 1 answer is {}", part1.to_formatted_string(&Locale::fi));
    println!("Part 2 answer is {}", part2.to_formatted_string(&Locale::fi));

    println!("Part 1 answer is {}", part1);
    println!("Part 2 answer is {}", part2);

}


fn read_contents(cont: &str) -> (u64, u64) {
    println!("Vect 0 answer is {}", get_results(&[0], 75).to_formatted_string(&Locale::fi));
    let nums: Vec<u64> = cont.split_whitespace().map(|m| {m.parse::<u64>().unwrap() }).collect();
    (get_results(&nums, 25), get_results(&nums, 75))
}


#[memoize]
// Recursive function to calculate the length of vector that
// results from applying the rules to 'input', 'iter' times
fn apply_rec(input: u64, iter: usize)-> usize {
    if iter == 0 {
        // if this is the last iteration, return 1
        return 1;
    }
    if input == 0 {
        // 0 turns into 1
        return apply_rec(1, iter - 1);
    }

    // This line calculates the number of digits in the input
    // u64.pow() expects a u32
    let siz = successors(Some(input), |&n| (n >= 10).then_some(n / 10)).count() as u32;

    // numbers with even number of digits are split in half
    // 2024 turns into 20, 24
    if siz % 2 == 0 {
        let spl = siz / 2;
        return apply_rec(input / 10_u64.pow(spl), iter - 1) + apply_rec(input % 10_u64.pow(spl), iter - 1)
    }
    // Other values are multiplied by 2024
    apply_rec(input * 2024, iter -1)
}


fn get_results(input: &[u64], rounds: usize) -> u64 {
    input.iter().map(|i| {
        apply_rec(*i, rounds) as u64
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        // example data
        // part1 result (55312) was given
        let a = "125 17";
        assert_eq!(read_contents(&a).0, 55312);
        assert_eq!(read_contents(&a).1, 65601038650482);
    }

    #[test]
    fn test_iteration() {
        // iter = 0 should always give 1
        for i in 0..100 {
            assert_eq!(apply_rec(i, 0), 1);
        }
        // [0] goes to
        // [1] after 1 round, lenght is 1
        assert_eq!(apply_rec(0, 1), 1);
        // [2024] after 2 rounds, length is still 1
        assert_eq!(apply_rec(0, 2), 1);
        // [20, 24], after 3 rounds, length is now 2
        assert_eq!(apply_rec(0, 3), 2);
        // [2, 0, 2, 4], after 4 rounds, length is now 4
        assert_eq!(apply_rec(0, 4), 4);
        // [4048, 1, 4048, 8096], after 5 rounds, length is still 4
        assert_eq!(apply_rec(0, 5), 4);
        // [40, 48, 2024, 40,48, 80,96], after 6 rounds, length is now 7
        assert_eq!(apply_rec(0, 6), 7);
        // 12 should be split in half
        assert_eq!(apply_rec(12, 1), 2);
        // 1244 should be split in half
        assert_eq!(apply_rec(1244, 1), 2);
        // 1244 should be split in 4 after 2 rounds
        assert_eq!(apply_rec(1244, 2), 4);
    }
}
