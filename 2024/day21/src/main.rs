use clap::Parser;
use std::{fs, collections::BTreeMap};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

struct Status {
    RobotA: char, // RobotA is controlled by you
    RobotB: char, // A controls B
    RobotC: char, // B controls C
}


struct ButtonLookup {
    numeric: BTreeMap<char, (i64,i64)>,
    directional: BTreeMap<char, (i64,i64)>,
}

enum KeypadType {
    Numeric,
    Directional,
}

impl ButtonLookup {
    fn new() -> Self {
        let mut me = ButtonLookup { numeric: BTreeMap::new(), directional: BTreeMap::new() };
        me.numeric.insert('7', (0,0),);
        me.numeric.insert('8', (1,0),);
        me.numeric.insert('9', (2,0),);
        me.numeric.insert('4', (0,1),);
        me.numeric.insert('5', (1,1),);
        me.numeric.insert('6', (2,1),);
        me.numeric.insert('1', (0,2),);
        me.numeric.insert('2', (1,2),);
        me.numeric.insert('3', (2,2),);
        me.numeric.insert('0', (1,3),);
        me.numeric.insert('A', (2,3),);

        // illegal position
        me.numeric.insert('#', (0,3),);

        me.directional.insert('<', (0,1),);
        me.directional.insert('^', (1,0),);
        me.directional.insert('v', (1,1),);
        me.directional.insert('A', (2,0),);
        me.directional.insert('>', (2,1),);
        // illegal position
        me.directional.insert('#', (0,0),);
        me
    }
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let (part1, part2) = read_contents(&contents);
    println!("Part 1 answer is {part1}");
    // 208196 is too high
    // 209880 is too high
    // 212128 is too high
    // 217676
    println!("Part 2 answer is {part2}");
}

fn taxicab(x1: i64, y1: i64, x2: i64, y2: i64) -> i64{
    i64::abs(x1-x2) + i64::abs(y1-y2)
}

fn read_contents(cont: &str) -> (i64, i64) {
    let targets = cont.lines().map(|m| {
        m.chars().collect::<Vec<char>>()
    }).collect::<Vec<_>>();

    let lookup = ButtonLookup::new();
    let tmp = targets.iter().map(|seq| {
        (get_numeric(&seq), get_steps(&seq, &lookup))
    }).collect::<Vec<(i64,i64)>>();
    dbg!(&tmp);
    let part1 = tmp.iter().map(|(x,y)| x * y).sum();
    (part1,0)
}

fn print_seq(seq: &Vec<char>) {
    println!("\n{}", &seq.iter().collect::<String>());
    let str = &seq.iter().collect::<String>();
    for ln in str.split('A') {
        println!("{ln}A");
    }
}

fn get_shortest(seq: &Vec<char>, keypad_type: KeypadType, lookup: &ButtonLookup, leftfirst: bool) -> Vec<char> {
    let mut new_seq = Vec::new();
    let mut pos = match keypad_type {
        KeypadType::Directional => lookup.directional.get(&'A').unwrap(),
        KeypadType::Numeric => lookup.numeric.get(&'A').unwrap(),
    };
    let mut old_pos = 'A';
    let illegal_pos = match keypad_type {
        KeypadType::Directional => lookup.directional.get(&'#').unwrap(),
        KeypadType::Numeric => lookup.numeric.get(&'#').unwrap(),
    };
    for i in 0..seq.len() {
        let mut tmp_seq: Vec<char> = Vec::new();
        let target = seq[i]; 
        let target_pos = match keypad_type {
            KeypadType::Directional => lookup.directional.get(&target).unwrap(),
            KeypadType::Numeric => lookup.numeric.get(&target).unwrap(),
        };
        let mut nowleft = leftfirst;
        if pos.1 == illegal_pos.1 { // We are on the same row
            if target_pos.0 == illegal_pos.0 { // We are heading to the illegal position
                nowleft = false;
            }
        } 
        let dx = target_pos.0 - pos.0;
        let dy = target_pos.1 - pos.1;
        // Its best to right + down, or up + left
        // Not down + right or left + up
        //
        // right/up and down/left, order does not matter
        //
        // going right, horizontal first
        if dx > 0 {
            for _ in 0..dx {
                // dx steps right
                new_seq.push('>');
                tmp_seq.push('>');
            }
            if dy > 0 {
                for _ in 0..dy {
                    // dy steps down
                    new_seq.push('v');
                    tmp_seq.push('v');
                }
            } else {
                for _ in 0..-dy {
                    // -dy steps up
                    new_seq.push('^');
                    tmp_seq.push('^');
                }
            }
        }
        else {
            // Going left
            // Vertical should come first
            if nowleft {
                for _ in 0..-dx {
                    new_seq.push('<');
                    tmp_seq.push('<');
                }
            }
            if dy > 0 {
                for _ in 0..dy {
                    // dy steps down
                    new_seq.push('v');
                    tmp_seq.push('v');
                }
            } else {
                for _ in 0..-dy {
                    // dy steps up
                    new_seq.push('^');
                    tmp_seq.push('^');
                }
            }

            if !nowleft {
                for _ in 0..-dx {
                    new_seq.push('<');
                    tmp_seq.push('<');
                }
            }
        }
        new_seq.push('A');
        tmp_seq.push('A');

        // println!("Going from {old_pos} to {target}");
        // dbg!(&tmp_seq);
        // dbg!(&new_seq);
        
        old_pos = target;
        pos = target_pos;
    }
    

    new_seq

}

