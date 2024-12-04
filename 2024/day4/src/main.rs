use clap::Parser;
use std::fs;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

#[derive(Debug, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
    Other,
}

impl Letter {
    fn expect_next(&self) -> Option<Letter> {
        match self {
            Letter::X => Some(Letter::M),
            Letter::M => Some(Letter::A),
            Letter::A => Some(Letter::S),
            Letter::S => None,
            Letter::Other => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Letter::X => 'X',
            Letter::M => 'M',
            Letter::A => 'A',
            Letter::S => 'S',
            Letter::Other => '_',
        }
    }
}

#[derive(Debug, EnumIter)]
enum Directions {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

impl Directions {
    fn get_dir(&self) -> (i64, i64) {
        match self {
            Directions::N => (-1, 0),
            Directions::NE => (-1, 1),
            Directions::E => (0, 1),
            Directions::SE => (1, 1),
            Directions::S => (1, 0),
            Directions::SW => (1, -1),
            Directions::W => (0, -1),
            Directions::NW => (-1, -1),
        }
    }
}

fn parse_letter(ltr: char) -> Letter {
    match ltr {
        'X' => Letter::X,
        'M' => Letter::M,
        'A' => Letter::A,
        'S' => Letter::S,
        _ => {Letter::Other},
    }
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let (part1, part2) = get_part1(&contents);
    println!("Part 1 answer is {}", part1);  
    println!("Part 2 answer is {}", part2); 
}

fn get_element<T>(grid: &[Vec<T>], i:usize, j: usize) -> Option<&T> {
    grid.get(i)?.get(j)
}

fn get_part1(cont: &str) -> (i64, i64) {
    let grid = cont.lines().map(|ln| {
        ln.chars().map(parse_letter).collect::<Vec<Letter>>()
    }).collect::<Vec<Vec<Letter>>>();
    let x_vals = grid.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter_map(|(j, ltr)| {
            match ltr {
                Letter::X => Some((i as i64,j as i64)),
                _ => None,
            }
        }).collect::<Vec<(i64,i64)>>()
    }).collect::<Vec<(i64, i64)>>();
    let part1 = x_vals.iter().map(|(i,j)| {
        Directions::iter().map(|dir| {
            let add = dir.get_dir();
            let mut coord = (*i,*j);
            let mut expect_next = Letter::X.expect_next();
            while expect_next.is_some() {
                coord = (coord.0 + add.0, coord.1 + add.1);
                if (coord.0 < 0) | (coord.1 < 0) {
                    return 0
                }
                let res: Option<&_> = get_element(&grid, coord.0 as usize, coord.1 as usize);
                if res.is_some() {
                    if res.unwrap() != &expect_next.unwrap() {
                        return 0
                    } else {
                        expect_next = res.unwrap().expect_next();
                    }
                } else {
                    return 0
                }
            }
            1
        }).sum::<i64>()
    }).sum();
    let a_vals = grid.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter_map(|(j, ltr)| {
            match ltr {
                Letter::A => Some((i as i64,j as i64)),
                _ => None,
            }
        }).collect::<Vec<(i64,i64)>>()
    }).collect::<Vec<(i64, i64)>>();
    let part2 = a_vals.iter().map(|(i,j)| {
        // NE, NW, SE, SW
        let check_these = [(-1,-1), (-1,1), (1,1), (1, -1)];
        let str = check_these.iter().filter_map(|(i_add, j_add)| {
            let i_new = i + i_add;
            let j_new = j + j_add;
            Some(get_element(&grid, i_new as usize, j_new as usize)?.to_char())

        }).collect::<String>();
        //  let str = letters.iter().map(|ltr| ltr.to_char()).collect::<String>();
        match str.as_str() {
            "MMSS" => 1, // M's at top, S at bottom
            "MSSM" => 1, // M's at left, S at right
            "SMMS" => 1, // M's at right, S at left
            "SSMM" => 1, // M's at bottom, S at top
            _ => 0,
        }
    }).sum();
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let a = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(get_part1(&a).0, 18);
    }

    #[test]
    fn part2() {
        let a = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(get_part1(&a).1, 9);
    }

}
