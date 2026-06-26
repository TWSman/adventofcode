use clap::Parser;
use std::collections::BTreeMap;
use std::fs;
use std::str;
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
    // 289 is too high
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

#[derive(Debug, Clone)]
struct Rule {
    id: usize,
    c: Option<char>, // Some rules just match a single char
    rules: Vec<Vec<usize>>,
}

impl Rule {
    fn new(ln: &str) -> Self {
        let (name, res) = ln.split_once(":").unwrap();
        let mut rules = Vec::new();
        let c = if res.contains('"') {
            Some(res.trim().chars().nth(1).unwrap())
        } else {
            for spl in res.split('|') {
                let mut rule = Vec::new();
                for v in spl.split_whitespace() {
                    rule.push(v.trim().parse::<usize>().unwrap());
                }
                rules.push(rule);
            }
            None
        };
        Rule {
            id: name.parse::<usize>().unwrap(),
            c,
            rules,
        }
    }
}

fn read_input(cont: &str) -> (BTreeMap<usize, Rule>, Vec<String>) {
    let mut rules = BTreeMap::new();
    let mut words = Vec::new();
    for row in cont.lines() {
        if row.contains(": ") {
            let rule = Rule::new(row);
            rules.insert(rule.id, rule);
        } else if !row.is_empty() {
            words.push(row.to_string());
        }
    }
    (rules, words)
}

fn read_contents(cont: &str) -> (i64, i64) {
    let (rules, words) = read_input(cont);
    let part1 = get_part1(&words, &rules);
    let part2 = get_part2(&words, &rules);
    (part1, part2)
}

fn get_part1(words: &[String], rules: &BTreeMap<usize, Rule>) -> i64 {
    let potential = get_strings(0, rules);
    let mut count = 0;
    for pot in potential {
        if words.contains(&pot) {
            count += 1;
        }
    }
    count as i64
}

fn get_strings(rule_id: usize, rules: &BTreeMap<usize, Rule>) -> Vec<String> {
    let mut output = Vec::new();
    let rule = rules.get(&rule_id).unwrap();
    if let Some(c) = rule.c {
        output.push(c.to_string());
    } else {
        for option in &rule.rules {
            let mut strs = vec!["".to_string()];
            for r in option.iter() {
                let strings = get_strings(*r, rules);
                let mut new_strs = Vec::new();
                for s1 in &strs {
                    for s2 in &strings {
                        new_strs.push(format!("{}{}", s1, s2));
                    }
                }
                strs = new_strs;
            }
            for s in &strs {
                output.push(s.clone());
            }
        }
    }
    output
}

fn get_part2(words: &[String], rules: &BTreeMap<usize, Rule>) -> i64 {
    if !rules.contains_key(&42) {
        return 0;
    }
    // This logic assumes that rule 0 is 0: 8 11
    assert_eq!(rules.get(&0).unwrap().rules, vec![vec![8, 11]]);

    // Rule 8 is: 42 | 42 8
    // Rule 11 is: 42 31 | 42 11 31
    //
    // Target is to find matches for rule 0 i.e.
    // 0: 8 11
    // i.e. (42 | 42 8) (42 31 | 42 11 31)
    //
    // 1,1: 42 42 31 // No recursion
    // 2,1: 42 42 42 31 // 1 level of recursion for rule 8
    // 1,2: 42 42 42 31 31 // 1 level of recursion for rule 11
    // 2,2: 42 42 42 42 31 31 // 1 level of recursion for both rules
    //
    // Thus every match will start with
    // n + m  units of 42
    // And end with
    // m units of 31,
    // where m >= 1, n >= 1
    // Total length will be n * l + 2m * l

    let potential42 = get_strings(42, rules);
    let potential31 = get_strings(31, rules);
    let len_min = potential42
        .iter()
        .map(|l| l.len())
        .min()
        .unwrap()
        .min(potential31.iter().map(|l| l.len()).min().unwrap());
    let len_max = potential42
        .iter()
        .map(|l| l.len())
        .max()
        .unwrap()
        .max(potential31.iter().map(|l| l.len()).max().unwrap());

    // Every potential match for rules 42 and 31 seems to have the exact same length
    // Both in the example and for actual input
    assert_eq!(len_min, len_max);

    let mut count = 0;
    for word in words {
        let mut matches = false;
        // Singe every match for rules 42 and 31 has the same length we can split the word into
        // equal length segment
        let segments = word
            .as_bytes()
            .chunks(len_min)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();

        let segment_count = segments.len();

        for n in 1..=segment_count {
            for m in 1..=segment_count {
                if n + 2 * m > segment_count {
                    break;
                }
                if matches || n + 2 * m != segment_count {
                    continue;
                }
                let mut valid = true;
                for seg in segments.iter().take(n) {
                    if !potential42.contains(&seg.to_string()) {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    continue;
                }
                for j in 0..m {
                    if !potential42.contains(&segments[n + j].to_string()) {
                        valid = false;
                        break;
                    }
                    if !potential31.contains(&segments[n + m + j].to_string()) {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    matches = true;
                }
            }
        }
        if matches {
            count += 1;
        }
    }
    count as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        assert_eq!(read_contents(&a).0, 2);
    }

    #[test]
    fn rule() {
        let a = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        let (rules, words) = read_input(&a);
        dbg!(&rules);
        assert_eq!(get_strings(4, &rules), ["a"]);

        assert_eq!(get_strings(2, &rules), ["aa", "bb"]);
        assert_eq!(
            get_strings(1, &rules),
            [
                "aaab", "aaba", "bbab", "bbba", "abaa", "abbb", "baaa", "babb",
            ]
        );

        assert_eq!(
            get_strings(0, &rules),
            [
                "aaaabb", "aaabab", "abbabb", "abbbab", "aabaab", "aabbbb", "abaaab", "ababbb",
            ]
        );
    }

    #[test]
    fn part2() {
        let a = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        assert_eq!(read_contents(&a).0, 3);
        assert_eq!(read_contents(&a).1, 12);

        // abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa DOES NOT match
        // bbabbbbaabaabba matches
        // babbbbaabbbbbabbbbbbaabaaabaaa matches
        // aaabbbbbbaaaabaababaabababbabaaabbababababaaa matches
        // bbbbbbbaaaabbbbaaabbabaaa matches
        // bbbababbbbaaaaaaaabbababaaababaabab matches
        // ababaaaaaabaaab matches
        // ababaaaaabbbaba matches
        // baabbaaaabbaaaababbaababb matches
        // abbbbabbbbaaaababbbbbbaaaababb matches
        // aaaaabbaabaaaaababaa matches
        // aaaabbaaaabbaaa DOES NOT match
        // aaaabbaabbaaaaaaabbbabbbaaabbaabaaa matches
        // babaaabbbaaabaababbaabababaaab DOES NOT match
        // aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba matches
    }
}
