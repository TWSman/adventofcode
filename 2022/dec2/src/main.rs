#[macro_use]
extern crate num_derive;

use std::env;
use std::fs;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, PartialEq, FromPrimitive)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, PartialEq)]
enum RPSResult {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, Clone)]
struct Game {
    them: RPS,
    our: RPS,
}

#[derive(Debug, Clone)]
struct Game2 {
    them: RPS,
    result: RPSResult,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // let filename: &str = "input.txt";
    read_file(filename);
}


fn get_game(ln: &str) -> Option<Game> {
    let spl = ln.split(" ").collect::<Vec<&str>>();
    let them = match spl[0] {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => return Option::None
    };
    let our = match spl[1] {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => return Option::None
    };
    Some(Game {them, our})
}

fn get_game_alt(ln: &str) -> Option<Game2> {
    let spl = ln.split(" ").collect::<Vec<&str>>();
    let them = match spl[0] {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => return Option::None
    };
    let result = match spl[1] {
        "X" => RPSResult::Lose,
        "Y" => RPSResult::Draw,
        "Z" => RPSResult::Win,
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

fn get_score(played: RPS) -> i32 {
    match played {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn win(d: RPS) -> RPS {
    FromPrimitive::from_u8((d as u8 + 1) % 3).unwrap()
}

fn lose(d: RPS) -> RPS {
    FromPrimitive::from_u8((d as u8 + 2) % 3).unwrap()
}

fn read_file(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    let lines = contents.split("\n");

    let mut total_score = 0;
    let mut total_alt_score = 0;
    for ln in lines {
        dbg!(&ln);
        if ln == "" {
            break;
        }
        let game = get_game(&ln).unwrap();
        let score = match game {
            //We win,
            Game {them: RPS::Rock, our: RPS::Paper } => get_score(RPS::Paper) + 6,
            Game {them: RPS::Paper, our: RPS::Scissors } => get_score(RPS::Scissors) + 6,
            Game {them: RPS::Scissors, our: RPS::Rock } => get_score(RPS::Rock) + 6,
            // It's a draw
            Game {them: x, our: y } if x == y => get_score(y) + 3,
            // We lose
            game => get_score(game.our),
        };
        total_score += score;

        let game_alt = get_game_alt(&ln).unwrap();
        dbg!(&game_alt);
        let alt_score = match game_alt {
            //Draw,
            Game2 {them: x, result: RPSResult::Draw } => get_score(x) + 3,
            Game2 {them: x, result: RPSResult::Win } => get_score(win(x)) + 6,
            Game2 {them: x, result: RPSResult::Lose } => get_score(lose(x)),
        };
        total_alt_score += alt_score;
        dbg!(&alt_score);
    }
    println!("We got score {total_score}");
    println!("We got alt score {total_alt_score}");
}
