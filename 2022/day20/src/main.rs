use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}


fn read_contents(cont: &str) -> (i64, i64) {
    let list = cont.lines().map(|v| {
        v.parse::<i64>().unwrap()
    }).collect::<Vec<i64>>();

    let part1 = get_part1(&list);
    let part2 = get_part2(&list);
    (part1, part2)
}


fn get_score(list: &[i64]) -> i64 {
    // Scoring: find the index of 0, then sum the values at index +1000, +2000, +3000 (mod length)
    let n = list.len();
    let startind = list.iter().enumerate().find(|(_i, v)| **v == 0).unwrap().0;
    list[(startind + 1000) % n] + list[(startind + 2000) % n] + list[(startind + 3000) % n]
}

fn get_part1(list: &[i64]) -> i64 {
    let new_list = reorder(list, 1, None);
    get_score(&new_list)
}

fn reorder(list: &[i64], repeats: usize, loops: Option<usize>) -> Vec<i64> {
    let n = list.len();
    // Ind holds the current indices of the original list 
    // (index i of ind, gives the current location of index i in the original list)
    let mut ind = (0..n).collect::<Vec<usize>>();
    let max_i = loops.unwrap_or(n);
    for _ in 0..repeats {
        for i in 0..max_i {
            let val = list[i]; // Value at original index i
            let mov = val % (n as i64 - 1); // Move of (n-1) is the same as no move at all
            if mov == 0 {
                // 0 does not move
                continue;
            }
            let current_ind = ind[i]; // Current index of value i
            let mut new_ind = current_ind as i64 + mov;
            if new_ind <= 0 && mov != 0 {
                // If a negative move would move to first of list, it is now the last position
                new_ind += -1 + n as i64;
            }
            if new_ind > n as i64 - 1 {
                // If a positive move would to last of list, it is now the first position
                new_ind -= -1 + n as i64;
            }
            assert!(new_ind >= 0);
            assert!(new_ind < n as i64);
            let new_ind = new_ind as usize;

            // Remove this index, which means that all indices larger than current index decrease by 1
            for (j, v) in ind.iter_mut().enumerate() {
                if j == i {
                    continue;
                }
                if *v > current_ind {
                    *v -= 1;
                }
            }
            ind[i] = new_ind;
            for (j, v) in ind.iter_mut().enumerate() {
                if j == i {
                    continue;
                }
                if *v >= new_ind {
                    *v += 1;
                }
            }
        }
    }
    let mut new_order = vec![0; n];
    for i in 0..n {
        new_order[ind[i]] = list[i];
    }
    new_order
}

fn get_part2(list: &[i64]) -> i64 {
    let decryption_key = 811589153;
    let new_list = list.iter().map(|v| v * decryption_key).collect::<Vec<i64>>();
    // In part2 the list should be reordered 10 times
    let new_order = reorder(&new_list, 10, None);
    get_score(&new_order)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let list = vec![1, 2, -3, 3, -2, 0, 4];
        // 1 moves from ind 0 to ind 1: [2, 1, -3, 3, -2, 0, 4]
        assert_eq!(reorder(&list, 1, Some(1)), vec![2, 1, -3, 3, -2, 0, 4]);
        // 2 moves from ind 0 to ind 2: [1, -3, 2, 3, -2, 0, 4]
        assert_eq!(reorder(&list, 1, Some(2)), vec![1, -3, 2, 3, -2, 0, 4]);
        // -3 moves from ind 1 to ind -3, i.e. 4
        assert_eq!(reorder(&list, 1, Some(3)), vec![1, 2, 3, -2, -3, 0, 4]);
        // 3 moves from ind 2 to ind 5
        assert_eq!(reorder(&list, 1, Some(4)), vec![1, 2, -2, -3, 0, 3, 4]);
        // -2 moves from ind 2 to ind -1, i.e. 6
        assert_eq!(reorder(&list, 1, Some(5)), vec![1, 2, -3, 0, 3, 4, -2]);
        // 0 does not move
        assert_eq!(reorder(&list, 1, Some(6)), vec![1, 2, -3, 0, 3, 4, -2]);
        // 4 moves from ind 5 to ind 3 i.e. 10
        assert_eq!(reorder(&list, 1, Some(7)), vec![1, 2, -3, 4, 0, 3, -2]);

        let a ="1
2
-3
3
-2
0
4";
        assert_eq!(read_contents(&a).0, 3);
    }
    #[test]
    fn part2() {
        let a ="1
2
-3
3
-2
0
4";
        assert_eq!(read_contents(&a).1, 1623178306);
    
    }
}

// What happens to 6
//[  1,   2,  -3, _6_,  -2,   0,  4]; ind3
//[  1,   2,  -3,  -2, _5_,   0,  4]; // First move, ind is 4
//[  1,   2,  -3,  -2,   0, _4_,  4]; // Second move, ind is 5
//[_3_,   1,   2,  -3,  -2,   0,  4]; // third move, ind is 6, but moves to 0
//[  1, _2_,   2,  -3,  -2,   0,  4]; // fourth move, ind is 1
//[  1,   2, _1_,  -3,  -2,   0,  4]; // fifth move, ind is 2
//[  1,   2,  -3, _0_,  -2,   0,  4]; // last move, ind is 3
