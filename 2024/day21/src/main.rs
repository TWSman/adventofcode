use clap::Parser;
use std::fs;
use itertools::Itertools;
use memoize::memoize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}


#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)] 
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
                val => panic!("Unknown character {}", val),
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);

    println!("Part 1 answer is {part1}");
    println!("Part 2 answer is {part2}");
}

fn read_contents(cont: &str) -> (i64, i64) {
    let targets = cont.lines().map(|m| {
        m.chars().collect::<String>()
    }).collect::<Vec<String>>();

    let tmp = targets.iter().map(|seq| {
        (get_numeric(&seq), get_steps(&seq, 3), get_steps(&seq, 26))
    }).collect::<Vec<(i64,i64, i64)>>();
    let part1 = tmp.iter().map(|(x,y,_z)| x * y).sum();
    let part2 = tmp.iter().map(|(x,_y,z)| x * z).sum();
    (part1, part2)
}


// Calculate steps it takes to move from button X to Y
fn get_jump(start: char, end: char, keypad_type: KeypadType) -> String {
    let start_pos = button_lookup(start, keypad_type);
    let end_pos = button_lookup(end, keypad_type);
    let dx = end_pos.0 - start_pos.0;
    let dy = end_pos.1 - start_pos.1;
    let mut tmp_seq = get_tmp_seq(dx,dy);
    tmp_seq.push('A');
    tmp_seq
}

#[memoize]
fn get_jump_len(start: char, end: char, kb: KeypadType, rec: i64) -> i64 {
    // Robot A starts from position 'start' and wants to go to 'end'
    // How many steps does robot N take, to achieve this. Robot N is now starting from A
    let jumps = get_jump(start, end, kb);     
    if rec == 1 {
        // Robot N is the one controlling A
        return jumps.len() as i64
    }
    let mut shortest_sum: Option<i64> = None;

    for perm in jumps.chars().permutations(jumps.len()).unique() {
        if !check_legal(&perm.iter().collect::<String>(), button_lookup(start, kb), button_lookup('#', kb)) {
            continue;
        }
        if perm[perm.len() -1] != 'A' {
            continue;
        }
        let mut sum = 0;
        for j in 0..perm.len() {
            let start_ = if j > 0 {
                perm.get(j-1).unwrap()
            } else {
                &'A'
            };
            let end_ = perm.get(j).unwrap();
            sum += get_jump_len(*start_, *end_, kb, rec-1)
        }
        if shortest_sum.is_none() {
            shortest_sum = Some(sum);
        } else if shortest_sum.unwrap() > sum {
            shortest_sum = Some(sum);
        }
    }
    shortest_sum.unwrap()
}


fn get_tmp_seq(dx: i64, dy: i64) -> String {
    let mut tmp_seq = Vec::new();
    // Usually Its best to go right + down, or up + left
    if dx > 0 {
        // Going right, horizontal first
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
        // Going left, horizontal first
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

    tmp_seq.iter().collect::<String>()
}

fn get_shortest_with_options(seq: &String, keypad_type: KeypadType) -> Vec<String> {
    let mut new_seqs: Vec<String> = Vec::new();
    let mut pos = button_lookup('A', keypad_type);
    let illegal_pos = button_lookup('#', keypad_type);
    new_seqs.push(String::new());
    for i in 0..seq.len() {
        let n_start = new_seqs[0].len();
        let target = seq.chars().nth(i).unwrap(); 
        let target_pos = button_lookup(target, keypad_type);
        let dx = target_pos.0 - pos.0;
        let dy = target_pos.1 - pos.1;

        let tmp_seq = get_tmp_seq(dx, dy);
        //println!("Going from {} to {}", old_pos, target);
        if tmp_seq.len() > 1 {
            let mut x: Vec<String> = Vec::new();
            // Check each permutation
            for perm in tmp_seq.chars().permutations(tmp_seq.len()).unique() {
                let p2 = &perm.iter().copied().collect::<String>();
                // If this permutation is legal
                if check_legal(p2, pos, illegal_pos) {
                    // Copy all old sequences
                    for vec in &new_seqs {
                        let mut xx = vec.clone();
                        for s in &perm {
                            xx.push(*s);
                        }
                        xx.push('A');
                        x.push(xx);
                    }
                }
            }
            new_seqs = x;
        } else {
            for new_seq in new_seqs.iter_mut() {
                for v in tmp_seq.chars() {
                    new_seq.push(v);
                }
                new_seq.push('A');
                assert!(new_seq.len() == 1+ n_start + tmp_seq.len());
            }
        }
        pos = target_pos;
    }
    let n = new_seqs.first().unwrap().len();
    for s in &new_seqs {
        assert!(s.len() == n);
    }
    new_seqs
}

fn check_legal(seq: &String, start: (i64, i64), illegal: (i64, i64)) -> bool {
    let mut pos = start;
    for s in seq.chars() {
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
    true
}


fn get_steps(seq: &String, robot_count: i64) -> i64 {
    println!("{}", seq);
    let mut shortest = None;
    let options = get_shortest_with_options(seq, KeypadType::Numeric);// Sequence that robot B does to move C
    println!("Found {} options", options.len());
    for opt in options {
        let mut steps = 0;
        for j in 0..opt.len() {
            let start_ = if j > 0 {
                opt.chars().nth(j-1).unwrap()
            } else {
                'A'
            };
            let end_ = opt.chars().nth(j).unwrap();
            steps += get_jump_len(start_, end_, KeypadType::Directional, robot_count - 1)
        }
        if shortest.is_none() {
            shortest = Some(steps);
        } else if shortest.unwrap() > steps {
            shortest = Some(steps);
        }
    }
    println!("Found {} steps for {}", shortest.unwrap(), &seq);
    shortest.unwrap()
}

fn get_numeric(input: &str) -> i64 {
    input.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap()
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
    fn steps() {

        let input = String::from("029A");
        assert_eq!(get_steps(&input, 3), 68);

        let inputb = String::from("980A");
        assert_eq!(get_steps(&inputb, 3), 60);

        let inputc= String::from("179A");
        assert_eq!(get_steps(&inputc, 3), 68);

        let input4= String::from("456A");
        assert_eq!(get_steps(&input4, 3), 64);

        let expect5 = String::from("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A");

        let input5 = String::from("379A");
        assert_eq!(get_steps(&input5, 3), 64);
    }

    #[test]
    fn jumps() {
        let kb = KeypadType::Directional;
        assert_eq!(get_jump_len('<', '>', kb, 1), 3);
        assert_eq!(get_jump_len('<', '<', kb, 1), 1);
        assert_eq!(get_jump_len('<', 'A', kb, 1), 4);

        assert_eq!(get_jump_len('A', '^', kb, 2), 8);
    }
}

