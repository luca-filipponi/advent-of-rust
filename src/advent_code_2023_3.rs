use std::path::Path;

use crate::helpers::read_input;

pub fn engine_part() {
    let input = read_input(Path::new("src/resources/2023_3"))
        .expect("failed to read file");

    // convert to an array of char
    let mut engine_matrix: Vec<Vec<char>> = input
        .iter()
        .map(|s| s.chars().collect()).collect();

    let mut numbers: Vec<i64> = vec![];
    let mut all_numbers: Vec<i64> = vec![];

    for i in 0..engine_matrix.len() {
        // when we find a number, want to skip till the end so we
        // don't process it more than once
        let mut j = 0;
        while j < engine_matrix[i].len(){
            // Access each element along with i and j indices
            let element = engine_matrix[i][j];
            // println!("Matrix[{}][{}] = {}", i, j, element);

            // Modify the value based on some condition
            // (This is just an example; you can customize it based on your needs)
            if element.is_numeric() {
                // need to find the start and end of this number in the row

                // find the start of number backward
                let mut start = j;
                for k in j..0 {
                    if engine_matrix[i][k].is_numeric() {
                        start = k;
                    } else {
                        break
                    }
                }

                // find the start of number backward
                let mut end = j;
                for k in j..engine_matrix[i].len() {
                    if engine_matrix[i][k].is_numeric() {
                        end = k;
                    } else {
                        break
                    }
                }

                print!("number start at {} end at {} \n", start, end);

                let number: String = engine_matrix[i][start..=end].iter().collect();

                print!("number {} \n", number);

                all_numbers.push(number.parse().unwrap());

                for k in start..=end {
                    let mut checks: Vec<(i64, i64)> = vec![];
                    // check left-right-up-down-diagonal to see if there is symbol
                    //start and end are valid, but the new indexes might not

                    let up = (i as i64 - 1, k as i64);
                    let diagonal_up_right = (i as i64 - 1, k as i64+ 1);
                    let left = (i as i64, k as i64- 1);
                    let diagonal_up_left = (i as i64 - 1, k as i64- 1);
                    let diagonal_down_left = (i as i64 + 1, k as i64- 1);
                    let down = (i as i64 + 1, k as i64);
                    let right = (i as i64, k as i64+ 1);
                    let diagonal_down_right = (i as i64 + 1, k as i64+ 1);

                    checks.push(up);
                    checks.push(diagonal_up_right);
                    checks.push(left);
                    checks.push(diagonal_up_left);
                    checks.push(diagonal_down_left);
                    checks.push(down);
                    checks.push(right);
                    checks.push(diagonal_down_right);


                    let is_special = checks.iter().fold(false, |b, check| -> bool {
                        if within_bound(check.0, check.1, engine_matrix.len() -1) {
                            let to_check = engine_matrix[check.0 as usize][check.1 as usize];
                            return b || is_special_char(to_check);
                        }
                        return b;
                    });

                    if is_special {
                        numbers.push(number.parse::<i64>().unwrap());
                        // we don't need to check the rest
                        j = end +1;
                        break
                    }
                }
                j = end +1;
            }
            j += 1;
            }
        }


    print!("{:?} \n\n\n", engine_matrix);
    print!("All numbers: {:?} \n\n\n", all_numbers);
    print!("Special: {:?} \n\n", numbers);

    let total: i64 = numbers.iter().sum();
    print!("Solution: {:?} \n\n", total);

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
        engine_part();
    }
}