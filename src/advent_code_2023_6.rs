#![allow(dead_code)]

use std::path::Path;
use std::time::Instant;

use crate::helpers::read_input;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64
}

pub fn races() {
    let start = Instant::now();

    let input = read_input(Path::new("src/resources/2023_6"))
        .expect("failed to read file");

    let times: Vec<i64> = input[0]
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect();

    let distances: Vec<i64> = input[1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect();


    let races: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|t| Race{ time: *t.0, distance: *t.1 })
        .collect();

    println!("{:?}", races);

    let total = races.iter().fold(1, |total, game| -> i64 {
        let mut ways_to_win = 0;
        for i in 0..=game.time {
            let speed = i;
            let time = game.time-i;
            let distance = speed * time;

            if distance > game.distance {
                ways_to_win +=1;
            }
        }
        print!("{} \n", ways_to_win);
        total * ways_to_win
    });

    println!("{:?}", total);

    let duration = start.elapsed();

    println!("tookL {:?}", duration);

}


#[cfg(test)]
mod tests {
    // Import necessary items from the code being tested
    use super::*;

    // Test function with the #[test] attribute
    #[test]
    fn test_races() {
        // Arrange
        races();
    }
}