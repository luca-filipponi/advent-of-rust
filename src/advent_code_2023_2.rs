use std::env::current_exe;
use std::path::Path;

use crate::helpers::read_input;

enum Color {
    Red,
    Green,
    Blue
}

#[derive(Debug)]
struct Game {
    id :i64,
    extractions: Vec<Extraction>
}
#[derive(Debug)]
struct MinSubset {
    red: i64,
    green: i64,
    blue: i64,
}

#[derive(Debug)]
struct Extraction {
    red: i64,
    green: i64,
    blue: i64,
}
pub fn possible_games_sum() {
    // every line represent random extraction of cubes. I don't think
    // i really need to care about the extractions in every game, but rather
    // just the max cube for every color that are extracted in every game
    // I have 12 red cubes, 13 green cubes, and 14 blue cubes
    // but probably is easier to just model this as a filter on every exrtaction
    let input = read_input(Path::new("src/resources/2023_2"))
        .expect("failed to read file");

    let games: Vec<Game> = input.iter().map(|g| to_game(g)).collect();

    // now i have all the games, can filter out the games where extractions won't match

    let count = games.iter().fold(0, |mut total, game| -> i64 {
        if game_valid(game) {
            print!("Valid game: {:?} \n", game);
            total += game.id
        } else {
            print!("Invalid game: {:?} \n", game);
        }
        total
    });

    print!("Part 2 solution: {:?}", count);

    // for every input line i just care about the extractions

    // i already have an array of games, can re-use it

    let sum_power_set = games.iter().fold(0, |mut total, game| -> i64 {
        let power_set = power_set(game);
        total += power_set.red * power_set.green * power_set.blue;
        total
    });
    //

    print!("Part 2.2 solution: {:?}", sum_power_set);
}

fn power_set(game: &Game) -> Extraction {
    let extractions = &game.extractions;
    let min_subset = Extraction {
        red: 0,
        green: 0,
        blue: 0,
    };
    extractions.into_iter().fold(min_subset, |mut state: Extraction, current| -> Extraction {
        state.red = state.red.max(current.red);
        state.green = state.green.max(current.green);
        state.blue = state.blue.max(current.blue);
        state
    })
}

fn game_valid(game: &Game) -> bool{
    let extractions = &game.extractions;
    extractions.into_iter().fold(true, |b, extraction| -> bool {
        // only 12 red cubes, 13 green cubes, and 14 blue cubes
        b && ( extraction.red <= 12 && extraction.blue <= 14 && extraction.green <= 13)
    })
}

// Input is like: Game 1: 7 blue, 6 green, 3 red; 3 red, 5 green, 1 blue; 1 red, 5 green, 8 blue; 3 red, 1 green, 5 blue
// Output will be
fn to_game(s: &String) ->  Game {
    let mut split_games_extractions = s.split(":");
    let game_string = split_games_extractions.next().unwrap();
    let game_id = game_string.split(" ").last()
        .unwrap().parse().unwrap();
    let extractions: Vec<Extraction> = split_games_extractions
        .last()
        .unwrap()
        .split(";")
        .map( |s| to_extraction(s)).collect();

    Game {
        extractions,
        id: game_id,
    }
}

// // input is the full line Game 1: 1 Red, 1 blue; 1 green;
// fn to_subset(s: &String) ->  MinSubset {
//     let mut split_games_extractions = s.split(":");
//     // the id is not important
//     // let game_string = split_games_extractions.next().unwrap();
//     // let game_id = game_string.split(" ").last()
//     //     .unwrap().parse().unwrap();
//
//
//     let extractions= split_games_extractions
//         .last()
//         .unwrap()
//         .split(";") // 1 Red, 1 blue; 1 green;
//         .map( |s| create_subset(s)).collect();
//     extractions
// }
// 11 red, 2 blue, 5 green, 5 green
fn to_extraction(s: &str) -> Extraction {
    // good case of folding over state, can split by "," and then accumulate and
    // count all colors

    let state = Extraction {
        red: 0,
        green: 0,
        blue: 0,
    };
    s.split(",").fold( state, |mut state, entry| -> Extraction {
        // an entry is: 15 green
        // print!("Entry: {} \n", entry);
        let amount_and_color: Vec<String> = entry
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        let amount: i64 = amount_and_color.get(0).unwrap().parse().unwrap();
        let color: Color = string_to_color(amount_and_color.last().unwrap());
        match color {
            Color::Red => state.red += amount,
            Color::Green => state.green += amount,
            Color::Blue => state.blue += amount,
        }
        state
    })
}

fn create_subset(extraction: &str) -> MinSubset {
    // good case of folding over state, can split by "," and then accumulate and
    // count all colors

    let state = MinSubset {
        red: 0,
        green: 0,
        blue: 0,
    };
    extraction.split(",").fold(state, |mut state, entry| -> MinSubset {
        // an entry is: 15 green
        // print!("Entry: {} \n", entry);
        let amount_and_color: Vec<String> = entry
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        let amount: i64 = amount_and_color.get(0).unwrap().parse().unwrap();
        let color: Color = string_to_color(amount_and_color.last().unwrap());

        match color {
            Color::Red => if amount > state.red { state.red = amount},
            Color::Green => if amount > state.green { state.green = amount},
            Color::Blue => if amount > state.green { state.green = amount},
        }
        state
    })
}


fn string_to_color(s: &str) -> Color {
    match s.to_lowercase().as_str() {
        "red" => Color::Red,
        "blue" => Color::Blue,
        "green" => Color::Green,
         &_ => todo!(), // how to handle this properly?
    }
}
#[cfg(test)]
mod tests {
    // Import necessary items from the code being tested
    use super::*;

    // Test function with the #[test] attribute
    #[test]
    fn test_possible_game_sum() {
        // Arrange
        let result = possible_games_sum();
    }
}