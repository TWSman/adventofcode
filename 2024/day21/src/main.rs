use clap::Parser;
use std::{fs, collections::BTreeMap};
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Clone, Copy, Debug)] 
enum KeypadType {
    Numeric,
    Directional,
}

fn button_lookup(c: char, keypad_type: KeypadType) -> (i64, i64) {
    match keypad_type {
        KeypadType::Numeric => {
            match c  {
                '7'=> (0,0),
                '8'=> (1,0),
                '9'=> (2,0),
                '4'=> (0,1),
                '5'=> (1,1),
                '6'=> (2,1),
                '1'=> (0,2),
                '2'=> (1,2),
                '3'=> (2,2),
                '0'=> (1,3),
                'A'=> (2,3),
                '#' => (0,3),
                _ => panic!(""),
            }
        }
        KeypadType::Directional => {
            match c {
                '<' => (0,1),
                '^' => (1,0),
                'v' => (1,1),
                'A' => (2,0),
                '>' => (2,1),
                '#' => (0,0),
                    _ => panic!(""),
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);

    println!("Part 1 answer is {part1}");
    assert!(part1 == 202648);
    println!("Part 2 answer is {part2}");
}

fn read_contents(cont: &str) -> (i64, i64) {
    let targets = cont.lines().map(|m| {
        m.chars().collect::<Vec<char>>()
    }).collect::<Vec<_>>();

    let tmp = targets.iter().map(|seq| {
        (get_numeric(&seq), get_steps(&seq, 3), get_steps(&seq, 3))
    }).collect::<Vec<(i64,i64, i64)>>();
    let part1 = tmp.iter().map(|(x,y,_z)| x * y).sum();
    let part2 = tmp.iter().map(|(x,_y,z)| x * z).sum();
    (part1, part2)
}

fn seq_to_string(seq: &Vec<char>) -> String {
    seq.iter().collect::<String>()
}

fn print_seq(seq: &Vec<char>) {
    let str = seq_to_string(&seq);
    println!("{}", str);
}

fn get_shortest(seq: &Vec<char>, keypad_type: KeypadType) -> Vec<char> {
    let mut new_seq = Vec::new();
    let mut pos = button_lookup('A', keypad_type);
    let illegal_pos = button_lookup('#', keypad_type);
    let mut _old_pos = 'A';
    for i in 0..seq.len() {
        let target = seq[i]; 
        let target_pos = button_lookup(target, keypad_type);
        let dx = target_pos.0 - pos.0;
        let dy = target_pos.1 - pos.1;

        let mut tmp_seq = get_tmp_seq(dx, dy);
        tmp_seq.push('A');
        for v in tmp_seq {
            new_seq.push(v);
        }
        _old_pos = target;
        pos = target_pos;
    }
    

    new_seq

}

fn get_tmp_seq(dx: i64, dy: i64) -> Vec<char>{
    let mut tmp_seq = Vec::new();
    // Its best to go right + down, or up + left
    // Not down + right or left + up
    //
    // right/up and down/left, order does not matter
    //
    // going right, horizontal first
    if dx > 0 {
        for _ in 0..dx {
            // dx steps right
            tmp_seq.push('>');
        }
        if dy > 0 {
            for _ in 0..dy {
                // dy steps down
                tmp_seq.push('v');
            }
        } else {
            for _ in 0..-dy {
                // -dy steps up
                tmp_seq.push('^');
            }
        }
    } else {
        if dy > 0 {
            for _ in 0..dy {
                // dy steps down
                tmp_seq.push('v');
            }
        } else {
            for _ in 0..-dy {
                // dy steps up
                tmp_seq.push('^');
            }
        }
        for _ in 0..-dx {
            // dx steps left
            tmp_seq.push('<');
        }
    }

    tmp_seq
}

fn get_shortest_with_options(seq: &Vec<char>, keypad_type: KeypadType) -> Vec<Vec<char>> {
    let mut new_seqs: Vec<Vec<char>> = Vec::new();
    let mut pos = button_lookup('A', keypad_type);
    let illegal_pos = button_lookup('#', keypad_type);
    let mut old_pos = 'A';
    new_seqs.push(Vec::new());
    for i in 0..seq.len() {
        let n_start = new_seqs[0].len();
        let target = seq[i]; 
        let target_pos = button_lookup(target, keypad_type);
        let mut nowleft = true;
        if pos.1 == illegal_pos.1 { // We are on the same row
            if target_pos.0 == illegal_pos.0 { // We are heading to the illegal position
                nowleft = true;
            }
        } 
        let dx = target_pos.0 - pos.0;
        let dy = target_pos.1 - pos.1;

        let tmp_seq = get_tmp_seq(dx, dy);
        //println!("Going from {} to {}", old_pos, target);
        if tmp_seq.len() > 1 {
            let mut x: Vec<Vec<char>> = Vec::new();
            // Check each permutation
            for perm in tmp_seq.iter().permutations(tmp_seq.len()).unique() {
                let p2 = &perm.iter().map(|c| **c).collect::<Vec<char>>();
                // If this permutation is legal
                if check_legal(p2, pos, illegal_pos) {
                    // Copy all old sequences
                    for vec in &new_seqs {
                        let mut xx = vec.clone();
                        for s in &perm {
                            xx.push(**s);
                        }
                        xx.push('A');
                        x.push(xx);
                    }
                }
            }
            new_seqs = x;
        } else {
            for new_seq in new_seqs.iter_mut() {
                for v in &tmp_seq {
                    new_seq.push(*v);
                }
                new_seq.push('A');
                assert!(new_seq.len() == 1+ n_start + tmp_seq.len());
            }
        }
        old_pos = target;
        pos = target_pos;
    }
    let n = new_seqs.first().unwrap().len();
    for s in &new_seqs {
        assert!(s.len() == n);
    }
    new_seqs
}

fn check_legal(seq: &Vec<char>, start: (i64, i64), illegal: (i64, i64)) -> bool {
    let mut pos = start;
    for s in seq {
        pos = match s {
            '>' => (pos.0 + 1, pos.1),
            '<' => (pos.0 - 1, pos.1),
            '^' => (pos.0, pos.1 - 1),
            'v' => (pos.0, pos.1 + 1),
            'A' => return true,
            _ => panic!(),
        };
        if pos == illegal {
            return false;
        }
    }
    return true
}

fn get_steps(seq: &Vec<char>, robot_count: i64) -> i64 {
    print_seq(&seq);
    let mut shortest = 999;
    let options = get_shortest_with_options(seq, KeypadType::Numeric);// Sequence that robot B does to move C
    println!("Found {} options", options.len());
    for opt in options {
        let mut new_seq = opt.clone();
        let mut i = 1;
        loop {
            dbg!(&i);
            i += 1;
            new_seq = get_shortest(&new_seq, KeypadType::Directional);
            if i == robot_count {
                println!("{} steps for {}", new_seq.len(), seq_to_string(&opt));
                if new_seq.len() < shortest {
                    shortest = new_seq.len();
                }
                break;
            }
        }
    }
    println!("Found {} steps for {}", shortest, &seq.iter().collect::<String>());
    shortest as i64
}

fn get_numeric(input: &Vec<char>) -> i64 {
    input.iter().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example() {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!(read_contents(&input).0, 126384);

    }

    #[test]
    fn shortest() {

        let input = vec!['0', '2', '9', 'A'];
        let expect = vec!['<', 'A', '^', 'A', '>', '^', '^', 'A', 'v', 'v', 'v', 'A'];
        assert_eq!(get_shortest(&input, KeypadType::Numeric), expect);

        let expect_next = String::from("v<<A>>^A<A>AvA<^AA>A<vAAA>^A").chars().collect::<Vec<char>>();
        let second = get_shortest(&expect, KeypadType::Directional);
        assert_eq!(expect_next.len(), second.len());
        
        let expect_third = String::from("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A").chars().collect::<Vec<char>>();
        let third = get_shortest(&expect_next, KeypadType::Directional);
        assert_eq!(third.len(), expect_third.len());


        // Fiffth example
        //
        // Expected sequences should be:
        // 379A
        // ^A<<^^A>>AvvvA
        // <A>Av<<AA>^AA>AvAA^A<vAAA>^A
        // <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

        let input5 = vec!['3', '7', '9', 'A']; // Given input
        let expect5c = String::from("^A<<^^A>>AvvvA").chars().collect::<Vec<char>>(); // Result in
                                                                                      // example

        let expect5b  = String::from("<A>Av<<AA>^AA>AvAA^A<vAAA>^A").chars().collect::<Vec<char>>(); // Result in example

        let res2 = get_shortest(&expect5c, KeypadType::Directional);
        //assert_eq!(res2, expect5b);

        let expect5a = String::from("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A").chars().collect::<Vec<char>>(); // result in example
        let res3 = get_shortest(&expect5b, KeypadType::Directional);
        let res31 = get_shortest(&res2, KeypadType::Directional);
        assert_eq!(res3.len(), expect5a.len());
        assert_eq!(res3.len(), res31.len());
    }

    #[test]
    fn steps() {

        let input = vec!['0', '2', '9', 'A'];
        assert_eq!(get_steps(&input, 3), 68);

        let inputb = vec!['9', '8', '0', 'A'];
        assert_eq!(get_steps(&inputb, 3), 60);

        let inputc = vec!['1', '7', '9', 'A'];
        assert_eq!(get_steps(&inputc, 3), 68);

        let input4 = vec!['4', '5', '6', 'A'];
        assert_eq!(get_steps(&input4, 3), 64);

        let expect5 = String::from("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A").chars().collect::<Vec<char>>();
        print_seq(&expect5);
        let input5 = vec!['3', '7', '9', 'A'];
        assert_eq!(get_steps(&input5, 3), 64);
    }
}

