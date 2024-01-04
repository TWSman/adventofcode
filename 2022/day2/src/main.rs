#[macro_use]
extern crate num_derive;

use std::env;
use std::fs;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, PartialEq, FromPrimitive)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, PartialEq)]
enum RpsResult {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, Clone)]
struct Game {
    them: Rps,
    our: Rps,
}

#[derive(Debug, Clone)]
struct Game2 {
    them: Rps,
    result: RpsResult,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // let filename: &str = "input.txt";
    read_file(filename);
}


fn get_game(ln: &str) -> Option<Game> {
    let spl = ln.split(' ').collect::<Vec<&str>>();
    let them = match spl[0] {
        "A" => Rps::Rock,
        "B" => Rps::Paper,
        "C" => Rps::Scissors,
        _ => return Option::None
    };
    let our = match spl[1] {
        "X" => Rps::Rock,
        "Y" => Rps::Paper,
        "Z" => Rps::Scissors,
        _ => return Option::None
    };
    Some(Game {them, our})
}

fn get_game_alt(ln: &str) -> Option<Game2> {
    let spl = ln.split(' ').collect::<Vec<&str>>();
    let them = match spl[0] {
        "A" => Rps::Rock,
        "B" => Rps::Paper,
        "C" => Rps::Scissors,
        _ => return Option::None
    };
    let result = match spl[1] {
        "X" => RpsResult::Lose,
        "Y" => RpsResult::Draw,
        "Z" => RpsResult::Win,
        _ => return Option::None
    };
    Some(Game2 {them, result})
}

// Win: 6
// Draw: 3
// Lose: 0
// Rock: 1
// Paper: 2
// Scissors: 3
//

fn get_score(played: Rps) -> i32 {
    match played {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    }
}

fn win(d: Rps) -> Rps {
    FromPrimitive::from_u8((d as u8 + 1) % 3).unwrap()
}

fn lose(d: Rps) -> Rps {
    FromPrimitive::from_u8((d as u8 + 2) % 3).unwrap()
}

fn read_file(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    let lines = contents.split('\n');

    let mut total_score = 0;
    let mut total_alt_score = 0;
    for ln in lines {
        dbg!(&ln);
        if ln.is_empty() {
            break;
        }
        let game = get_game(ln).unwrap();
        let score = match game {
            //We win,
            Game {them: Rps::Rock, our: Rps::Paper } => get_score(Rps::Paper) + 6,
            Game {them: Rps::Paper, our: Rps::Scissors } => get_score(Rps::Scissors) + 6,
            Game {them: Rps::Scissors, our: Rps::Rock } => get_score(Rps::Rock) + 6,
            // It's a draw
            Game {them: x, our: y } if x == y => get_score(y) + 3,
            // We lose
            game => get_score(game.our),
        };
        total_score += score;

        let game_alt = get_game_alt(ln).unwrap();
        dbg!(&game_alt);
        let alt_score = match game_alt {
            //Draw,
            Game2 {them: x, result: RpsResult::Draw } => get_score(x) + 3,
            Game2 {them: x, result: RpsResult::Win } => get_score(win(x)) + 6,
            Game2 {them: x, result: RpsResult::Lose } => get_score(lose(x)),
        };
        total_alt_score += alt_score;
        dbg!(&alt_score);
    }
    println!("We got score {total_score}");
    println!("We got alt score {total_alt_score}");
}
