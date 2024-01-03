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

fn read_contents(cont: &str) -> (usize, usize) {
    let mut seeds: Vec<usize> = vec![];
    let mut map_seed_to_soil: Mapping = Mapping::new();
    let mut map_soil_to_fertilizer: Mapping = Mapping::new();
    let mut map_fertilizer_to_water: Mapping = Mapping::new();
    let mut map_water_to_light: Mapping = Mapping::new();
    let mut map_light_to_temperature: Mapping = Mapping::new();
    let mut map_temperature_to_humidity: Mapping = Mapping::new();
    let mut map_humidity_to_location: Mapping = Mapping::new();
    let mut current_mapping: &mut Mapping = &mut map_seed_to_soil;

    for ln in cont.lines() {
        match ln {
            _ if ln.starts_with("seeds") => {
                seeds = ln.split_whitespace().filter_map(|m| { match m.parse::<usize>()
                    { Ok(val) => Some(val),
                        Err(_) => None,
                    }
                }).collect();
                dbg!(&seeds);
            },
            _ if ln.starts_with("seed-to-soil map") => {
                current_mapping = &mut map_seed_to_soil; 
            },
            _ if ln.starts_with("soil-to-fertilizer map") => {
                current_mapping = &mut map_soil_to_fertilizer; 
            },
            _ if ln.starts_with("fertilizer-to-water map") => {
                current_mapping = &mut map_fertilizer_to_water; 
            },
            _ if ln.starts_with("water-to-light map")=>  {
                current_mapping = &mut map_water_to_light; 
            },
            _ if ln.starts_with("light-to-temperature map") => {
                current_mapping = &mut map_light_to_temperature; 
            },
            _ if ln.starts_with("temperature-to-humidity map") => {
                current_mapping = &mut map_temperature_to_humidity; 
            },
            _ if ln.starts_with("humidity-to-location map")=>  {
                current_mapping = &mut map_humidity_to_location; 
            } ,
            _ => {
                dbg!(&ln);
                let splits: Vec<usize> = ln.split_whitespace().map(|m| m.parse::<usize>().unwrap()).collect();
                if splits.is_empty() {
                    continue;
                }
                assert_eq!(splits.len(), 3);
                let dest = splits[0];
                let source = splits[1];
                let l = splits[2];
                let r = Range {start: source, end: source + l -1, offset: (dest as i64 - source as i64)};
                current_mapping.ranges.push(r);
            }
        }
    }

    let locations = seeds.iter().map(|i| {
        map_humidity_to_location.map(
            map_temperature_to_humidity.map(
                map_light_to_temperature.map(
                    map_water_to_light.map(
                        map_fertilizer_to_water.map(
                            map_soil_to_fertilizer.map(
                                map_seed_to_soil.map(*i)
                            )
                        )
                    )
                )
            )
        )
    });
    let min_loc = locations.min().unwrap();
    dbg!(min_loc);

    let mut seeds2: Vec<usize> = vec![];
    let tmp = seeds.clone();
    let pairs = tmp.chunks(2);
    for p in pairs {
        let start = p[0];
        let count = p[1];
        for i in 0..count {
            seeds2.push(start + i);
        }
    }

    let locations2 = seeds2.iter().map(|i| {
        map_humidity_to_location.map(
            map_temperature_to_humidity.map(
                map_light_to_temperature.map(
                    map_water_to_light.map(
                        map_fertilizer_to_water.map(
                            map_soil_to_fertilizer.map(
                                map_seed_to_soil.map(*i)
                            )
                        )
                    )
                )
            )
        )
    });
    let min_loc2 = locations2.min().unwrap();
    (min_loc, min_loc2)
}

    #[derive(Debug)]
    struct Range {
        start: usize,
        end: usize,
        offset: i64,
    }

    impl Range {
        fn includes(&self, ind: usize) -> bool {
            (ind >= self.start) & (ind <= self.end)
        }
    }


    #[derive(Debug)]
    struct Mapping {
        ranges: Vec<Range>,
    }

    impl Mapping {
        fn new() -> Mapping {
            Mapping {ranges: vec![]}
        }

        fn map(&self, ind: usize) -> usize {
            for r in &self.ranges {
                if r.includes(ind) {
                    return (ind as i64 + r.offset) as usize;
                }
            }
            ind
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        // First number: destination range start
        // Second number: Source Range start
        // Third number: Range length
        // Unmapped number correspond to 
        fn conts() {
            let a: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
            assert_eq!(read_contents(&a).0, 35);
            assert_eq!(read_contents(&a).1, 46);
        }
        #[test]
        fn range() {
            let r = Range {start: 2, end: 10, offset: 10};
            assert!(r.includes(5));
            assert!(r.includes(2));
            assert!(r.includes(10));
            assert!(!r.includes(11));
            assert!(!r.includes(1));
        }

        #[test]
        fn maps() {
            let r1 = Range {start: 2, end: 10, offset: 10};
            let r2 = Range {start: 22, end: 33, offset: 10};
            let map = Mapping {ranges: vec![r1, r2]};
            assert_eq!(map.map(2), 12);
            assert_eq!(map.map(11), 11);
        }
        // fn lines() {
        //     assert_eq!(read_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), (8,4));
        //     assert_eq!(read_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), (2,2));
        //     assert_eq!(read_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), (2,2));
        //     assert_eq!(read_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), (1,1));
        //     assert_eq!(read_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), (0,0));
        //     assert_eq!(read_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), (0,0));
        // }
    }
