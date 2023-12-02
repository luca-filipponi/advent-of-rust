// doing first exercise of advent of code 2022

use std::path::Path;

use crate::helpers::read_input;

#[allow(dead_code)]
struct State {
    current_total: i64,
    calories_per_elf: Vec<i64>
}
#[allow(dead_code)]
pub fn calculate_most_calories_2022_1() {
    let calories = read_input(Path::new("src/resources/warmup_input.txt"))
        .expect("coudn't read input");

    // i need to split the vector in sub vector every time there is an empty element

    // what i'm going to do is to fold using a state which represent the current count
    // which is going to be added to the
    let state = State {
        current_total: 0,
        calories_per_elf: vec![],
    };

    let result = calories.iter().fold(state, |mut s, elem| {
        if elem == "" {
            s.calories_per_elf.push(s.current_total);
            s.current_total = 0;
        } else {
            s.current_total += elem.parse::<i64>().unwrap();
        }
        s
    });

    let max_calories = result.calories_per_elf.iter().max().unwrap();

    print!("{:?}", max_calories);
}

