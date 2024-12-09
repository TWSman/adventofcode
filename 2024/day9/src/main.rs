use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents, false);
    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn read_contents(cont: &str, debug: bool) -> (u64, u64) {
    let file_sys = read_input(cont);
    print_file_sys(&file_sys);
    let part1 =  get_part1(&file_sys, debug);
    // Use same file sys representation for part2
    // Part2 would probably work better with another representation
    let part2 =  get_part2(&file_sys, debug);
    (part1 , part2)
}

fn print_file_sys(file_sys: &[Option<u64>]) {
    println!("{}", file_sys.iter().map(|m| {
        m.as_ref().map_or('.', |val| char::from_digit(u32::try_from(*val).unwrap() % 36, 36).unwrap_or('#'))
    }).collect::<String>());
}

fn get_part1(file_sys: &[Option<u64>], debug: bool) -> u64 {
    let mut new_file_sys = file_sys.to_owned();
    let empty_blocks: Vec<usize> = file_sys.iter().enumerate().filter_map(|(i,m)| {
        if m.is_none() {
            Some(i)
        } else {
            None
        }
    }).collect();
    let mut file_id: u64;
    for i_empty in empty_blocks {
        if i_empty >= new_file_sys.len() {
            break;
        }
        if debug {
            dbg!(&i_empty);
            dbg!(&new_file_sys.len());
        }
        loop {
            if i_empty >= new_file_sys.len() {
                break
            }
            match new_file_sys.pop().unwrap() {
                None => (),
                Some(val) => {
                    file_id = val;
                    new_file_sys[i_empty] = Some(file_id);
                    if debug {
                        print_file_sys(&new_file_sys);
                    }
                    break;
                }
            };
        }
    }
    if debug {
        print_file_sys(&new_file_sys);
    }
    get_checksum(&new_file_sys)
}

fn get_part2(file_sys: &[Option<u64>], debug: bool) -> u64 {
    let mut new_file_sys = file_sys.to_owned();
    let max_file_id = new_file_sys.last().unwrap().unwrap();
    println!("Final file id is {max_file_id}");
    if debug {
        print_file_sys(&new_file_sys);
    }
    for file_id in (1..=max_file_id).rev() {
        if file_id % 100 == 0 {
            dbg!(&file_id);
        }
        let mut indices: Vec<usize> = Vec::new();
        for (i,f) in new_file_sys.iter().enumerate() {
            if *f == Some(file_id) {
                indices.push(i);
            } else if !indices.is_empty() {
                break;
            }
        }
        let count = indices.len();
        let end = indices.first().unwrap() - 1;
        let mut found = 0;
        for i in 0..=end {
            if new_file_sys.get(i).unwrap().is_some() {
                // This position is full
                found = 0;
                continue;
            }
            // This position is empty
            found += 1;

            if found >= count {
                // Block is long enough
                for (j, ind) in indices.iter().enumerate() {
                    new_file_sys[i - j] = Some(file_id);
                    new_file_sys[*ind] = None;
                }
                break;
            }
        }
        if debug {
            print_file_sys(&new_file_sys);
        }
    }
    print_file_sys(&new_file_sys);
    get_checksum(&new_file_sys)
}

fn get_checksum(file_sys: &[Option<u64>]) -> u64{
    file_sys.iter().enumerate().map(|(i,m)| {
        m.as_ref().map_or(0, |val| i as u64 * val)
    }).sum()
}

fn read_input(cont: &str) -> Vec<Option<u64>> {
    cont.chars().enumerate().flat_map(|(i,m)| {
        let file_ind = (i / 2) as u64;
        m.to_digit(10).map_or_else(|| vec![None; 0], |c|
            if i % 2 == 0 {
                vec![Some(file_ind); c as usize]
            } else {
                vec![None; c as usize]
            })
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let a = "2333133121414131402";
        assert_eq!(read_contents(&a, true).0, 1928);
        assert_eq!(read_contents(&a, true).1, 2858);
    }

    #[test]
    fn full() {
        // Take the start of main puzzle input
        let a = "7148927114311312621";
        assert_eq!(read_contents(&a, true).0, 2824);
        assert_eq!(read_contents(&a, true).1, 2914);

        let a = "714892711431131262132720221453126624699";
        assert_eq!(read_contents(&a, true).0, 30319);
        assert_eq!(read_contents(&a, true).1, 45807);
    }

}