fn get_steps(seq: &Vec<char>, lookup: &ButtonLookup) -> i64 {
    //println!("{}", &seq.iter().collect::<String>());
    //print_seq(&seq);
    let mut shortest = 999;
    for i in 0..8 {
        let seq_b = get_shortest(seq, KeypadType::Numeric, lookup, i%2 == 0);// Sequence that robot B does to move C
                
        //print_seq(&seq_b);
        let seq_a = get_shortest(&seq_b, KeypadType::Directional, lookup, (i / 2) % 2 == 0);// Sequence that robot A does to move
                                                                  // B
        //println!("{}", &seq_a.iter().collect::<String>());
        //print_seq(&seq_a);
        let seq_you = get_shortest(&seq_a, KeypadType::Directional, lookup, (i / 4) % 2 == 0);// Sequence that you do to control
                                                                    // robot A
        //print_seq(&seq_you);
        if seq_you.len() < shortest {
            shortest = seq_you.len();
        }
    }
    println!("Found {} steps for {}", shortest, &seq.iter().collect::<String>());
    //println!("{}", 
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
        let lookup = ButtonLookup::new();

        let input = vec!['0', '2', '9', 'A'];
        let expect = vec!['<', 'A', '^', 'A', '>', '^', '^', 'A', 'v', 'v', 'v', 'A'];
        assert_eq!(get_shortest(&input, KeypadType::Numeric, &lookup, false), expect);

        let expect_next = String::from("v<<A>>^A<A>AvA<^AA>A<vAAA>^A").chars().collect::<Vec<char>>();
        assert_eq!(get_shortest(&expect, KeypadType::Directional, &lookup, false).len(), expect_next.len());
        
        let expect_third = String::from("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A").chars().collect::<Vec<char>>();
        let third = get_shortest(&expect_next, KeypadType::Directional, &lookup, false);
        //print_seq(&third);
        //print_seq(&expect_third);
        assert_eq!(third.len(), expect_third.len());


        // Fiffth example
        //
        // Expected sequences should be:
        // 379A
        // ^A<<^^A>>AvvvA
        // <A>Av<<AA>^AA>AvAA^A<vAAA>^A
        // <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A


        // Get sequences
        // ^A^^<<A>>AvvvA
        // <A>A<AAv<AA>>^AvAA^Av<AAA>^A
        // v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
        //
        let input5 = vec!['3', '7', '9', 'A']; // Given input
        let expect5c = String::from("^A<<^^A>>AvvvA").chars().collect::<Vec<char>>(); // Result in
                                                                                      // example

        let res = get_shortest(&input5, KeypadType::Numeric, &lookup, true);
        assert_eq!(res.len(), expect5c.len());
        //assert_eq!(res, expect5c);

        let expect5b  = String::from("<A>Av<<AA>^AA>AvAA^A<vAAA>^A").chars().collect::<Vec<char>>(); // Result in example

        let res2 = get_shortest(&expect5c, KeypadType::Directional, &lookup, true);
        let res22 = get_shortest(&res, KeypadType::Directional, &lookup, true);
        assert_eq!(res22.len(), res2.len());
        //assert_eq!(res2, expect5b);

        let expect5a = String::from("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A").chars().collect::<Vec<char>>(); // result in example
        //print_seq(&expect4);
        //print_seq(&expect5);
        let res3 = get_shortest(&expect5b, KeypadType::Directional, &lookup, true);
        let res31 = get_shortest(&res2, KeypadType::Directional, &lookup, true);
        let res32 = get_shortest(&res22, KeypadType::Directional, &lookup, true);
        assert_eq!(res3.len(), expect5a.len());
        assert_eq!(res3.len(), res31.len());
        // assert_eq!(res3.len(), res32.len());
    }

    #[test]
    fn steps() {
        let lookup = ButtonLookup::new();

        let input = vec!['0', '2', '9', 'A'];
        assert_eq!(get_steps(&input, &lookup), 68);

        let inputb = vec!['9', '8', '0', 'A'];
        assert_eq!(get_steps(&inputb, &lookup), 60);

        let inputc = vec!['1', '7', '9', 'A'];
        assert_eq!(get_steps(&inputc, &lookup), 68);

        let input4 = vec!['4', '5', '6', 'A'];
        assert_eq!(get_steps(&input4, &lookup), 64);

        let expect5 = String::from("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A").chars().collect::<Vec<char>>();
        print_seq(&expect5);
        let input5 = vec!['3', '7', '9', 'A'];
        assert_eq!(get_steps(&input5, &lookup), 64);
    }

        // Expected
        // ^A<<^^A>>AvvvA
        // <A>Av<<AA>^AA>AvAA^A<vAAA>^A
        //
        // <A
        // >A   
        // ^A   results in 3
        //
        // v<<A    <     // 4   // <vA, <A <A >>A //  8
        // A       <     // 1   // A              //  1
        // >^A     ^     // 3   // vA, ^<A, >A    //  7
        // A       ^     // 1   // A              //  1
        // >A      A     // 2   // vA ^A          //  4 Left first
        // <<^^A results in 7
        //
        // vA
        // A
        // ^A   9
        //
        // <vA
        // A
        // A
        // >^A  A
        //
        // I get
        // ^A^^<<A>>AvvvA
        // <A>A<AAv<AA>>^AvAA^Av<AAA>^A
        //
        // <A
        // >A    3
        //
        // <A    ^         // 2    <A >A          // 4
        // A     ^         // 1    A              // 1
        // v<A   <         // 3    v<A, <A, >^A   // 8
        // A     <         // 1    A              // 1
        // >>^A  A         // 4    vA, A, ^<A >A  // 8      Up first
        // ^^<<A  results in 7
        //
        // vA
        // A
        // ^A    9
        //
        // v<A
        // A
        // A
        // >^A    A
        // Lead to different number of steps
}

