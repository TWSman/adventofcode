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
        match &self {
            Letter::X => Some(Letter::M),
            Letter::M => Some(Letter::A),
            Letter::A => Some(Letter::S),
            Letter::S => None,
            Letter::Other => None,
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

fn parseLetter(ltr: char) -> Letter {
    match ltr {
        'X' => Letter::X,
        'M' => Letter::M,
        'A' => Letter::A,
        'S' => Letter::S,
        other => {dbg!(&other); Letter::Other},
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

fn get_element<T>(grid: &Vec<Vec<T>>, i:usize, j: usize) -> Option<&T> {
    grid.get(i)?.get(j)
}

fn get_part1(cont: &str) -> (i64, i64) {
    let grid = cont.lines().map(|ln| {
        ln.chars().map(parseLetter).collect::<Vec<Letter>>()
    }).collect::<Vec<Vec<Letter>>>();
    let x_vals = grid.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().filter_map(|(j, ltr)| {
            match ltr {
                Letter::X => Some((i as i64,j as i64)),
                _ => None,
            }
        }).collect::<Vec<(i64,i64)>>()
    }).flatten().collect::<Vec<(i64, i64)>>();
    dbg!(&x_vals);
    let part1 = x_vals.iter().map(|(i,j)| {
        dbg!(&(i,j));
        Directions::iter().map(|dir| {
            dbg!(&dir);
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
    let a_vals = grid.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().filter_map(|(j, ltr)| {
            match ltr {
                Letter::A => Some((i as i64,j as i64)),
                _ => None,
            }
        }).collect::<Vec<(i64,i64)>>()
    }).flatten().collect::<Vec<(i64, i64)>>();
    let part2 = a_vals.iter().map(|(i,j)| {
        let check_these = vec![(-1,-1), (-1,1), (1,1), (1, -1)];
        //let cross_vals = 
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
