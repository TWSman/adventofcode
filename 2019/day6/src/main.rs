use clap::Parser;
use std::collections::BTreeMap;
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

fn read_contents(cont: &str) -> (i64, i64) {
    let constellation = Constellation::new(cont);

    let part1 = constellation.get_total_orbits();
    let part2 = constellation.get_part2();
    (part1, part2)
}

#[derive(Debug)]
struct Connection {
    center: String,
    satellite: String,
}

impl Connection {
    fn new(ln: &str) -> Self {
        if let Some((a, b)) = ln.split_once(')') {
            Self {
                center: a.to_string(),
                satellite: b.to_string(),
            }
        } else {
            panic!("Invalid connection line: {}", ln);
        }
    }
}

fn read_connections(cont: &str) -> Vec<Connection> {
    cont.lines().map(Connection::new).collect()
}

fn get_map(connections: &Vec<Connection>) -> BTreeMap<String, Vec<String>> {
    let mut map = BTreeMap::new();
    for conn in connections {
        map.entry(conn.center.clone())
            .or_insert_with(Vec::new)
            .push(conn.satellite.clone());
    }
    map
}

fn get_map_inverted(connections: &Vec<Connection>) -> BTreeMap<String, String> {
    let mut map_invert = BTreeMap::new();
    for conn in connections {
        assert!(!map_invert.contains_key(&conn.satellite));
        map_invert.insert(conn.satellite.clone(), conn.center.clone());
    }
    map_invert
}

struct Constellation {
    map: BTreeMap<String, Vec<String>>,
    map_invert: BTreeMap<String, String>, // satellite -> center
    orbits_around: BTreeMap<String, i64>,
}

impl Constellation {
    fn new(cont: &str) -> Self {
        let connections = read_connections(cont);
        let map = get_map(&connections);
        let inverted = get_map_inverted(&connections);
        let mut constellation = Self {
            map,
            map_invert: inverted,
            orbits_around: BTreeMap::new(),
        };
        let keys = constellation.map.keys().cloned().collect::<Vec<String>>();
        for node in keys {
            let count = constellation.get_orbits_around(&node);
            constellation.orbits_around.insert(node.clone(), count);
        }
        constellation
    }

    fn get_total_orbits(&self) -> i64 {
        self.orbits_around.values().sum()
    }

    fn get_path(&self, node: &str) -> Vec<String> {
        let mut path = vec![];
        let mut current = node.to_owned();
        while let Some(center) = self.map_invert.get(&current) {
            path.push(center.clone());
            current = center.clone();
        }

        path
    }

    fn get_orbits_around(&mut self, node: &String) -> i64 {
        if self.orbits_around.contains_key(node) {
            //println!("Cache hit for node {}", node);
            return *self.orbits_around.get(node).unwrap();
        }
        let mut total = 0;
        if let Some(satellites) = self.map.get(node).cloned() {
            for satellite in satellites.iter() {
                total += 1 + self.get_orbits_around(satellite);
            }
        }
        self.orbits_around.insert(node.clone(), total);
        total
    }

    fn get_part2(&self) -> i64 {
        // These will return empty vectors if YOU or SAN are not present
        let path_to_you = self.get_path("YOU");
        let path_to_san = self.get_path("SAN");
        if path_to_you.is_empty() || path_to_san.is_empty() {
            return 0;
        }
        let first_common = path_to_you
            .iter()
            .find(|item| path_to_san.contains(item))
            .unwrap();
        let ind_in_you = path_to_you
            .iter()
            .position(|item| item == first_common)
            .unwrap();
        let ind_in_san = path_to_san
            .iter()
            .position(|item| item == first_common)
            .unwrap();
        (ind_in_you + ind_in_san) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let mut constellation = Constellation::new(&a);
        assert_eq!(constellation.get_orbits_around(&"L".to_string()), 0);
        assert_eq!(constellation.get_orbits_around(&"K".to_string()), 1);
        assert_eq!(constellation.get_orbits_around(&"J".to_string()), 2);
        assert_eq!(constellation.get_orbits_around(&"F".to_string()), 0);
        assert_eq!(constellation.get_orbits_around(&"E".to_string()), 4);
        assert_eq!(read_contents(a).0, 42);
    }

    #[test]
    fn part2() {
        let a = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        assert_eq!(read_contents(a).1, 4);
    }
}
