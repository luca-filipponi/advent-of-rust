#![allow(dead_code)]
use std::collections::HashSet;
use std::path::Path;

use crate::helpers::read_input;

#[derive(Debug)]
struct ScratchCard {
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
            winning_num: parsed_winner,
            actual_num: actual_num,
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