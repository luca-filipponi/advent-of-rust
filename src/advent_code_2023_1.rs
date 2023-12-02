use std::path::Path;
use crate::helpers::read_input;

pub fn calibration_value_sum() {
    let calibration_values_encrypted = read_input(Path::new("src/resources/2023_1"))
        .expect("failed to read file");

    let only_digit: Vec<String> = calibration_values_encrypted
        .iter()
        .map(|s| s.chars().filter(|&c| c.is_numeric()).collect()).collect();

    let first_and_last: Vec<String> = only_digit.iter()
        .map(|digits| {
            let first = digits.chars().next().unwrap();
            let last = digits.chars().last().unwrap();
            format!("{}{}", first, last)
        })
        .collect();

    let numbers: Vec<i64> = first_and_last.iter()
        .map(|digits| digits.parse::<i64>().unwrap())
        .collect();

    let total: i64 = numbers.iter().sum();

    print!("Part 1 solution: {:?} \n", total);

    // part two, the original input has string representing numbers
    // the easiest thing to do is probably to just convert the initial string by replacing
    // "one" -> "1" and then reuse what is already done

    let part_2_corrected_input: Vec<String> = calibration_values_encrypted
        .iter()
        .map(|s| replace_with_string_digit(s)).collect();

    let only_digit: Vec<String> = part_2_corrected_input
        .iter()
        .map(|s| s.chars().filter(|&c| c.is_numeric()).collect()).collect();

    let first_and_last: Vec<String> = only_digit.iter()
        .map(|digits| {
            let first = digits.chars().next().unwrap();
            let last = digits.chars().last().unwrap();
            format!("{}{}", first, last)
        })
        .collect();

    let numbers: Vec<i64> = first_and_last.iter()
        .map(|digits| digits.parse::<i64>().unwrap())
        .collect();

    let total: i64 = numbers.iter().sum();

    print!("Part 2 solution: {:?} \n", total);

}

fn replace_with_string_digit(s: &String) -> String {
    //there is an extra annoying case to handle which is when the last letter of the string
    // digit can be the start of the new one, like oneeight. This can go one forever
    // eightwoneightwo....
    // Since i'm doing this in order, i can replace but leave letter that can be used from
    // other combination for one, two, tree and five. No need for the eight because can
    // only be used from already replaced numbers
    s
        .replace("one", "o1e")
        .replace("two", "t2")
        .replace("three", "3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")



    // this is more complicate than i thought, but i can trick the system by alwa
}
