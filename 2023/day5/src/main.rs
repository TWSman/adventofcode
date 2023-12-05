use clap::Parser;
use std::fs;
use std::collections::HashSet;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    println!("Hello, world!");
}

fn read_contents(cont: &str) -> (i32, i32) {

}

#[derive(Debug)]
Struct range {
    start: usize; 
    end: usiz;
}

//#[derive(Debug)]
//Struct mapping {
    //ranges
//}

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
        assert_eq!(read_contents(&a).0, 13);
        // assert_eq!(read_contents(&a).1, 30 );
    }
    #[test]
    // fn lines() {
    //     assert_eq!(read_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), (8,4));
    //     assert_eq!(read_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), (2,2));
    //     assert_eq!(read_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), (2,2));
    //     assert_eq!(read_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), (1,1));
    //     assert_eq!(read_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), (0,0));
    //     assert_eq!(read_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), (0,0));
    // }
}
