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

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("\n########################");
    println!("Part 1 answer is {}", res.0);
    println!("Part 2 answer is {}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<u32>,
}

#[derive(Debug, Clone)]
struct Field {
    name: String,
    valid: Vec<(u32, u32)>,
}

impl Field {
    fn new(ln: &str) -> Self {
        let (name, res) = ln.split_once(":").unwrap();
        let (a, b) = res.trim().split_once(" or ").unwrap();
        let (a1, a2) = a.split_once('-').unwrap();
        let (b1, b2) = b.split_once('-').unwrap();
        let valid = vec![
            (a1.parse::<u32>().unwrap(), a2.parse::<u32>().unwrap()),
            (b1.parse::<u32>().unwrap(), b2.parse::<u32>().unwrap()),
        ];
        Field {
            name: name.to_string(),
            valid,
        }
    }

    fn valid(&self, number: u32) -> bool {
        for range in &self.valid {
            if number >= range.0 && number <= range.1 {
                return true;
            }
        }
        false
    }
}

fn read_contents(cont: &str) -> (i64, i64) {
    let mut fields = Vec::new();
    let mut tickets = Vec::new();
    for row in cont.lines() {
        if row.contains(": ") {
            fields.push(Field::new(row));
        } else if row.contains(':') {
            continue;
        } else if row.len() > 2 {
            tickets.push(Ticket {
                values: row
                    .split(',')
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            });
        }
    }
    let myticket = tickets.remove(0);
    let (part1, valid_tickets) = get_part1(&tickets, &fields);
    let part2 = get_part2(&valid_tickets, &fields, myticket);
    (part1, part2)
}

fn get_part1(tickets: &[Ticket], fields: &[Field]) -> (i64, Vec<Ticket>) {
    let mut invalid_sum = 0;
    let mut valids = Vec::new();
    for ticket in tickets {
        let mut ticket_valid = true;
        for value in &ticket.values {
            let mut valid = false;
            for field in fields {
                valid |= field.valid(*value);
            }
            if !valid {
                // This value can't be valid
                ticket_valid = false;
                // Keep track of the sum
                invalid_sum += *value as i64;
            }
        }
        // If the ticket is valid add it to the valid list
        if ticket_valid {
            valids.push(ticket.clone());
        }
    }
    (invalid_sum, valids)
}

fn get_part2(tickets: &[Ticket], fields: &[Field], myticket: Ticket) -> i64 {
    let field_count = tickets[0].values.len();

    // First scan for possible mappings
    let mut possible = BTreeMap::new();
    for field in fields {
        let mut tmp = vec![true; field_count];
        for (i, item) in tmp.iter_mut().enumerate() {
            for ticket in tickets {
                let val = ticket.values[i];
                if !field.valid(val) {
                    *item = false;
                    break;
                }
            }
        }
        possible.insert(
            field.name.clone(),
            tmp.iter()
                .enumerate()
                .filter_map(|(i, v)| if *v { Some(i) } else { None })
                .collect::<BTreeSet<_>>(),
        );
    }

    // Determine the final mapping
    let mut mapping = BTreeMap::new();
    loop {
        let mut done = true;
        for (name, p) in possible.iter() {
            if p.len() == 1 {
                done = false;
                mapping.insert(*p.iter().next().unwrap(), name.clone());
            }
        }
        for ind in mapping.keys() {
            for (_name, p) in possible.iter_mut() {
                p.remove(ind);
            }
        }
        if done {
            break;
        }
    }

    let mut result = 1;
    for (i, f_name) in mapping.iter() {
        if f_name.starts_with("departure") {
            dbg!(&f_name);
            result *= myticket.values[*i] as i64;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(read_contents(&a).0, 71);
    }

    #[test]
    fn part2() {
        let a = "class: 1-3 or 5-7
departure: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(read_contents(&a).1, 7);
    }
}
