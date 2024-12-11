use std::iter::successors;
use memoize::memoize;

fn main() {
    let input = "77 515 6779622 6 91370 959685 0 9861";
    let (part1, part2) = read_contents(&input);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn read_input(cont: &str) -> Vec<u64> {
    let nums: Vec<u64> = cont.split_whitespace().map(|m| {m.parse::<u64>().unwrap() }).collect();
    nums
}


fn read_contents(cont: &str) -> (u64, u64) {
    let input = read_input(cont);
    (get_part1(&input), get_part2(&input))
}


fn one_val(input: u64, iter: usize) -> Vec<u64> {
    let mut output = vec![input];
    for j in 0..iter {
        println!("j: {j} / {iter}");
        output = output.iter().enumerate().flat_map(|(i,m)| {
            //if i % 1000 == 0 {
                //println!("i: {i}");
            //}
            apply(*m)
        }
            ).collect();
    }
    output
}

#[memoize]
fn apply(input: u64) -> Vec<u64> {
    if input == 0 {
        return vec![1];
    }
    let siz = successors(Some(input), |&n| (n >= 10).then_some(n / 10)).count() as u32;
    if siz % 2 == 0 {
        let spl = siz / 2;
        return vec![input / 10_u64.pow(spl), input % 10_u64.pow(spl)]
    }
    vec![input * 2024]
}

fn get_part1(input: &Vec<u64>) -> u64 {
    input.iter().flat_map(|i| {
        one_val(*i, 25)
    }).count() as u64
}

fn get_part2(input: &Vec<u64>) -> u64 {
    input.iter().flat_map(|i| {
        one_val(*i, 75)
    }).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let a = "125 17";
        assert_eq!(read_contents(&a).0, 55312);
        assert_eq!(read_contents(&a).1, 81);
    }


    #[test]
    fn apply2() {
        assert_eq!(apply(0), vec![1]);
        assert_eq!(apply(1), vec![2024]);
        assert_eq!(apply(11), vec![1,1]);
        assert_eq!(apply(12), vec![1,2]);
        assert_eq!(apply(1244), vec![12,44]);
    }

}
