#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

use crate::helpers::read_input;

#[derive(Debug)]
struct ScratchCard {
    card_id: i64,
    winning_num: HashSet<i64>,
    actual_num: HashSet<i64>,

}

pub fn scratch_cards() {
    let input = read_input(Path::new("src/resources/2023_4"))
        .expect("failed to read file");

    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let scratch_cards: Vec<ScratchCard> = input.iter().map(|s| {
        let winners_and_extractions: Vec<&str> = s.split("|").collect();


        // Card 1: 41 48 83 86 17
        let winning_split: Vec<&str> = winners_and_extractions[0].split(":").collect();

        let card_and_id: Vec<&str> = winning_split[0].split(" ").filter(|s| !s.is_empty()).collect();

        let card_id = card_and_id[1];

        // 41 48 83 86 17
        let winning_num: Vec<&str> = winning_split[1].split(" ").collect();

        let actual_num: HashSet<i64> = winners_and_extractions[1]
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let parsed_winner: HashSet<i64> = winning_num
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        ScratchCard {
            card_id: card_id.parse::<i64>().unwrap(),
            winning_num: parsed_winner,
            actual_num,
        }
    }).collect();


    let total_point = scratch_cards.iter().fold(0, |total, card| -> i64 {
        let actual_winners_count = card.winning_num.intersection(&card.actual_num).count();
        match actual_winners_count {
            0 => total,
            1 => total + 1,
            x =>  total + i64::pow(2, (x - 1) as u32)
        }
    });

    println!("{:?}", scratch_cards);
    println!("{:?}", total_point);


    println!("Part 2 ---- {:?}", total_point);
    let start = Instant::now();


    let map: HashMap<_, _> = scratch_cards.into_iter().fold(HashMap::new(), |mut acc, scratch_card| {

        // now i now that i am on card X and have Y winners. This card itself could have
        // more than one occurrence.
        let winner_count = scratch_card.winning_num.intersection(&scratch_card.actual_num).count();
        let card_id = scratch_card.card_id;

        // the multiplier represent how many repeated card i already have of this one
        let multiplier: i64 = match acc.get(&card_id) {
            None => 1,
            Some(y) => *y +1
        };
        // println!("Multiplier {} for {}: {:?}", multiplier, card_id, acc);

        // add the winner for current id
        * acc.entry(card_id).or_insert(0) += 1;

        for _ in 0..multiplier {
            for i in 1..=winner_count {
                let next_card_id = card_id + i as i64;
                * acc.entry(next_card_id).or_insert(0) += 1;
            }
        }
        acc
    });

    let total: i64 = map.values().sum();
    println!("Part 4.1: {:?}, took {:?}", total, start.elapsed());


}

fn within_bound(i: i64, j: i64, end: usize) -> bool{
    return i >= 0 && i < end as i64 && j >= 0 && j < end as i64;
}


fn is_special_char(element: char) -> bool {
    !element.is_numeric() && element != '.'
}


#[cfg(test)]
mod tests {
    // Import necessary items from the code being tested
    use super::*;

    // Test function with the #[test] attribute
    #[test]
    fn test_engine_part() {
        // Arrange
        scratch_cards();
    }
}